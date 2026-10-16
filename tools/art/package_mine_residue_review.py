"""SM05 native64 ground-marker review. Existing assets reused by native code."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-65'

def dump(name, value):
    (OUT/name).write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf8')

def gif(path, images, kind):
    pixels = []
    for n in (14, 44, 116, 132, 164, 220, 448, 472):
        pixels.extend(images[n].resize((192,192),Image.Resampling.NEAREST).get_flattened_data())
    chosen = [p for p in pixels if (p[0]>p[1]*1.35 and p[0]>p[2]*1.35) if kind=='FEU'] if kind=='FEU' else [p for p in pixels if p[1]>p[0]+15 and p[1]>p[2]+15] if kind=='POISON' else [p for p in pixels if p[2]>p[0]+15]
    colours=[]
    for count, group in [(128,pixels),(128,chosen or pixels)]:
        strip=Image.new('RGB',(256,(len(group)+255)//256));strip.putdata(group)
        colours+=strip.quantize(colors=count,method=Image.Quantize.MEDIANCUT).getpalette()[:count*3]
    palette=Image.new('P',(1,1));palette.putpalette(colours);cache={};frames=[]
    for image in images:
        key=hashlib.sha256(image.tobytes()).hexdigest()
        if key not in cache:cache[key]=image.quantize(palette=palette,dither=Image.Dither.NONE)
        frames.append(cache[key])
    times=[round(i*1000/24/10)*10 for i in range(len(frames)+1)]
    frames[0].save(path,save_all=True,append_images=frames[1:],duration=[b-a for a,b in zip(times,times[1:])],loop=0,disposal=2,optimize=False)

def package():
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h for n,h in proof.items())
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8')); checks=[]
    for e in entries:
        folder=OUT/'native'/e['folder']; rows=json.loads((folder/'timeline.json').read_text(encoding='utf8'))
        assert rows[112]['zone_remaining']==13 and rows[112]['zone_applications']==1
        assert rows[160]['entrant_inside'] and rows[160]['zone_remaining']==12
        assert rows[448]['zone_remaining']==0 and rows[448]['zone_applications']==13
        assert all(t['player_hp']==100 for t in rows)
        assert rows[112]['hp'][2]==400 and rows[448]['hp'][3]==400
        if e['type']=='FEU':assert rows[448]['hp']==[348,348,364,400]
        elif e['type']=='POISON':assert rows[448]['hp']==[119,119,352,400] and rows[448]['poison_ticks']==[5,5,0,0]
        else:assert rows[448]['hp']==[384,384,400,400] and not any(rows[448]['zone_gel']) and rows[160]['zone_gel'][2]
        images=[Image.open(folder/f'{n:03}.png').convert('RGB').crop((32,88,608,664)) for n in range(480)]
        dest=OUT/'frames'/e['variant'];dest.mkdir(parents=True,exist_ok=True);keys={};indices=[]
        for n,image in enumerate(images):
            key=hashlib.sha256(image.tobytes()).hexdigest()
            if key not in keys:keys[key]=n;image.save(dest/f'{n:03}.png',compress_level=1)
            indices.append(keys[key])
        for idle in e['idle_images']:
            step=idle['step'];idlekeys={};idleindices=[]
            assert idle['hp']==rows[e['action_frames'][step-1]+23]['hp']
            for n in range(24):
                image=Image.open(folder/f'idle-{step}-{n:03}.png').convert('RGB').crop((32,88,608,664));key=hashlib.sha256(image.tobytes()).hexdigest()
                if key not in idlekeys:idlekeys[key]=n;image.save(dest/f'idle-{step}-{n:03}.png',compress_level=1)
                idleindices.append(idlekeys[key])
            idle['image_indices']=idleindices
        # Room walls may never receive ground/explosion paint. Coatings are checked too.
        for n in e['control_frames']:
            control=Image.open(folder/f'control-{n:03}.png').convert('RGB').crop((32,88,608,664));delta=ImageChops.difference(images[n],control)
            for y,row in enumerate(e['map'].splitlines()):
                for x,tile in enumerate(row):
                    if tile=='#':assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox() is None,(e['variant'],n,x,y)
            if n in (132,164):
                for x,y in e['area']:
                    count=sum(max(pixel)>5 for pixel in delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).get_flattened_data())
                    assert count>1400,(e['variant'],n,x,y,count)
                checks.append({'variant':e['variant'],'frame':n,'all_zone_cells_marked':True,'walls_clear':True})
            if n==464:assert delta.getbbox() is None,(e['variant'],'residue outlives expiry')
        e.update(timeline=rows,image_indices=indices)
        gif(OUT/f'{e["variant"]}-64.gif',images,e['type'])
        for n in (116,132,164,464):images[n].save(OUT/f'{e["variant"]}-phase-{n}.png')
        print(e['variant']+' packaged',flush=True)
    script=(ROOT/'tools/art/mines_residue_player.js').read_text(encoding='utf8').replace('MANIFEST',json.dumps(entries,ensure_ascii=False),1)
    options='<option value="FEU">Feu · sol enflammé</option><option value="POISON">Poison · gaz toxique</option><option value="FROID">Froid · sol gelé</option>'
    gallery=''.join(f'<a href="{e["variant"]}-64.gif">{e["title"]} · {e["case"]}</a>' for e in entries)
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Résidu persistant · aperçu 65</title>
<style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0a0f13;color:#e0e7e7;font:16px system-ui}h1{font-size:27px}h2{font-size:21px}p{line-height:1.5;color:#a5b9c5}button,select{font:inherit;color:#e0e7e7;background:#202931;border:1px solid #566574;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:8px;flex-wrap:wrap;align-items:center}.grid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:12px}article{min-width:0;background:#11181e;padding:16px}canvas{display:block;width:100%;height:auto;background:#080c0e}.native-view .grid{grid-template-columns:repeat(2,608px);width:1232px}.native-view canvas{width:576px;height:576px}#demo{background:#344852;border-color:#9ab4c4}#status{min-height:48px}.note{font-size:13px}.gallery{display:flex;flex-wrap:wrap;gap:14px}a{color:#bbc7ff}label{display:flex;gap:7px;align-items:center}</style>
<main><h1>73 · Résidu persistant · cases de 64 px</h1><p>L’explosion laisse place à un marqueur de danger couvrant toute sa zone : sol enflammé, gaz toxique ou sol gelé. Un ennemi entre ensuite dans la zone pour en montrer les effets.</p>
<div class="controls"><select id="scenario"><option value="HAUT">Terrain ouvert</option><option value="CLOISON">Contre une cloison</option></select><button id="demo">Rejouer la séquence</button><button id="cast">Poser la mine</button><button id="next">Action suivante</button><button id="expire">Aller à l’expiration</button><button id="reset">Recommencer</button><button id="pause">Pause</button><label><input id="native-view" type="checkbox">Taille native</label></div>
<p id="status">Chargement…</p><div class="grid"><article><select id="left-type">OPTIONS</select><h2 id="left-title">Sol enflammé</h2><canvas width="576" height="576"></canvas></article><article><select id="right-type">RIGHT_OPTIONS</select><h2 id="right-title">Nuage de poison</h2><canvas width="576" height="576"></canvas></article></div>
<p class="note">La démonstration joue jusqu’à l’entrée du nouvel ennemi, puis s’arrête. Attendre anime le terrain sans avancer les tours. « Tour suivant » avance les compteurs ; « Aller à l’expiration » affiche directement la fin des treize applications préparées pour examiner la disparition du marqueur.</p>
<p>Durée : <strong>13 tours</strong>. Prochain rang : <strong>14 tours</strong>. Exemple sans Explosion étendue : rayon d’une case, identique pour l’explosion et le résidu. Feu : 3 dégâts par application. Poison : 4, en plus de l’Empoisonné de l’explosion. Le froid applique Gel ; les réussites sont préparées pour cette revue.</p>
<p class="note">La première application a lieu dès la création de la zone et compte dans les treize. Le terrain prend visuellement le relais après la détonation. Le joueur reste protégé. Les chiffres sont provisoires ; les profils, l’IA et le combat définitif seront raccordés séparément.</p>
<p class="note">Prototype du laboratoire, sans intégration en campagne. Les monstres actuels seront adaptés au 64 px après les animations. <a href="../skill-animation-review-64/cote-a-cote.html">Comparer les mines sans résidu</a></p><details><summary>Animations complètes par variante</summary><div class="gallery">GALLERY</div></details></main><script>SCRIPT</script></html>'''
    html=html.replace('RIGHT_OPTIONS',options.replace('value="POISON"','value="POISON" selected')).replace('OPTIONS',options).replace('GALLERY',gallery).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf8');(OUT/'cote-a-cote.html').write_text(html,encoding='utf8')
    dump('verification.json',{'native_views':6,'tile':64,'source_images_unchanged':len(proof),'checks':checks,'application_count':13,'entry_and_expiry_checked':True})
    dump('review.json',{'review_id':'SM05-ANIM02-SQUARE','date':'2026-10-14','status':'awaiting_visual_review','candidate_number':73,'skill':'SM05','variants':['FEU','POISON','FROID'],'new_visuals':'Square ground marker after burst, all nine affected cells; low fire, toxic gas, ice floor; entrant then expiry.','references':['mine-square-footprint-direction-decisions.json','mine-residue-full-zone-marker-direction-decisions.json','mine-residue-elemental-ground-visual-direction-decisions.json','mine-residue-visual-handoff-direction-decisions.json','persistent-immediate-application-count-acceptance-decisions.json','review-individual-description-SM05-02.json'],'duration':13,'radius_fixture':1,'SM04_in_fixture':False,'geometry':'Square3x3 in open floor, clipped by obstacles','damage_prepared':{'ground_fire':3,'ground_poison':4,'explosions':[13,19,16],'poison_status':42},'first_application':'Immediate; counts among13; replaces historical delay under INTERACT-TIME01.','source_assets_reused':True,'native_visual_approval':False,'campaign_integration':False,'pending':'Full resolver, AI, status aggregation and resistances; SM04 extended footprint remains a separate review.'})
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/mines_residue.rs').read_bytes());(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())

if __name__=='__main__':package()
