"""ART-FINISH-02: authoring metadata and non-destructive native conditioning."""
from pathlib import Path
import hashlib
import json
import math
import sys
from PIL import Image, ImageChops, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/art-finish-review-02'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02'
OUT = ROOT / 'artifacts/art-quality-audit-01'

ENTRIES = [
    ('FIN-14','Technomancien','core:technomancien',201,None,
     'The technomancer class: sealed graphite helmet and respirator, graphite technical suit, muted blue chest armor and blue shoulder panels, small restrained orange utility details, dark gloves and practical boots. Keep the human proportions, closed face, existing armor layout and short jacket silhouette.'),
    ('FIN-15','Traqueur','core:traqueur',202,None,
     'The tracker class: dark muted blue hood and opaque face covering, charcoal tactical vest/jacket with blue shoulder cloth, dirty beige/tan combat trousers, dark practical boots and gloves. Keep the blue hood, pale trousers, human proportions, full face concealment and existing light-armored silhouette.'),
    ('FIN-16','Arcaniste','core:arcaniste',203,None,
     'The arcanist class: a worn deep-crimson hooded long robe with a dark inner layer, muted crimson sleeves, simple narrow light-ivory closed face mask, dark gloves and boots peeking from the split hem. Preserve the red robe silhouette and full face concealment; restrained worn fabric rather than luminous magic or medieval decorations.'),
    ('FIN-17','Métamorphe','core:metamorphe',204,None,
     'The metamorph class in HUMAN form: dirty pale-ivory hood and closed pale mask, worn ochre/olive short coat with darker straps, charcoal-grey trousers and dark gloves and boots. Preserve the mustard/olive coat, ivory hood, compact silhouette and fully hidden face. No exposed mutations, tentacles, bone spikes or new limbs.'),
    ('FIN-18','Carapace Chrysalide','core:carapace_chrysalide',213,103,
     'The Chrysalide armored outfit: segmented olive-green/lime-green hard armor over a charcoal undersuit, compact sealed pointed angular green helmet, green shoulder, chest, thigh and shin shell plates, dark joints, gloves and boots. Preserve the green carapace shape and heavier-than-cloth silhouette; use matte worn industrial green, not shiny plastic, glowing green or actual insect anatomy.'),
    ('FIN-19','Combinaison fantôme','core:combinaison_fantome',214,105,
     'The Ghost technical suit: a fully closed dark hood/helmet, black-charcoal flexible armored suit, clear muted turquoise/cyan shoulder and upper-chest plate accents, dark gloves, grey shin protection and dark boots. Preserve the slim dark silhouette and turquoise accent locations. Opaque concealed face. The cyan is material color, not emitted neon light.'),
    ('FIN-20','Manteau de mue','core:manteau_de_mue',215,106,
     'The molting-coat outfit: rust-orange hood, split long rust-orange outer coat over a dirty ivory front tabard/inner cloth, black-charcoal arms and trousers, practical dark gloves and boots, completely covered face with dark mask. Preserve the orange-and-ivory coat silhouette and long hem. Worn cloth and dark flexible layers; no biological appendages, wings, magic or new armor.'),
    ('FIN-21','Plastron Bastion','core:plastron_bastion',216,107,
     'The Bastion breastplate outfit: bulky dirty-ivory heavy chest, shoulder, forearm and leg armor plates over a dark charcoal undersuit, broad enclosed ivory helmet, dark gloves and boots. Preserve the broad heavy-armored silhouette, rounded rectangular armor masses and complete opaque face protection; distinguish heavy ceramic/metal shells from the lighter Cerbere reference.'),
]

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

def prepare():
    ART.mkdir(parents=True,exist_ok=True)
    DOC.mkdir(parents=True,exist_ok=True)
    common = ('Use case: style-transfer. Production sprite revision for Project RL, a grimdark futuristic/cyberpunk roguelike on 64px world cells. '
              'Input image 1 is the EDIT TARGET: retain its identity, material colors, clothing/armor layout, proportions and pose. '
              'Input images 2 and 3 are ONLY the already accepted finish references (Bastion and Cerbere), never characters to copy. '
              'Render the target as a complete detailed source master with the same matte worn game painting, broad readable shadow planes, believable folds and material volumes, controlled irregular dark contours, restrained highlights and locally placed wear. '
              'No glossy plastic, sterile vector/cel shading, thick uniform outlines or excessive noisy microdetail. Readable after reduction to about 60 game pixels tall. '
              'Same slightly elevated frontal standing sprite view as the targets; not isometric and not directly looking down on the head. Neutral diffuse overhead lighting. '
              'Whole figure from top of head to soles, all limbs complete, generous clear margins, TRUE TRANSPARENT background. No ground, floor, scene, cast shadow, frame, words, labels, glow or effects. '
              'Standing with feet a little apart and EMPTY gloved fists held slightly out to the sides at waist level, bent elbows, ready for separate weapon attachments. No weapon or new accessory. ')
    records = []
    for code,label,identity,outfit,torso,specific in ENTRIES:
        records.append(dict(id=code,label=label,identity=identity,outfit=outfit,torso=torso,
            prompt=common+specific,
            referenced_image_paths=[str(ART/f'{code}-reference.png').replace('\\','/'),
                str(ROOT/'assets/source-art/art-finish-review-01/FIN-01-source.png').replace('\\','/'),
                str(ROOT/'assets/source-art/art-finish-review-01/FIN-02-source.png').replace('\\','/')],
            transparent_background=True))
    write(ART/'prompts.json',dict(review='ART-FINISH-02',mode='built-in imagegen',status='candidate_not_integrated',assets=records))
    baseline = DOC/'production-before.json'
    if not baseline.exists():
        paths = [p for p in (ROOT/'assets/sprites').rglob('*') if p.is_file()]
        paths += [ROOT/'assets/maps/prison-street-expanded-01'/n for n in ['street.json','street.png','layout.txt']]
        write(baseline,{p.relative_to(ROOT).as_posix():sha(p) for p in paths})
    print('Prepared 8 outfit requests, FIN-14 through FIN-21; normal assets frozen')

SCALE = 8
SINGLE_HANDS = {14:(83,62),15:(84,62),16:(84,62),17:(83,62),
                18:(84,63),19:(83,62),20:(83,62),21:(83,63)}
RIFLE_HANDS = {n:((59,61),(73,64)) for n in range(14,22)}

def normalized(name, factor=SCALE, *, art=ART):
    image = Image.open(art/f'{name}-source.png').convert('RGBA')
    alpha = image.getchannel('A')
    box = alpha.point(lambda a:255 if a>220 else 0).getbbox()
    assert box and min(box[:2])>0 and box[2]<image.width and box[3]<image.height, name
    body = image.crop(box)
    # Remove source-export haze; final reduction reconstructs soft native edges.
    body.putalpha(body.getchannel('A').point(lambda a:0 if a<180 else a))
    ratio = min(44/body.width,60/body.height)*factor
    body = body.resize((round(body.width*ratio),round(body.height*ratio)),Image.Resampling.LANCZOS)
    result = Image.new('RGBA',(128*factor,128*factor))
    result.alpha_composite(body,(64*factor-body.width//2,96*factor-body.height))
    return result

def selected(source, polygons):
    mask = Image.new('L',source.size)
    draw = ImageDraw.Draw(mask)
    for polygon in polygons:
        draw.polygon([(round(x*SCALE),round(y*SCALE)) for x,y in polygon],fill=255)
    result = source.copy()
    result.putalpha(ImageChops.multiply(source.getchannel('A'),mask))
    return result,mask

def save_layer(image, name, *, art=ART):
    image.resize((128,128),Image.Resampling.LANCZOS).save(art/'native'/f'{name}.png')

def weapon(pose, anchors):
    ident = 'FIN-13' if pose=='single' else 'FIN-12'
    image = Image.open(ROOT/f'assets/source-art/art-finish-review-01/{ident}-source.png').convert('RGBA')
    box = image.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
    image = image.crop(box)
    image.putalpha(image.getchannel('A').point(lambda a:0 if a<40 else a))
    length = 17 if pose=='single' else 34
    image.thumbnail((length*SCALE,18*SCALE),Image.Resampling.LANCZOS)
    result = Image.new('RGBA',(128*SCALE,128*SCALE))
    if pose=='single':
        x,y=anchors
        result.alpha_composite(image,(round(x*SCALE-image.width*.24),round(y*SCALE-image.height*.64)))
    else:
        (x,y),(xx,yy)=anchors
        # The source pose's actual hand line determines its physical gun angle.
        angle = math.atan2(yy-y,xx-x)
        src = Image.new('RGBA',result.size)
        pivot=(x*SCALE,y*SCALE)
        src.alpha_composite(image,(round(pivot[0]-image.width*.29),round(pivot[1]-image.height*.60)))
        result=src.rotate(-math.degrees(angle),Image.Resampling.BICUBIC,center=pivot)
    return result,length

def condition():
    assert not (DOC/'acceptance.json').exists(), 'Accepted exports are frozen'
    (ART/'native').mkdir(parents=True,exist_ok=True)
    records=[]
    for code,label,identity,outfit,torso,_ in ENTRIES:
        base=normalized(code)
        save_layer(base,code)
        arms=[[(27,46),(53,46),(57,51),(55,57),(52,75),(27,75)],
              [(75,46),(101,46),(101,75),(76,75),(73,57),(71,51)]]
        _,mask=selected(base,arms)
        core=base.copy()
        core.putalpha(ImageChops.multiply(base.getchannel('A'),ImageChops.invert(mask)))
        alternate=normalized(code+'-RIFLE')
        rifle_arms=[[(40,46),(55,46),(59,53),(65,57),(64,71),(43,74)],
                    [(72,46),(88,46),(88,74),(66,74),(64,59),(70,53)]]
        sleeves,_=selected(alternate,rifle_arms)
        configs={}
        number=int(code[-2:])
        for pose in ['single','rifle']:
            posed=base.copy() if pose=='single' else core.copy()
            if pose=='rifle': posed.alpha_composite(sleeves)
            anchors=SINGLE_HANDS[number] if pose=='single' else RIFLE_HANDS[number]
            points=[anchors] if pose=='single' else anchors
            fingers,_=selected(posed,[[(x-2,y-2),(x+2,y-2),(x+2,y+2),(x-2,y+2)] for x,y in points])
            held,length=weapon(pose,anchors)
            names=[f'{code}-{pose}-{part}' for part in ['body','weapon','fingers']]
            for image,name in zip([posed,held,fingers],names): save_layer(image,name)
            original=Image.open(ART/'native'/f'{code}.png')
            registered=Image.open(ART/'native'/f'{code}-{pose}-body.png')
            # Keep the fixed core exact even at the finite reduction kernel's
            # edge: registration is part of layer assembly, not a new painting.
            for area in [(0,0,128,44),(0,80,128,128)]:
                registered.paste(original.crop(area),area[:2])
            registered.save(ART/'native'/f'{code}-{pose}-body.png')
            composed=registered.copy()
            for name in names[1:]: composed.alpha_composite(Image.open(ART/'native'/f'{name}.png'))
            composed.save(ART/'native'/f'{code}-{pose}.png')
            fixed=[]
            target=Image.open(ART/'native'/f'{code}-{pose}-body.png')
            for area in [(0,0,128,44),(0,80,128,128)]:
                assert ImageChops.difference(original.crop(area),target.crop(area)).getbbox() is None,(code,pose,area)
                fixed.append(list(area))
            configs[pose]=dict(layers=[n+'.png' for n in names],hand_anchors=anchors,held_length=length,fixed_regions=fixed)
        native=Image.open(ART/'native'/f'{code}.png')
        records.append(dict(code=code,label=label,identity=identity,outfit=outfit,torso=torso,
            source_sha256=sha(ART/f'{code}-source.png'),native_sha256=sha(ART/'native'/f'{code}.png'),
            size=[128,128],foot=[64,96],bounds=list(native.getbbox()),status='pending_visual_review',configurations=configs))
    for file,digest in json.loads((DOC/'production-before.json').read_text('utf-8')).items():
        assert sha(ROOT/file)==digest,file
    write(DOC/'review.json',dict(review='ART-FINISH-02',status='pending_visual_review',
        ordinary_runtime_unchanged=True,native_cell=64,drawings=8,configuration_samples=24,entries=records))
    print('Conditioned 8 complete outfits and 24 configurations; fixed core verified; normal assets unchanged')

if __name__ == '__main__':
    condition() if '--condition' in sys.argv else prepare()
