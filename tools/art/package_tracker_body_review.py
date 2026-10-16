"""Verify the isolated native composition; publish the shared static review page."""
import hashlib
import json
import re
from pathlib import Path
from PIL import Image, ImageChops

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=BASE/'skill-animation-review-96'

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    assert not (REVIEW/'visual-acceptance-01.json').exists()
    preservation=json.loads((REVIEW/'preservation.json').read_text(encoding='utf-8'))
    for p,h in preservation.items():assert sha(ROOT/p)==h,p
    before=Image.open(REVIEW/'_capture/scene-before.png').convert('RGB')
    after=Image.open(REVIEW/'_capture/scene-after.png').convert('RGB')
    assert before.size==after.size==(576,576)
    difference=ImageChops.difference(before,after)
    box=difference.getbbox();assert box is not None
    # The changed body/contact shadow must not alter another drone, player or UI.
    assert box[0]>=244 and box[1]>=250 and box[2]<=332 and box[3]<=330,box
    before.save(REVIEW/'scene-before.png');after.save(REVIEW/'scene-after.png')
    template=(ROOT/'tools/art/tracker_body_review_template.html').read_text(encoding='utf-8')
    for path in re.findall(r'(?:src|href)="([^"]+)"',template):
        assert (BASE/path).is_file(),path
    (BASE/'apercu-animations.html').write_text(template,encoding='utf-8')
    export=json.loads((REVIEW/'export-verification.json').read_text(encoding='utf-8'))
    metadata={'date':'2026-10-15','review_id':'D58-RES64-01','number':107,'candidate':'D58-BODY01','status':'visual_proposal_not_accepted',
        'user_authorization':'ok on va faire ca alors :)','scope':'Create and present a real64 redraw of the existing Traqueur; this is not acceptance of the new artwork.',
        'tool':'built-in image_gen','prompt':'prompt.json','source':'D58-BODY01-source.png','native':'D58-BODY01-native128.png',
        'logical_tile':[64,64],'body_size':export['body_size'],'canvas':[128,128],
        'retained_identity':['Compact hunched left-facing quadruped','Long bone-white sensor-head casing','Military olive upper shell','Graphite frame and joints','Four articulated legs with bone-white armor and dark feet','Melee drone role and all content IDs'],
        'accepted_existing_assets_preserved':True,'new_art_to_approve':['Source drawing','Direct-size export and shown static composition'],
        'animation_bank_or_campaign_integration':False,'skill_icon_and_corpse_unchanged':True,
        'source_sha256':export['source_sha256'],'native_sha256':export['native_sha256']}
    (REVIEW/'review.json').write_text(json.dumps(metadata,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    proof={'native_scenes':2,'native_tile':64,'only_tracker_region_differs':True,'changed_bounds':box,
        'scene_before_sha256':sha(REVIEW/'scene-before.png'),'scene_after_sha256':sha(REVIEW/'scene-after.png'),
        'current_page_image_paths_exist':True,'original_assets_and_accepted_animations_preserved':True,
        'old_uniform_2x2_blocks_percent':export['previous_repeated_2x2_blocks']['uniform_percent'],
        'new_uniform_2x2_blocks_percent':export['new_repeated_2x2_blocks']['uniform_percent'],
        'method':'Native Macroquad static captures and offline image checks; no browser-render verification.'}
    (REVIEW/'validation.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'scenes':2,'changed_bounds':box,'preserved_paths':len(preservation)}))

if __name__=='__main__':main()
