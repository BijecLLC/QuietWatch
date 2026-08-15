#!/usr/bin/env python3
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


OUT = Path(__file__).resolve().parent
SCALE = 3


def rgba(hex_color, alpha=255):
    hex_color = hex_color.lstrip("#")
    return tuple(int(hex_color[i : i + 2], 16) for i in (0, 2, 4)) + (alpha,)


def font(size, bold=False):
    candidates = [
        ("/System/Library/Fonts/Avenir Next.ttc", 8 if bold else 7),
        ("/System/Library/Fonts/HelveticaNeue.ttc", 1 if bold else 0),
        ("/System/Library/Fonts/SFNS.ttf", 0),
        ("/Library/Fonts/Arial Unicode.ttf", 0),
    ]
    for candidate, index in candidates:
        path = Path(candidate)
        if not path.exists():
            continue
        try:
            return ImageFont.truetype(str(path), size=size, index=index)
        except Exception:
            try:
                return ImageFont.truetype(str(path), size=size)
            except Exception:
                pass
    return ImageFont.load_default(size=size)


def canvas(width, height, bg=None):
    return Image.new("RGBA", (width * SCALE, height * SCALE), bg or (0, 0, 0, 0))


def downsample(img):
    return img.resize((img.width // SCALE, img.height // SCALE), Image.Resampling.LANCZOS)


def text_width(value, size, bold=False):
    draw = ImageDraw.Draw(Image.new("RGBA", (1, 1)))
    bbox = draw.textbbox((0, 0), value, font=font(size * SCALE, bold=bold))
    return (bbox[2] - bbox[0]) / SCALE


def text(draw, xy, value, size, color, bold=False):
    draw.text((xy[0] * SCALE, xy[1] * SCALE), value, font=font(size * SCALE, bold=bold), fill=rgba(color))


def vertical_gradient(img, top, bottom):
    draw = ImageDraw.Draw(img)
    for y in range(img.height):
        t = y / max(1, img.height - 1)
        a = rgba(top)
        b = rgba(bottom)
        color = tuple(int(a[i] + (b[i] - a[i]) * t) for i in range(4))
        draw.line([(0, y), (img.width, y)], fill=color)


def draw_mark(img, x, y, size):
    draw = ImageDraw.Draw(img)
    box = (x * SCALE, y * SCALE, (x + size) * SCALE, (y + size) * SCALE)
    draw.rounded_rectangle(box, radius=int(size * 0.22 * SCALE), fill=rgba("#0b1220"))
    inset = size * 0.075
    draw.rounded_rectangle(
        (
            (x + inset) * SCALE,
            (y + inset) * SCALE,
            (x + size - inset) * SCALE,
            (y + size - inset) * SCALE,
        ),
        radius=int(size * 0.17 * SCALE),
        outline=rgba("#1f3347"),
        width=max(2, int(size * 0.012 * SCALE)),
    )

    qw_font = font(int(size * 0.34 * SCALE), bold=True)
    q = "Q"
    w = "W"
    q_bbox = draw.textbbox((0, 0), q, font=qw_font)
    w_bbox = draw.textbbox((0, 0), w, font=qw_font)
    q_width = q_bbox[2] - q_bbox[0]
    w_width = w_bbox[2] - w_bbox[0]
    gap = int(size * 0.018 * SCALE)
    total = q_width + gap + w_width
    baseline_y = int((y + size * 0.265) * SCALE)
    start_x = int((x + size / 2) * SCALE - total / 2)
    draw.text((start_x, baseline_y), q, font=qw_font, fill=rgba("#f8fafc"))
    draw.text((start_x + q_width + gap, baseline_y), w, font=qw_font, fill=rgba("#67e8f9"))


def draw_wordmark(draw, x, y, size, quiet_color, watch_color):
    quiet = "Quiet"
    quiet_width = text_width(quiet, size, True)
    text(draw, (x, y), quiet, size, quiet_color, True)
    text(draw, (x + quiet_width - 4, y), "Watch", size, watch_color, True)


def save_mark():
    img = canvas(1024, 1024)
    draw_mark(img, 92, 92, 840)
    downsample(img).save(OUT / "quietwatch-mark.png")


def save_logo():
    img = canvas(1600, 480)
    draw = ImageDraw.Draw(img)
    draw_mark(img, 60, 72, 336)
    draw_wordmark(draw, 470, 136, 124, "#0f172a", "#0891b2")
    text(draw, (476, 276), "comfortable loudness for movies and TV", 34, "#475569")
    downsample(img).save(OUT / "quietwatch-logo.png")


def save_banner():
    img = canvas(1600, 520)
    vertical_gradient(img, "#08111d", "#102235")
    draw = ImageDraw.Draw(img)
    draw_mark(img, 132, 106, 308)
    draw_wordmark(draw, 526, 132, 128, "#f8fafc", "#67e8f9")
    text(draw, (533, 284), "Level movie and TV audio without riding the volume knob.", 36, "#cbd5e1")
    downsample(img).save(OUT / "quietwatch-readme-banner.png")


if __name__ == "__main__":
    save_mark()
    save_logo()
    save_banner()
