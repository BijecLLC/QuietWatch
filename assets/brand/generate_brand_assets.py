#!/usr/bin/env python3
from pathlib import Path
import math

from PIL import Image, ImageDraw, ImageFont


OUT = Path(__file__).resolve().parent
SCALE = 3


def rgba(hex_color, alpha=255):
    hex_color = hex_color.lstrip("#")
    return tuple(int(hex_color[i : i + 2], 16) for i in (0, 2, 4)) + (alpha,)


def font(size, bold=False):
    candidates = [
        ("/System/Library/Fonts/Avenir Next.ttc", 0 if bold else 7),
        ("/System/Library/Fonts/HelveticaNeue.ttc", 1 if bold else 0),
        ("/System/Library/Fonts/SFNS.ttf", 0),
        ("/Library/Fonts/Arial Unicode.ttf", 0),
    ]
    for candidate, index in candidates:
        path = Path(candidate)
        if path.exists():
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


def draw_vertical_gradient(img, box, top, bottom, radius):
    x0, y0, x1, y1 = [int(v * SCALE) for v in box]
    radius = int(radius * SCALE)
    w, h = x1 - x0, y1 - y0
    grad = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    pixels = grad.load()
    for y in range(h):
        t = y / max(1, h - 1)
        mid = 0.58
        if t < mid:
            local = t / mid
            a, b = rgba(top), rgba("#132238")
        else:
            local = (t - mid) / (1 - mid)
            a, b = rgba("#132238"), rgba(bottom)
        color = tuple(int(a[i] + (b[i] - a[i]) * local) for i in range(4))
        for x in range(w):
            pixels[x, y] = color
    mask = Image.new("L", (w, h), 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, w, h), radius=radius, fill=255)
    img.paste(grad, (x0, y0), mask)


def eye_points(cx, cy, width, height, samples=52):
    top = []
    bottom = []
    for i in range(samples + 1):
        t = i / samples
        x = cx - width / 2 + width * t
        y = cy - height * math.sin(math.pi * t)
        top.append((x * SCALE, y * SCALE))
        bottom.append((x * SCALE, (cy + height * math.sin(math.pi * t)) * SCALE))
    return top + list(reversed(bottom))


def draw_eye_mark(draw, cx, cy, width, eye_h, iris_r, pupil_r, play_scale):
    draw.polygon(eye_points(cx, cy, width, eye_h), fill=rgba("#e5f7ff", 245))
    draw.polygon(eye_points(cx, cy, width * 0.78, eye_h * 0.58), fill=rgba("#0d1b2d"))
    draw.ellipse(
        (
            (cx - iris_r) * SCALE,
            (cy - iris_r) * SCALE,
            (cx + iris_r) * SCALE,
            (cy + iris_r) * SCALE,
        ),
        fill=rgba("#22d3ee"),
    )
    draw.ellipse(
        (
            (cx - iris_r * 0.78) * SCALE,
            (cy - iris_r * 0.78) * SCALE,
            (cx + iris_r * 0.78) * SCALE,
            (cy + iris_r * 0.78) * SCALE,
        ),
        fill=rgba("#34d399", 205),
    )
    draw.ellipse(
        (
            (cx - pupil_r) * SCALE,
            (cy - pupil_r) * SCALE,
            (cx + pupil_r) * SCALE,
            (cy + pupil_r) * SCALE,
        ),
        fill=rgba("#08111d"),
    )
    p = play_scale
    draw.polygon(
        [
            ((cx - p * 0.35) * SCALE, (cy - p * 0.55) * SCALE),
            ((cx - p * 0.35) * SCALE, (cy + p * 0.55) * SCALE),
            ((cx + p * 0.55) * SCALE, cy * SCALE),
        ],
        fill=rgba("#f8fafc"),
    )


def draw_wave(draw, x, y, width, height, stroke):
    points = []
    for i in range(100):
        t = i / 99
        px = x + width * t
        amp = height * (0.28 + 0.72 * abs(math.sin(math.pi * t)))
        py = y + math.sin(t * math.pi * 6.0) * amp
        points.append((px * SCALE, py * SCALE))
    draw.line(points, fill=rgba("#67e8f9"), width=stroke * SCALE, joint="curve")
    draw.line([(x * SCALE, (y + height + 10) * SCALE), ((x + width) * SCALE, (y + height + 10) * SCALE)], fill=rgba("#a7f3d0", 185), width=max(2, stroke // 3) * SCALE)


def draw_mark(img, box):
    draw = ImageDraw.Draw(img)
    x, y, size = box
    draw_vertical_gradient(img, (x, y, x + size, y + size), "#0b1220", "#09111d", size * 0.24)
    cx, cy = x + size * 0.51, y + size * 0.49
    draw_eye_mark(draw, cx, cy, size * 0.78, size * 0.18, size * 0.16, size * 0.095, size * 0.15)
    draw_wave(draw, x + size * 0.18, y + size * 0.77, size * 0.65, size * 0.035, max(5, int(size * 0.041)))


def text(draw, xy, value, size, color, bold=False):
    draw.text((xy[0] * SCALE, xy[1] * SCALE), value, font=font(size * SCALE, bold=bold), fill=rgba(color))


def text_width(value, size, bold=False):
    bbox = ImageDraw.Draw(Image.new("RGBA", (1, 1))).textbbox((0, 0), value, font=font(size * SCALE, bold=bold))
    return (bbox[2] - bbox[0]) / SCALE


def save_mark():
    img = canvas(1024, 1024)
    draw_mark(img, (92, 92, 840))
    downsample(img).save(OUT / "quietwatch-mark.png")


def save_logo():
    img = canvas(1600, 480)
    draw = ImageDraw.Draw(img)
    draw_mark(img, (54, 66, 348))
    quiet_w = text_width("Quiet", 124, True)
    text(draw, (468, 136), "Quiet", 124, "#0f172a", True)
    text(draw, (468 + quiet_w - 6, 136), "Watch", 124, "#0891b2", True)
    text(draw, (473, 273), "comfortable loudness for movies and TV", 34, "#475569")
    draw.line([(475 * SCALE, 344 * SCALE), (1011 * SCALE, 344 * SCALE)], fill=rgba("#22d3ee"), width=7 * SCALE)
    draw.line([(1043 * SCALE, 344 * SCALE), (1169 * SCALE, 344 * SCALE)], fill=rgba("#34d399"), width=7 * SCALE)
    downsample(img).save(OUT / "quietwatch-logo.png")


def save_banner():
    img = canvas(1600, 520, rgba("#08111d"))
    draw = ImageDraw.Draw(img)
    for x in range(img.width):
        t = x / max(1, img.width - 1)
        color = (
            int(8 + 8 * math.sin(t * math.pi)),
            int(17 + 17 * math.sin(t * math.pi)),
            int(29 + 24 * math.sin(t * math.pi)),
            255,
        )
        draw.line([(x, 0), (x, img.height)], fill=color)
    draw_wave(draw, 1040, 350, 430, 34, 7)
    draw.line([(1040 * SCALE, 380 * SCALE), (1488 * SCALE, 380 * SCALE)], fill=rgba("#67e8f9", 135), width=8 * SCALE)
    draw_mark(img, (124, 92, 332))
    quiet_w = text_width("Quiet", 130, True)
    text(draw, (526, 128), "Quiet", 130, "#f8fafc", True)
    text(draw, (526 + quiet_w - 7, 128), "Watch", 130, "#67e8f9", True)
    text(draw, (533, 278), "Level movie and TV audio without riding the volume knob.", 36, "#cbd5e1")
    downsample(img).save(OUT / "quietwatch-readme-banner.png")


if __name__ == "__main__":
    save_mark()
    save_logo()
    save_banner()
