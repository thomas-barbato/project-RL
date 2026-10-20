"""Publish the concrete combat and native-flame candidates, without activation."""
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/combat-fixes-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/combat-fixes-01'


def publish():
    if (DOC / 'acceptance-integration-02.json').exists():
        from record_combat_fixes_02 import publish as publish_accepted
        publish_accepted()
        return
    DOC.mkdir(parents=True, exist_ok=True)
    rows = json.loads((OUT / 'diagnostic.json').read_text(encoding='utf8'))
    checks = json.loads((OUT / 'engine/checks.json').read_text(encoding='utf8'))
    assert checks['directions'] == 4 and not checks['normal_activation']
    for name in ('droite', 'gauche', 'haut', 'bas'):
        frames = [Image.open(OUT / 'engine' / f'{name}-{i:02}.png').convert('RGB') for i in range(16)]
        assert all(f.width >= 1210 and f.height >= 720 for f in frames)
        assert all(max(v[1] for v in f.getextrema()) > 100 for f in frames), 'Empty framebuffer capture'
        frames[7].save(OUT / f'{name}-native.png')
        frames[0].save(OUT / f'{name}.gif', save_all=True, append_images=frames[1:],
                       duration=90, loop=0, disposal=2)
        # Exact native stills remain durable review references.
        frames[7].save(DOC / f'{name}-native.png')
    (DOC / 'proposal.json').write_text(json.dumps(dict(
        combat_review='COMBAT-START-01', visual_review='FIRE-NATIVE-01',
        status='awaiting_numeric_and_visual_approval', generation_with_city_population=163,
        city_groups=5, city_monsters=15, source_population='U01-POP-01',
        city_activation_authorized_by='il faudra aussi ajouter comme prévu les monstres dans la ville :)',
        raw_damage_candidates=dict(level_1_melee=[2,4],level_1_ranged=[1,5],
                                   level_2_melee=[3,5],level_2_ranged=[2,6]),
        limits=['Only these early city/sewer profiles', 'No XP change', 'No accuracy change',
                'No global armor change', 'No resurrection activation'],
        diagnostic=rows, native_fire=checks,
    ), ensure_ascii=False, indent=2) + '\n', encoding='utf8')
    table = ''
    for level, number, label in [(1,22,'Mêlée · niveau 1'),(1,24,'Tir · niveau 1'),
                                 (2,22,'Mêlée · niveau 2'),(2,24,'Tir · niveau 2')]:
        before = next(r for r in rows if not r['proposed'] and r['level']==level and r['monster']==number)
        after = next(r for r in rows if r['proposed'] and r['level']==level and r['monster']==number)
        net = after['damage_sum']/after['hits']
        table += f'<tr><td>{label}</td><td>{before["raw_damage"]} → {after["raw_damage"]}</td><td>{net:g} PV par coup réussi</td></tr>'
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Combats et flammes natives</title>
<style>body{background:#111719;color:#d9dedb;font:16px/1.6 system-ui;margin:24px}main{max-width:1240px;margin:auto}
h1,h2{color:#d8c39b}a{color:#9bd1de}.notice{background:#26332e;padding:16px}table{border-collapse:collapse;width:100%}
td,th{padding:10px;border:1px solid #4a5858;text-align:left}.native{overflow:auto;border:1px solid #4a5858;margin:20px 0}
img{display:block;max-width:none}summary{cursor:pointer}section{margin:36px 0}</style><main>
<h1>Combats de début de jeu et lance-flammes</h1>
<p class="notice">Les cinq groupes déjà validés sont ajoutés à la ville pour les nouvelles parties.
Les deux corrections proposées ci-dessous attendent leur accord séparé.</p>
<section><h2>COMBAT-START-01 · Les coups sont absorbés</h2>
<p>Avec le Bastion initial, le test de 32 tours mesure 20 coups de mêlée réussis sur 24 tentatives,
et 9 tirs réussis sur 9. Son armure absorbe tous ces dégâts. Les chances de toucher mesurées sont
79 % en mêlée et 87 % au tir : augmenter la précision ne résoudrait donc pas ce cas.</p>
<p>Je propose de relever seulement les dégâts de base de ces premiers ennemis.
Le tableau montre le résultat réellement calculé par le moteur face au Bastion initial,
après ses défenses et les modificateurs offensifs actuels.</p>
<table><tr><th>Profil</th><th>Dégâts de base actuels → proposés</th><th>Résultat proposé face au Bastion</th></tr>'''+table+'''</table>
<p>Ces montants restent une première base d’essai. Les autres classes, équipements et défenses changent les dégâts subis.
Les PV des ennemis, leur précision, l’armure du joueur, le butin, l’XP et la réanimation restent inchangés.</p></section>
<section><h2>FIRE-NATIVE-01 · Dessin détaillé à la taille du jeu</h2>
<p>À gauche, le rendu actuel. À droite, l’atlas de flammes déjà accepté, dessiné dans les mêmes cases de 64 px.
L’ancien rendu est même plus petit que prévu : il est calculé en 16 × 16, puis agrandi.
Le nouveau dessin part de 128 × 128 et est réduit à la taille de jeu. La portée et les dégâts du lance-flammes ne changent pas.</p>'''
    for name in ('droite','gauche','haut','bas'):
        html += f'<details {"open" if name=="droite" else ""}><summary>Direction : {name}</summary><div class="native"><img src="{name}.gif" alt="Comparaison animée vers {name}"></div><p><a href="{name}-native.png">Image fixe native</a></p></details>'
    html += '''</section><p>Jeu normal : <a href="../../Jouer.cmd">Jouer.cmd</a>, puis nouvelle partie.
Le personnage commence au camp et rejoint la ville par le portail sud.
La trappe des égouts reste en X 2, Y 14 ; les anciennes sauvegardes conservent leur monde.</p></main></html>'''
    html = html.replace('</main>', '<p><a href="../sewer-review-04/index.html">Revoir la composition acceptée des égouts</a></p></main>')
    (OUT / 'index.html').write_text(html, encoding='utf8')
    current = html
    for name in ('droite', 'gauche', 'haut', 'bas'):
        for suffix in ('.gif', '-native.png'):
            current = current.replace(f'"{name}{suffix}"', f'"../combat-fixes-01/{name}{suffix}"')
    (ROOT / 'artifacts/street-defects-review-01/index.html').write_text(current, encoding='utf8')
    print('COMBAT-START-01 and FIRE-NATIVE-01 published; no numeric or visual activation.')


if __name__ == '__main__':
    publish()
