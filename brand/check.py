#!/usr/bin/env python3

import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import struct
from urllib.parse import unquote, urlsplit
from xml.etree import ElementTree
import zlib

ROOT = Path(__file__).resolve().parent


class References(HTMLParser):
    def __init__(self):
        super().__init__()
        self.links = []
        self.ids = set()

    def handle_starttag(self, tag, attrs):
        values = dict(attrs)
        if "id" in values:
            assert values["id"] not in self.ids, f"Duplicate HTML id: {values['id']}"
            self.ids.add(values["id"])
        self.links.extend(values[key] for key in ("href", "src") if key in values)


def luminance(color):
    rgb = [int(color[i:i + 2], 16) / 255 for i in (1, 3, 5)]
    channels = [value / 12.92 if value <= 0.04045 else ((value + 0.055) / 1.055) ** 2.4 for value in rgb]
    return sum(value * weight for value, weight in zip(channels, (0.2126, 0.7152, 0.0722)))


def main():
    parser = References()
    parser.feed((ROOT / "index.html").read_text(encoding="utf-8"))
    for link in parser.links:
        target = urlsplit(link)
        assert not target.scheme and not target.netloc, f"Preview has an external dependency: {link}"
        if target.path:
            assert (ROOT / unquote(target.path)).is_file(), f"Missing file: {link}"
        elif target.fragment:
            assert unquote(target.fragment) in parser.ids, f"Missing anchor: {link}"
    css = (ROOT / "style.css").read_text(encoding="utf-8")
    for source in re.findall(r'url\(["\']?([^"\')]+)', css):
        assert (ROOT / source).is_file(), f"Missing CSS resource: {source}"
    manifest = json.loads((ROOT / "manifest.json").read_text())
    for name in manifest["svg_assets"]:
        image = ElementTree.parse(ROOT / "assets" / name).getroot()
        assert image.tag == "{http://www.w3.org/2000/svg}svg"
        assert int(image.attrib["width"]) > 0 and int(image.attrib["height"]) > 0
        assert image.find("{http://www.w3.org/2000/svg}title") is not None
        if name == "social-card.svg":
            assert "SIL OPEN FONT LICENSE Version 1.1" in "".join(image.itertext())
        else:
            assert image.find(".//{http://www.w3.org/2000/svg}text") is None
    for name, digest in manifest["fonts_sha256"].items():
        assert hashlib.sha256((ROOT / "fonts" / name).read_bytes()).hexdigest() == digest
    assert "SIL OPEN FONT LICENSE Version 1.1" in (ROOT / "fonts/OFL.txt").read_text()
    colors = json.loads((ROOT / "tokens.json").read_text())["color"]
    for first, second in (("graphite", "chalk"), ("blue", "chalk"), ("steel", "chalk"), ("graphite", "line"), ("pass", "chalk"), ("change", "chalk"), ("review", "chalk")):
        low, high = sorted((luminance(colors[first]), luminance(colors[second])))
        ratio = (high + .05) / (low + .05)
        assert ratio >= 4.5, f"Insufficient contrast: {first}/{second} {ratio:.2f}:1"
        print(f"  {first}/{second}: {ratio:.2f}:1")
    for name, dimensions in manifest["raster_exports"].items():
        data = (ROOT / "assets" / name).read_bytes()
        assert data[:8] == b"\x89PNG\r\n\x1a\n", f"Invalid PNG: {name}"
        assert list(struct.unpack(">II", data[16:24])) == dimensions, f"Wrong dimensions: {name}"
        offset = 8
        source_found = False
        while offset < len(data):
            length = struct.unpack(">I", data[offset:offset + 4])[0]
            kind = data[offset + 4:offset + 8]
            payload = data[offset + 8:offset + 8 + length]
            crc = struct.unpack(">I", data[offset + 8 + length:offset + 12 + length])[0]
            assert zlib.crc32(kind + payload) & 0xFFFFFFFF == crc, f"Invalid PNG chunk: {name}"
            if kind == b"tEXt" and payload.startswith(b"Source\x00"):
                source_found = name.replace(".png", ".svg").encode() in payload
            offset += 12 + length
        assert source_found, f"Missing SVG provenance: {name}"
    print(f"PASS: {len(manifest['svg_assets'])} SVGs, 2 PNGs, local references, font hashes, and contrast.")


if __name__ == "__main__":
    main()
