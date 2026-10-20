"""ART-FINISH-03 authoring only; existing production banks remain read-only."""
from pathlib import Path
import json
import sys
from PIL import Image, ImageChops
import art_finish_batch_02 as previous
from art_finish_batch_02 import ROOT, sha, write, selected, weapon

ART = ROOT / 'assets/source-art/art-finish-review-03'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-03'
OUT = ROOT / 'artifacts/art-quality-audit-01'
ENTRIES = [
    ('FIN-22', 'Tablier de cuir', 'core:patched_plating', 208, 108),
    ('FIN-23', 'Tenue de membranes', 'core:veste_de_membranes', 209, 110),
    ('FIN-24', 'Combinaison Peau-Seconde', 'core:veste_de_peau_seconde', 210, 111),
    ('FIN-25', 'Tenue du technicien', 'core:veste_de_veille', 211, 112),
]
DETAILS = [
    'Leather apron outfit: matte dark blue sleeves and shoulders, a dirty cream/ivory long leather work apron reaching the upper thighs, dark trousers with blue knee panels, black gloves and dark boots. Dark head covering with a short ochre brown strip above an opaque black face covering. Preserve the apron, its length and the blue/ivory/brown color placement.',
    'Membrane outfit: a muted steel/slate blue-grey enclosed high-collared long coat with split tails at the thighs, a pale grey central chest panel, charcoal limbs and boots, dark blue-grey full helmet with a short broad horizontal visor. Preserve its angular hood/helmet and dark blue-grey coat silhouette. The membrane is clothing, not a monster body.',
    'Second-skin suit: a slim dark muted blue bodysuit with broad dirty ivory shoulder caps, ivory chest armor and ivory belt/upper pelvis plates, blue thigh and shin protection, a sealed navy helmet and opaque face covering, black gloves and dark boots. Preserve the slim human proportions and distinct ivory plates on blue material; do not copy the heavy Bastion armor.',
    'Technician outfit: charcoal work coveralls with dirty cream/ivory shoulder guards, two small ivory chest patches, dark teal/grey torso and trousers, restrained beige knee pads and dark boots, completely enclosed pale ivory head mask/helmet. Preserve its narrow functional working-clothes silhouette and modest small armor patches; no lab coat or added gadgets.',
]


def prepare():
    ART.mkdir(parents=True, exist_ok=True)
    DOC.mkdir(parents=True, exist_ok=True)
    atlas = Image.open(ROOT / 'assets/sprites/flat-agents/reviewed-128.png')
    for code, _, _, outfit, _ in ENTRIES:
        index = (outfit - 200) * 3 * 16
        x, y = index % 16 * 128, index // 16 * 128
        atlas.crop((x, y, x+128, y+128)).save(ART / f'{code}-reference.png')
    common = ('Use case: style-transfer. Asset type: complete production character sprite for Project RL, a grimdark futuristic/cyberpunk roguelike with native 64px cells. '
              'Input image 1 is the EDIT TARGET, the existing outfit: retain identity, material colors, clothing/armor layout, proportions, stance and full face covering. '
              'Input image 2 is ONLY the accepted finish reference Bastion, never a character to copy. '
              'Give the target the same matte worn game painting, broad readable shadow planes, believable folds and material volumes, controlled irregular dark contours, restrained highlights and locally placed wear. '
              'No shiny plastic, sterile vector/cel shading, thick uniform outlines, noisy microdetail or giant shoulder armor. Readable at about 60 game pixels tall. '
              'Same slightly elevated frontal standing view as the target and reference, no isometric view. Neutral diffuse overhead light. '
              'Whole figure from head to soles, complete limbs, generous margins, TRUE TRANSPARENT background. No scene, ground, cast shadow, words, frame, glow or effects. '
              'Feet slightly apart; empty gloved fists slightly out to the sides at waist level, bent elbows, ready for separate weapon attachments. No weapon or new accessory. ')
    write(ART/'prompts.json',dict(review='ART-FINISH-03',mode='built-in imagegen',assets=[
        dict(id=e[0],label=e[1],identity=e[2],prompt=common+detail,transparent_background=True,
             referenced_image_paths=[str(ART/f'{e[0]}-reference.png'),
               str(ROOT/'assets/source-art/art-finish-review-01/FIN-01-source.png')])
        for e,detail in zip(ENTRIES,DETAILS)]))
    baseline = DOC / 'production-before.json'
    if not baseline.exists():
        paths = [p for p in (ROOT/'assets/sprites').rglob('*') if p.is_file()]
        paths += [ROOT/'assets/maps/prison-street-expanded-01'/n for n in ['street.json','street.png','layout.txt']]
        write(baseline, {p.relative_to(ROOT).as_posix(): sha(p) for p in paths})
    print('Prepared FIN-22 to FIN-25 references; no normal artwork changed')


def condition(*, art=ART, doc=DOC, entries=ENTRIES, review_id='ART-FINISH-03'):
    assert not (doc/'acceptance.json').exists(), 'Accepted exports are frozen'
    (art/'native').mkdir(parents=True, exist_ok=True)
    records=[]
    for code,label,identity,outfit,torso in entries:
        base=previous.normalized(code, art=art)
        previous.save_layer(base, code, art=art)
        arms=[[(27,46),(53,46),(57,51),(55,57),(52,75),(27,75)],
              [(75,46),(101,46),(101,75),(76,75),(73,57),(71,51)]]
        _,mask=selected(base, arms)
        core=base.copy()
        core.putalpha(ImageChops.multiply(base.getchannel('A'),ImageChops.invert(mask)))
        alternate=previous.normalized(code+'-RIFLE', art=art)
        sleeves,_=selected(alternate, [[(40,46),(55,46),(59,53),(65,57),(64,71),(43,74)],
                                      [(72,46),(88,46),(88,74),(66,74),(64,59),(70,53)]])
        configs={}
        for pose in ['single','rifle']:
            posed=base.copy() if pose=='single' else core.copy()
            if pose=='rifle': posed.alpha_composite(sleeves)
            anchors=(84,62) if pose=='single' else ((59,61),(73,64))
            points=[anchors] if pose=='single' else anchors
            fingers,_=selected(posed, [[(x-2,y-2),(x+2,y-2),(x+2,y+2),(x-2,y+2)] for x,y in points])
            held,length=weapon(pose, anchors)
            names=[f'{code}-{pose}-{part}' for part in ['body','weapon','fingers']]
            for image,name in zip([posed,held,fingers],names): previous.save_layer(image,name,art=art)
            original=Image.open(art/'native'/f'{code}.png')
            registered=Image.open(art/'native'/f'{code}-{pose}-body.png')
            fixed=[(0,0,128,44),(0,80,128,128)]
            for area in fixed: registered.paste(original.crop(area),area[:2])
            registered.save(art/'native'/f'{code}-{pose}-body.png')
            composed=registered.copy()
            for name in names[1:]: composed.alpha_composite(Image.open(art/'native'/f'{name}.png'))
            composed.save(art/'native'/f'{code}-{pose}.png')
            configs[pose]=dict(layers=[n+'.png' for n in names],hand_anchors=anchors,
                               held_length=length,fixed_regions=fixed)
        native=Image.open(art/'native'/f'{code}.png')
        records.append(dict(code=code,label=label,identity=identity,outfit=outfit,torso=torso,
            source_sha256=sha(art/f'{code}-source.png'),native_sha256=sha(art/'native'/f'{code}.png'),
            size=[128,128],foot=[64,96],bounds=list(native.getbbox()),
            status='pending_visual_review',configurations=configs))
    for file,digest in json.loads((doc/'production-before.json').read_text('utf-8')).items():
        assert sha(ROOT/file)==digest,file
    write(doc/'review.json',dict(review=review_id,status='pending_visual_review',
        ordinary_runtime_unchanged=True,native_cell=64,drawings=len(entries),configuration_samples=len(entries)*3,entries=records))
    print(f'Conditioned {len(entries)} outfits and {len(entries)*3} configurations; production banks unchanged')


if __name__=='__main__':
    condition() if '--condition' in sys.argv else prepare()
