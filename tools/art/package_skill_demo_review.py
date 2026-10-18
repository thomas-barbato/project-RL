"""Package integrated active-skill scenes from exact cropped native framebuffers."""
from pathlib import Path
from html import escape
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/skill-learn-buttons-review-01"


def pack(entry):
    name = entry["skill"]
    size = tuple(round(v) for v in entry["bounds"][2:])
    paths = sorted((OUT / "active-scenes" / name).glob("[0-9][0-9][0-9].rgba"))
    assert len(paths) == entry["frames"], name
    frames = [Image.frombytes("RGBA", size, p.read_bytes()).transpose(Image.Transpose.FLIP_TOP_BOTTOM) for p in paths]
    unique, timing = [], []
    for frame in frames:
        if unique and frame.tobytes() == unique[-1].tobytes():
            timing[-1] += entry["frame_duration_ms"]
        else:
            unique.append(frame)
            timing.append(entry["frame_duration_ms"])
    assert len(unique) > 1, (name, "no animated sample")
    output = OUT / "scenes" / (name + ".png")
    unique[0].save(output, save_all=True, append_images=unique[1:], duration=timing, loop=0, disposal=0, blend=0, optimize=False)
    with Image.open(output) as encoded:
        assert encoded.n_frames == len(unique), name
        for i, expected in enumerate(unique):
            encoded.seek(i)
            assert encoded.convert("RGBA").tobytes() == expected.tobytes(), (name, i)
            assert abs(encoded.info["duration"] - timing[i]) < 1
    frames[4].save(OUT / "scenes" / (name + "-pause.png"))
    return {"skill": name, "file": "scenes/" + output.name, "pixels": size,
            "frames": len(unique), "captured_frames": len(frames), "native_pixels_preserved": True,
            "sha256": hashlib.sha256(output.read_bytes()).hexdigest()}


def main():
    report = json.loads((OUT / "integration-native.json").read_text(encoding="utf-8"))
    catalogue = json.loads((ROOT / "assets/skills/agent/catalogue.json").read_text(encoding="utf-8"))
    active_ids = {s["id"] for s in catalogue["skills"] if s["kind"] == "active"}
    assert len(active_ids) == 68 and {e["skill"] for e in report["examples"]} == active_ids
    (OUT / "scenes").mkdir(exist_ok=True)
    report["animations"] = [pack(e) for e in report["examples"]]
    report["packed_native_pixels_verified"] = True
    (OUT / "integration-native.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for name, skill in [("braise", "FE01"), ("souffle", "FE02"), ("boule", "FE05")]:
        (OUT / ("scene-" + name + ".png")).write_bytes((OUT / "scenes" / (skill + ".png")).read_bytes())
    options = "".join('<option value="' + escape(e["skill"]) + '"' + (' selected' if e["skill"] == "FE02" else '') + '>' + escape(e["skill"] + " · " + e["title"]) + '</option>' for e in report["examples"])
    html = """<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>Compétences · 68 scènes intégrées</title>
<style>:root{color-scheme:dark;font:16px/1.5 system-ui;background:#090c12;color:#e7edf5}
body{margin:24px;max-width:1500px}h1{font-size:25px}p{max-width:1060px;color:#b8bfcb}
.native{width:max-content;max-width:100%;overflow:auto}img{display:block;max-width:none}
nav{display:flex;gap:12px;margin:16px 0}button,select{font:inherit;padding:8px 14px;border:1px solid #596174;background:#1b222e;color:#e7edf5}
button{cursor:pointer}button[aria-pressed=true]{border-color:#a693ff;color:#d5ccff}
:focus-visible{outline:2px solid #c0b5ff;outline-offset:4px}details{margin:24px 0}code,a{color:#cbbfff}.note{font-size:14px;color:#8f9cab}
</style><h1>68 compétences actives · intégrées au jeu</h1>
<p>Cliquer sur une image de compétence ouvre son exemple de base directement dans la zone centrale, sous les icônes. Le nom est centré, sans mention « DÉMONSTRATION », sans panneau ajouté. Un contour fin délimite la scène. La fiche descriptive à droite et les branches gardent toute leur hauteur.</p>
<p>Le bouton + apprend ou améliore uniquement la compétence. Les exemples restent au rang 1, indépendamment du niveau appris. Les passifs n’ouvrent aucune scène.</p>
<p>Le bouton « Utiliser » est supprimé. « Fermer » se trouve sous les six raccourcis, dans le bas de la fenêtre.</p>
<nav><button data-theme="metal" aria-pressed="true">Métal</button><button data-theme="terminal" aria-pressed="false">Vieux terminal</button></nav>
<div class="native"><img id="window" src="fenetre-metal.png" width="1280" height="800" alt="La scène se trouve directement sous les compétences, dans la zone centrale."></div>
<details><summary>Parcourir les 68 exemples animés</summary><p><label for="skill">Compétence : </label><select id="skill">OPTIONS</select></p>
<div class="native"><img id="scene" src="scenes/FE02.png" width="480" height="222" alt="Exemple de base de la compétence choisie."></div>
<p>Les armes, cibles, drones, blessures ou effets nécessaires sont préparés uniquement dans cet exemple isolé. Les protections, soins, invocations, déplacements et transformations ont chacun leur cas adapté.</p></details>
<details><summary>Compétence passive</summary><div class="native"><img src="passif.png" width="1280" height="800" alt="Contagion est passive : aucune scène n’est affichée."></div></details>
<p class="note">SKILL-DEMO-01 · version intégrée · 68 actives · simulation séparée, sans changement des points, tours, ressources, RNG ou sauvegarde de la partie. Les grands effets sont cadrés dans la miniature ; le monde du jeu et ses sources restent en 64 px.</p>
<script>document.querySelectorAll('[data-theme]').forEach(button=>button.addEventListener('click',()=>{
 document.querySelectorAll('[data-theme]').forEach(item=>item.setAttribute('aria-pressed',String(item===button)));
 document.getElementById('window').src='fenetre-'+button.dataset.theme+'.png';}));
document.getElementById('skill').addEventListener('change',event=>{document.getElementById('scene').src='scenes/'+event.target.value+'.png';});</script></html>""".replace("OPTIONS", options)
    (OUT / "index.html").write_text(html, encoding="utf-8")
    print(json.dumps({"active_count": len(report["animations"]), "native_frames": sum(a["captured_frames"] for a in report["animations"]), "pixel_checks": "passed"}))


if __name__ == "__main__":
    main()
