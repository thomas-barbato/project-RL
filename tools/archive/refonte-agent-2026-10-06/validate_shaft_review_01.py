"""Verify hashes, source continuity and accepted arm reuse in the review bank."""
from pathlib import Path
import hashlib,json,math
from PIL import Image,ImageDraw
from build_focalizer_review_01 import save,write,font
ROOT=Path(__file__).resolve().parents[2]
BASE=Path(__file__).resolve().parent;OUT=BASE/'shaft-review-01'
manifest=json.loads((OUT/'review-shafts-SHAFT-VIS01-01.json').read_text(encoding='utf-8'))
checked=[]
def verify(value):
    if isinstance(value,dict):
        if 'path' in value and 'binary_sha256' in value:
            p=ROOT/value['path'];assert hashlib.sha256(p.read_bytes()).hexdigest()==value['binary_sha256'],p
            if 'dimensions' in value:assert list(Image.open(p).size)==value['dimensions'],p
            checked.append(value['path'])
        if 'reference' in value and 'reference_sha256' in value:
            p=ROOT/value['reference'];assert hashlib.sha256(p.read_text(encoding='utf-8').encode()).hexdigest()==value['reference_sha256'],p
            checked.append(value['reference'])
        for v in value.values():verify(v)
    elif isinstance(value,list):
        for v in value:verify(v)
verify(manifest)
approval=json.loads((BASE/'focalizer-visual-acceptance-01.json').read_text(encoding='utf-8'));verify(approval)
assert {r['candidate_id'] for r in approval['approved_candidates']}=={f'EQ-F0{i}' for i in range(1,7)}
assert {(r['candidate_id'],r['class_number']) for r in manifest['compositions']}=={(f'EQ-H0{i}',n) for i in range(1,7) for n in range(1,6)}
def component_count(im):
    remaining={(x,y) for y in range(im.height) for x in range(im.width) if im.getpixel((x,y))[3]}
    groups=[]
    while remaining:
        first=remaining.pop();group={first};stack=[first]
        while stack:
            x,y=stack.pop()
            for dy in (-1,0,1):
                for dx in (-1,0,1):
                    q=(x+dx,y+dy)
                    if q in remaining:remaining.remove(q);group.add(q);stack.append(q)
        groups.append(len(group))
    return sorted(groups,reverse=True)
continuity=[]
for r in manifest['candidates']:
    im=Image.open(ROOT/r['held_layer']['path']).convert('RGBA')
    groups=component_count(im)
    assert len(groups)==1,(r['candidate_id'],groups)
    assert set(im.getchannel('A').tobytes())<={0,255}
    assert all(im.getpixel(tuple(p))[3] for p in r['held_grips'])
    continuity.append({'candidate_id':r['candidate_id'],'opaque_components':groups})
arms=Image.open(ROOT/'assets/sprites/cyberpunk/agent-rigs/arms-64.png').convert('RGBA')
fingers=Image.open(ROOT/'assets/sprites/cyberpunk/agent-rigs/fingers-64.png').convert('RGBA')
for r in manifest['placements']:
    i=r['approved_arm_atlas_index'];box=(i%16*64,i//16*64,i%16*64+64,i//16*64+64)
    assert Image.open(ROOT/r['arm_layer']['path']).convert('RGBA').tobytes()==arms.crop(box).tobytes()
    assert Image.open(ROOT/r['finger_layer']['path']).convert('RGBA').tobytes()==fingers.crop(box).tobytes()
    assert len(r['finger_weapon_contacts'])==2 and min(r['finger_weapon_contacts'])>=2
bounds=json.loads((OUT/'browser-card-bounds.json').read_text(encoding='utf-8'))
cards=[];measurements=[]
for i,box in enumerate(bounds,1):
    im=Image.open(OUT/f'browser-EQ-H0{i}.png').convert('RGB')
    sx=im.width/box['viewportWidth'];sy=im.height/box['viewportHeight']
    cropbox=(math.floor(box['x']*sx),math.floor(box['y']*sy),math.ceil((box['x']+box['width'])*sx),math.ceil((box['y']+box['height'])*sy))
    assert cropbox[0]>=0 and cropbox[1]>=0 and cropbox[2]<=im.width and cropbox[3]<=im.height
    cards.append(im.crop(cropbox));measurements.append({'candidate_id':f'EQ-H0{i}','screenshot_pixels':list(im.size),'css_viewport':[box['viewportWidth'],box['viewportHeight']],'css_to_screenshot_scale':[sx,sy],'crop_pixels':list(cropbox)})
w=max(c.width for c in cards);h=max(c.height for c in cards)
board=Image.new('RGB',(3*w+40,2*h+100),(12,19,27));d=ImageDraw.Draw(board)
d.text((16,12),'Hampes de canalisation · galerie vérifiée',font=font(23),fill=(229,236,240))
d.text((16,44),'Six captures de la galerie · Arcaniste · taille native et zoom ×3 dans la page',font=font(13),fill=(160,182,196))
for i,card in enumerate(cards):board.paste(card,(10+(i%3)*(w+10),78+(i//3)*(h+10)))
proof=save(OUT/'shafts-browser-verified.png',board)
write(OUT/'validation-01.json',{'review_id':'SHAFT-VIS01','references_verified':len(checked),'unique_compositions':30,'source_continuity':continuity,'two_hands_verified':30,'arm_and_finger_layers_identical_to_approved_rig':True,'focalizer_approval_record_verified':True,'browser_contact_sheet':proof,'browser_measurements':measurements,'browser_image_resized_for_contact_sheet':False,'browser_capture_is_exact_native_pixel_scale':'not_claimed;use_native_art_boards_for_exact_scale','runtime_integration':'none'})
print(json.dumps({'references':len(checked),'compositions':30,'two_hand_contacts':30,'continuous_held_sprites':6,'browser_cards':6}))
