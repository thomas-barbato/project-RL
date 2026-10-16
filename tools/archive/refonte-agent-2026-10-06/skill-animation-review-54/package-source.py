"""ANIM-S24-A02: target-head icon revision, preserve the displayed A01 gallery."""
import argparse,hashlib,json,shutil
from pathlib import Path
from PIL import Image
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_shot_buffs as prior
import prepare_agent_skill_animation_hybrids as converter
from add_browser_animation_comparison import add_comparison
ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-54'
PREVIOUS=OUT.with_name('skill-animation-review-53')

def prepare():
    assert not OUT.exists();OUT.mkdir()
    previous_review=json.loads((PREVIOUS/'review.json').read_text(encoding='utf8'))
    assert previous_review['candidates'][0]['variant']=='T01-A01'
    proof={}
    for p in PREVIOUS.rglob('*'):
        if p.is_file() and 'native-mark-reaction-draft'not in p.parts:
            proof[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    original=json.loads((PREVIOUS/'review-protection.json').read_text(encoding='utf8'))
    for name,digest in original.items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest
    proof.update(original)
    base.dump(OUT/'review-protection.json',proof)
    base.dump(OUT/'drawing-pack.json',{'new_raster_generation':False,'source':'assets/skills/agent/T01-compact.png','source_sha256':hashlib.sha256((ROOT/'assets/skills/agent/T01-compact.png').read_bytes()).hexdigest(),'HUD_badge':'Existing icon24px in26px dark badge, anchored above marked target; follows shared cosmetic reaction; rendered as status overlay.','unchanged':'Numbers, targeting, reticle, physical wounds, actors/poses and T03-A01 mastery.'})
    base.dump(OUT/'feedback.json',{'date':'2026-10-14','user_reply':"j'aimerais que marquage ballistique laisse une icone au dessus de la tête de la cible",'variant':'T01-A02','direction_approved':'Add persistent icon above marked target.','native_render_status':'awaiting_visual_review','other_candidate':'T03-A01 remains unapproved; no implicit approval from this change request.'})
    (OUT/'Ouvrir_laboratoire.ps1').write_text((PREVIOUS/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-marquage-maitrise'","'--lot-marquage-maitrise' '--marquage-icone-cible'"),encoding='utf-8-sig')
    shutil.copyfile(PREVIOUS/'progress-audit.json',OUT/'progress-audit.json')
    print(json.dumps({'protected':len(proof),'new_raster':False}))

def package():
    assert not (OUT/'visual-acceptance-01.json').exists()
    captured=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));assert len(captured)==15
    previous=json.loads((PREVIOUS/'native/capture.json').read_text(encoding='utf8'))
    for e in previous:
        if e['id']=='T03':
            shutil.copytree(PREVIOUS/'native'/e['folder'],OUT/'native'/e['folder'])
            captured.append(e)
    base.dump(OUT/'native/capture.json',captured)
    for e in captured:
        if e['id']!='T01':continue
        timeline=json.loads((OUT/'native'/e['folder']/'timeline.json').read_text(encoding='utf8'))
        assert all(t['overhead_icon_visible']==(t['remaining']>0)for t in timeline)
        assert not timeline[14]['overhead_icon_visible']
        if e['permitted']:
            assert timeline[17]['overhead_icon_visible']
            if e['case']in ['HAUT','DROITE','GAUCHE','BAS','MANQUE','RAFALE','SURVOL']:assert not timeline[48]['overhead_icon_visible']
            if e['quiet']:assert timeline[-1]['overhead_icon_visible']
            if e['case']=='EXPIRATION':assert timeline[128]['overhead_icon_visible']and not timeline[148]['overhead_icon_visible']
            if e['case']=='REMPLACEMENT':assert timeline[88]['marked_target']==[6,2]and timeline[88]['overhead_icon_visible']
    prior.OUT=OUT
    prior.CANDIDATES=[{'number':57,'id':'T01','name':'Marquage balistique','variant':'T01-A02'},{'number':58,'id':'T03','name':'Maîtrise des attaques à distance','variant':'T03-A01'}]
    previous_GIFs={a['file']:a for a in json.loads((PREVIOUS/'verification.json').read_text(encoding='utf8'))['animations']}
    prior.REUSED_GIFS={e['variant']:{'path':PREVIOUS/f"{e['variant']}-64.gif",'record':previous_GIFs[f"{e['variant']}-64.gif"]}for e in captured if e['id']=='T03'}
    prior.package()
    unchanged=[]
    for e in captured:
        if e['id']=='T03':
            file=f"{e['variant']}-64.gif"
            assert hashlib.sha256((OUT/file).read_bytes()).hexdigest()==hashlib.sha256((PREVIOUS/file).read_bytes()).hexdigest(),file
            unchanged.append(file)
    for name in ['index.html','cote-a-cote.html']:
        p=OUT/name;html=p.read_text(encoding='utf8')
        html=html.replace('ANIM-S24 · candidats','ANIM-S24-A02 · candidats')
        html=html.replace('les traits rouges se referment sur la cible.','les traits rouges se referment sur la cible. L’icône de la compétence reste au-dessus de sa tête tant que la marque est active.')
        p.write_text(html,encoding='utf8')
    review=json.loads((OUT/'review.json').read_text(encoding='utf8'))
    review.update(date='2026-10-14',revision='T01-A02',previous_review='skill-animation-review-53',visuals=review['visuals']+' Existing compact icon remains above the marked target, follows its cosmetic reaction and disappears on consumption, expiry or replacement.',excluded_draft='No superseded draft is a candidate in this gallery; previous A01 gallery preserved separately.')
    base.dump(OUT/'review.json',review)
    checks=json.loads((OUT/'verification.json').read_text(encoding='utf8'))
    checks.update(overhead_icon_lifetime_checked=True,mastery_GIFs_byte_identical=unchanged)
    base.dump(OUT/'verification.json',checks)
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    (OUT/'base-package-source.py').write_bytes((ROOT/'tools/art/prepare_agent_skill_animation_shot_buffs.py').read_bytes())
    print(json.dumps({'target_badge_lifetime':'passed','mastery_unchanged':len(unchanged)}))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--prepare',action='store_true');p.add_argument('--convert-stream',action='store_true');a=p.parse_args()
    if a.prepare:prepare()
    elif a.convert_stream:converter.OUT=OUT;converter.convert()
    else:package()
