#!/usr/bin/env python3

import argparse
import base64
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import zlib
from xml.etree import ElementTree
from xml.sax.saxutils import escape

ROOT = Path(__file__).resolve().parent
ASSETS = ROOT / "assets"
COLORS = {
    "blue": "#2347F5",
    "graphite": "#17262C",
    "chalk": "#F3F6F7",
    "steel": "#586A74",
    "line": "#CFD8DC",
    "pass": "#166443",
    "change": "#A23D27",
    "review": "#805907",
}
MARK = '<path d="M12 8H56V20H38V44H52V56H8V44H26V20H12Z"/>'
LETTERS = (
    '<path d="M4 0H18V14H4ZM4 24H18V80H4Z"/>'
    '<path d="M34 80V24H48V30C53 24 60 22 67 22C84 22 94 32 94 51V80H80V52C80 40 75 35 67 35C56 35 48 43 48 55V80Z"/>'
    '<path d="M104 24H120L134 63L148 24H164L142 80H126Z"/>'
    '<path fill-rule="evenodd" d="M218 24H232V80H218V74C213 79 207 82 199 82C181 82 170 69 170 52C170 35 181 22 199 22C207 22 213 25 218 30ZM201 35C190 35 184 42 184 52C184 62 190 69 201 69C212 69 218 62 218 52C218 42 212 35 201 35Z"/>'
    '<path d="M248 80V24H262V32C267 24 275 21 286 23V38C272 35 262 42 262 55V80Z"/>'
)


def svg(name, width, height, title, content):
    text = (
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}" role="img" aria-labelledby="title">\n'
        f'<title id="title">{title}</title>\n'
        '<metadata>Original Invar vector artwork. Invariant spine identity. '
        'Source: brand/make_assets.py. No generative imagery.</metadata>\n'
        f'{content}\n</svg>\n'
    )
    ElementTree.fromstring(text)
    (ASSETS / name).write_text(text, encoding="utf-8")


def lockup(mark_color, type_color):
    return (
        f'<g fill="{mark_color}" transform="translate(0 5) scale(1.2)">{MARK}</g>'
        f'<g fill="{type_color}" transform="translate(91 6) scale(.82)">{LETTERS}</g>'
    )


def export_png():
    browser = ["agent-browser", "--session", f"invar-brand-export-{os.getpid()}"]
    try:
        for name, width, height in (("icon", 512, 512), ("social-card", 1200, 630)):
            output = ASSETS / f"{name}.png"
            for arguments in (
                ["set", "viewport", str(width), str(height), "1"],
                ["open", (ASSETS / f"{name}.svg").as_uri()],
                ["eval", "document.fonts.ready.then(() => true)"],
                ["--screenshot-format", "png", "screenshot", str(output)],
            ):
                subprocess.run(browser + arguments, check=True, stdout=subprocess.DEVNULL, timeout=30)
            data = output.read_bytes()
            assert data[:8] == b"\x89PNG\r\n\x1a\n"
            assert struct.unpack(">II", data[16:24]) == (width, height)
            source = f"Source\0Original Invar vector artwork; source: brand/assets/{name}.svg; renderer: Chromium via agent-browser; no generative imagery.".encode("latin-1")
            chunk = struct.pack(">I", len(source)) + b"tEXt" + source + struct.pack(">I", zlib.crc32(b"tEXt" + source) & 0xFFFFFFFF)
            output.write_bytes(data[:33] + chunk + data[33:])
    finally:
        subprocess.run(browser + ["close"], check=True, stdout=subprocess.DEVNULL, timeout=30)
    print("Exported 2 PNGs with source metadata.")


def main():
    parser = argparse.ArgumentParser(description="Regenerate the Invar brand assets.")
    parser.add_argument("--png", action="store_true", help="Also render PNGs using agent-browser and Chromium.")
    args = parser.parse_args()
    ASSETS.mkdir(exist_ok=True)
    for name, mark, lettering in (
        ("logo-primary.svg", COLORS["blue"], COLORS["graphite"]),
        ("logo-mono.svg", COLORS["graphite"], COLORS["graphite"]),
        ("logo-reversed.svg", COLORS["chalk"], COLORS["chalk"]),
    ):
        svg(name, 340, 82, "Invar", lockup(mark, lettering))
    for name, color in (("mark.svg", COLORS["blue"]), ("mark-reversed.svg", COLORS["chalk"])):
        svg(name, 64, 64, "Invar invariant spine mark", f'<g fill="{color}">{MARK}</g>')
    svg("wordmark.svg", 294, 90, "Invar wordmark", f'<g fill="{COLORS["graphite"]}">{LETTERS}</g>')
    svg("icon.svg", 512, 512, "Invar icon", f'<path fill="{COLORS["blue"]}" d="M0 0H512V512H0Z"/><g fill="{COLORS["chalk"]}" transform="translate(64 64) scale(6)">{MARK}</g>')
    svg("favicon.svg", 32, 32, "Invar", f'<path fill="{COLORS["blue"]}" d="M0 0H32V32H0Z"/><g fill="{COLORS["chalk"]}" transform="translate(4 4) scale(.375)">{MARK}</g>')
    font = base64.b64encode((ROOT / "fonts/Barlow-Bold.ttf").read_bytes()).decode("ascii")
    font_license = "\n".join(line.rstrip() for line in (ROOT / "fonts/OFL.txt").read_text(encoding="utf-8").splitlines())
    social = (
        f'<metadata>{escape(font_license)}</metadata>'
        f'<defs><style>@font-face{{font-family:Barlow;src:url(data:font/ttf;base64,{font});font-weight:700}}'
        'text{font-family:Barlow,Arial,sans-serif;font-weight:700}</style></defs>'
        f'<path fill="{COLORS["chalk"]}" d="M0 0H1200V630H0Z"/>'
        f'<path fill="{COLORS["blue"]}" d="M888 0H1200V630H888Z"/>'
        f'<g transform="translate(57 48) scale(.64)">{lockup(COLORS["blue"], COLORS["graphite"])}</g>'
        f'<g fill="{COLORS["chalk"]}" transform="translate(917 188) scale(4)">{MARK}</g>'
        f'<text x="64" y="275" font-size="79" fill="{COLORS["graphite"]}">Change code.</text>'
        f'<text x="64" y="366" font-size="79" fill="{COLORS["blue"]}">Preserve behavior.</text>'
        f'<path stroke="{COLORS["line"]}" stroke-width="2" d="M64 455H824"/>'
        f'<text x="64" y="514" font-size="23" fill="{COLORS["graphite"]}">Capture. Review. Verify.</text>'
        f'<text x="64" y="552" font-size="18" fill="{COLORS["steel"]}">Local command verification for code changes.</text>'
    )
    svg("social-card.svg", 1200, 630, "Invar. Change code. Preserve behavior.", social)
    tokens = {
        "color": COLORS,
        "font": {"sans": "Barlow", "mono": "ui-monospace, SFMono-Regular, Consolas, monospace"},
        "font_weight": {"regular": 400, "bold": 700},
        "spacing_px": [4, 8, 12, 16, 24, 32, 48, 64, 96],
        "logo": {"minimum_lockup_width_px": 144, "minimum_mark_width_px": 16, "clear_space": "At least one stem width on every side; prefer two."},
    }
    (ROOT / "tokens.json").write_text(json.dumps(tokens, indent=2) + "\n", encoding="utf-8")
    declarations = [f"  --invar-{key}: {value};" for key, value in COLORS.items()]
    declarations += ['  --invar-font-sans: "Barlow", sans-serif;', f'  --invar-font-mono: {tokens["font"]["mono"]};']
    (ROOT / "tokens.css").write_text(":root {\n" + "\n".join(declarations) + "\n}\n", encoding="utf-8")
    manifest = {
        "identity": "Invariant spine",
        "artwork": "Original vector geometry. Logos contain paths, not font references.",
        "raster_exports": {"icon.png": [512, 512], "social-card.png": [1200, 630]},
        "font_source": "https://github.com/google/fonts/tree/main/ofl/barlow",
        "font_license": "fonts/OFL.txt",
        "fonts_sha256": {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((ROOT / "fonts").glob("*.ttf"))},
        "svg_assets": sorted(p.name for p in ASSETS.glob("*.svg")),
    }
    (ROOT / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    assert len(manifest["svg_assets"]) == 9
    assert all((ROOT / "fonts" / name).read_bytes()[:4] == b"\x00\x01\x00\x00" for name in manifest["fonts_sha256"])
    print("Generated and parsed 9 SVG assets, tokens, and font provenance.")
    if args.png:
        export_png()


if __name__ == "__main__":
    main()
