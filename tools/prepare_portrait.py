#!/usr/bin/env python3
"""Prepare small, metadata-free green portrait PNGs for LocketHUD POC."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
from typing import Iterable

from PIL import Image, ImageEnhance, ImageFilter, ImageOps


PROFILE_NAMES = ("natural-green", "quantized-8", "quantized-16", "dithered")


def parse_crop_aspect(value: str) -> tuple[int, int]:
    try:
        width_text, height_text = value.split(":", 1)
        width, height = int(width_text), int(height_text)
    except (ValueError, TypeError) as error:
        raise argparse.ArgumentTypeError("crop aspect must look like WIDTH:HEIGHT") from error
    if width <= 0 or height <= 0:
        raise argparse.ArgumentTypeError("crop aspect values must be positive")
    return width, height


def center_crop(image: Image.Image, aspect: tuple[int, int]) -> Image.Image:
    target_ratio = aspect[0] / aspect[1]
    source_ratio = image.width / image.height
    if math.isclose(source_ratio, target_ratio, rel_tol=1e-6):
        return image.copy()
    if source_ratio > target_ratio:
        new_width = max(1, round(image.height * target_ratio))
        left = (image.width - new_width) // 2
        return image.crop((left, 0, left + new_width, image.height))
    new_height = max(1, round(image.width / target_ratio))
    top = (image.height - new_height) // 2
    return image.crop((0, top, image.width, top + new_height))


def resize_to_fit(image: Image.Image, max_width: int, max_height: int | None) -> Image.Image:
    height_limit = max_height if max_height is not None else image.height
    scale = min(max_width / image.width, height_limit / image.height, 1.0)
    output_size = (max(1, round(image.width * scale)), max(1, round(image.height * scale)))
    return image.resize(output_size, Image.Resampling.LANCZOS)


def apply_gamma(gray: Image.Image, gamma: float) -> Image.Image:
    if math.isclose(gamma, 1.0):
        return gray
    table = [round(255 * ((value / 255) ** (1.0 / gamma))) for value in range(256)]
    return gray.point(table)


def quantize_gray(gray: Image.Image, levels: int) -> Image.Image:
    scale = levels - 1
    return gray.point(lambda value: round(value * scale / 255) * 255 // scale)


def floyd_steinberg(gray: Image.Image, levels: int) -> Image.Image:
    width, height = gray.size
    pixels = [float(value) for value in gray.getdata()]
    scale = levels - 1

    def add(x: int, y: int, error: float, weight: float) -> None:
        if 0 <= x < width and 0 <= y < height:
            index = y * width + x
            pixels[index] = min(255.0, max(0.0, pixels[index] + error * weight))

    for y in range(height):
        for x in range(width):
            index = y * width + x
            old_value = pixels[index]
            new_value = round(old_value * scale / 255) * 255 / scale
            pixels[index] = new_value
            error = old_value - new_value
            add(x + 1, y, error, 7 / 16)
            add(x - 1, y + 1, error, 3 / 16)
            add(x, y + 1, error, 5 / 16)
            add(x + 1, y + 1, error, 1 / 16)

    output = Image.new("L", (width, height))
    output.putdata([round(value) for value in pixels])
    return output


def green_rgba(gray: Image.Image, alpha: Image.Image) -> Image.Image:
    zero = Image.new("L", gray.size, 0)
    blue = gray.point(lambda value: round(value * 0.30))
    return Image.merge("RGBA", (zero, gray, blue, alpha))


def process_image(
    source: Image.Image,
    profile: str,
    max_width: int,
    max_height: int | None,
    crop_aspect: tuple[int, int] | None,
    gamma: float,
    contrast: float,
    sharpen: float,
) -> Image.Image:
    if profile not in PROFILE_NAMES:
        raise ValueError(f"unsupported profile: {profile}")
    oriented = ImageOps.exif_transpose(source).convert("RGBA")
    cropped = center_crop(oriented, crop_aspect) if crop_aspect else oriented.copy()
    resized = resize_to_fit(cropped, max_width, max_height)
    red, green, blue, alpha = resized.split()
    gray = Image.merge("RGB", (red, green, blue)).convert("L")
    gray = ImageEnhance.Contrast(gray).enhance(contrast)
    gray = apply_gamma(gray, gamma)
    if sharpen > 0:
        gray = gray.filter(ImageFilter.UnsharpMask(radius=1.0, percent=round(100 * sharpen), threshold=2))
    if profile == "quantized-8":
        gray = quantize_gray(gray, 8)
    elif profile == "quantized-16":
        gray = quantize_gray(gray, 16)
    elif profile == "dithered":
        gray = floyd_steinberg(gray, 8)
    return green_rgba(gray, alpha)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def run(
    input_path: Path,
    output_dir: Path,
    profiles: Iterable[str],
    max_width: int,
    max_height: int | None,
    crop_aspect: tuple[int, int] | None,
    gamma: float,
    contrast: float,
    sharpen: float,
) -> dict[str, object]:
    if not input_path.is_file():
        raise FileNotFoundError("input image does not exist")
    if input_path.suffix.lower() not in {".png", ".jpg", ".jpeg", ".webp"}:
        raise ValueError("input must be PNG, JPEG, or WebP")
    if max_width <= 0 or (max_height is not None and max_height <= 0):
        raise ValueError("maximum dimensions must be positive")
    if gamma <= 0 or contrast <= 0 or sharpen < 0:
        raise ValueError("gamma/contrast must be positive and sharpen cannot be negative")

    selected_profiles = tuple(dict.fromkeys(profiles))
    unknown = sorted(set(selected_profiles) - set(PROFILE_NAMES))
    if unknown:
        raise ValueError(f"unsupported profiles: {', '.join(unknown)}")
    if not selected_profiles:
        raise ValueError("at least one profile is required")

    output_dir.mkdir(parents=True, exist_ok=True)
    results: list[dict[str, object]] = []
    with Image.open(input_path) as source:
        for profile in selected_profiles:
            output_path = output_dir / f"portrait_{profile}.png"
            if output_path.resolve() == input_path.resolve():
                raise ValueError("output cannot overwrite the input image")
            image = process_image(
                source,
                profile,
                max_width,
                max_height,
                crop_aspect,
                gamma,
                contrast,
                sharpen,
            )
            image.save(output_path, format="PNG", optimize=True)
            results.append(
                {
                    "profile": profile,
                    "file": output_path.name,
                    "width": image.width,
                    "height": image.height,
                    "sha256": sha256(output_path),
                },
            )
            print(f"Wrote {output_path.name} ({image.width}x{image.height})")

    parameters = {
        "schema_version": 1,
        "input_sha256": sha256(input_path),
        "max_width": max_width,
        "max_height": max_height,
        "crop_aspect": list(crop_aspect) if crop_aspect else None,
        "gamma": gamma,
        "contrast": contrast,
        "sharpen": sharpen,
        "outputs": results,
    }
    parameters_path = output_dir / "processing.json"
    parameters_path.write_text(json.dumps(parameters, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("Wrote processing.json")
    return parameters


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--max-width", type=int, default=120)
    parser.add_argument("--max-height", type=int)
    parser.add_argument("--crop-aspect", type=parse_crop_aspect)
    parser.add_argument("--gamma", type=float, default=1.0)
    parser.add_argument("--contrast", type=float, default=1.0)
    parser.add_argument("--sharpen", type=float, default=0.35)
    parser.add_argument("--profiles", default=",".join(PROFILE_NAMES))
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    profiles = tuple(value.strip() for value in args.profiles.split(",") if value.strip())
    try:
        run(
            input_path=args.input,
            output_dir=args.output_dir,
            profiles=profiles,
            max_width=args.max_width,
            max_height=args.max_height,
            crop_aspect=args.crop_aspect,
            gamma=args.gamma,
            contrast=args.contrast,
            sharpen=args.sharpen,
        )
    except (FileNotFoundError, OSError, ValueError) as error:
        raise SystemExit(f"error: {error}") from error


if __name__ == "__main__":
    main()
