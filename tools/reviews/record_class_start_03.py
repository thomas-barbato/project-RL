"""Record the accepted class base skin/loadout and refresh the existing review."""
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/class-start-03'


def publish():
    rows = []
    files = []
    for slug in ('bastion', 'technomancien', 'traqueur', 'arcaniste', 'metamorphe'):
        source = ROOT / f'content/core/classes/{slug}.json5'
        data = json.loads(source.read_text(encoding='utf-8-sig'))
        assert all(not entry['item'].startswith('core:tenue_') for entry in data['starting_items'])
        rows.append(dict(character_class=data['id'], starting_weapon=data['starting_weapons'][0],
                         first_bar_slot=data['agent_start']['skills'][0], armor_item=None))
        files.append(source)
    record = dict(review='CLASS-START-03', date='2026-10-20', generation=165,
                  status='accepted_and_integrated', approval='ok alors on fait ca :)',
                  source_of_approval='The user approved the combined synthesis before requesting implementation.',
                  classes=rows, new_artwork=False, class_drawings_preserved=True,
                  weapon_bonuses_added=False, starting_active_assigned_once=True,
                  empty_bar_preserved_after_resume=True, historical_loadouts_preserved=True,
                  previous_step='STARTER-SKIN-02',
                  source_sha256={str(p.relative_to(ROOT)).replace('\\', '/'):hashlib.sha256(p.read_bytes()).hexdigest() for p in files})
    DOC.mkdir(parents=True, exist_ok=True)
    (DOC / 'acceptance.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n', encoding='utf8')
    start = '<!-- CLASS-START-03 BEGIN -->'
    end = '<!-- CLASS-START-03 END -->'
    block = '''<section id="class-start"><h2>CLASS-START-03 · Départ sans objet d’armure</h2>
<p class="notice">Le joueur commence sans armure équipée et sans tenue dans son inventaire.
Son modèle de classe constitue son apparence de base ; équiper une armure le remplace, la retirer rétablit cette apparence.</p>
<p>Chaque classe garde son arme ordinaire, sans bonus d’objet. Son active initiale est placée dans le premier raccourci :
Frappe de rupture, drone du Technomancien, Marquage balistique, Braise ou transformation du Métamorphe.
Le joueur peut déplacer ou retirer ce raccourci ; une barre entièrement vidée reste vide après reprise.</p>
<p>La génération 165 applique ce départ dans le jeu normal, via <a href="../../Jouer.cmd">Jouer.cmd</a>
ou <code>cargo run --locked</code>. Les anciennes campagnes gardent leur équipement et leur barre enregistrés.</p></section>'''
    capture_root = ROOT / 'artifacts/class-start-03'
    examples = ''
    for name, caption in (
        ('bastion-inventory', 'Bastion : torse et apparence vides, arme ordinaire'),
        ('metamorphe-world', 'Métamorphe : modèle de classe sans armure, active au premier raccourci'),
    ):
        if (capture_root / name / 'cold-start.png').is_file():
            examples += f'<p>{caption}</p><div class="native"><img src="../class-start-03/{name}/cold-start.png" alt="{caption}"></div>'
    if examples:
        block = block.replace('</section>', '<details><summary>Contrôles dans le moteur à taille native</summary>'+examples+'</details></section>')
    for folder in ('combat-fixes-01', 'street-defects-review-01'):
        page_path = ROOT / 'artifacts' / folder / 'index.html'
        if not page_path.exists():
            continue
        page = page_path.read_text(encoding='utf8')
        page = re.sub(re.escape(start)+'.*?'+re.escape(end), '', page, flags=re.S)
        page = page.replace('</h1>', '</h1>'+start+block+end, 1)
        page = page.replace('Tenues de départ à 0 protection et sans malus d’esquive ;',
                            'Étape précédente STARTER-SKIN-02 : tenues à 0, remplacées désormais par l’apparence de classe sans objet ;')
        page_path.write_text(page, encoding='utf8')
    print('CLASS-START-03 recorded; current review updated without changing accepted images.')


if __name__ == '__main__':
    publish()
