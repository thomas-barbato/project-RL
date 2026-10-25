"""Package imagegen weapon candidates and source-rig poses without integration.

Painting changes come from imagegen. Pillow only frames/reduces, composites the
exported production-rig layers, adds the proposed contact shadow, and lays out
the native64 review. The active sprite banks are never written.
"""
from pathlib import Path
import hashlib
import json
import math
import shutil

from PIL import Image, ImageChops, ImageDraw, ImageFilter, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/weapon-readability-review-01'
DOC = ROOT / 'docs/reviews/weapon-readability-01'
SOURCE = ROOT / 'assets/source-art/weapon-readability-01'
NAMES = ['Bastion', 'Technomancien', 'Traqueuse', 'Arcaniste', 'Métamorphe']


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', 'utf-8')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    return json.loads(path.read_text('utf-8'))


def load(path):
    return Image.open(path).convert('RGBA')


def light_actor(im):
    """Same 75% actor lightening as the current production shader, no scene light."""
    result = im.copy()
    old = im.load()
    new = result.load()
    for y in range(im.height):
        for x in range(im.width):
            r, g, b, a = old[x, y]
            if not a:
                continue
            lum = .2126*r + .7152*g + .0722*b
            rim = (x == 0 or y == 0 or old[x-1, y][3] < 96 or old[x, y-1][3] < 96)
            gain = 9 * (1 - lum/255 + rim)
            new[x, y] = tuple(min(255, math.floor(c+gain+.5)) for c in (r, g, b)) + (a,)
    return result


def composite(layers, shadow=False):
    body, weapon, fingers = layers
    result = body.copy()
    if shadow:
        # One native pixel, restricted to the body under the weapon, no actor halo.
        alpha = weapon.getchannel('A')
        ring = ImageChops.subtract(alpha.filter(ImageFilter.MaxFilter(3)), alpha)
        ring = ImageChops.multiply(ring, body.getchannel('A'))
        shade = Image.new('RGBA', body.size, (8, 11, 15, 0))
        shade.putalpha(ring.point(lambda a: round(a*.90)))
        result = Image.alpha_composite(result, shade)
    result = Image.alpha_composite(result, weapon)
    result = Image.alpha_composite(result, fingers)
    return light_actor(result).crop((16, 16, 112, 112))


def board(candidate, cell_dir, mode='final'):
    canvas = Image.new('RGB', (610, 562), '#182126')
    draw = ImageDraw.Draw(canvas)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    draw.text((8, 8), candidate+' · 25 tenues · taille native, case logique 64 px', font=font, fill='white')
    for rank in range(5):
        draw.text((137+rank*96, 35), f'{rank+1:02}', font=font, fill='#b9c7cd')
    for cls, name in enumerate(NAMES):
        draw.text((5, 100+cls*100), name, font=font, fill='white')
        for rank in range(5):
            im = load(cell_dir/f'{candidate}-{cls*5+rank:02}-{mode}.png')
            canvas.paste(im, (116+rank*96, 55+cls*100), im)
    canvas.save(OUT/f'{candidate}-all-native.png', optimize=True)


HTML = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Armes · revue 01 · 64 px</title>
<style>
*{box-sizing:border-box}body{margin:0;background:#10181c;color:#e6ecee;font:14px/1.45 system-ui,sans-serif}
main{max-width:780px;margin:auto;padding:14px}h1{font-size:23px;line-height:1.2;margin:6px 0 10px}h2{font-size:18px;margin:22px 0 8px}p{margin:8px 0;color:#bbc8cd}
.tag{font-size:11px;letter-spacing:.09em;color:#b9d6d0}.controls{display:flex;flex-wrap:wrap;gap:8px;align-items:center;margin:12px 0}
button,select{font:inherit;color:#e6ecee;background:#243239;border:1px solid #50626a;border-radius:5px;padding:6px 8px}button.active{background:#3f655d;border-color:#94bfb4}label{display:flex;align-items:center;gap:5px}a{color:#a6d4ff}
.scroll{overflow-x:auto}.compare{width:376px;display:grid;grid-template-columns:88px repeat(3,96px);align-items:center;row-gap:4px;font-size:12px}
.head{text-align:center;min-height:38px;color:#b8cbd2}.who{padding-right:5px;font-size:12px}.tile{width:96px;height:96px;background:var(--ground,#303a40)}.tile img{width:96px;height:96px;image-rendering:pixelated;display:block}
.gray .tile img,.gray .isolated img,.gray .full{filter:grayscale(1)}.isolated{display:flex;align-items:center;gap:28px;background:var(--ground,#303a40);padding:18px;min-height:98px;width:376px}.isolated img{image-rendering:pixelated;object-fit:contain}
.note{font-size:12px;color:#a5b4bb}details{margin:18px 0}summary{cursor:pointer;color:#d9e5e7}.full{display:block;max-width:none}li{margin:6px 0}hr{border:0;border-top:1px solid #34444b;margin:22px 0}
</style><main>
<div class="tag">WEAPON-READABILITY01 · PROPOSITION À VALIDER</div>
<h1>Hache et fusil : formes et prises plus lisibles</h1>
<p>Deux nouveaux dessins, sur les cinq classes. Les personnages sont affichés à taille réelle : <strong>64 px par case</strong>, avec leurs débordements.</p>
<div class="controls"><button class="active" data-weapon="WR-AX01">01 · Hache</button><button data-weapon="WR-RF01">02 · Fusil</button></div>
<div class="controls"><label>Armure <select id="rank"><option value="0">01 · légère</option><option value="1">02</option><option value="2">03</option><option value="3">04</option><option value="4" selected>05 · lourde</option></select></label>
<label>Fond <select id="ground"><option value="#303a40">Gris</option><option value="#11191d">Sombre</option><option value="#b7b5aa">Clair</option></select></label><label><input id="gray" type="checkbox">Nuances de gris</label></div>
<p id="description"></p>
<p class="note"><a href="#compare">Afficher les cinq classes ensemble ↓</a></p>
<div class="scroll"><div class="compare" id="compare"></div></div>
<p class="note">00 : actuel · 01 : nouveau dessin + pose ajustée · 02 : même version avec ombre de séparation de 1 px sur le corps. Deux mains attachées à l’arme ; doigts au premier plan.</p>
<h2 id="isolated-title">Dessin isolé</h2><div class="scroll"><div class="isolated" id="isolated"></div></div>
<p class="note">À gauche : dessin seul à taille native. À droite : mêmes pixels au zoom ×4.</p>
<details><summary>Les 25 tenues avec la version 02, à taille native</summary><div class="scroll"><img class="full" id="all" alt="Toutes les tenues, 64 px natifs"></div></details>
<details><summary>Les quatre changements proposés</summary><ol><li>Hampe de bois continue et matériaux identifiables.</li><li>Grandes formes simplifiées : tête de hache ; crosse, corps et canon du fusil.</li><li>Fine ombre entre l’arme et la tenue, sans contour autour du personnage.</li><li>Prises déplacées avec les avant-bras : hache à la taille, fusil légèrement incliné et décalé pour dégager le canon.</li></ol></details>
<hr><p class="note">Compositions de revue à partir des couches du jeu, avec l’éclaircissement actuel reproduit. Ce sont des propositions hors jeu, sans éclairage de scène. L’intégration attend la validation des dessins et des prises.</p>
<p><a href="../actor-readability-integration-02/index.html">Revoir le rendu actuellement intégré</a> · <a href="review.json">Fiche des candidats</a></p>
</main><script>
const classes=['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe'];let weapon='WR-AX01';
function draw(){let rank=Number(document.querySelector('#rank').value);document.querySelector('#compare').innerHTML='<div></div><div class="head">00 · Actuel</div><div class="head">01 · Dessin<br>et pose</div><div class="head">02 · Avec<br>séparation</div>'+classes.map((name,i)=>'<div class="who">'+name+'</div>'+['before','drawn','final'].map(mode=>'<div class="tile"><img width="96" height="96" src="cells/'+weapon+'-'+String(i*5+rank).padStart(2,'0')+'-'+mode+'.png" alt="'+name+', armure '+(rank+1)+', '+mode+'"></div>').join('')).join('');
let axe=weapon==='WR-AX01';document.querySelector('#description').textContent=axe?'01 · WR-AX01 — Hampe en bois brun, tête simplifiée en acier, tenue horizontale à hauteur de taille.':'02 · WR-RF01 — Crosse brune, boîtier gris éclairci, chargeur sable et canon dégagé. Taille adaptée à chaque classe.';
let file=axe?'axe':'rifle';document.querySelector('#isolated').innerHTML='<img src="'+file+'-native.png" alt="Dessin isolé à taille native"><img src="'+file+'-zoom4.png" alt="Même dessin au zoom quatre">';document.querySelector('#all').src=weapon+'-all-native.png';}
document.querySelectorAll('[data-weapon]').forEach(b=>b.onclick=()=>{weapon=b.dataset.weapon;document.querySelectorAll('[data-weapon]').forEach(x=>x.classList.toggle('active',x===b));draw()});document.querySelector('#rank').onchange=draw;
document.querySelector('#ground').onchange=e=>document.body.style.setProperty('--ground',e.target.value);document.querySelector('#gray').onchange=e=>document.body.classList.toggle('gray',e.target.checked);draw();
</script></html>'''


def main():
    manifest = read(ROOT/'assets/sprites/class-armors-01/manifest.json')
    protected = {entry['path']: entry['sha256'] for entry in manifest['atlases'].values()}
    for path, digest in protected.items():
        assert sha(ROOT/path) == digest, path
    checks = read(OUT/'candidate-checks.json')
    if checks['exported_pngs_need_vertical_flip']:
        for path in (OUT/'candidate-layers').glob('*.png'):
            Image.open(path).transpose(Image.Transpose.FLIP_TOP_BOTTOM).save(path)
        checks['exported_pngs_need_vertical_flip'] = False
        write(OUT/'candidate-checks.json', checks)
    cell_dir = OUT/'cells'
    cell_dir.mkdir(exist_ok=True)
    candidates = read(OUT/'candidates.json')['candidates']
    for c in candidates:
        name = 'axe' if c['model'] == 66 else 'rifle'
        native = load(ROOT/c['native'])
        native.save(OUT/f'{name}-native.png')
        native.resize((native.width*4, native.height*4), Image.Resampling.NEAREST).save(OUT/f'{name}-zoom4.png')
        for index in range(25):
            old = [load(OUT/'sources'/f'outfit-{index:02}-weapon-{c["model"]:03}-layer-{i}.png') for i in range(3)]
            new = [load(OUT/'candidate-layers'/f'{c["id"]}-{index:02}-layer-{i}.png') for i in range(3)]
            for mode, layers, shadow in [('before', old, False), ('drawn', new, False), ('final', new, True)]:
                composite(layers, shadow).save(cell_dir/f'{c["id"]}-{index:02}-{mode}.png', optimize=True)
            # The character's head and feet stay pixel-identical outside the arms.
            for box in [(58, 32, 71, 49), (32, 83, 96, 96)]:
                assert old[0].crop(box).tobytes() == new[0].crop(box).tobytes(), (c['id'], index, box)
        board(c['id'], cell_dir)
        c['source'] = str((SOURCE/'generated'/f'{name}-01.png').relative_to(ROOT)).replace('\\', '/')
        c['source_sha256'] = sha(ROOT/c['source'])
        c['native_sha256'] = sha(ROOT/c['native'])
    record = dict(review_id='WEAPON-READABILITY01',
                  status='accepted' if (DOC/'acceptance.json').exists() else 'awaiting_visual_approval',
                  native_tile=64, outfits=25, candidates=candidates, runtime_integration=False,
                  arms='existing accepted source limbs transformed with the production geometry',
                  hands='100 candidate grip contacts checked against weapon alpha',
                  shadow='1 native px, clipped to the body, 90 percent opacity',
                  lighting='current actor shader lightening reproduced; no scene lighting',
                  source_tool='built-in imagegen', protected_active_atlases=protected,
                  actual_before_layers='read-only export of the current production compositor',
                  comparisons=150, controls=['weapon','armor stage','dark/light ground','grayscale'])
    write(DOC/'review.json', record)
    write(OUT/'review.json', record)
    (OUT/'index.html').write_text(HTML, 'utf-8')
    (DOC/'README.md').write_text(
        '# WEAPON-READABILITY01 — proposition\n\n'
        'Le joueur demande les quatre améliorations proposées et exige de voir les dessins avant intégration.\n'
        'Candidats : 01 WR-AX01 (hache), 02 WR-RF01 (fusil). Les sources imagegen et exports natifs sont conservés dans assets/source-art/weapon-readability-01/.\n\n'
        'Comparer 00 actuel, 01 dessin et pose, 02 avec ombre, sur les 25 tenues. Aucun candidat intégré.\n'
        'Aperçu : artifacts/weapon-readability-review-01/index.html. Les couches avant proviennent du compositeur actif ; les poses candidates utilisent sa géométrie dans un diagnostic ignoré.\n'
        'Les identités, statistiques et sauvegardes restent inchangées. Les autres modèles attendent la validation de ce premier essai.\n', 'utf-8')
    print(json.dumps(dict(candidates=2, outfits=25, comparisons=150,
                         active_atlases_verified=len(protected), runtime_integration=False)))


if __name__ == '__main__':
    main()
