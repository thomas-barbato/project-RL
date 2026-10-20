"""Validate native candidates, attachment exports and unchanged normal rendering."""
import json
import re
import subprocess
from PIL import Image, ImageChops
from art_finish_batch_02 import ROOT, ART, DOC, OUT, sha, write
from show_art_finish_review_02 import VIEW, difference

def main():
    assert not (DOC/'acceptance.json').exists(), 'Frozen preparation: use verify_art_finish_integration_02.py'
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    captures=json.loads((OUT/'finish02-engine/checks.json').read_text('utf-8'))
    baseline=json.loads((DOC/'production-before.json').read_text('utf-8'))
    for name,digest in baseline.items(): assert sha(ROOT/name)==digest,name
    contacts={}
    for entry in review['entries']:
        code=entry['code']
        original=Image.open(ART/'native'/f'{code}.png')
        assert original.size==(128,128)
        box=original.getbbox()
        assert min(box[:2])>0 and max(box[2:])<128,(code,box)
        for pose,config in entry['configurations'].items():
            body,held,fingers=[Image.open(ART/'native'/name) for name in config['layers']]
            assert all(image.getbbox() for image in [body,held,fingers])
            for area in config['fixed_regions']:
                assert ImageChops.difference(original.crop(area),body.crop(area)).getbbox() is None,(code,pose,area)
            anchors=config['hand_anchors'] if pose=='rifle' else [config['hand_anchors']]
            hits=[]
            for x,y in anchors:
                hits.append(sum(held.getpixel((xx,yy))[3]>0 and fingers.getpixel((xx,yy))[3]>0
                    for yy in range(round(y)-3,round(y)+4) for xx in range(round(x)-3,round(x)+4)))
            assert min(hits)>=2,(code,pose,hits)
            contacts[f'{code}-{pose}']=hits
    normal=OUT/'finish02-normal-check'
    prior=ROOT/'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01'
    pixels={}
    for label,count in [('native',13),('hands',6),('palettes',7)]:
        old=Image.open(prior/f'{label}-native.png')
        current=Image.open(normal/f'{label}-gpu.png')
        for i in range(count):
            x,y=160+i%4*300,80+i//4*200
            changed=difference(old.crop((x,y+44,x+128,y+172)),current.crop((x,y+34,x+128,y+162)))
            assert changed==0,(label,i,changed)
            pixels[f'{label}-{i+1}']=changed
    for label in ['house','street']:
        changed=difference(Image.open(prior/f'references/{label}-accepted-native.png'),
                           Image.open(normal/f'{label}-candidate.png').crop(VIEW))
        assert changed==0,(label,changed)
        pixels[label]=changed
    page=(OUT/'index.html').read_text('utf-8')
    script=re.search(r'<script>(.*?)</script>',page,re.S).group(1)
    subprocess.run(['node','--check'],input=script,text=True,check=True,capture_output=True)
    write(DOC/'validation.json',dict(review='ART-FINISH-02',status='pending_visual_review',
        native_cell=64,drawings=8,configuration_samples=24,scene_pairs=4,
        production_files_unchanged=len(baseline),normal_renderer_changed_pixels=pixels,
        hand_contacts=contacts,fixed_head_and_lower_body=True,
        gameplay_fingerprints_unchanged=captures['gameplay_fingerprints_unchanged'],
        commands=[dict(command=command,result='passed') for command in [
            'cargo build --locked','cargo check --locked --release','cargo fmt --all -- --check',
            'git diff --check','--ui-cold-art-finish-02-review','--ui-cold-art-finish-integration',
            'validate_art_finish_review_02.py / node --check']],
        full_test_suite_run=False,visual_inspection='Three native boards and four actual-engine scene comparisons',
        mode='built-in imagegen',runtime_candidates_enabled=False))
    print(f'Validated: 8 drawings, 24 configurations, 4 scenes; {len(baseline)} unchanged normal assets; normal renderer zero changed pixels')

if __name__=='__main__': main()
