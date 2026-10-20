"""Publish native engine captures of the opt-in U01 population trial."""
from pathlib import Path
from html import escape
from html.parser import HTMLParser
from urllib.parse import urlsplit, unquote
import hashlib
import json
import shutil
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/street-population-review-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-population-review-01'
PLAN = ROOT / 'assets/trials/street-population-01/population.json'
MAP = ROOT / 'assets/maps/prison-street-expanded-01'

def read(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))

def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')

def map_hashes():
    return {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in MAP.rglob('*') if p.is_file()}

def publish():
    plan, checks = read(PLAN), read(OUT/'engine/checks.json')
    assert checks['review'] == plan['review'] and checks['count'] == 15
    baseline = read(DOC/'map-before.json')
    assert baseline == map_hashes(), 'The normal map was modified during the trial work'
    scenes = [('arrival', 'Arrivée · approche immédiate dégagée'),
              ('first-cultists', 'G1 · deux cultistes au contact · niveau 1'),
              ('west-zombies', 'G2 · trois zombies dans le détour ouest · niveau 1'),
              ('central-cultists', 'G3 · deux combattants et un tireur · niveau 1'),
              ('north-zombies', 'G4 · trois zombies dans la rue nord · niveau 2'),
              ('last-cultists', 'G5 · trois combattants et un tireur · niveau 2')]
    # Preflight all frames before publishing. One source pixel = one display pixel.
    for key, _ in scenes:
        with Image.open(OUT/'engine'/f'{key}.png') as im:
            assert im.width >= 1552 and im.height >= 894, (key, im.size)
            assert im.convert('RGB').getbbox(), f'Empty frame: {key}'
    (OUT/'native').mkdir(parents=True, exist_ok=True)
    refs = DOC/'references'; refs.mkdir(parents=True, exist_ok=True)
    for key, _ in scenes:
        with Image.open(OUT/'engine'/f'{key}.png') as im:
            im.convert('RGB').crop((334, 50, 1552, 894)).save(OUT/'native'/f'{key}.png')
        shutil.copy2(OUT/'native'/f'{key}.png', refs/f'{key}.png')
    profiles = {}
    for p in checks['profiles']:
        profile = (p['name'], p['number'] == 24, p['reward']['level'], p['hp'],
                   p['raw_damage'], p['range'], p['reward']['xp'])
        profiles[profile] = profiles.get(profile, 0)+1
    rows = ''.join('<tr>'+''.join(f'<td>{escape(str(v))}</td>' for v in
                   [name, 'pistolet' if ranged else 'contact', level, count, hp, damage, reach, xp])+'</tr>'
                   for (name, ranged, level, hp, damage, reach, xp), count in sorted(profiles.items()))
    prefix = '../street-population-review-01/'
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>U01-POP-01 · Premier peuplement</title>
<style>*{box-sizing:border-box}body{background:#10171b;color:#d6d9d8;font:16px/1.55 system-ui;margin:0;padding:24px}
main{max-width:1280px;margin:auto}h1,h2{color:#dac39c}h1{font-size:25px}h2{font-size:20px;margin-top:30px}
a{color:#a9d1de}nav{display:flex;gap:18px;flex-wrap:wrap}code{color:#e4d2ad}table{border-collapse:collapse;width:100%}
th,td{padding:8px 12px;border-bottom:1px solid #3c4549;text-align:left}figure{margin:18px 0 36px}
.viewport{overflow:auto;max-width:100%;border:1px solid #536268;background:#000}img{display:block;max-width:none}
.note{border-left:3px solid #8d805f;padding:8px 18px;background:#1b2328}figcaption{color:#9eacad;margin:8px 0}
</style><main><h1>U01-POP-01 · Premier peuplement des Abords de la prison</h1>
<p>Essai jouable séparé : <strong>15 ennemis ordinaires répartis en cinq petits groupes</strong>.
Les six premières cases autour de l’arrivée sont libres d’ennemis au départ. Les groupes peuvent ensuite se déplacer et poursuivre le joueur.</p>
<nav><a href="#jouer">Lancer l’essai</a><a href="#valeurs">Profils de combat</a><a href="#arrival">Arrivée</a>
<a href="../street-defense-review-10/index.html">Revue précédente : poste de garde</a></nav>
<section id="jouer"><h2>Pour jouer</h2><p>Depuis le dossier du projet : <code>cargo run --locked</code>,
ou ouvrir <a href="../../Jouer.cmd">Jouer.cmd</a>, puis nouvelle partie.</p>
<p>Ce peuplement est intégré à la ville normale. Le joueur choisit sa classe au camp, puis rejoint la rue par le portail sud. L'ancien lanceur d'essai a été supprimé.
On utilise les commandes habituelles. Le portail de la prison permet de revenir au camp puis dans la rue, sans recréer les groupes.</p>
<p class="note">Les deux gardes conservent leur présentation visuelle. Leur avertissement et leur combat automatique seront ajoutés plus tard.
Les obstacles refusés ne sont pas remis. La sortie vers la carte suivante reste à construire.</p></section>
<section id="valeurs"><h2>Valeurs réutilisées pour cet essai</h2>
<p>Profils ordinaires déjà présents dans les tables de surface : aucune nouvelle capacité ni affixe.
Les dégâts ci-dessous sont les valeurs de base, avant les attributs, protections et autres modificateurs du combat.
L’équilibrage et les récompenses restent à évaluer après le test.</p>
<table><thead><tr><th>Nom</th><th>Attaque</th><th>Niveau</th><th>Nombre</th><th>PV</th><th>Dégâts de base</th><th>Portée</th><th>XP de base</th></tr></thead><tbody>'''+rows+'''</tbody></table></section>
<h2>Captures du moteur · échelle native 64 px</h2><p>Ces vues montrent les positions initiales, avant le premier tour des ennemis. Les images ne sont pas agrandies ; faire défiler horizontalement si nécessaire.</p>'''
    for key, title in scenes:
        html += f'<section id="{key}"><h2>{escape(title)}</h2><figure><div class="viewport"><img src="{prefix}native/{key}.png" width="1218" height="844" alt="{escape(title)}"></div><figcaption>Capture réelle · une case = 64 px · vision habituelle conservée.</figcaption></figure></section>'
    html += '</main></html>'
    class Refs(HTMLParser):
        def __init__(self): super().__init__(); self.refs=[]
        def handle_starttag(self, tag, attrs): self.refs.extend(v for k,v in attrs if k in ('href','src'))
    parser=Refs(); parser.feed(html)
    for ref in parser.refs:
        path=urlsplit(ref).path
        if path: assert (OUT/unquote(path)).resolve().is_file(), ref
    (OUT/'index.html').write_text(html, encoding='utf-8')
    # Reuse the current review tab; the defense review remains available above.
    (ROOT/'artifacts/street-defects-review-01/index.html').write_text(html, encoding='utf-8')
    write(DOC/'engine-checks.json', checks)
    write(DOC/'map-after.json', map_hashes())
    write(DOC/'acceptance-01.json', {
        'review':'U01-POP-01', 'date':'2026-10-20', 'user_reply':'on va essayer :)',
        'scope':'isolated_playable_trial', 'composition':plan,
        'new_art':False, 'new_powers':False, 'normal_campaign_activated':False,
        'combat_profiles':'existing simulation_overworld human_habitat and surface_wilds ordinary rules; unchanged',
        'new_final_balance_approved':False, 'guard_combat_implemented':False,
        'player_profile':checks['player_profile'],
        'normal_map_sha256_unchanged':True,
        'references':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in refs.iterdir() if p.is_file()}})
    print('Published U01-POP-01: six native scenes, fifteen enemies; normal map hashes unchanged.')

if __name__ == '__main__':
    import sys
    if '--baseline' in sys.argv:
        path=DOC/'map-before.json'
        if path.exists(): assert read(path)==map_hashes()
        else: write(path,map_hashes())
        print('Normal map baseline verified.')
    else: publish()
