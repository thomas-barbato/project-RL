"""Record accepted starter garments/flames and publish measured offense, without balancing it."""
import hashlib
import html
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/combat-fixes-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/combat-fixes-01'


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf8')


def publish():
    for direction in ('droite', 'gauche', 'haut', 'bas'):
        frames = [Image.open(OUT / 'engine' / f'{direction}-{i:02}.png').convert('RGB') for i in range(16)]
        assert all(max(v[1] for v in f.getextrema()) > 100 for f in frames), 'Empty GPU capture'
        frames[7].save(OUT / f'{direction}-native.png')
        frames[7].save(DOC / f'{direction}-native.png')
        frames[0].save(OUT / f'{direction}.gif', save_all=True, append_images=frames[1:],
                       duration=90, loop=0, disposal=2)
    defenses = json.loads((OUT / 'starter-skin-02.json').read_text(encoding='utf8'))
    offense = json.loads((OUT / 'offense-audit-02.json').read_text(encoding='utf8'))
    for name in ('starter-skin-02.json', 'offense-audit-02.json'):
        (DOC / name).write_bytes((OUT / name).read_bytes())
    gpu = json.loads((OUT / 'engine/checks.json').read_text(encoding='utf8'))
    write_json(DOC / 'native-gpu-02.json', gpu)
    assert all(r['armor'] == (1 if r['generation'] == 164 else 4) for r in defenses)
    assert all(r['raw_damage'] == (2 if r['monster'] == 22 else 1) + r['level'] - 1 for r in defenses)
    files = ['assets/animations/agent/skill-animation-review-06/flame-atlas.png']
    files += [f'content/core/items/tenue_{name}.json5' for name in ('assaut', 'technique', 'rituelle', 'metamorphe', 'traque')]
    files += [f'docs/reviews/refonte-histoire-2026-10-16/combat-fixes-01/{d}-native.png'
              for d in ('droite', 'gauche', 'haut', 'bas')]
    record = dict(
        generation=164,
        normal_launcher='Jouer.cmd',
        native_gpu_capture=gpu,
        decisions=[dict(id='STARTER-SKIN-02', status='accepted_and_integrated',
                        user_request='mettre les armures de base du joueur à 0 ; elles serviront que de skin au départ',
                        armor=0, evasion_penalty=0, outfit_count=5, bastion_class_armor=1,
                        historical_generation_values_preserved=True),
                   dict(id='FIRE-NATIVE-01', status='accepted_and_integrated',
                        user_answer='Valider les flammes détaillées',
                        runtime_family='Flame', cell_px=64, source_frame_px=128,
                        range_and_damage_unchanged=True),
                   dict(id='COMBAT-START-01', status='superseded_before_activation',
                        replacement='STARTER-SKIN-02', enemy_damage_unchanged=True),
                   dict(id='U01-POP-01', status='integrated_since_generation_163',
                        monsters=15, groups=5, no_respawn_on_return=True),
                   dict(id='OFFENSE-START-02', status='diagnostic_only',
                        player_damage_unchanged=True, enemy_hp_unchanged=True, xp_unchanged=True)],
        source_sha256={name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in files},
    )
    write_json(DOC / 'acceptance-integration-02.json', record)
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Combat : intégration et dégâts</title>
<style>body{background:#111719;color:#d9dedb;font:16px/1.6 system-ui;margin:24px}main{max-width:1240px;margin:auto}
h1,h2{color:#d8c39b}a{color:#9bd1de}.notice{background:#26332e;padding:16px}.native{overflow:auto;border:1px solid #4a5858;margin:20px 0}
img{display:block;max-width:none}summary{cursor:pointer}section{margin:36px 0}.example{border-left:2px solid #677776;padding:4px 16px;margin:12px 0}</style><main>
<h1>Combat : corrections intégrées et dégâts observés</h1>
<p class="notice">Tenues de départ à 0 protection et sans malus d’esquive ; flammes détaillées acceptées ;
15 monstres en cinq groupes dans la ville des nouvelles parties. Les dégâts du joueur et des monstres, les PV ennemis et l’XP restent inchangés.</p>
<section><h2>STARTER-SKIN-02 · Tenues cosmétiques</h2>
<p>Les cinq tenues conservent leur apparence et leurs couleurs. Le Bastion garde son +1 d’armure de classe.
Les anciennes sauvegardes conservent leurs valeurs historiques. La hausse des dégâts ennemis proposée précédemment est remplacée par ce choix.</p>
<p>Avec les profils ennemis d’origine, la mêlée de niveau 1 retire maintenant 2 PV par coup réussi au Bastion.
Le tir de niveau 1 reste absorbé par son +1 de classe ; les autres classes n’ont pas ce bonus.</p></section>
<section id="offense"><h2>OFFENSE-START-02 · Dégâts du joueur face aux monstres</h2>
<p>Les profils ordinaires actuels n’ont que 7–8 PV au niveau 1 et 8–9 PV au niveau 2.
Voici les dégâts observés au niveau 1, avec les armes et attributs réels de départ, sur 32 graines indépendantes.
La cible de mesure est inerte, sans armure et avec esquive désactivée ; ses 1 000 PV évitent de masquer les dégâts par une mort prématurée.
Les plages ci-dessous sont des observations, pas des bornes exhaustives garanties.</p>'''
    for row in offense:
        values = row['observed_damage']
        amount = f'{min(values)}–{max(values)}' if len(values) > 1 else (str(values[0]) if values else '0')
        availability = 'appris au départ' if row['learned_at_start'] else 'exemple supplémentaire, non appris au départ'
        page += f'<div class="example"><strong>{html.escape(row["class"].removeprefix("core:"))} · {html.escape(row["name"])}</strong><br>{amount} dégâts observés · rang 1 · {availability}.</div>'
    page += '''<p>Marquage balistique prépare un tir ultérieur : son résultat de 0 pendant cette commande ne mesure pas le tir qui suit.</p>
<p>La mesure couvre la commande seule, sans additionner les brûlures futures, les invocations ni les attaques sous transformation.
Le problème ne se limite donc pas aux compétences : les attaques ordinaires doivent aussi être comparées aux PV de ces profils très fragiles.</p>
<p>Prochaine proposition : régler ensemble les PV des familles et les dégâts d’entrée, garder des ennemis fragiles dans les groupes,
et réserver davantage de résistance aux combattants lourds. Aucun nouveau coefficient global n’est appliqué ici.</p></section>
<section><h2>FIRE-NATIVE-01 · Flammes détaillées acceptées</h2>
<p>Comparaison conservée : ancien rendu à gauche, rendu accepté à droite, dans les mêmes cases de 64 px.
Les quatre directions et l’emprise viennent de l’attaque réelle du lance-flammes. Portée et dégâts inchangés.</p>'''
    for direction in ('droite', 'gauche', 'haut', 'bas'):
        page += f'<details {"open" if direction=="droite" else ""}><summary>{direction.capitalize()}</summary><div class="native"><img src="{direction}.gif" alt="Comparaison {direction}"></div><a href="{direction}-native.png">Image fixe à taille native</a></details>'
    page += '''</section><p><a href="../../Jouer.cmd">Jouer.cmd</a> ouvre le jeu normal, comme <code>cargo run --locked</code>.
Une nouvelle partie commence au camp de cryogénie, rejoint la ville peuplée, puis la trappe des égouts en X 2, Y 14.
Les lanceurs d’essai séparés ont été supprimés. Les anciennes campagnes conservent leurs réglages.</p></main></html>'''
    (OUT / 'index.html').write_text(page, encoding='utf8')
    for d in ('droite', 'gauche', 'haut', 'bas'):
        for suffix in ('.gif', '-native.png'):
            page = page.replace(f'"{d}{suffix}"', f'"../combat-fixes-01/{d}{suffix}"')
    (ROOT / 'artifacts/street-defects-review-01/index.html').write_text(page, encoding='utf8')
    if (ROOT / 'docs/reviews/refonte-histoire-2026-10-16/class-start-03/acceptance.json').exists():
        from record_class_start_03 import publish as publish_class_start
        publish_class_start()
    print('STARTER-SKIN-02 and FIRE-NATIVE-01 integrated; OFFENSE-START-02 published without rebalance.')


if __name__ == '__main__':
    publish()
