"""Add the three listening proposals to the existing review, without autoplay."""
from pathlib import Path
import json
import re

ROOT = Path(__file__).resolve().parents[2]
PAGE = ROOT / "artifacts/handcrafted-places-review-01/index.html"
DOC = ROOT / "docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01"


def publish():
    report = json.loads((DOC / "dialogue-audio-01.json").read_text(encoding="utf-8"))
    selected = (DOC / "dialogue-audio-selection-01.json").exists()
    page = PAGE.read_text(encoding="utf-8")
    page = re.sub(r"<!-- dialogue-audio:start -->.*?<!-- dialogue-audio:end -->", "", page, flags=re.S)
    items = "".join(
        f'<article><h3>{i:02} · {c["name"]}</h3>'
        f'<audio controls preload="metadata" src="dialogue-bubbles-01/{c["file"]}"></audio></article>'
        for i, c in enumerate(report["candidates"], 1)
    )
    section = f'''<!-- dialogue-audio:start --><section id="sons-dialogue-01" style="border:1px solid #526b62;margin:18px 0;background:#121b18"><h2>DIALOGUE-AUDIO-01 · trois essais à écouter</h2><p>01 : petits clics électroniques feutrés, mon premier choix. 02 : davantage de texture mécanique. 03 : impulsions plus numériques. Même durée et même cadence dans les trois essais ; les sons évitent les espaces et la ponctuation. Sons originaux préparés pour comparaison, pas encore intégrés au jeu.</p><div class="pair">{items}</div><p>Après choix, le son accompagnera uniquement l’écriture et s’arrêtera à la fin du texte ou à la fermeture de la bulle. La relecture sans animation restera silencieuse.</p><script>(()=>{{const sounds=[...document.querySelectorAll('#sons-dialogue-01 audio')];sounds.forEach(a=>a.addEventListener('play',()=>sounds.forEach(b=>{{if(b!==a){{b.pause();b.currentTime=0;}}}})));}})();</script></section><!-- dialogue-audio:end -->'''
    if selected:
        section = section.replace("01 : petits clics électroniques feutrés, mon premier choix.", "01 : petits clics électroniques feutrés.").replace(
            "Sons originaux préparés pour comparaison, pas encore intégrés au jeu.",
            "Le 03 est retenu provisoirement pour un essai jouable ; aucun choix sonore définitif.",
        ).replace(
            "Après choix, le son accompagnera uniquement l’écriture et s’arrêtera à la fin du texte ou à la fermeture de la bulle. La relecture sans animation restera silencieuse.",
            'Le son 03 accompagne l’écriture et s’arrête à la fin du texte ou à la fermeture. '
            'La lecture sans animation reste silencieuse. Lance le jeu habituel avec <code>cargo run</code>, '
            'puis appuie sur E près de l’officier du camp. '
            'Le <a href="../../Tester_dialogue_terminal.cmd">lanceur d’essai séparé</a> reste également disponible.',
        )
    # The listening controls are review controls, outside the simulated game.
    page = page.replace("e.target.closest('#bubble-reopen')", "e.target.closest('#bubble-reopen,#sons-dialogue-01')")
    page = page.replace("<!-- dialogue-bubbles:end -->", "<!-- dialogue-bubbles:end -->" + section, 1)
    PAGE.write_text(page, encoding="utf-8")
    print(PAGE)


if __name__ == "__main__":
    publish()
