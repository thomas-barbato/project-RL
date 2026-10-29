"""Publish unscaled engine captures; no source artwork is modified."""
from pathlib import Path
import argparse
import hashlib
import json
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]


def publish(folder: Path) -> None:
    folder = folder.resolve(strict=True)
    folder.relative_to(ROOT / "artifacts")
    data = json.loads((folder / "capture.json").read_text(encoding="utf-8"))
    width, height, count = data["width"], data["height"], data["frames"]
    raw = folder / "sequence.rgba"
    stride = width * height * 4
    if raw.stat().st_size != stride * count:
        raise ValueError("Incomplete engine sequence")

    def read_frame(source, number):
        source.seek(number * stride)
        image = Image.frombytes("RGBA", (width, height), source.read(stride))
        return image.transpose(Image.Transpose.FLIP_TOP_BOTTOM).convert("RGB")

    with raw.open("rb") as source:
        sample = Image.new("RGB", (width * 2, height * 2))
        for i, number in enumerate([0, 20, 34, 39]):
            sample.paste(read_frame(source, number), (i % 2 * width, i // 2 * height))
        palette = sample.quantize(colors=256, method=Image.Quantize.MEDIANCUT)
        frames = [read_frame(source, number).quantize(palette=palette, dither=Image.Dither.NONE)
                  for number in range(count)]
        frames[0].save(folder / "comparaison-native64.gif", save_all=True,
                       append_images=frames[1:], loop=0,
                       duration=1000 // data["fps"], optimize=False, disposal=1)
        idle = read_frame(source, 20)
        for i, (x, y) in enumerate(data["panel_origins"]):
            idle.crop((x, y, x + data["panel_width"], y + data["panel_height"])).save(
                folder / f"{i + 1:02}-{'reference' if i == 0 else 'proposition'}-native64.png")
        decoded = Image.open(folder / "comparaison-native64.gif")
        if decoded.n_frames != count:
            raise ValueError(f"Unexpected encoded frame count: {decoded.n_frames}")
        if decoded.size != (width, height):
            raise ValueError("Native geometry changed during publication")
        phases = Image.new("RGB", (896, 516), (9, 13, 18))
        labels = ImageDraw.Draw(phases)
        for i, number in enumerate([0, 5, 9, 13, 20, 27, 32, 34, 36, 39, 45, 65]):
            decoded.seek(number)
            x, y = data["panel_origins"][1]
            crop = decoded.convert("RGB").crop((x + 248, y + 208, x + 472, y + 352))
            left, top = i % 4 * 224, i // 4 * 172
            phases.paste(crop, (left, top + 24))
            labels.text((left + 7, top + 5), f"{number / data['fps']:.2f} s", fill=(217, 225, 232))
        phases.save(folder / "phase-check-native64.png")

    (folder / "apercu.html").write_text("""<!doctype html>
<html lang="fr"><meta charset="utf-8"><title>POLISH01 · Finition de la rue</title>
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>body{margin:0;background:#090d12;color:#d9e1e8;font:16px system-ui,sans-serif}
main{padding:24px;max-width:1360px;margin:auto}h1{font-size:25px;margin:0 0 12px}
p{color:#a5b3c0;max-width:85ch;line-height:1.6}button{padding:10px 16px;margin:0 8px 12px 0;
background:#192531;color:#e6edf3;border:1px solid #405368;border-radius:5px;cursor:pointer}
.viewport{overflow:auto;border:1px solid #2a3541}img{display:block;width:1360px;height:800px;
max-width:none}strong{color:#e0b774}a{color:#92cbe8}</style><main>
<h1>POLISH01 · Scène témoin à examiner</h1>
<p><strong>01 : référence contrôlée du rendu existant. 02 : finition proposée.</strong>
Les mêmes dessins, cases de 64 px et phases de marche sont utilisés dans les deux colonnes.
Le montage ne reproduit pas l'interface complète d'une partie.</p>
<button onclick="show('comparaison-native64.gif')">Marche et attaque</button>
<button onclick="show('comparison-20.png')">Arrêt</button>
<button onclick="show('comparison-34.png')">Contact</button>
<button onclick="show('reduced-motion.png')">Mouvement réduit</button>
<div class="viewport"><img id="comparison" src="comparaison-native64.gif" alt="Comparaison native de la rue"></div>
<p>Affichage à 100 %, sans redimensionnement des images. Sur petit écran, faire défiler horizontalement.
Les changements de lumière, de contours et d'impact restent des propositions sans activation dans le jeu normal.</p>
<p><a href="01-reference-native64.png">Scène 01 seule</a> ·
<a href="02-proposition-native64.png">Scène 02 seule</a></p>
<script>function show(file){document.getElementById('comparison').src=file}</script></main></html>
""", encoding="utf-8")
    outputs = [p for p in folder.iterdir() if p.is_file() and p != raw and p.name != "publication.json"]
    result = {"review": data["review"], "native_geometry_verified": True,
              "encoded_frames": count, "encoded_duration_ms": count * (1000 // data["fps"]),
              "temporary_bytes_removed": raw.stat().st_size,
              "outputs": [{"file": p.name, "bytes": p.stat().st_size,
                           "sha256": hashlib.sha256(p.read_bytes()).hexdigest()} for p in outputs]}
    (folder / "publication.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    # This raw stream was created only for this publisher and is not source art.
    raw.unlink()
    print(json.dumps({k: v for k, v in result.items() if k != "outputs"}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("folder", type=Path, nargs="?", default=ROOT / "artifacts/visual-polish-review-01")
    publish(parser.parse_args().folder)
