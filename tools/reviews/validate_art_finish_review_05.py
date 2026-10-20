"""Validate the isolated enemy preview and preservation of the accepted renderer."""
import json
import re
import subprocess
from PIL import Image
from art_finish_batch_05 import ROOT, ART, DOC, OUT, sha, write
from show_art_finish_review_02 import VIEW, difference


def main():
    assert not (DOC/'acceptance.json').exists(), 'Preparation validation must not rewrite an accepted review'
    before=json.loads((DOC/'production-before.json').read_text('utf-8'))
    for name,digest in before.items(): assert sha(ROOT/name)==digest,name
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    for e in review['entries']:
        code=e['code']; source=ART/f'{code}-source.png'; native=ART/'native'/f'{code}.png'
        assert sha(source)==e['source_sha256'] and sha(native)==e['native_sha256'],code
        image=Image.open(native).convert('RGBA'); alpha=image.getchannel('A')
        assert image.size==(e['side'],e['side']) and alpha.getextrema()==(0,255),code
        box=alpha.getbbox()
        assert list(box)==e['bounds'] and 0<box[0]<box[2]<e['side'],code
        assert 0<box[1]<box[3]==e['foot_anchor'][1]<e['side'],code
        assert box[2]-box[0]<=e['previous_bounds'][2]-e['previous_bounds'][0],code
        source=Image.open(source).convert('RGBA')
        box=source.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
        assert min(box[:2])>0 and box[2]<source.width and box[3]<source.height,code
    raw=Image.open(OUT/'finish05-engine/native-gpu.png')
    board=Image.open(DOC/'backgrounds-native.png')
    for i in range(12):
        x,y=20+i%4*300,80+i//4*200
        for c in range(2):
            xx=x+c*140
            assert difference(raw.crop((xx,y+34,xx+128,y+162)),board.crop((xx,y+44,xx+128,y+172)))==0,(i,c)
    previous=DOC.parent/'art-finish-review-04'
    normal=OUT/'finish05-normal-check'
    capture=json.loads((normal/'checks.json').read_text('utf-8'))
    assert capture['production_mode']==3 and capture['status']=='accepted_integration'
    panels={}
    for label,count in [('native',2),('hands',6)]:
        expected=Image.open(previous/f'{label}-native.png')
        actual=Image.open(normal/f'{label}-gpu.png')
        for i in range(count):
            x,y=160+i%4*300,80+i//4*200
            changed=difference(expected.crop((x,y+44,x+128,y+172)),actual.crop((x,y+34,x+128,y+162)))
            assert changed==0,(label,i,changed)
            panels[f'{label}-{i+1}']=changed
    scenes={}
    for label in ['house','street']:
        changed=difference(Image.open(previous/'references'/f'{label}-candidate-native.png'),
            Image.open(normal/f'{label}-integrated.png').crop(VIEW))
        assert changed==0,(label,changed)
        scenes[label]=changed
    page=(OUT/'index.html').read_text('utf-8')
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    js=re.search(r'<script>(.*?)</script>',page,re.S).group(1)
    js_path=OUT/'finish05-interaction-check.js'; js_path.write_text(js,encoding='utf-8')
    subprocess.run(['node','--check',str(js_path)],check=True)
    write(DOC/'validation.json',dict(review='ART-FINISH-05',status='pending_visual_review',
        drawings=4,complete_alpha_sources=True,native_foot_anchors_preserved=True,
        background_panels_exact=24,production_files_unchanged=len(before),
        ordinary_renderer_panels_changed_pixels=panels,ordinary_scene_changed_pixels=scenes,
        ordinary_gameplay_unchanged=capture['gameplay_fingerprints_unchanged'],
        preview_only=True,new_runtime_candidates=False,local_links_valid=True,javascript_syntax='passed'))
    print(f'Validated 4 complete sources, 24 native panels, {len(before)} unchanged resources; ordinary 8 panels + 2 scenes identical')


if __name__=='__main__': main()
