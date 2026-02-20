#!/usr/bin/env python3
"""Generate ROYGBIV color variants of the Wizard app icon.

Takes the base green icon (src-tauri/icons/icon.png) and hue-shifts it
to produce 7 color variants. For each variant, runs `cargo tauri icon`
to generate all platform sizes.

Requirements: pip install Pillow
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path

try:
    from PIL import Image
    import numpy as np
except ImportError:
    print("Error: Pillow and numpy are required. Install with:")
    print("  pip install Pillow numpy")
    sys.exit(1)

# Project paths
REPO_ROOT = Path(__file__).resolve().parent.parent
SRC_TAURI = REPO_ROOT / "src-tauri"
BASE_ICON = SRC_TAURI / "icons" / "icon.png"
COLORS_DIR = SRC_TAURI / "icons" / "colors"

# Hue shifts from green (120deg) in degrees
# Green is baseline (0 shift), others are relative
COLOR_SHIFTS = {
    "red":    -120,
    "orange":  -90,
    "yellow":  -60,
    "green":     0,
    "blue":    120,
    "indigo":  155,
    "violet":  180,
}


def shift_hue(image: Image.Image, degrees: int) -> Image.Image:
    """Shift the hue of an RGBA image by the given degrees."""
    if degrees == 0:
        return image.copy()

    # Split into RGB and alpha
    if image.mode != "RGBA":
        image = image.convert("RGBA")
    r, g, b, a = image.split()

    # Convert RGB to HSV
    rgb = Image.merge("RGB", (r, g, b))
    hsv = rgb.convert("HSV")
    h, s, v = hsv.split()

    # Shift hue channel (0-255 maps to 0-360 degrees)
    h_array = np.array(h, dtype=np.int16)
    shift_amount = int(degrees * 255 / 360)
    h_array = (h_array + shift_amount) % 256
    h_shifted = Image.fromarray(h_array.astype(np.uint8), mode="L")

    # Recombine
    hsv_shifted = Image.merge("HSV", (h_shifted, s, v))
    rgb_shifted = hsv_shifted.convert("RGB")
    r2, g2, b2 = rgb_shifted.split()
    return Image.merge("RGBA", (r2, g2, b2, a))


def generate_color_variant(color: str, degrees: int) -> None:
    """Generate a single color variant and all its platform sizes."""
    color_dir = COLORS_DIR / color
    color_dir.mkdir(parents=True, exist_ok=True)

    print(f"  Generating {color} (shift: {degrees:+d} deg)...")

    # Load base icon and shift hue
    base = Image.open(BASE_ICON)
    shifted = shift_hue(base, degrees)

    # Save the 1024x1024 source
    source_icon = color_dir / "icon.png"
    shifted.save(source_icon, "PNG")

    # Run cargo tauri icon to generate all platform sizes
    # This creates 32x32.png, 128x128.png, 128x128@2x.png, icon.icns, icon.ico, etc.
    result = subprocess.run(
        ["cargo", "tauri", "icon", str(source_icon), "--output", str(color_dir)],
        cwd=str(SRC_TAURI),
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        print(f"    Warning: cargo tauri icon failed for {color}:")
        print(f"    {result.stderr.strip()}")
        # Fallback: generate key sizes manually with Pillow
        print(f"    Falling back to manual resize...")
        for size, name in [(32, "32x32.png"), (128, "128x128.png"), (256, "128x128@2x.png")]:
            resized = shifted.resize((size, size), Image.LANCZOS)
            resized.save(color_dir / name, "PNG")
    else:
        print(f"    OK")


def main() -> None:
    if not BASE_ICON.exists():
        print(f"Error: Base icon not found at {BASE_ICON}")
        sys.exit(1)

    print(f"Base icon: {BASE_ICON}")
    print(f"Output dir: {COLORS_DIR}")
    print()

    for color, degrees in COLOR_SHIFTS.items():
        generate_color_variant(color, degrees)

    print()
    print("Done! Generated icon variants:")
    for color in COLOR_SHIFTS:
        color_dir = COLORS_DIR / color
        files = sorted(f.name for f in color_dir.iterdir() if f.is_file())
        print(f"  {color}/: {', '.join(files[:5])}{'...' if len(files) > 5 else ''}")


if __name__ == "__main__":
    main()
