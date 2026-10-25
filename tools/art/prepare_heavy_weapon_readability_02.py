"""Add four heavy-weapon proposals to the current native64 weapon review."""
from pathlib import Path
import json
import re

from PIL import Image, ImageDraw, ImageFont

from prepare_weapon_readability_review_01 import (
    ROOT, OUT, NAMES, HTML, read, write, sha, load, composite, board,
)

DOC = ROOT/'docs/reviews/weapon-readability-02'
SOURCE = ROOT/'assets/source-art/weapon-readability-02'
HEAVY = [
    ('WR-MG02', 'machinegun', '03 · Mitrailleuse', 'Mitrailleuse lourde',
     'Bloc moteur gris, boîte de munitions ocre et faisceau de canons dégagé. Les deux prises restent sur le corps de l’arme.'),
    ('WR-FT02', 'flamethrower', '04 · Lance-flammes', 'Lance-flammes · Korgan F-100',
     'Corps métallique clair, réservoir roux et embout cylindrique distinct. Deux poignées visibles, durite raccourcie.'),
    ('WR-GL02', 'grenade', '05 · Lance-grenades', 'Lance-grenades',
     'Gros barillet gris, crosse olive, canon court et deux poignées dégagées.'),
    ('WR-RL02', 'rocket', '06 · Lance-roquettes', 'Lance-roquettes',
     'Long tube gris avec arête claire, avant olive et anneau rouge. Deux prises sous le tube.'),
]


def overview(cell_dir):
    # Every actor stays at native scale; transparent margins alone are cropped.
    im = Image.new('RGB', (432, 548), '#182126')
    draw = ImageDraw.Draw(im)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 11)
    draw.text((6, 5), '03–06 · Armes lourdes · 64 px natifs · armures 05', font=font, fill='white')
    for col, label in enumerate(['03 · Mitr.', '04 · Flam.', '05 · Gren.', '06 · Roq.']):
        draw.text((84+col*88, 25), label, font=font, fill='#c1d2d7')
    for cls, name in enumerate(NAMES):
        draw.text((3, 79+cls*100), name, font=font, fill='white')
        for col, (candidate, *_rest) in enumerate(HEAVY):
            sprite = load(cell_dir/f'{candidate}-{cls*5+4:02}-final.png').crop((4, 0, 92, 96))
            im.paste(sprite, (80+col*88, 43+cls*100), sprite)
    im.save(OUT/'heavy-overview-native.png', optimize=True)


def page():
    weapons = [
        dict(id='WR-AX01', key='axe', label='01 · Hache ✓', title='Hache', approved=True,
             description='Hampe en bois brun, tête simplifiée en acier, tenue horizontale à hauteur de taille.'),
        dict(id='WR-RF01', key='rifle', label='02 · Fusil ✓', title='Fusil', approved=True,
             description='Crosse brune, boîtier gris éclairci, chargeur sable et canon dégagé. Taille adaptée à chaque classe.'),
    ] + [dict(id=ident, key=key, label=label, title=title, approved=False, description=description)
         for ident, key, label, title, description in HEAVY]
    buttons = ''.join(f'<button data-weapon="{w["id"]}">{w["label"]}</button>' for w in weapons)
    text = HTML.replace('<title>Armes · revue 01 · 64 px</title>', '<title>Armes lourdes · revue 02 · 64 px</title>')
    text = text.replace('WEAPON-READABILITY01 · PROPOSITION À VALIDER', 'WEAPON-READABILITY02 · ARMES LOURDES À VALIDER')
    text = text.replace('Hache et fusil : formes et prises plus lisibles', 'Armes lourdes : quatre silhouettes à comparer')
    text = text.replace('Deux nouveaux dessins, sur les cinq classes.', '03 à 06 : quatre familles d’armes lourdes sur les cinq classes. La hache et le fusil (01–02) sont validés.')
    text = text.replace('<p id="description"></p>', '<p id="description"></p><p class="note"><a href="#heavy-overview" id="show-overview">Comparer les quatre armes lourdes ↓</a></p>')
    text = re.sub(r'<div class="controls"><button.*?</div>', '<div class="controls">'+buttons+'</div>', text, count=1)
    text = text.replace('<h2 id="isolated-title">', '<details id="heavy-overview"><summary>Vue d’ensemble des quatre armes lourdes, à taille native</summary><div class="scroll"><img class="full" src="heavy-overview-native.png" width="432" height="548" alt="Armes lourdes 03 à 06 sur les cinq classes, sans réduction"></div></details><h2 id="isolated-title">')
    text = text.replace('Hampe de bois continue et matériaux identifiables.', 'Matériaux identifiables par de grandes faces claires et sombres.')
    text = text.replace('Grandes formes simplifiées : tête de hache ; crosse, corps et canon du fusil.', 'Chaque famille garde sa silhouette : faisceau de canons, réservoir, barillet ou tube long.')
    text = text.replace('Prises déplacées avec les avant-bras : hache à la taille, fusil légèrement incliné et décalé pour dégager le canon.', 'Avant-bras déplacés avec les poignées ; tailles adaptées aux cinq classes, sans modifier leur corpulence.')
    text = text.replace('L’intégration attend la validation des dessins et des prises.', 'Les propositions 03–06 attendent ta validation. Les dessins et prises 01–02 sont validés et conservés.')
    text = text.replace('<a href="review.json">Fiche des candidats</a>', '<a href="heavy-review.json">Fiche des armes lourdes</a> · <a href="../../docs/reviews/weapon-readability-02/prompts.json">Sources et prompts imagegen</a>')
    script = '''<script>
const weapons=WEAPONS;
const classes=['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe'];
let weapon=weapons.find(w=>w.id==='WR-MG02');
function draw(){
let rank=Number(document.querySelector('#rank').value);
document.querySelector('#compare').innerHTML='<div></div><div class="head">00 · Actuel</div><div class="head">01 · Dessin<br>et pose</div><div class="head">02 · Avec<br>séparation</div>'+classes.map((name,i)=>'<div class="who">'+name+'</div>'+['before','drawn','final'].map(mode=>'<div class="tile"><img width="96" height="96" src="cells/'+weapon.id+'-'+String(i*5+rank).padStart(2,'0')+'-'+mode+'.png" alt="'+name+', armure '+(rank+1)+', '+mode+'"></div>').join('')).join('');
document.querySelector('#description').textContent=weapon.title+' · '+weapon.id+' — '+weapon.description+(weapon.approved?' [VALIDÉ]':' [À VALIDER]');
document.querySelector('#isolated').innerHTML='<img src="'+weapon.key+'-native.png" alt="Dessin isolé à taille native"><img src="'+weapon.key+'-zoom4.png" alt="Même dessin au zoom quatre">';
document.querySelector('#all').src=weapon.id+'-all-native.png';
document.querySelectorAll('[data-weapon]').forEach(b=>b.classList.toggle('active',b.dataset.weapon===weapon.id));
}
document.querySelectorAll('[data-weapon]').forEach(b=>b.onclick=()=>{weapon=weapons.find(w=>w.id===b.dataset.weapon);draw()});
document.querySelector('#rank').onchange=draw;
document.querySelector('#ground').onchange=e=>document.body.style.setProperty('--ground',e.target.value);
document.querySelector('#gray').onchange=e=>document.body.classList.toggle('gray',e.target.checked);
document.querySelector('#show-overview').onclick=()=>{document.querySelector('#heavy-overview').open=true};
draw();
</script>'''.replace('WEAPONS', json.dumps(weapons, ensure_ascii=False))
    return re.sub(r'<script>.*?</script>', script, text, flags=re.S)


def main():
    prior = read(ROOT/'docs/reviews/weapon-readability-01/accepted-cell-hashes.json')
    for path, digest in prior.items():
        assert sha(ROOT/path) == digest, 'Approved composition changed: '+path
    manifest = read(ROOT/'assets/sprites/class-armors-01/manifest.json')
    for ref in manifest['atlases'].values():
        assert sha(ROOT/ref['path']) == ref['sha256']
    checks = read(OUT/'candidate-checks.json')
    if checks['exported_pngs_need_vertical_flip']:
        for path in (OUT/'candidate-layers').glob('*.png'):
            Image.open(path).transpose(Image.Transpose.FLIP_TOP_BOTTOM).save(path)
        checks['exported_pngs_need_vertical_flip'] = False
        write(OUT/'candidate-checks.json', checks)
    candidates = {c['id']: c for c in read(OUT/'candidates.json')['candidates']}
    head_masks = load(ROOT/manifest['atlases']['head-mask']['path'])
    head_columns = head_masks.width//128
    selected = []
    cell_dir = OUT/'cells'
    for ident, key, label, title, description in HEAVY:
        c = candidates[ident]
        native = load(ROOT/c['native'])
        native.save(OUT/f'{key}-native.png')
        native.resize((native.width*4, native.height*4), Image.Resampling.NEAREST).save(OUT/f'{key}-zoom4.png')
        for index in range(25):
            old = [load(OUT/'sources'/f'outfit-{index:02}-weapon-{c["model"]:03}-layer-{layer}.png') for layer in range(3)]
            new = [load(OUT/'candidate-layers'/f'{ident}-{index:02}-layer-{layer}.png') for layer in range(3)]
            for mode, layers, shadow in [('before',old,False), ('drawn',new,False), ('final',new,True)]:
                composite(layers, shadow).save(cell_dir/f'{ident}-{index:02}-{mode}.png', optimize=True)
            head = head_masks.crop((index%head_columns*128, index//head_columns*128,
                                    index%head_columns*128+128, index//head_columns*128+128)).getchannel('A')
            # Use the actual head ownership mask: a fixed rectangle also catches
            # neighboring sleeve-edge pixels which should move with the arm.
            old_bytes, new_bytes = old[0].tobytes(), new[0].tobytes()
            assert all(old_bytes[n*4:n*4+4] == new_bytes[n*4:n*4+4]
                       for n,owned in enumerate(head.tobytes()) if owned), (ident,index,'head')
            assert old[0].crop((32,83,96,96)).tobytes() == new[0].crop((32,83,96,96)).tobytes(), (ident,index,'feet')
        board(ident, cell_dir)
        c.update(label=label, title=title, description=description,
                 source=f'assets/source-art/weapon-readability-02/generated/{key}-02.png')
        c['source_sha256'] = sha(ROOT/c['source'])
        c['native_sha256'] = sha(ROOT/c['native'])
        selected.append(c)
    overview(cell_dir)
    for path, digest in prior.items():
        assert sha(ROOT/path) == digest
    record = dict(review_id='WEAPON-READABILITY02', status='awaiting_visual_approval',
                  native_tile=64, outfits=25, candidates=selected, runtime_integration=False,
                  accepted_previous_candidates=['WR-AX01','WR-RF01'], accepted_previous_cells_preserved=len(prior),
                  unchanged_active_atlases=23, heavy_family_count=4, existing_heavy_models=27,
                  source_tool='built-in imagegen', heavy_grip_contacts_checked=200,
                  source_limbs='accepted production-rig sleeves, reposed with production geometry',
                  shadow='1 native px, clipped to the body, same as approved first wave',
                  color_variants='not created', game_rules_changes=False, head_and_feet_checks=200,
                  scope='Four representative family designs; other statistical model identities are preserved.',
                  lighting='current actor lightening reproduced, no scene lighting', review_cells=300)
    write(DOC/'review.json', record)
    write(OUT/'heavy-review.json', record)
    (OUT/'index.html').write_text(page(), 'utf-8')
    (DOC/'README.md').write_text(
        '# WEAPON-READABILITY02 — armes lourdes\n\n'
        'Le joueur valide la hache WR-AX01 et le fusil WR-RF01, puis demande à voir les armes lourdes.\n'
        '03 WR-MG02 : mitrailleuse lourde (modèle 31). 04 WR-FT02 : Korgan F-100, lance-flammes (17).\n'
        '05 WR-GL02 : lance-grenades (23). 06 WR-RL02 : lance-roquettes (51).\n\n'
        'Comparer 00 actuel, 01 dessin et pose, 02 ombre de séparation, sur les 25 tenues en 64 px.\n'
        'Les quatre familles représentent 27 modèles existants ; cette revue ne les remplace pas automatiquement.\n'
        'Les quatre candidats restent à valider. Les 150 cellules de la première vague restent identiques.\n'
        'Aucun changement des ressources actives, statistiques, identités ou sauvegardes pendant cette revue.\n'
        'Aperçu courant : artifacts/weapon-readability-review-01/index.html. Sources et prompts : prompts.json.\n', 'utf-8')
    print(json.dumps(dict(new_candidates=4, heavy_outfit_cases=100, grip_contacts=200,
                         new_comparisons=300, approved_comparisons_preserved=len(prior), runtime_integration=False)))


if __name__ == '__main__':
    main()
