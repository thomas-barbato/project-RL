"""ANIM-S30-REV02: original versus revised fire detonation, frozen original intact."""
import copy
import hashlib
import json
import shutil
from pathlib import Path
from PIL import Image,ImageChops
import prepare_agent_skill_animation_mines_opening as base
from add_browser_animation_comparison import add_comparison
from repair_mine_preview_playback import repair_review

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-63'
OLD=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-60'

def dump(name,obj):
    (OUT/name).write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n',encoding='utf8')

def package():
    assert not (OUT/'visual-acceptance-01.json').exists()
    proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
    for n,h in proof.items():assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
    old_manifest=json.loads((OLD/'index.html').read_text(encoding='utf8').split('const manifest=',1)[1].split(';let ready',1)[0])
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));combined=[];checks=[];animations=[]
    for e in entries:
        native=OUT/'native'/e['folder'];timeline=json.loads((native/'timeline.json').read_text(encoding='utf8'));images=[Image.open(native/f'{n:03}.png').convert('RGB')for n in range(160)];frames=[im.crop((32,88,608,664))for im in images]
        dest=OUT/'frames'/e['variant'];dest.mkdir(parents=True,exist_ok=True);keys={};indices=[]
        for n,im in enumerate(frames):
            h=hashlib.sha256(im.tobytes()).hexdigest()
            if h not in keys:keys[h]=n;im.save(dest/f'{n:03}.png',compress_level=1)
            indices.append(keys[h])
        hovers=[]
        for n in e['hover_frames']:
            Image.open(native/f'hover-{n:03}.png').crop((32,88,608,664)).save(dest/f'hover-{n:03}.png',compress_level=1)
            hovers.append({'frame':n,'file':f'hover-{n:03}.png','step':timeline[n]['step'],'led_on':timeline[n]['led_on']})
        for n in e['control_frames']:
            delta=ImageChops.difference(images[n],Image.open(native/f'control-{n:03}.png').convert('RGB')).crop((32,88,608,664))
            for y,row in enumerate(e['map'].splitlines()):
                for x,c in enumerate(row):
                    if c=='#':assert delta.crop((x*64,y*64,(x+1)*64,(y+1)*64)).getbbox()is None,(e['variant'],n,x,y)
        ref=copy.deepcopy(next(o for o in old_manifest if o['id']=='SM00'and o['case']==e['case']));ref['title']='Feu · version précédente'
        for n in set(ref['image_indices']):
            origin=OLD/'frames'/ref['variant']/f'{n:03}.png';target=OUT/'frames'/ref['variant']/f'{n:03}.png';target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(origin,target);assert hashlib.sha256(origin.read_bytes()).hexdigest()==hashlib.sha256(target.read_bytes()).hexdigest()
        for h in ref['hover_images']:shutil.copyfile(OLD/'frames'/ref['variant']/h['file'],OUT/'frames'/ref['variant']/h['file'])
        shutil.copyfile(OLD/f'{ref["variant"]}-64.gif',OUT/f'{ref["variant"]}-64.gif')
        assert timeline==ref['timeline'],e['case']
        e.update(timeline=timeline,image_indices=indices,hover_images=hovers,id='SM01',title='Feu · explosion révisée',source_skill='SM00')
        combined.extend([ref,e]);animations.append(base.gif(OUT/f'{e["variant"]}-64.gif',frames))
        checks.append({'variant':e['variant'],'walls_clear':True,'prepared_mechanics_identical_to_accepted_fire':True,'baseline_pixels_preserved':True})
        if e['case']=='HAUT':
            for n in [44,54,112,114,116,120,124,132,148]:images[n].save(OUT/f'SM00-phase-{n:03}.png')
        print(e['variant']+' packaged',flush=True)
    base.OUT=OUT;base.write_html(combined)
    page=(OUT/'index.html').read_text(encoding='utf8')
    replacements={
        'ANIM-S30':'ANIM-S30-REV02','Mines · lot 30':'Mine de feu · explosion révisée',
        'Une mine métallique au sol, sa LED centrale clignotante et son explosion. À droite, Réseau de mines permet d’en conserver cinq au rang 11.':'À gauche, l’explosion de feu précédente ; à droite, une détonation redessinée avec un front de feu plus marqué. La mine, les chiffres et les règles restent identiques.',
        '68 · Pose de mine':'Feu · version précédente','69 · Réseau de mines':'68 · Feu — explosion révisée',
        'Une mine simultanée sans le passif. Le voyant clignote dès la pose ; elle reste inactive avant son armement.':'Référence déjà validée, conservée sans retouche.',
        'Cinq mines au rang 11. Le scénario de capacité remplace la plus ancienne par la nouvelle mine.':'Nouvelle détonation de feu : pression, flammes et fragments qui se dispersent.',
        'href="mine-source.png"':'href="../skill-animation-review-60/mine-source.png"',
        'href="imagegen-mine-prompt.txt"':'href="../skill-animation-review-60/imagegen-mine-prompt.txt"'
    }
    for old,new in replacements.items():assert old in page,old;page=page.replace(old,new)
    page=page.replace('</main>','<p><a href="../skill-animation-review-62/cote-a-cote.html">Voir aussi les nouvelles explosions Poison et Froid</a></p><p class="note">Avec Résidu persistant, l’explosion laissera ensuite place au sol enflammé, au gaz toxique ou au sol gelé. Ces résidus seront présentés dans leur lot dédié ; SM05 est absent de cette comparaison.</p><p class="note">Nouvelle explosion réalisée avec imagegen intégré : <a href="fire-burst-source.png">source</a> · <a href="imagegen-fire-burst-prompt.txt">prompt</a>.</p></main>',1)
    (OUT/'index.html').write_text(page,encoding='utf8');add_comparison(OUT);repair_review(OUT)
    dump('review.json',{'batch':'ANIM-S30-REV02','date':'2026-10-14','status':'awaiting_visual_review','campaign_integration':False,'candidates':[{'number':68,'id':'SM00','name':'Pose de mine — explosion de feu révisée','variant':'SM00-A02','previous_accepted':'SM00-A01'}],'scope':'Explosion visual only. Same mine, redLED, prepared13fire, radius1, cost10, arming1, cooldown1, player immunity and turns. Source draw and brief180px/.44s/.88alpha explosion await review. No newBurn or terrainresidue.','preserved_reference':'review60 original Fire frames/GIF copied pixel-identically; timeline exact match.','residue_policy':'WithSM05, residual terrain takes visual relay after burst; fireground/gas/frozenground fixed. Mechanical immediate first application remains. DedicatedSM05 preview follows this explosion review.'})
    dump('verification.json',{'fresh_native_views':len(entries),'comparison_views':len(combined),'tile':64,'resampled':False,'protected_files_unchanged':len(proof),'checks':checks,'animations':animations})
    (OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/mines_fire_refined.rs').read_bytes());(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'fresh_views':len(entries),'baseline_unchanged':True,'checks':'passed'}))

if __name__=='__main__':package()
