"""Pack native animated grenade review; no redraw of actors or VFX pixels."""
import hashlib
import json
import sys
import os
from pathlib import Path
from PIL import Image, ImageChops

ROOT=Path(__file__).resolve().parents[2]
REVIEW=int(sys.argv[1]) if len(sys.argv)>1 else 72
assert REVIEW in (72,73)
OUT=ROOT/f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{REVIEW}'
NATIVE=sys.argv[2] if len(sys.argv)>2 else 'native'
assert (OUT/NATIVE).resolve().is_relative_to(OUT.resolve())
read=lambda p:json.loads(p.read_text(encoding='utf8'))
entries=read(OUT/NATIVE/'capture.json')
checks=[]
def rotate(x,y,d):
    return [(8-y,x),(y,8-x),(x,y),(8-x,8-y)][d]
def gif(path,frames):
    palette_source=Image.new('RGB',(576*4,576))
    for i,n in enumerate([12,20,27,35]):palette_source.paste(frames[n],(i*576,0))
    palette=palette_source.quantize(colors=256,method=Image.Quantize.MEDIANCUT)
    converted=[f.quantize(palette=palette,dither=Image.Dither.NONE)for f in frames]
    converted[0].save(path,save_all=True,append_images=converted[1:],duration=[40 if i%3 else 50 for i in range(len(frames))],loop=0,disposal=2,optimize=False)
for e in entries:
    native=OUT/NATIVE/e['folder'];timeline=read(native/'timeline.json')
    target=OUT/'frames'/e['variant'];target.mkdir(parents=True,exist_ok=True)
    indices=[];known={};images=[]
    for n in range(e['frames']):
        im=Image.open(native/f'{n:03}.png').convert('RGB').crop((32,88,608,664));images.append(im)
        key=hashlib.sha256(im.tobytes()).hexdigest()
        if key not in known:known[key]=n;im.save(target/f'{n:03}.png',compress_level=1)
        indices.append(known[key])
    e.update(image_indices=indices,timeline=timeline,native_source=NATIVE)
    proof=ImageChops.difference(Image.open(native/'effect-proof-027.png').convert('RGB'),Image.open(native/'terrain-proof-027.png').convert('RGB')).crop((32,88,608,664))
    for y,row in enumerate(e['map'].splitlines()):
        for x,c in enumerate(row):
            if c=='#':
                tx,ty=rotate(x,y,e['direction']);assert proof.crop((tx*64,ty*64,(tx+1)*64,(ty+1)*64)).getbbox()is None,(e['variant'],x,y)
    assert all(t['player_hp']==100 for t in timeline)
    if e['case']==2:
        assert all(t['energy']==100 and t['turn']==0 and not t['projectile'] and not t['explosion'] for t in timeline)
        assert proof.getbbox()is None
    else:
        assert timeline[18]['energy']==92 and timeline[18]['cooldown']==2
        assert not timeline[18]['preview']
        assert sum(t['projectile']for t in timeline)>=4
        assert sum(t['explosion']for t in timeline)>=9
        assert sum(timeline[40]['contacts'])==2
        assert timeline[40]['hp']==timeline[63]['hp']==timeline[-1]['hp']
        if REVIEW==73:
            assert e['falloff']
            assert all(hp==100-damage for hp,damage in zip(timeline[40]['hp'],timeline[40]['damage']))
            if e['case']==0:
                assert len(e['mechanical_cells'])==9
                assert sorted(d for d in timeline[40]['damage'] if d)==[12,24]
        assert timeline[40]['turn']==timeline[63]['turn']==1
        assert timeline[63]['cooldown']==2 and timeline[64]['cooldown']==1 and timeline[88]['cooldown']==0
        assert not timeline[40]['projectile'] and not timeline[40]['explosion']
        assert proof.getbbox()is not None
    gif(OUT/f"{e['variant']}-64.gif",images)
    checks.append({'variant':e['variant'],'tile':64,'walls_stop_painted_burst':True,'one_response_per_target':True,'player_protected':True,'ephemeral_effect_finishes':True,'waiting_changes_no_turn_or_health':True,'invalid_cast_atomic':e['case']==2})
(OUT/'animations.json').write_text(json.dumps(entries,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
player=(ROOT/'tools/art/grenades_review_player.js').read_text(encoding='utf8')
script='const ENTRIES='+json.dumps(entries,ensure_ascii=False).replace('</','<\\/')+';\n'+player
(OUT/'player.js').write_text(script,encoding='utf8')
html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>Grenades · lot 33</title><style>html{color-scheme:dark}body{margin:0;background:#0d151b;color:#dce4e8;font:16px system-ui,sans-serif}main{max-width:1320px;margin:auto;padding:22px}h1{font-size:27px;margin:0 0 12px}h2{font-size:21px;margin:0 0 7px}p{line-height:1.5;margin:8px 0}.controls{display:flex;flex-wrap:wrap;gap:10px;align-items:center;margin:15px 0}button,select{font:inherit;background:#21323e;color:#e2eaef;border:1px solid #5b6e7c;border-radius:4px;padding:9px;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:24px}.card{background:#14212a;padding:16px;border:1px solid #3b5060}canvas{display:block;width:100%;max-width:576px;aspect-ratio:1;background:#080e12;margin:15px auto}.native canvas{width:576px;max-width:none}.card{overflow:auto}.note{color:#a9bdc8}.badge{font-size:14px;color:#88d5b5}.details{display:grid;grid-template-columns:1fr 1fr;gap:22px;margin-top:20px}a{color:#8acdee}.status{font-size:15px;min-height:50px}input[type=range]{width:180px}@media(max-width:850px){.pair,.details{grid-template-columns:1fr}}</style><main>
<h1>Grenades · Lancer et portée améliorée</h1><p>ANIM-S33 · candidats 74 / SG00-A01 et 77 / SG03-A01 · cases natives de 64 px</p><p>À gauche, le lancer atteint le groupe proche. À droite, le même lancer bénéficie de Portée de lancer et atteint le groupe plus éloigné. Le passif conserve le projectile, les dégâts et l’explosion.</p>
<div class="controls"><label>Scène <select id="scene"><option value="0">Deux groupes · portée</option><option value="1">Explosion contre une cloison</option><option value="2">Trajectoire bloquée</option><option value="3">Près du joueur</option></select></label><label>Direction <select id="direction"><option value="2">Haut</option><option value="0">Droite</option><option value="1">Gauche</option><option value="3">Bas</option></select></label><label><input type="checkbox" id="native" checked> Affichage 64 px réels</label></div>
<div class="controls"><button id="replay">Rejouer la démo</button><button id="reset">Prévisualiser</button><button id="cast">Lancer</button><button id="next">Tour suivant</button><button id="pause">Pause</button><label><input type="checkbox" id="auto" checked> Démo répétée</label><label>Vitesse <select id="speed"><option value="1">Normale</option><option value="0.5">Ralenti ×0,5</option><option value="0.25">Ralenti ×0,25</option></select></label></div>
<div class="pair native" id="pair"><section class="card"><h2>74 · Lancer de grenade</h2><p class="note">Portée : 4 cases · sans Portée de lancer</p><canvas width="576" height="576"></canvas><p class="status" id="state0"></p></section><section class="card"><h2>77 · Portée de lancer</h2><p class="note">Portée : 8 cases · passif au rang effectif 11</p><canvas width="576" height="576"></canvas><p class="status" id="state1"></p></section></div>
<p class="badge" id="phase">Chargement des captures natives…</p><p class="note">La démo répétée réinitialise la scène. En mode manuel, attendre anime le projectile et l’impact sans avancer les compteurs. Tour suivant passe une action préparée. La zone de prévisualisation disparaît après un lancer validé.</p>
<div class="details"><section><h2>Lancer de grenade · effet actuel</h2><p><em>Vous lancez une grenade au milieu des ennemis pour semer le chaos.</em></p><p>Variante explosive · rayon d’une case · énergie : 8 · recharge : 2 tours.</p><p>Choisissez une case visible à portée. Les murs bloquent la trajectoire. Le joueur est protégé de ses propres dégâts.</p><p>Achat unique : un point. Les nombres affichés sont préparés pour la revue : cibles à 100 PV et dégâts physiques de 24, sans nouveau réglage d’équilibrage.</p></section><section><h2>Portée de lancer · rang effectif 11</h2><p><em>Vous projetez vos grenades avec plus d’efficacité.</em></p><p>Portée supplémentaire : 4 cases. Portée totale du lancer : 8 cases.</p><p>Passif permanent nécessitant Lancer de grenade. Aucun buff temporaire ni lancement distinct n’est ajouté.</p><p>Rang suivant via équipement : portée identique à ce rang. Les rangs appris restent plafonnés à dix ; les bonus d’équipement poursuivent la progression.</p></section></div>
<p class="note">L’animation est proposée à la validation. Les effets de ces grenades ne sont pas encore raccordés au combat de campagne. Les PNJ actuels seront adaptés après les animations.</p><p><a id="gif0" href="">GIF du lancer</a> · <a id="gif1" href="">GIF de portée améliorée</a> · <a href="generation.json">Sources et prompts imagegen intégré</a> · <a href="../skill-spatial-review-01/index.html">Portées et vision validées</a></p><script src="player.js"></script></main></html>'''
for name in ('index.html','cote-a-cote.html'):(OUT/name).write_text(html,encoding='utf8')
(OUT/'native-geometry-verification.json').write_text(json.dumps({'views':len(entries),'checks':checks,'sources_are_native_readbacks':True},indent=2)+'\n',encoding='utf8')
(OUT/'review.json').write_text(json.dumps({'batch':'ANIM-S33','date':'2026-10-14','status':'awaiting_visual_review','candidates':[{'number':74,'id':'SG00','variant':'SG00-A01'},{'number':77,'id':'SG03','variant':'SG03-A01'}],'scope':'Projectile de grenade, explosion physique éphémère, sang, réaction d’impact, portée du passif et prévisualisation ; quatre directions et murs.','campaign_integration':False,'geometry':'Rayon circulaire de base conservé ; peinture naturelle peut dépasser légèrement des cases, mais les obstacles coupent le dessin.','damage_values':'prepared demonstration, no balancing approval'},ensure_ascii=False,indent=2)+'\n',encoding='utf8')
(OUT/'progress-audit.json').write_text(json.dumps({'date':'2026-10-14','previous_unshown':38,'newly_shown':['SG00','SG03'],'remaining_unshown':36,'visual_status':'SG00-A01 et SG03-A01 attendent leur validation ; SG01, SG02 et SG04 restent à présenter.'},ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(f'{len(entries)} vues natives, murs et tours vérifiés')
if REVIEW==73:
    html=html.replace('Grenades · lot 33','Grenades · dégâts dégressifs')
    html=html.replace('Grenades · Lancer et portée améliorée','Grenades · carré de 3×3 et dégâts dégressifs')
    html=html.replace('ANIM-S33 · candidats 74 / SG00-A01 et 77 / SG03-A01',
                      'AOE-FALLOFF01 · candidats 74 / SG00-A02 et 77 / SG03-A02')
    html=html.replace('Variante explosive · rayon d’une case', 'Variante explosive · carré de 3×3 cases')
    html=html.replace('cibles à 100 PV et dégâts physiques de 24, sans nouveau réglage d’équilibrage.',
                      'cibles à 100 PV, dégâts physiques de 24 au centre et 12 sur les huit cases autour. Le profil 100 % / 50 % est un essai à valider.')
    html=html.replace('L’animation est proposée à la validation.',
                      'Le carré de 3×3 correspond à votre demande. Le profil de dégâts dégressifs reste proposé à la validation.')
    html=html.replace('<a href="generation.json">Sources et prompts imagegen intégré</a>',
                      '<a href="proposal.json">Règle proposée</a> · <a href="../skill-animation-review-72/generation.json">Dessins et prompts conservés</a> · <a href="../skill-animation-review-72/cote-a-cote.html">Aperçu précédent</a>')
    grid='''<section class="card" style="margin-top:24px"><h2>Essai pour les grandes zones centrées</h2><p>Les couches carrées descendent progressivement de la pleine puissance au centre à la moitié au bord. Les diagonales appartiennent à la même couche que les côtés. Les murs retirent des cases sans modifier la dégressivité des cases restantes.</p><div class="controls"><label>Taille <select id="falloff-radius"><option value="1">3×3</option><option value="2">5×5</option><option value="3">7×7</option><option value="4">9×9</option><option value="5">11×11</option></select></label><label>Dégâts au centre <input id="falloff-base" type="number" min="1" max="1000" value="100" style="font:inherit;width:80px;padding:8px"></label></div><p class="note">Schéma de règle : chaque nombre représente les dégâts avant défenses sur cette case. Il ne représente pas le rendu du plateau.</p><div id="falloff-grid" style="display:grid;gap:3px;width:max-content;max-width:100%;overflow:auto"></div><p id="falloff-total"></p><p>Cette baisse réduit les dégâts par cible éloignée. Une grande zone reste capable de toucher beaucoup d’ennemis : ses dégâts de base, sa durée, son coût et sa recharge devront aussi participer à l’équilibrage.</p><p class="note">Proposition pour les grenades explosives et les sorts à zone centrée. Les cônes, vagues, contrôles et dégâts persistants devront être examinés selon leur fonctionnement avant d’appliquer une règle générale.</p></section><script src="falloff-proposal.js"></script>'''
    html=html.replace('<script src="player.js"></script>',grid+'<script src="player.js"></script>')
    for name in ('index.html','cote-a-cote.html'):(OUT/name).write_text(html,encoding='utf8')
    (OUT/'falloff-proposal.js').write_text((ROOT/'tools/art/falloff_proposal_player.js').read_text(encoding='utf8'),encoding='utf8')
    (OUT/'review.json').write_text(json.dumps({'batch':'AOE-FALLOFF01','date':'2026-10-14','status':'awaiting_numeric_and_visual_review','candidates':[{'number':74,'id':'SG00','variant':'SG00-A02'},{'number':77,'id':'SG03','variant':'SG03-A02'}],'square_requested':True,'numeric_profile_approved':False,'campaign_integration':False,'source_drawings_unchanged':'../skill-animation-review-72/generation.json','prepared_damage':{'centre':24,'eight_neighbours':12}},ensure_ascii=False,indent=2)+'\n',encoding='utf8')
    (OUT/'progress-audit.json').write_text(json.dumps({'date':'2026-10-14','remaining_unshown':36,'new_skills_shown':[],'revision_only':['SG00-A02','SG03-A02']},ensure_ascii=False,indent=2)+'\n',encoding='utf8')
    acceptance=OUT/'visual-and-numeric-acceptance-01.json'
    if acceptance.exists():
        for name in ('index.html','cote-a-cote.html'):
            page=(OUT/name).read_text(encoding='utf8').replace('un essai à valider','un profil validé').replace('Le profil de dégâts dégressifs reste proposé à la validation.','Le profil de dégâts dégressifs est validé.').replace('Règle proposée','Règle validée').replace('Proposition pour les grenades explosives et les sorts à zone centrée.','Règle validée pour les grenades explosives et les sorts à zone centrée.').replace('href="proposal.json"','href="accepted-rule.json"')
            (OUT/name).write_text(page,encoding='utf8')
        status=read(OUT/'review.json');status.update(status='approved',numeric_profile_approved=True,visual_approved=True,acceptance=acceptance.name,shared_rule_integrated=True)
        (OUT/'review.json').write_text(json.dumps(status,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
