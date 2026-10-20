"""Prepare four existing enemy designs for ART-FINISH-05, without runtime changes."""
import json
from pathlib import Path
from PIL import Image
from art_finish_batch_02 import ROOT, sha, write

ART = ROOT/'assets/source-art/art-finish-review-05'
DOC = ROOT/'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-05'
OUT = ROOT/'artifacts/art-quality-audit-01'
ENTRIES = [
    ('FIN-28',14,'Cultiste','Pistolet','Preserve the human cultist: dark charcoal hood with muted red inner edge, ivory closed angular mask, red chest/tabard and sleeve cuffs over dark practical trousers, gloves and boots. One small pale-metal pistol held in the extended hand to the left of the image. Keep the pistol straight, fingers wrapped around its grip, and empty other hand. No visible human face, no mutations.'),
    ('FIN-29',16,'Cultiste','Bouclier et masse','Preserve the human armored cultist: charcoal hood with muted red edge, fully closed ivory mask, dark chest and shoulder plates, red tabard, dark trousers and boots. Large dark rectangular riot shield with one muted red vertical panel on image right, straight short industrial mace in the hand at image left. Preserve both grips and the broad shield silhouette. No visible human face, horns or new weapon.'),
    ('FIN-30',6,'Possédé','Hybride quadrupède','Preserve this exact low four-limbed hunched possessed hybrid: pale diseased flesh, red exposed muscle in existing seams, orange dorsal machine shell, dark mechanical collars, one prosthetic forelimb ending in a blunt gripper, organic clawed remaining limbs, large open toothy animal mouth on image right, one small cold blue optical implant. Keep every limb, machine attachment and head direction. Flesh should feel organic and unpleasant, machinery matte and battered. Do not add spikes, limbs, cables, gore sprays or magic.'),
    ('FIN-31',120,'Zombie','Détenu réanimé','Preserve this exact unarmed bald reanimated prisoner: sickly pale grey-beige face and skin, dark sunken eyes, slumped head, charcoal prison coverall with a broad dirty mustard-yellow chest band, one yellow wrist cuff, rolled or torn sleeves, heavy dark boots, lowered grasping hands. Retain its human proportions and stumbling stance. Worn dirty fabric and subtly unhealthy skin, no new dismemberment, exposed organs, equipment, weapon, glow or magic.'),
]


def prepare():
    assert not (DOC/'acceptance.json').exists(), 'Accepted review is frozen'
    ART.mkdir(parents=True,exist_ok=True)
    DOC.mkdir(parents=True,exist_ok=True)
    manifest=json.loads((ROOT/'assets/sprites/campaign-monsters/manifest.json').read_text('utf-8'))
    common=('Use case: style-transfer. Production sprite revision for Project RL, a grimdark futuristic/cyberpunk roguelike. '
        'Input image 1 is the EDIT TARGET: preserve its identity, silhouette, pose, view, relative anatomy, existing equipment and material colors. '
        'Input image 2 is ONLY an already accepted finish reference, not the subject to copy. '
        'Match its matte worn game painting, broad readable shadow masses, believable material volumes, restrained irregular dark contours and localized wear. '
        'No glossy plastic, clean vector/cel shading, uniform thick black contours, new ornamentation or noisy microdetail. '
        'The final subject must stay readable at roughly 60 game pixels high, without increasing its relative world size. '
        'Same slightly elevated frontal sprite view as the target, no isometric or overhead redesign. Neutral diffuse overhead lighting. '
        'Render one WHOLE complete figure including all fingertips, weapons and soles, generous clear margins and a TRUE TRANSPARENT background. '
        'No floor, scene, ground contact shadow, glow, aura, labels, words or border. ')
    prompts=[]
    for code,number,label,detail,specific in ENTRIES:
        entry=next(e for e in manifest['entries'] if e['number']==number and e['variant']==0)
        reference=ART/f'{code}-reference.png'
        Image.open(ROOT/'assets/sprites/campaign-monsters'/entry['tile']).save(reference)
        finish='FIN-08' if number==6 else 'FIN-06'
        prompts.append(dict(id=code,number=number,label=label,detail=detail,
            prompt=common+specific,transparent_background=True,
            referenced_image_paths=[(ROOT/entry['source']).as_posix(),
                (ROOT/f'assets/source-art/art-finish-review-01/{finish}-source.png').as_posix()]))
    write(ART/'prompts.json',dict(review='ART-FINISH-05',mode='built-in imagegen',status='candidate_not_integrated',assets=prompts))
    baseline=DOC/'production-before.json'
    if not baseline.exists():
        paths=[p for p in (ROOT/'assets/sprites').rglob('*') if p.is_file()]
        paths += [ROOT/'assets/maps/prison-street-expanded-01'/n for n in ['street.json','street.png','layout.txt']]
        write(baseline,{p.relative_to(ROOT).as_posix():sha(p) for p in paths})
    print('ART-FINISH-05 prepared: FIN-28..31, original identities and native bounds retained')


def condition():
    assert not (DOC/'acceptance.json').exists(), 'Accepted review is frozen'
    native=ART/'native'; native.mkdir(exist_ok=True)
    manifest=json.loads((ROOT/'assets/sprites/campaign-monsters/manifest.json').read_text('utf-8'))
    records=[]
    for code,number,label,detail,_ in ENTRIES:
        old=next(e for e in manifest['entries'] if e['number']==number and e['variant']==0)
        source=ART/f'{code}-source.png'
        image=Image.open(source).convert('RGBA')
        box=image.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
        assert box and min(box[:2])>0 and box[2]<image.width and box[3]<image.height,code
        whole=image.crop(box)
        whole.putalpha(whole.getchannel('A').point(lambda a:0 if a<40 else a))
        b=old['bounds']; size=(b[2]-b[0],b[3]-b[1])
        ratio=min(size[0]/whole.width,size[1]/whole.height)
        whole=whole.resize((round(whole.width*ratio),round(whole.height*ratio)),Image.Resampling.LANCZOS)
        result=Image.new('RGBA',(old['side'],old['side']))
        foot=old['foot_anchor']
        result.alpha_composite(whole,(foot[0]-whole.width//2,foot[1]-whole.height))
        assert result.getbbox() and result.getbbox()[0]>0 and result.getbbox()[2]<old['side'],code
        path=native/f'{code}.png'; result.save(path)
        records.append(dict(code=code,number=number,variant=0,label=label,detail=detail,
            original_code=old['code'],role=old['role'],status='pending_visual_review',
            source_sha256=sha(source),native_sha256=sha(path),side=old['side'],
            foot_anchor=foot,previous_bounds=b,bounds=list(result.getbbox())))
    write(DOC/'review.json',dict(review='ART-FINISH-05',status='pending_visual_review',
        native_cell=64,drawings=4,entries=records,ordinary_runtime_unchanged=True,
        palette_scope='Original A only; other variants retain previous artwork pending their own review'))


if __name__=='__main__':
    import sys
    condition() if '--condition' in sys.argv else prepare()
