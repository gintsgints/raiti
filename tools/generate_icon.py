#!/usr/bin/env python3
"""Draws the Raiti app icon as a keycap and writes the macOS iconset.

Needs Pillow. The icon is a single keycap carrying the app's initial, with the
home row bump along its lower edge, drawn in the same Iosevka face and key
greys the app itself uses.

    python3 tools/generate_icon.py [--variant dark|light] [--out DIR]
"""

import argparse
import math
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parent.parent
FONT = ROOT / "fonts" / "iosevka-term-bold.ttf"

SIZE = 1024
SS = 4  # supersampling factor, downsampled at the end for clean edges
CANVAS = SIZE * SS

# macOS icon grid: the shape sits in 824 of the 1024 canvas.
CAP = int(824 * SS)
CORNER_N = 5.0  # superellipse exponent, close to the macOS squircle

VARIANTS = {
    "dark": {
        "cap_top": (58, 58, 62),
        "cap_bottom": (24, 24, 27),
        "face_top": (82, 82, 88),
        "face_bottom": (44, 44, 49),
        "letter": (242, 242, 242),
        "bump": (232, 163, 61),
    },
    "light": {
        "cap_top": (232, 232, 232),
        "cap_bottom": (182, 182, 182),
        "face_top": (250, 250, 250),
        "face_bottom": (209, 209, 209),
        "letter": (24, 24, 24),
        "bump": (120, 120, 120),
    },
}


def squircle(cx, cy, half, steps=1440):
    """Superellipse, the rounded square macOS uses for app icons."""
    points = []
    for step in range(steps):
        t = 2 * math.pi * step / steps
        cos_t, sin_t = math.cos(t), math.sin(t)
        x = math.copysign(abs(cos_t) ** (2 / CORNER_N), cos_t)
        y = math.copysign(abs(sin_t) ** (2 / CORNER_N), sin_t)
        points.append((cx + half * x, cy + half * y))
    return points


def vertical_gradient(size, top, bottom):
    gradient = Image.new("RGB", (1, size[1]))
    for y in range(size[1]):
        ratio = y / max(size[1] - 1, 1)
        gradient.putpixel(
            (0, y),
            tuple(round(t + (b - t) * ratio) for t, b in zip(top, bottom)),
        )
    return gradient.resize(size, Image.Resampling.BILINEAR)


def mask_of(points):
    mask = Image.new("L", (CANVAS, CANVAS), 0)
    ImageDraw.Draw(mask).polygon(points, fill=255)
    return mask


def draw(colors):
    icon = Image.new("RGBA", (CANVAS, CANVAS), (0, 0, 0, 0))
    centre = CANVAS / 2
    cap = squircle(centre, centre, CAP / 2)

    # Shadow, offset down so the cap looks like it sits on the surface.
    shadow = Image.new("RGBA", (CANVAS, CANVAS), (0, 0, 0, 0))
    ImageDraw.Draw(shadow).polygon(
        [(x, y + 26 * SS) for x, y in cap], fill=(0, 0, 0, 90)
    )
    icon.alpha_composite(shadow.filter(ImageFilter.GaussianBlur(20 * SS)))

    # Cap body.
    body = vertical_gradient((CANVAS, CANVAS), colors["cap_top"], colors["cap_bottom"])
    icon.paste(body, (0, 0), mask_of(cap))

    # Top face, inset and lifted, which is what reads as a key rather than a tile.
    face_half = CAP / 2 - 46 * SS
    face_centre_y = centre - 16 * SS
    face = squircle(centre, face_centre_y, face_half)
    face_fill = vertical_gradient(
        (CANVAS, CANVAS), colors["face_top"], colors["face_bottom"]
    )
    icon.paste(face_fill, (0, 0), mask_of(face))

    draw_on = ImageDraw.Draw(icon)

    # The initial, in the face used throughout the app.
    font = ImageFont.truetype(str(FONT), int(430 * SS))
    left, top, right, bottom = font.getbbox("R")
    draw_on.text(
        (centre - (left + right) / 2, face_centre_y - (top + bottom) / 2 - 26 * SS),
        "R",
        font=font,
        fill=colors["letter"],
    )

    # Home row bump, the ridge that tells a touch typist where the fingers go.
    bump_width, bump_height = 190 * SS, 26 * SS
    bump_y = face_centre_y + face_half - 104 * SS
    draw_on.rounded_rectangle(
        [
            centre - bump_width / 2,
            bump_y,
            centre + bump_width / 2,
            bump_y + bump_height,
        ],
        radius=bump_height / 2,
        fill=colors["bump"],
    )

    return icon.resize((SIZE, SIZE), Image.Resampling.LANCZOS)


def write_iconset(icon, iconset):
    iconset.mkdir(parents=True, exist_ok=True)
    for base in (16, 32, 128, 256, 512):
        for scale in (1, 2):
            pixels = base * scale
            suffix = "@2x" if scale == 2 else ""
            icon.resize((pixels, pixels), Image.Resampling.LANCZOS).save(
                iconset / f"icon_{base}x{base}{suffix}.png"
            )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--variant", choices=sorted(VARIANTS), default="dark")
    parser.add_argument("--out", type=Path, default=ROOT / "macos")
    args = parser.parse_args()

    icon = draw(VARIANTS[args.variant])
    args.out.mkdir(parents=True, exist_ok=True)
    icon.save(args.out / "icon.png")

    iconset = args.out / "raiti.iconset"
    write_iconset(icon, iconset)
    subprocess.run(
        ["iconutil", "-c", "icns", str(iconset), "-o", str(args.out / "raiti.icns")],
        check=True,
    )
    print(f"wrote {args.out / 'icon.png'} and {args.out / 'raiti.icns'}")


if __name__ == "__main__":
    main()
