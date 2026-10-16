"""SM04 review: compare radius1 and4 using approved mine/residue sources."""
import hashlib
import json
import os
import sys
from pathlib import Path
from PIL import Image, ImageChops

ROOT=Path(__file__).resolve().parents[2]
REVIEW=int(sys.argv[1]) if len(sys.argv)>1 else 70
OUT=ROOT/f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{REVIEW}'
def dump(name,value):
    (OUT/name).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
def package():
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in proof.items())
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));checks=[]
    for e in entries:
        source=OUT/'native'/e['folder'];dest=OUT/'frames'/e['variant'];dest.mkdir(parents=True)
        rows=json.loads((source/'timeline.json').read_text(encoding='utf8'))
        assert len(e['area'])==({('R1','HAUT'):9,('R4','HAUT'):81,('R1','CLOISON'):6,('R4','CLOISON'):45}[(e['profile'],e['case'])])
        assert rows[88]['zone_applications']==1 and rows[88]['zone_remaining']==13
        assert rows[120]['zone_applications']==2 and rows[120]['zone_remaining']==12
        assert rows[176]['zone_applications']==13 and rows[176]['zone_remaining']==0
        assert all(t['player_hp']==100 for t in rows)
        assert rows[88]['hp'][2]==(400 if e['profile']=='R1' else {'FEU':384,'POISON':335,'FROID':384}[e['type']])
        assert rows[88]['hp'][3]==(400 if e['profile']=='R1' or e['case']=='CLOISON' else {'FEU':384,'POISON':335,'FROID':384}[e['type']])
        images=[Image.open(source/f'{n:03}.png').convert('RGB').crop((32,88,608,664)) for n in range(192)]
        def encode_group(names):
            known={};indices=[]
            for n,name in enumerate(names):
                full=source/name;im=Image.open(full).convert('RGB').crop((32,88,608,664));key=hashlib.sha256(im.tobytes()).hexdigest()
                if key not in known:known[key]=n;os.link(full,dest/name)
                indices.append(known[key])
            return indices
        e['image_indices']=encode_group([f'{n:03}.png' for n in range(192)])
        for idle in e['idle_images']:idle['image_indices']=encode_group([f'idle-{idle["step"]}-{n:03}.png' for n in range(24)])
        e['hover_indices']=encode_group([f'hover-{n:03}.png' for n in range(24)])
        # Clipping and expiry checked independently of the prepared target stats.
        for n in e['control_frames']:
            delta=ImageChops.difference(images[n],Image.open(source/f'control-{n:03}.png').convert('RGB').crop((32,88,608,664)))
            if e['case']=='CLOISON':
                assert delta.crop((320,0,384,576)).getbbox() is None,(e['variant'],n,'wall paint')
                assert delta.crop((384,0,576,576)).getbbox() is None,(e['variant'],n,'paint through wall')
            if n==180:assert delta.getbbox() is None,(e['variant'],'residue after expiry')
            if n in (100,156):
                counts=[sum(max(p)>5 for p in delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).get_flattened_data()) for x,y in e['area']]
                assert min(counts)>100,(e['variant'],n,min(counts))
                checks.append({'variant':e['variant'],'frame':n,'every_affected_cell_marked':True,'minimum_changed_pixels':min(counts)})
        # Stable palette from the same approved render sources, with all event phases.
        palette_images=[images[min(n,191)] for n in range(480)]
        for old,new in [(14,14),(44,44),(116,92),(132,100),(164,124),(220,156),(448,180),(472,188)]:palette_images[old]=images[new]
        # Use the existing palette maker with the full192-frame sequence: its
        # sample indices are remapped locally, never changing native browser PNGs.
        sampled=OUT/(e['variant']+'-64.gif')
        inherited=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-70'/sampled.name
        if REVIEW==71 and e['profile']=='R4':os.link(inherited,sampled)
        else:gif_short(sampled,images,palette_images,e['type'])
        for n in (14,92,100,124,180):images[n].save(OUT/f'{e["variant"]}-phase-{n}.png')
        e['timeline']=rows
        print(e['variant']+' packaged',flush=True)
    script=(ROOT/'tools/art/mines_extended_player.js').read_text(encoding='utf8').replace('MANIFEST',json.dumps(entries,ensure_ascii=False),1)
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Explosion étendue · aperçu70</title><style>*{box-sizing:border-box}body{margin:0 auto;max-width:1320px;padding:24px;background:#0a0f13;color:#e0e7e7;font:16px system-ui}p{color:#a5b9c5;line-height:1.5}h1{font-size:27px}h2{font-size:21px}button,select{font:inherit;color:#e0e7e7;background:#202931;border:1px solid #566574;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:8px;flex-wrap:wrap;align-items:center}.grid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:12px}article{min-width:0;background:#11181e;padding:16px}canvas{display:block;width:100%;height:auto}.native-view .grid{grid-template-columns:repeat(2,608px);width:1232px}.native-view canvas{width:576px;height:576px}.note{font-size:13px}a{color:#bbc7ff}</style><h1>72 · Explosion étendue · cases de64px</h1><p>À gauche : rayon d’une case (3×3). À droite : rayon de quatre cases au rang11 (9×9). L’explosion et le résidu partagent les mêmes cases ; les obstacles coupent les effets. Dessins de la revue69 et Gel validé conservés.</p><div class="controls"><select id="type"><option value="FEU">Feu</option><option value="POISON">Poison</option><option value="FROID">Froid</option></select><select id="scenario"><option value="HAUT">Terrain ouvert</option><option value="CLOISON">Contre une cloison</option></select><button id="demo">Rejouer</button><button id="cast">Poser la mine</button><button id="next">Action suivante</button><button id="expire">Aller à l’expiration</button><button id="reset">Recommencer</button><button id="pause">Pause</button><label><input id="native-view" type="checkbox">Taille native</label></div><p id="status">Chargement…</p><div class="grid"><article><h2>Sans Explosion étendue · rayon1</h2><canvas width="576" height="576"></canvas></article><article><h2>Explosion étendue · rang11 · rayon4</h2><canvas width="576" height="576"></canvas></article></div><p>La démonstration joue jusqu’à la seconde application puis reste animée. Pour voir l’aperçu au survol : recommencez, posez et armez la mine, puis placez la souris dessus. Le contour clignotant disparaît au lancement de l’attaque.</p><p class="note">La galerie montre les deux premières applications puis l’expiration. Le laboratoire natif permet d’avancer tous les tours. L’attente anime les effets sans consommer de tour.</p><p><em>Vous amplifiez le souffle de vos mines pour atteindre les ennemis plus éloignés.</em></p><p>Effets actuels — rang11 : rayon de4cases. Prochain rang —12 : rayon inchangé à ce rang.</p><p class="note">Passif nécessitant Pose de mine, pour les mines de feu, poison et glace et leurs résidus. Exemple avec Résidu persistant au rang11 (13tours) ; les dégâts et réussites de statut sont préparés, sans nouveau réglage d’équilibrage. Les cases affichées constituent un extrait de salle plus vaste. Aucun effet n’est intégré au combat de campagne.</p><p><a href="../skill-animation-review-69/cote-a-cote.html">Référence Résidu persistant validée</a></p><script>SCRIPT</script></html>'''
    for before,after in [('aperçu70',f'aperçu {REVIEW}'),('de64px','de 64 px'),('rang11','rang 11'),('rang12','rang 12'),('rang1','rang 1'),('rayon1','rayon 1'),('rayon4','rayon 4'),('de4cases','de 4 cases'),('—12','— 12'),('13tours','13 tours'),('revue69','revue 69')]:html=html.replace(before,after)
    for name in ('index.html','cote-a-cote.html'):(OUT/name).write_text(html.replace('SCRIPT',script),encoding='utf8')
    dump('verification.json',{'views':len(entries),'tile':64,'checks':checks,'player_protected':True,'walls_clip_burst_and_residue':True,'expiry_clear':True})
    dump('review.json',{'review_id':'SM04-ANIM01','date':'2026-10-14','status':'awaiting_visual_review','number':72,'skill':'SM04','profiles':{'base_radius':1,'effective_rank11_radius':4},'types':['FEU','POISON','FROID'],'cases':['HAUT','CLOISON'],'references':['review-individual-description-SM04-02.json','mine-square-footprint-direction-decisions.json','skill-animation-review-69/visual-acceptance-01.json'],'scope':'Paired native64 preview, enlarged burst and full residue, armed-mine hover and complete wall shielding. Approved drawings reused; no new timed buff or active skill.','geometry':'Square within radius, reachable floor; these prepared open/complete-wall cases do not define unshown corner/door rules.','campaign_integration':False})
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/mines_residue.rs').read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())

def gif_short(path,images,samples,kind):
    # Palette selection inherited from the approved proof pipeline, adapted to
    # this short visual scenario; preserve native source frames for the viewer.
    pixels=[]
    for n in (14,44,116,132,164,220,448,472):pixels.extend(samples[n].resize((192,192),Image.Resampling.NEAREST).get_flattened_data())
    selected=[p for p in pixels if (p[0]>p[1]*1.35 and p[0]>p[2]*1.35)] if kind=='FEU' else [p for p in pixels if p[1]>p[0]+15 and p[1]>p[2]+15] if kind=='POISON' else [p for p in pixels if p[2]>p[0]+15]
    colors=[]
    for group in (pixels,selected or pixels):
        strip=Image.new('RGB',(256,(len(group)+255)//256));strip.putdata(group)
        colors+=strip.quantize(colors=128,method=Image.Quantize.MEDIANCUT).getpalette()[:384]
    palette=Image.new('P',(1,1));palette.putpalette(colors);cache={};frames=[]
    for im in images:
        key=hashlib.sha256(im.tobytes()).hexdigest()
        if key not in cache:cache[key]=im.quantize(palette=palette,dither=Image.Dither.NONE)
        frames.append(cache[key])
    times=[round(i*1000/24/10)*10 for i in range(len(frames)+1)]
    frames[0].save(path,save_all=True,append_images=frames[1:],duration=[b-a for a,b in zip(times,times[1:])],loop=0,disposal=2,optimize=False)

if __name__=='__main__':package()
