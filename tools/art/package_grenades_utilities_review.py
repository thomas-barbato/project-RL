"""Native utility-grenade gallery. Reuses approved drawings without rewriting them."""
import hashlib
import json
import sys
from pathlib import Path
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
REVIEW = int(sys.argv[1]) if len(sys.argv)>1 else 74
assert REVIEW in (74,75)
OUT = ROOT/f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{REVIEW}'
PREVIOUS = ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-74'
if (ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-75/visual-acceptance-01.json').exists():
    raise SystemExit('Revue 75 validée : conserver ces captures et leurs références de la revue 74 ; préparer une nouvelle révision pour toute modification.')
read = lambda p: json.loads(p.read_text(encoding='utf8'))
def write(p, data):
    p.write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf8')

entries = read(OUT/'native/capture.json')
assert len(entries) == 48
override=OUT/'native-lure/capture.json'
if override.exists():
    revised={e['variant']:e for e in read(override)}
    assert len(revised)==12
    entries=[dict(revised[e['variant']],native_source='native-lure') if e['id']=='SG02' else dict(e,native_source='native') for e in entries]
previous_by_case = {(e['id'], e['case'], e['direction']):e for e in read(PREVIOUS/'animations.json')} if REVIEW==75 else {}
eye_override=OUT/'native-eye/capture.json'
if REVIEW==75 and eye_override.exists():
    revised={e['variant']:e for e in read(eye_override)}
    assert len(revised)==12
    entries=[dict(revised[e['variant']],native_source='native-eye') if e['id']=='SG01' else e for e in entries]
def rotate(x,y,d): return [(8-y,x),(y,8-x),(x,y),(8-x,8-y)][d]
checks = []
for e in entries:
    native = OUT/e.get('native_source','native')/e['folder']
    timeline = read(native/'timeline.json')
    previous = previous_by_case.get((e['id'], e['case'], e['direction']))
    previous_native = PREVIOUS/previous['native_source']/previous['folder'] if previous else None
    if previous:
        assert timeline == previous['timeline'], ('gameplay timeline changed',e['variant'])
    def native_image(name):
        current = native/name
        if current.exists(): return current
        assert previous_native is not None
        preserved = previous_native/name
        assert preserved.exists(),preserved
        return preserved
    folder = OUT/'frames'/e['variant']; folder.mkdir(parents=True, exist_ok=True)
    indices, known, images, provenance = [], {}, [], []
    for n in range(e['frames']):
        source = native_image(f'{n:03}.png')
        provenance.append(str(source.relative_to(ROOT)))
        im = Image.open(source).convert('RGB').crop((32,88,608,664))
        images.append(im)
        key = hashlib.sha256(im.tobytes()).hexdigest()
        if key not in known:
            known[key] = n
            im.save(folder/f'{n:03}.png', compress_level=1)
        indices.append(known[key])
    e.update(image_indices=indices,timeline=timeline,native_source=e.get('native_source','native'))
    if REVIEW==75: e.update(native_frame_sources=provenance,previous_variant=previous['variant'])
    proof = ImageChops.difference(Image.open(native_image('effect-proof-027.png')).convert('RGB'), Image.open(native_image('terrain-proof-027.png')).convert('RGB')).crop((32,88,608,664))
    for y,row in enumerate(e['map'].splitlines()):
        for x,c in enumerate(row):
            if c == '#':
                tx,ty=rotate(x,y,e['direction'])
                assert proof.crop((tx*64,ty*64,(tx+1)*64,(ty+1)*64)).getbbox() is None,(e['variant'],x,y)
    if e['case'] == 2:
        assert all(t['energy']==100 and t['turn']==0 and t['preview'] for t in timeline)
        assert proof.getbbox() is None
    else:
        assert timeline[18]['energy']==92 and not timeline[18]['preview']
        assert timeline[40]['turn']==timeline[63]['turn']==1
        assert timeline[63]['cooldown']==2 and timeline[64]['cooldown']==1 and timeline[88]['cooldown']==0
        assert timeline[40]['hp']==timeline[63]['hp']==timeline[-1]['hp']
        assert proof.getbbox() is not None
        if e['id'] == 'SG01':
            assert timeline[40]['blindness']==timeline[63]['blindness']==[12,12,0]
            assert timeline[64]['blindness']==[11,11,0]
            assert timeline[88]['blindness']==[10,10,0]
            assert timeline[112]['blindness']==[0,0,0]
        elif e['id'] == 'SG02':
            assert timeline[40]['noise_turns']==timeline[63]['noise_turns']==2
            assert timeline[64]['noise_turns']==1 and timeline[88]['noise_turns']==0
            assert timeline[40]['attempts']==timeline[-1]['attempts']==[1,1,0]
            assert timeline[88]['attracted'][0] and timeline[112]['positions']!=timeline[88]['positions']
        else:
            assert timeline[40]['hp']==([50,75,100] if e['id']=='SG04' else [76,88,100])
    if e['id'] in ('SG01','SG02'): assert all(t['hp']==[100,100,100] for t in timeline)
    montage=Image.new('RGB',(576*4,576))
    for i,n in enumerate([12,21,27,67]): montage.paste(images[n],(i*576,0))
    palette=montage.quantize(colors=256,method=Image.Quantize.MEDIANCUT)
    frames=[im.quantize(palette=palette,dither=Image.Dither.NONE) for im in images]
    frames[0].save(OUT/f"{e['variant']}-64.gif",save_all=True,append_images=frames[1:],duration=[50 if i%3==0 else 40 for i in range(len(frames))],loop=0,disposal=2,optimize=False)
    checks.append({'variant':e['variant'],'walls_clip_effect':True,'idle_does_not_advance_turn':True,'invalid_cast_atomic':e['case']==2,'previous_gameplay_timeline_identical':bool(previous)})
write(OUT/'animations.json', entries)
write(OUT/'native-verification.json', {'views':48,'tile':64,'directions':4,'checks':checks})
player=(ROOT/'tools/art/grenades_utilities_player.js').read_text(encoding='utf8')
(OUT/'player.js').write_text('const REVIEW='+str(REVIEW)+';\nconst ENTRIES='+json.dumps(entries,ensure_ascii=False).replace('</','<\\/')+';\n'+player,encoding='utf8')
html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>Grenades · Aveuglement, Leurre et puissance</title><style>
html{color-scheme:dark}body{margin:0;background:#0d151b;color:#dce4e8;font:16px system-ui,sans-serif}main{max-width:1320px;margin:auto;padding:22px}h1{font-size:27px;margin:0 0 12px}h2{font-size:21px;margin:0 0 7px}p{line-height:1.5;margin:8px 0}.controls{display:flex;flex-wrap:wrap;gap:10px;align-items:center;margin:15px 0}button,select{font:inherit;background:#21323e;color:#e2eaef;border:1px solid #5b6e7c;border-radius:4px;padding:9px;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:24px}.card{background:#14212a;padding:16px;border:1px solid #3b5060;overflow:auto}canvas{display:block;width:100%;max-width:576px;aspect-ratio:1;background:#080e12;margin:15px auto}.native canvas{width:576px;max-width:none}.note{color:#a9bdc8}.badge{font-size:14px;color:#88d5b5}.details{display:grid;grid-template-columns:repeat(3,1fr);gap:22px;margin-top:20px}a{color:#8acdee}.status{font-size:15px;min-height:70px}@media(max-width:850px){.pair,.details{grid-template-columns:1fr}}
</style><main><h1>Grenades · aveuglement, Leurre et puissance</h1><p>ANIM-S34 · 75 / SG01-A01 · 76 / SG02-A01 · 78 / SG04-A01 · cases natives de 64 px</p>
<p>Deux variantes sans dégâts, puis une comparaison de l’explosion validée avec et sans Puissance des grenades. Les grenades cylindriques conservent leurs dessins et couleurs validés.</p>
<div class="controls"><label>Aperçu <select id="pair-mode"><option value="0">Aveuglante et Leurre</option><option value="1">Sans passif et Puissance des grenades</option></select></label><label>Scène <select id="scene"><option value="0">Terrain ouvert</option><option value="1">Au bord d’une cloison</option><option value="2">Trajectoire bloquée</option></select></label><label>Direction <select id="direction"><option value="2">Haut</option><option value="0">Droite</option><option value="1">Gauche</option><option value="3">Bas</option></select></label><label><input type="checkbox" id="native" checked> 64 px réels</label></div>
<div class="controls"><button id="replay">Rejouer la démo</button><button id="reset">Prévisualiser</button><button id="cast">Lancer</button><button id="next">Tour suivant</button><button id="expiry">Passer les 10 tours restants</button><button id="pause">Pause</button><label><input type="checkbox" id="auto" checked> Démo répétée</label><label>Vitesse <select id="speed"><option value="1">Normale</option><option value="0.5">Ralenti ×0,5</option><option value="0.25">Ralenti ×0,25</option></select></label></div>
<div class="pair native" id="pair"><section class="card"><h2 id="title0"></h2><p class="note" id="detail0"></p><canvas width="576" height="576"></canvas><p class="status" id="state0"></p></section><section class="card"><h2 id="title1"></h2><p class="note" id="detail1"></p><canvas width="576" height="576"></canvas><p class="status" id="state1"></p></section></div><p class="badge" id="phase">Chargement des captures natives…</p>
<p class="note">En mode manuel, attendre ne diminue ni les statuts ni la recharge. Le Leurre reste animé pendant ses deux tours d’émission. La démo passe ensuite deux tours, puis dix tours pour montrer la fin de l’aveuglement ; chaque passage est indiqué dans les compteurs.</p>
<div class="details"><section><h2>75 · Grenade aveuglante</h2><p>Éclat blanc bref, puis œil barré et tours restants sur les cibles affectées. Précision réduite de 30 %. Rang effectif 11 : aveuglement de 12 tours.</p><p>Aucun dégât. Les applications sur les mannequins sont préparées pour montrer le statut ; cet aperçu ne remplace pas les jets de résistance. Boss immunisés. L’éligibilité du joueur à son propre aveuglement reste à définir.</p></section>
<section><h2>76 · Leurre</h2><p>Grenade cylindrique bleue, ondes ambrées, bruit pendant 2 tours. Rang 1 utilisé ici pour voir les limites du champ sonore : 3 cases en terrain dégagé.</p><p>Le champ sonore vient de la propagation réelle sur la carte. Les murs atténuent le bruit. Les mannequins ordinaires attirés se rapprochent au passage des tours et continuent après l’arrêt de l’émission. Aucun dégât.</p><p>Une tentative d’attraction par cible. Les élites, priorités de combat et cas d’occupation du point d’impact ne sont pas simulés dans cette scène.</p></section>
<section><h2>78 · Puissance des grenades</h2><p>Passif permanent au rang effectif 11 : +110 % aux dégâts. Le projectile, la taille du carré 3×3 et l’explosion restent identiques.</p><p>Exemple préparé, avant défenses : 24 au centre / 12 au bord sans passif, puis 50 / 25 avec le passif. Le bonus n’ajoute aucun dégât à l’Aveuglante ou au Leurre.</p></section></div>
<p class="note">Nouveaux rendus proposés à la validation. Laboratoire séparé de la campagne ; les mannequins actuels seront adaptés après les animations.</p><p><a id="gif0" href="">GIF gauche</a> · <a id="gif1" href="">GIF droite</a> · <a href="review.json">Périmètre de revue et dessins conservés</a> · <a href="../../../docs/REVUE_ANIMATIONS_COMPETENCES_AGENT.md">Journal de revue</a></p><script src="player.js"></script></main></html>'''
if REVIEW==75:
    html=html.replace('ANIM-S34 · 75 / SG01-A01 · 76 / SG02-A01 · 78 / SG04-A01',
        'ANIM-S34-REV02 · 75 / SG01-A02 · 76 / SG02-A02 · 78 / SG04-A02')
    html=html.replace('Les grenades cylindriques conservent leurs dessins et couleurs validés.',
        'Les grenades du plateau sont redessinées en aplats simples, avec une silhouette d’environ 18 px de haut. Les icônes de l’arbre de compétences restent celles déjà validées.')
    html=html.replace('<div class="controls"><label>Aperçu',
        '<p class="controls"><img src="eye-symbol.svg" width="34" height="30" alt="Œil barré rouge"> Nouveau marqueur d’aveuglement : un œil avec pupille et barre rouge, puis les tours restants.</p><div class="controls"><label>Aperçu',1)
    html=html.replace('Périmètre de revue et dessins conservés', 'Périmètre de la correction')
    html=html.replace('<a href="../../../docs/REVUE_ANIMATIONS_COMPETENCES_AGENT.md">Journal de revue</a>',
        '<a href="generation.json">Dessins, source et prompt Imagegen</a> · <a href="../skill-animation-review-74/cote-a-cote.html">Version précédente</a>')
for name in ('index.html','cote-a-cote.html'): (OUT/name).write_text(html,encoding='utf8')
sources=[
    ROOT/'artifacts/refonte-agent-2026-10-06/skill-icon-review-53/SG01-C02-master.png',
    ROOT/'artifacts/refonte-agent-2026-10-06/skill-icon-review-53/SG02-C02-master.png',
    ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-72/grenade-source.png',
    ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-72/fragment-burst-source.png',
]
record={'batch':'ANIM-S34' if REVIEW==74 else 'ANIM-S34-REV02','date':'2026-10-14','status':'awaiting_visual_review','candidates':[{'number':75,'id':'SG01','variant':'SG01-A01' if REVIEW==74 else 'SG01-A02'},{'number':76,'id':'SG02','variant':'SG02-A01' if REVIEW==74 else 'SG02-A02'},{'number':78,'id':'SG04','variant':'SG04-A01' if REVIEW==74 else 'SG04-A02'}],'campaign_integration':False,'new_rules':False,'prepared_responses':True,'source_drawings_reused':[{ 'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}for p in sources],'player_self_blindness':'not_decided','probability_and_campaign_AI':'not_simulated'}
if REVIEW==74:
    record.update(reference_only='SG00-A02',grenade_rendering='Black background keyed only by the runtime shader; source images unchanged.')
else:
    record.update(reference_mechanics_only='SG00-A03; projectile also revised',grenade_rendering='Simplified RGBA source sprites, 18 native pixels tall, original shapes and colors preserved.',status_symbol='Almond eye, visible pupil, red diagonal bar; runtime code-native glyph, SVG reference.',generation='generation.json',same_gameplay_timelines=48,previous_gallery_preserved='../skill-animation-review-74/cote-a-cote.html')
write(OUT/'review.json',record)
write(OUT/'progress-audit.json',{'previous_unshown':36 if REVIEW==74 else 33,'newly_shown':['SG01','SG02','SG04'] if REVIEW==74 else [],'revision_only':REVIEW==75,'remaining_unshown':33,'status':'awaiting_visual_review'})
print('48 vues natives conditionnées ; correction visuelle à valider.' if REVIEW==75 else '48 vues natives conditionnées ; trois nouveaux candidats, 33 animations encore à présenter.')
