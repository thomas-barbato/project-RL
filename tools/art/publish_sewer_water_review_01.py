"""Encode native engine frames and verify the water-only comparison."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageChops, ImageDraw, ImageOps

ROOT = Path(__file__).resolve().parents[2]
FOLDER = ROOT / "artifacts/sewer-water-review-01"


def publish():
    data = json.loads((FOLDER / "capture.json").read_text(encoding="utf-8"))
    canals = data["review"].startswith("EG-CAN02")
    width, height, count = data["width"], data["height"], data["frames"]
    stride = width * height * 4
    raw = FOLDER / "sequence.rgba"
    if raw.stat().st_size != stride * count:
        raise ValueError("Incomplete engine sequence")

    def frame(source, number):
        source.seek(number * stride)
        return Image.frombytes("RGBA", (width, height), source.read(stride)).transpose(
            Image.Transpose.FLIP_TOP_BOTTOM).convert("RGB")

    def panel(image, number):
        x, y = data["panel_origins"][number]
        return image.crop((x, y, x + data["panel_width"], y + data["panel_height"]))

    comparisons = []
    for index in range(1, 4):
        image = Image.open(FOLDER / f"variant-{index}-native64.png").convert("RGB")
        plan = json.loads((ROOT / f"assets/maps/prison-sewer-01/variant-{index}/plan.json").read_text())
        proposal = json.loads((ROOT / f"assets/source-art/sewer-water-02/variant-{index}-plan.json").read_text()) if canals else plan
        focus = ([13, 35] if index == 3 else [22, 19]) if canals else ([31, 17] if index == 2 else next(v["at"] for v in plan["views"] if v["id"] == "collector"))
        allowed = {(x, y) for y, row in enumerate(plan["layout"]) for x, c in enumerate(row)
                   if c == "~" or canals and proposal["layout"][y][x] != c}
        first = [focus[0] - 5, focus[1] - 3]
        before, after = panel(image, 0), panel(image, 1)
        changed = outside = 0
        for y in range(before.height):
            for x in range(before.width):
                if before.getpixel((x, y)) == after.getpixel((x, y)):
                    continue
                changed += 1
                wx, wy = first[0] + x // 64, first[1] + y // 64
                if (wx, wy) not in allowed:
                    outside += 1
        if not changed or outside:
            raise ValueError(f"Variant {index}: changed={changed}, outside water={outside}")
        comparisons.append({"variant": index, "changed_water_pixels": changed,
                            "changed_pixels_outside_water": outside, "focus": focus})
        if canals:
            world_before=Image.open(FOLDER/f"variant-{index}-before-world-native64.png").convert("RGB")
            world_after=Image.open(FOLDER/f"variant-{index}-proposed-world-native64.png").convert("RGB")
            expected=(plan["width"]*64,plan["height"]*64)
            if world_before.size!=expected or world_after.size!=expected:
                raise ValueError("Full map native size changed")
            mask=Image.new("L",expected,0)
            brush=ImageDraw.Draw(mask)
            for x,y in allowed:brush.rectangle((x*64,y*64,x*64+63,y*64+63),fill=255)
            delta=ImageChops.difference(world_before,world_after)
            outside=ImageChops.multiply(delta,ImageOps.invert(mask).convert("RGB"))
            if outside.getbbox() is not None:
                raise ValueError(f"Full map {index}: a pixel outside channel cells changed")
            comparisons[-1]["full_native_map_size"]=list(expected)
            comparisons[-1]["full_map_pixels_outside_channels_changed"]=0
            # Clearly labelled overview only; all art-review crops stay native.
            overview=Image.new("RGB",(1544,760),(9,13,18))
            label=ImageDraw.Draw(overview)
            label.text((16,12),f'EG-CAN02 - variante {index} - vue globale 1:4 (cases 16 px)',fill=(218,228,234))
            for col,(image,title) in enumerate([(world_before,"01 - anciens canaux"),(world_after,"02 - reseau relie")]):
                label.text((16+col*768,34),title,fill=(218,228,234))
                overview.paste(image.resize((752,704),Image.Resampling.LANCZOS),(16+col*768,56))
            overview.save(FOLDER/f"variant-{index}-overview.png")

    with raw.open("rb") as source:
        first, later = frame(source, 0), frame(source, 60)
        motion = ImageChops.difference(panel(first, 1), panel(later, 1)).tobytes()
        animated_pixels = sum(any(motion[i:i + 3]) for i in range(0, len(motion), 3))
        if animated_pixels == 0:
            raise ValueError("Proposed water does not animate")
        sample = Image.new("RGB", (width * 2, height * 2))
        for i, number in enumerate([0, 20, 40, 60]):
            sample.paste(frame(source, number), (i % 2 * width, i // 2 * height))
        palette = sample.quantize(colors=256, method=Image.Quantize.MEDIANCUT)
        frames = [frame(source, i).quantize(palette=palette, dither=Image.Dither.NONE) for i in range(count)]
        frames[0].save(FOLDER / "comparaison-native64.gif", save_all=True, append_images=frames[1:],
                       loop=0, duration=1000 // data["fps"], optimize=True, disposal=1)
        decoded = Image.open(FOLDER / "comparaison-native64.gif")
        duration = 0
        for i in range(decoded.n_frames):
            decoded.seek(i)
            duration += decoded.info.get("duration", 0)
        if decoded.size != (width, height) or duration != count * (1000 // data["fps"]):
            raise ValueError("Encoded size or duration changed")
        decoded.seek(0)
        encoded_first=panel(decoded.convert("RGB"),1)
        decoded.seek(60)
        encoded_later=panel(decoded.convert("RGB"),1)
        motion=ImageChops.difference(encoded_first,encoded_later).tobytes()
        encoded_changed=sum(any(motion[i:i+3]) for i in range(0,len(motion),3))
        if not encoded_changed:raise ValueError("Encoded water is static")
        (FOLDER/'encoded-motion.json').write_text(json.dumps({'encoded_water_different_pixels_between_0_and_3_seconds':encoded_changed,'temporary_rgba_stream_removed_after_publication':True},indent=2)+'\n')
        phases=Image.new("RGB",(640,608),(9,13,18))
        brush=ImageDraw.Draw(phases)
        for row,number in enumerate([0,20,40,60]):
            decoded.seek(number)
            phases.paste(panel(decoded.convert("RGB"),1).crop((0,128,640,256)),(0,row*152+24))
            brush.text((8,row*152+5),f'{number/data["fps"]:.0f} s',fill=(218,228,234))
        phases.save(FOLDER/'water-phases-native64.png')
        frame(source, 20).save(FOLDER / "comparaison-native64.png")
        for i in range(2):
            panel(frame(source, 20), i).save(FOLDER / f"{i + 1:02}-native64.png")

    html='''<!doctype html><html lang="fr"><meta charset="utf-8">
<title>EG-EAU01 · Égouts</title><meta name="viewport" content="width=device-width,initial-scale=1">
<style>body{margin:24px;background:#090d12;color:#dae4ea;font:16px system-ui}h1{font-size:24px}
p{max-width:90ch;line-height:1.6}.viewport{overflow:auto}img{width:1360px;height:672px;max-width:none;display:block}a{color:#94cad8}</style>
<h1>EG-EAU01 · Eau et berges des égouts</h1><p>01 : rendu actuel. 02 : courant lent, rides discrètes,
canaux raccordés et bord d'eau assombri. Même décor et éclairage. Cases de 64 px, sans agrandissement.
Les lampes, rats et personnage sont figés pour isoler le mouvement de l'eau.</p>
<div class="viewport"><img src="comparaison-native64.gif" alt="Comparaison animée native des égouts"></div>
<p>Proposition séparée du jeu normal. <a href="variant-1-native64.png">Variante 1</a> ·
<a href="variant-2-native64.png">Variante 2</a> · <a href="variant-3-native64.png">Variante 3</a> ·
<a href="reduced-motion.png">Mouvement réduit</a></p></html>'''
    if canals:
        html=html.replace('EG-EAU01','EG-CAN02').replace('Eau et berges des égouts','Réseau de canaux des égouts')
        html=html.replace('01 : rendu actuel. 02 : courant lent, rides discrètes,\ncanaux raccordés et bord d\'eau assombri.', '01 : anciens canaux avec la nouvelle eau. 02 : réseau prolongé et relié, avec de nouvelles traversées sur grilles.')
        html=html.replace('</html>','<p>Vues globales à échelle réduite : '+ ' · '.join(f'<a href="variant-{i}-overview.png">Carte {i}</a>' for i in range(1,4))+'</p><p>Cartes complètes à taille native : '+ ' · '.join(f'<a href="variant-{i}-proposed-world-native64.png">Carte {i}</a>' for i in range(1,4))+'</p></html>')
    (FOLDER / "apercu.html").write_text(html,encoding="utf-8")
    result = {"review": data["review"], "native_geometry_verified": True,
              "frames_submitted": count, "encoded_frames": decoded.n_frames,
              "duration_ms": duration, "animated_pixels_between_0_and_3_seconds": animated_pixels,
              "comparisons": comparisons, "temporary_bytes_removed": raw.stat().st_size}
    result["outputs"] = [{"file": p.name, "sha256": hashlib.sha256(p.read_bytes()).hexdigest(),
                          "bytes": p.stat().st_size} for p in FOLDER.iterdir()
                         if p.is_file() and p.name not in ("sequence.rgba", "publication.json")]
    (FOLDER / "publication.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    raw.unlink()
    print(json.dumps({k: v for k, v in result.items() if k != "outputs"}))


if __name__ == "__main__":
    publish()
