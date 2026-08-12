#!/usr/bin/env python3
"""Generate non-personal test portraits and calibration PNGs."""

from pathlib import Path

from PIL import Image, ImageDraw


ROOT = Path(__file__).resolve().parents[1]
GREEN = (0, 255, 90, 255)
DIM_GREEN = (0, 110, 38, 255)


def synthetic_portrait(transparent: bool) -> Image.Image:
    background = (0, 0, 0, 0) if transparent else (0, 0, 0, 255)
    image = Image.new("RGBA", (192, 320), background)
    draw = ImageDraw.Draw(image)
    draw.ellipse((50, 24, 142, 122), fill=(0, 120, 42, 230), outline=GREEN, width=4)
    draw.ellipse((70, 62, 79, 71), fill=GREEN)
    draw.ellipse((113, 62, 122, 71), fill=GREEN)
    draw.arc((76, 73, 116, 101), 15, 165, fill=GREEN, width=3)
    draw.rounded_rectangle((28, 132, 164, 302), radius=52, fill=(0, 90, 32, 220), outline=GREEN, width=4)
    draw.line((96, 132, 96, 294), fill=DIM_GREEN, width=2)
    return image


def alpha_test() -> Image.Image:
    image = Image.new("RGBA", (160, 100), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)
    for index, alpha in enumerate((32, 64, 96, 128, 160, 192, 224, 255)):
        left = index * 20
        draw.rectangle((left, 0, left + 19, 99), fill=(0, 255, 90, alpha))
    draw.ellipse((35, 20, 125, 80), fill=(0, 255, 90, 150), outline=GREEN, width=2)
    return image


def level_strip(levels: int) -> Image.Image:
    image = Image.new("RGBA", (levels * 16, 48), (0, 0, 0, 255))
    draw = ImageDraw.Draw(image)
    for index in range(levels):
        value = round(255 * index / (levels - 1))
        draw.rectangle((index * 16, 0, (index + 1) * 16 - 1, 47), fill=(0, value, round(value * 0.3), 255))
    return image


def main() -> None:
    resource_dir = ROOT / "glasses-app/src/main/res/drawable-nodpi"
    calibration_dir = ROOT / "test-assets/calibration"
    portrait_dir = ROOT / "test-assets/synthetic-portraits"
    for directory in (resource_dir, calibration_dir, portrait_dir):
        directory.mkdir(parents=True, exist_ok=True)

    transparent = synthetic_portrait(transparent=True)
    rectangular = synthetic_portrait(transparent=False)
    alpha = alpha_test()
    transparent.save(resource_dir / "portrait_default.png", format="PNG", optimize=True)
    alpha.save(resource_dir / "alpha_test.png", format="PNG", optimize=True)
    transparent.save(portrait_dir / "portrait_transparent.png", format="PNG", optimize=True)
    rectangular.save(portrait_dir / "portrait_rectangular.png", format="PNG", optimize=True)
    alpha.save(calibration_dir / "alpha_test.png", format="PNG", optimize=True)
    level_strip(8).save(calibration_dir / "green_levels_8.png", format="PNG", optimize=True)
    level_strip(16).save(calibration_dir / "green_levels_16.png", format="PNG", optimize=True)
    print("Generated calibration and synthetic portrait assets")


if __name__ == "__main__":
    main()
