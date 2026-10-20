"""Verify candidate attachments and confirm that ordinary world art is unchanged."""
import json
import re
import subprocess
from PIL import Image,ImageChops
from art_finish_batch_03 import ROOT,ART,DOC,OUT,sha,write
from show_art_finish_review_02 import VIEW,difference


def main():
    assert not (DOC/'acceptance.json').exists(), 'Frozen preparation: use integration verification'
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    capture=json.loads((OUT/'finish03-engine/checks.json').read_text('utf-8'))
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
            assert all(im.getbbox() for im in [body,held,fingers]),(code,pose)
            for area in config['fixed_regions']:
                assert not ImageChops.difference(original.crop(area),body.crop(area)).getbbox(),(code,pose,area)
            anchors=config['hand_anchors'] if pose=='rifle' else [config['hand_anchors']]
            hits=[sum(held.getpixel((xx,yy))[3]>0 and fingers.getpixel((xx,yy))[3]>0
                for yy in range(round(y)-3,round(y)+4) for xx in range(round(x)-3,round(x)+4)) for x,y in anchors]
            assert min(hits)>=2,(code,pose,hits)
            contacts[f'{code}-{pose}']=hits
    normal=OUT/'finish03-normal-check'
    prior=DOC.parent/'art-finish-review-02'
    pixels={}
    for label,count in [('native',8),('classes-hands',12),('armors-hands',12)]:
        old=Image.open(prior/f'{label}-native.png')
        current=Image.open(normal/f'{label}-gpu.png')
        for i in range(count):
            x,y=160+i%4*300,80+i//4*200
            changed=difference(old.crop((x,y+44,x+128,y+172)),current.crop((x,y+34,x+128,y+162)))
            assert changed==0,(label,i,changed)
            pixels[f'{label}-{i+1}']=changed
    for label in ['class-house','classes-street','armor-house','armors-street']:
        changed=difference(Image.open(prior/f'references/{label}-candidate-native.png'),
                           Image.open(normal/f'{label}-integrated.png').crop(VIEW))
        assert changed==0,(label,changed)
        pixels[label]=changed
    page=(OUT/'index.html').read_text('utf-8')
    script=re.search(r'<script>(.*?)</script>',page,re.S).group(1)
    subprocess.run(['node','--check'],input=script,text=True,check=True,capture_output=True)
    write(DOC/'validation.json',dict(review='ART-FINISH-03',status='pending_visual_review',
          native_cell=64,drawings=4,configuration_samples=12,scene_pairs=2,
          production_files_unchanged=len(baseline),normal_renderer_changed_pixels=pixels,
          hand_contacts=contacts,fixed_head_and_lower_body=True,
          gameplay_fingerprints_unchanged=capture['gameplay_fingerprints_unchanged'],
          full_test_suite_run=False,mode='built-in imagegen',runtime_candidates_enabled=False,
          browser_check='Static local links and node --check; no browser automation'))
    print(f'Validated 4 drawings / 12 configurations / 2 scenes; {len(baseline)} existing resources unchanged')


if __name__=='__main__': main()
