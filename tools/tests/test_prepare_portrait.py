import sys
import tempfile
import unittest
from pathlib import Path

from PIL import Image


TOOLS_DIR = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(TOOLS_DIR))

import prepare_portrait  # noqa: E402


class PreparePortraitTests(unittest.TestCase):
    def test_outputs_preserve_alpha_and_remove_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source_path = root / "source.png"
            output_dir = root / "output"
            source = Image.new("RGBA", (200, 400), (200, 150, 100, 128))
            source.save(source_path, pnginfo=None)

            result = prepare_portrait.run(
                input_path=source_path,
                output_dir=output_dir,
                profiles=prepare_portrait.PROFILE_NAMES,
                max_width=100,
                max_height=150,
                crop_aspect=None,
                gamma=1.0,
                contrast=1.0,
                sharpen=0.2,
            )

            self.assertEqual(4, len(result["outputs"]))
            for profile in prepare_portrait.PROFILE_NAMES:
                path = output_dir / f"portrait_{profile}.png"
                self.assertTrue(path.is_file())
                with Image.open(path) as output:
                    self.assertEqual("RGBA", output.mode)
                    self.assertEqual((75, 150), output.size)
                    self.assertEqual(128, output.getchannel("A").getextrema()[0])
                    self.assertNotIn("exif", output.info)
            self.assertTrue((output_dir / "processing.json").is_file())

    def test_center_crop_uses_requested_aspect(self) -> None:
        image = Image.new("RGBA", (400, 200), (0, 0, 0, 255))
        cropped = prepare_portrait.center_crop(image, (1, 1))
        self.assertEqual((200, 200), cropped.size)

    def test_invalid_profile_is_rejected(self) -> None:
        image = Image.new("RGBA", (20, 20), (0, 0, 0, 255))
        with self.assertRaises(ValueError):
            prepare_portrait.process_image(image, "cloud", 20, None, None, 1.0, 1.0, 0.0)


if __name__ == "__main__":
    unittest.main()
