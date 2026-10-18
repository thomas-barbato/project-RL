"""Pack native FX-FOOTPRINT-01 captures, without resizing or redrawing pixels."""
from pathlib import Path
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/walk-preview-review-01/effect-footprints"
SHEETS = [
    ("souffle-couloir-1", "Souffle de feu intégré : 3 × 2 cases au rang 1, gauche et haut", 632, 6),
    ("souffle-couloir-4", "Souffle de feu intégré : 3 × 5 cases au rang 4, droite et bas", 632, 6),
    ("braise", "Braise : projectile monocible, quatre directions", 416, 4),
    ("occulte", "Déflagration occulte : rangs 1 et 10", 928, 6),
    ("boule-feu", "Boule de feu : rangs 1 et 10", 928, 13),
    ("nova", "Nova de gel : rangs 1 et 10", 928, 7),
    ("blizzard", "Blizzard : rangs 1 et 10", 928, 6),
    ("vague", "Vague de gel : rangs 1 et 10, deux directions", 928, 6),
    ("aiguille", "Aiguille de gel : gauche et haut", 736, 4),
    ("fleche", "Flèche occulte : gauche et haut", 736, 4),
]


def pack(paths, output, height=None, duration=35, final_pause=700, width=1280, preview=False):
    frames = []
    for path in paths:
        frame = Image.open(path).convert("RGBA")
        if height is not None:
            frame = frame.crop((0, 0, width, height))
        frames.append(frame)
    delays = [duration] * len(frames)
    delays[-1] += final_pause
    if preview:
        delays[0] += 550
    # Consecutive identical native frames are encoded once with their full duration.
    unique, timing = [], []
    for frame, delay in zip(frames, delays):
        if unique and unique[-1].tobytes() == frame.tobytes():
            timing[-1] += delay
        else:
            unique.append(frame)
            timing.append(delay)
    unique[0].save(output, save_all=True, append_images=unique[1:],
                   duration=timing, loop=0, disposal=0, blend=0, optimize=False)
    encoded = Image.open(output)
    assert encoded.n_frames == len(unique), (output, encoded.n_frames, len(unique))
    for i, expected in enumerate(unique):
        encoded.seek(i)
        assert expected.tobytes() == encoded.convert("RGBA").tobytes(), (output, i)
        assert abs(encoded.info["duration"]-timing[i]) < 1, (output, i)
    return {"file": output.name, "frames": len(unique), "captured_frames":len(frames), "dimensions": frames[0].size,
            "native_frame_pixels_equal": True}


def main():
    report = []
    html = ["""<!doctype html><html lang="fr"><meta charset="utf-8">
<title>Zones, trajectoires et animations · FX-FOOTPRINT-01</title>
<style>:root{color-scheme:dark;font:16px/1.5 system-ui;background:#0b1117;color:#d5e0e4}
body{margin:24px;max-width:1800px}h1{font-size:26px;color:#e6e0cb}h2{font-size:20px;margin-top:38px}
a{color:#a8dafa}p{max-width:1060px}.note{font-size:14px;color:#a5b7c4}
.native{overflow:auto;width:max-content;max-width:100%;border:1px solid #354956}img{display:block;max-width:none}
button{font:inherit;background:#172633;border:1px solid #456075;color:#d6e8f4;padding:8px 12px;margin:10px 0}
nav a{display:inline-block;margin:4px 14px 4px 0}</style>
<h1>Zones et animations · FX-FOOTPRINT-01</h1>
<p>Version validée et intégrée au jeu. Souffle de feu forme un couloir de 3 cases de large,
2 cases de long au rang 1, puis une case de plus par rang. Les planches montrent d'abord la visée,
puis sa disparition pendant l'animation. Ces captures utilisent le lancement normal et le fonctionnement réel des compétences.</p>
<p class="note">64 px par case · pixels du moteur conservés sans agrandissement · défilement horizontal si nécessaire.
Les autres planches, repliées ci-dessous, sont des diagnostics avec contours superposés pour contrôler les dimensions.
Ces contours de contrôle ne doivent pas rester après confirmation dans le jeu.</p>
<p><strong>Texte conservé pour Souffle de feu :</strong> « Vous déchaînez un torrent de flammes devant vous. »<br>
Largeur : 3 cases. Bloqué par les murs. Rang 1 : longueur de 2 cases. Prochain rang : longueur de 3 cases.</p>
<button id="pause">Mettre les animations en pause</button><nav>"""]
    html.extend(f'<a href="#{"planche-" if name.startswith("souffle-couloir") else ""}{name}">{name}</a>' for name, *_ in SHEETS)
    html.append('<a href="#transparence">Transparence et trajet</a></nav>')
    for name, title, height, still in SHEETS:
        width = 1728 if name in ("occulte","boule-feu","nova","blizzard","souffle","vague") else 1280
        report.append(pack([OUT / f"{name}-{i:02}.png" for i in range(16)], OUT / f"{name}.png", height, width=width, preview=name.startswith("souffle-couloir")))
        Image.open(OUT / f"{name}-{still:02}.png").crop((0, 0, width, height)).save(OUT / f"{name}-pause.png")
        if not name.startswith("souffle-couloir"):
            html.append(f'<details id="{name}"><summary>{title} · diagnostic avec contours de contrôle</summary>')
        html.append(f'<section id="planche-{name}"><h2>{title}</h2><div class="native"><img src="effect-footprints/{name}.png" '
                    f'data-motion="effect-footprints/{name}.png" data-still="effect-footprints/{name}-pause.png" '
                    f'width="{width}" height="{height}" alt="{title}"></div></section>')
        if not name.startswith("souffle-couloir"):
            html.append('</details>')
    marker = OUT / "markers"
    report.append(pack([marker / f"pulse-{i:02}.png" for i in range(12)], marker / "transparence.png", duration=92, final_pause=0))
    html.append('''<section id="transparence"><h2>Contours et trajet légèrement transparents</h2>
<p>Les chiffres et leurs cercles restent fixes et opaques. Les petits repères conservent leur pulsation progressive.</p>
<div class="native"><img src="effect-footprints/markers/transparence.png" data-motion="effect-footprints/markers/transparence.png"
data-still="effect-footprints/markers/pulse-06.png" width="1280" height="400" alt="Zones valide, invalide et chemin légèrement transparents"></div></section>
<p><a href="cercles-animes.html">Revoir la version actuellement intégrée des repères</a></p>
<script>let paused=false;const button=document.getElementById('pause');function show(){
document.querySelectorAll('img[data-motion]').forEach(image=>image.src=paused?image.dataset.still:image.dataset.motion);
button.textContent=paused?'Reprendre les animations':'Mettre les animations en pause';}
button.onclick=()=>{paused=!paused;show()};if(matchMedia('(prefers-reduced-motion: reduce)').matches){paused=true;show()}</script></html>''')
    (OUT.parent / "effets-zones.html").write_text("\n".join(html), encoding="utf-8")
    (OUT / "packing-checks.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
