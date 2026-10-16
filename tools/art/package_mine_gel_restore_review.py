"""Restore the accepted GEL-A05 renderer in the mine residue preview."""
import hashlib
import json
import os
from pathlib import Path
import package_mine_residue_snow_review as base

ROOT = Path(__file__).resolve().parents[2]
PREVIOUS = ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-68'
OUT = PREVIOUS.with_name('skill-animation-review-69')
original_gif = base.gif

def gif(path, images, kind):
    if kind != 'FROID':
        os.link(PREVIOUS/path.name, path)
    else:
        original_gif(path, images, kind)

def draw_body(source):
    start=source.index('fn draw(')
    start=source.index('{',start)
    depth=0
    for end in range(start,len(source)):
        if source[end]=='{':depth+=1
        elif source[end]=='}':depth-=1
        if depth==0:return ''.join(source[start:end+1].split())
    raise AssertionError('Unclosed draw body')

base.OUT=OUT
base.gif=gif
base.package()
accepted=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-28'
approval=json.loads((accepted/'visual-acceptance-01.json').read_text(encoding='utf8'))
atlas=accepted/'gel-atlas.png'
assert hashlib.sha256(atlas.read_bytes()).hexdigest()==approval['artifact_hashes']['gel-atlas.png']
current=(ROOT/'src/agent_skill_lab_review/fire_painted/frost/gel_status.rs').read_text(encoding='utf8')
assert draw_body(current)==draw_body((accepted/'render-source.rs').read_text(encoding='utf8'))
base.dump('gel-reference-verification.json',{
    'status':'GEL-A05 reused exactly',
    'accepted_on':'2026-10-12',
    'accepted_atlas_unchanged':True,
    'draw_body_identical_to_approved_review28':True,
    'frames':6,'atlas_cell_pixels':256,'speed':3,
    'composition':'72px; translucent torso/legs foreground, upper quarter excluded; sparse rear foot crystals; blended states',
    'new_drawing':False,
    'previous_bug':'Read six256px frames as four128px frames with a different placement and speed.'
})
for name in ('index.html','cote-a-cote.html'):
    p=OUT/name
    html=p.read_text(encoding='utf8').replace('aperçu 68','aperçu 69')
    html=html.replace('<h1>73 · Résidu persistant · cases de 64 px</h1>', '<h1>73 · Résidu persistant · Gel corrigé</h1><p>Gel reprend exactement son rendu validé : givre translucide devant le torse et les jambes, visage dégagé, quelques cristaux derrière les pieds. <a href="../skill-animation-review-28/index.html">Revoir la référence Gel validée</a></p>')
    p.write_text(html,encoding='utf8')
review=json.loads((OUT/'review.json').read_text(encoding='utf8'))
review.update(review_id='SM05-ANIM05-GEL-RESTORE',new_visuals='Restores existing accepted GEL-A05 rendering; snow/ice ground, Fire and Poison retained.',source_assets_reused=True,gel_status_visual_approval='Existing GEL-A05 approval, review28',new_drawing=False)
review['references'].append('skill-animation-review-28/visual-acceptance-01.json')
base.dump('review.json',review)
(OUT/'gel-render-source.rs').write_text(current,encoding='utf8')
(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
