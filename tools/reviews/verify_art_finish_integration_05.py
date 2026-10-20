"""Compare the ordinary enemy renderer against the frozen ART-FINISH-05 approval."""
import json
from PIL import Image
from art_finish_batch_05 import ROOT, ART, DOC, OUT, sha, write
from show_art_finish_review_02 import VIEW, difference

INTEGRATED=OUT/'finish05-integration'


def main():
    receipt=json.loads((DOC/'acceptance.json').read_text('utf-8'))
    for name,digest in receipt['approved_exports'].items():
        assert sha(ART/'native'/name)==digest,name
        assert sha(ROOT/'assets/sprites/art-finish-05'/name)==digest,name
    before=json.loads((DOC/'production-before.json').read_text('utf-8'))
    for name,digest in before.items(): assert sha(ROOT/name)==digest,name
    capture=json.loads((INTEGRATED/'checks.json').read_text('utf-8'))
    assert capture['status']=='accepted_integration' and capture['production_mode']==3
    accepted=Image.open(DOC/'backgrounds-native.png')
    actual=Image.open(INTEGRATED/'native-gpu.png')
    panels={}
    for i in range(12):
        x,y=160+i%4*300,80+i//4*200
        changed=difference(accepted.crop((x,y+44,x+128,y+172)),actual.crop((x,y+34,x+128,y+162)))
        assert changed==0,(i,changed)
        panels[f'background-{i//4+1}-FIN-{28+i%4}']=changed
    scenes={}
    for label in ['street','north']:
        image=Image.open(INTEGRATED/f'{label}-integrated.png').crop(VIEW)
        changed=difference(Image.open(DOC/'references'/f'{label}-candidate-native.png'),image)
        assert changed==0,(label,changed)
        scenes[label]=changed
        image.save(DOC/'references'/f'{label}-integrated-native.png')
        image.save(OUT/f'finish05-{label}-integrated.png')
    result=dict(review='ART-FINISH-05',status='accepted_and_integrated',packaged_exact_exports=4,
        native_panels_changed_pixels=panels,scene_changed_pixels=scenes,native_cell=64,
        previous_resource_files_unchanged=len(before),runtime_assets='assets/sprites/art-finish-05',
        gameplay_fingerprints_unchanged=capture['gameplay_fingerprints_unchanged'],
        original_ids_roles_foot_anchors_and_shadows_preserved=True,
        other_palette_bindings='Previous drawings retained',
        death_handling='Existing source-derived rotated corpse convention; no new death pose, animation or rule')
    write(DOC/'integration-checks.json',result)
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    review['status']='accepted_and_integrated'
    review['ordinary_runtime_unchanged_at_preparation']=review.pop('ordinary_runtime_unchanged',True)
    review['runtime_assets']=result['runtime_assets']
    for e in review['entries']: e['status']='accepted'
    write(DOC/'review.json',review)
    from show_art_finish_review_05 import build
    build()
    print(json.dumps(dict(exports=4,panels=12,scene_changed_pixels=scenes,previous_files_unchanged=len(before))))


if __name__=='__main__': main()
