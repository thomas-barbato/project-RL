"""Publish the compact current camp review without rebuilding historical drafts."""
from pathlib import Path
from html import escape
import json

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/handcrafted-places-review-01"
DOC = ROOT / "docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01"


def publish():
    checks = json.loads((OUT / "hover-outline-01/checks.json").read_text(encoding="utf-8"))
    assert not checks["outline_production_enabled"]
    names = {"officer": "Officier", "medical": "Médecin", "cultist": "Cultiste",
             "ally": "Allié", "large-demon": "Grand démon"}
    cards = []
    required = ["current/camp-native.png", "current/dialogue-native.png"]
    for scene in checks["scenes"]:
        key = scene["name"]
        required.extend(f"hover-outline-01/{key}{suffix}"
                        for suffix in ["-native.webp", "-without.png", "-0.png"])
        cards.append(f'''<article><h3>{escape(names[key])}</h3><p>{"Vert" if scene["tone"] == "Friendly" else "Rouge"} · taille native</p>
<img width="384" height="320" src="hover-outline-01/{key}-native.webp" alt="Contour au survol">
<details><summary>Comparer sans contour</summary><img width="384" height="320" src="hover-outline-01/{key}-without.png" alt="Même scène sans contour"></details>
<p><a href="hover-outline-01/{key}-0.png">Capture complète</a></p></article>''')
    sounds = []
    for number, label in [(1, "Terminal feutré"), (2, "Relais mécanique"), (3, "Impulsions numériques — retenu provisoirement")]:
        path = f"current/dialogue-audio-{number:02}.wav"
        required.append(path)
        sounds.append(f'<article><h3>{number:02} · {label}</h3><audio controls preload="metadata" src="{path}"></audio></article>')
    notifications = []
    for key, label in [("unread", "Nouveauté"), ("read", "Notification consultée"), ("ready", "Quête à rendre · symbole proposé")]:
        path = f"current/notification-{key}.png"
        required.append(path)
        notifications.append(f'<figure><figcaption>{label}</figcaption><img src="{path}" alt="Pictogramme à taille native"></figure>')
    assert all((OUT / path).is_file() for path in required), "Missing current review material"
    html = f'''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Project RL · camp et interface</title>
<style>*{{box-sizing:border-box}}body{{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}}header,section{{padding:18px 24px}}header,.copy{{max-width:1100px}}h1{{font-size:25px;color:#e5d2a8}}h2{{font-size:21px}}h3{{font-size:16px}}p{{line-height:1.5}}a{{color:#b9d3e9}}nav,.pair{{display:flex;flex-wrap:wrap;gap:20px}}section{{border-top:1px solid #33484e}}.viewport{{overflow:auto;max-width:100%;width:fit-content}}img{{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}}article{{max-width:100%;overflow:auto}}figcaption{{margin-bottom:10px}}</style>
<header><h1>Camp et interface · aperçu courant</h1><p>Le camp, ses animations et la bulle terminal fonctionnent dans le jeu habituel. Les anciennes maquettes et captures intermédiaires ont été retirées ; les sources et validations sont conservées.</p>
<nav><a href="#lighting">Camp</a><a href="#bulles-02">Dialogue intégré</a><a href="#sons-dialogue-01">Essais sonores</a><a href="#contours-r01">Contours encore en revue</a></nav><p>Lancer <code>cargo run</code> ou <a href="../../Jouer.cmd">Jouer.cmd</a>. Au camp, E permet de parler à l’officier.</p></header>
<section id="bulles-02"><h2>Bulle terminal · intégrée</h2><p>Écriture progressive, curseur bloc, fond légèrement transparent et contour continu. Clic ou Échap ferment la bulle. La notification consultée reste masquée. Le texte de l’officier est conservé.</p>
<div class="viewport"><img src="current/dialogue-native.png" alt="Dialogue dans le client normal"></div><div class="pair">{''.join(notifications)}</div></section>
<!-- dialogue-audio:start --><section id="sons-dialogue-01"><h2>Son d’écriture · choix 03 provisoire</h2><p>Le son 03 fonctionne dans le jeu. Les trois essais sont conservés pour permettre d’affiner le choix plus tard ; aucun son ne démarre automatiquement dans cette page.</p><div class="pair">{''.join(sounds)}</div>
<script>(()=>{{const sounds=[...document.querySelectorAll('audio')];sounds.forEach(a=>a.addEventListener('play',()=>sounds.forEach(b=>{{if(b!==a){{b.pause();b.currentTime=0;}}}})));}})();</script></section><!-- dialogue-audio:end -->
<!-- target-outline:start --><section id="contours-r01"><h2>HOVER-OUTLINE-01 · encore à valider</h2><p>Contour de 2 px suivant la silhouette complète et son arme : vert pour alliés et PNJ, rouge pour ennemis. Les témoins restent dans cette revue. Cette option n’est pas activée dans une partie normale.</p><div class="pair">{''.join(cards)}</div></section><!-- target-outline:end -->
<section id="lighting"><h2>PLACE-CAMP-01 r09b · camp validé</h2><p>Dernière vue d’ensemble conservée à taille native. Le décor, la cryogénie, les services, les éclairages et les animations sont intégrés au jeu.</p><div class="viewport"><img src="current/camp-native.png" alt="Camp validé à taille native"></div>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/README.md">Décisions et validations</a> · <a href="../../docs/maintenance/nettoyage-2026-10-18/camp-dialogues/README.md">Registre du nettoyage</a></p></section></html>'''
    (OUT / "index.html").write_text(html, encoding="utf-8")
    print(OUT / "index.html")


if __name__ == "__main__":
    publish()
