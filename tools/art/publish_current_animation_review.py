"""Replace the current animation preview without copying any image assets."""

import argparse
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
REVIEWS = ROOT / "artifacts/refonte-agent-2026-10-06"
CURRENT = REVIEWS / "apercu-animations.html"


def publish(review: int) -> Path:
    source = REVIEWS / f"skill-animation-review-{review:02d}" / "cote-a-cote.html"
    html = source.read_text(encoding="utf-8")
    if re.search(r"<base\b", html, re.IGNORECASE):
        raise ValueError(f"La galerie possède déjà une base de chemins : {source}")
    match = re.search(r"<html\b[^>]*>", html, re.IGNORECASE)
    if match is None:
        raise ValueError(f"Balise HTML introuvable : {source}")
    # Keep every relative image, script and GIF reference in its original folder.
    base = f'<base href="./{source.parent.name}/">'
    html = html[: match.end()] + base + html[match.end() :]
    CURRENT.write_text(html, encoding="utf-8")
    return CURRENT


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("review", type=int, help="Numéro du lot à afficher")
    args = parser.parse_args()
    print(publish(args.review))
