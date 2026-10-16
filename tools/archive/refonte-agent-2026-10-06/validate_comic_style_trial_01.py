"""Verify review references and save a genuine five-card browser proof board."""
from pathlib import Path
import hashlib,json,math
from PIL import Image,ImageDraw
from build_focalizer_review_01 import font,ref,save,write
ROOT=Path(__file__).resolve().parents[2];BASE=Path(__file__).resolve().parent;OUT=BASE/'comic-style-review-01'
data=json.loads((OUT/'review-BD-STYLE01-01.json').read_text(encoding='utf-8'))
checked=[]
def verify(obj):
    if isinstance(obj,dict):
        if 'path' in obj and 'binary_sha256' in obj:
            p=ROOT/obj['path'];assert hashlib.sha256(p.read_bytes()).hexdigest()==obj['binary_sha256'],p
            if 'dimensions' in obj:assert list(Image.open(p).size)==obj['dimensions'],p
            checked.append(obj['path'])
        if 'reference' in obj and 'reference_sha256' in obj:
            p=ROOT/obj['reference'];assert hashlib.sha256(p.read_text(encoding='utf-8').encode()).hexdigest()==obj['reference_sha256'],p
            checked.append(obj['reference'])
        for v in obj.values():verify(v)
    elif isinstance(obj,list):
        for v in obj:verify(v)
verify(data)
approval=json.loads((BASE/'shaft-visual-acceptance-01.json').read_text(encoding='utf-8'));verify(approval)
assert {r['candidate_id'] for r in approval['approved_candidates']}=={f'EQ-H0{i}' for i in range(1,7)}
assert {r['review_id'] for r in data['candidates']}=={f'BD-C0{i}' for i in range(1,6)}
for row in data['candidates']:
    raw=Image.open(ROOT/row['source']['path']).convert('RGBA')
    assert 0 in raw.getchannel('A').tobytes() and 255 in raw.getchannel('A').tobytes()
    for tile in (30,32):
        im=Image.open(ROOT/row['native_tiles'][str(tile)]['path']).convert('RGBA');assert im.size==(tile*2,tile*2)
        box=im.getchannel('A').getbbox();assert 0<box[0]<box[2]<im.width and 0<box[1]<box[3]<im.height
bounds=json.loads((OUT/'browser-card-bounds.json').read_text(encoding='utf-8'))
cards=[];measurements=[]
for i,box in enumerate(bounds,1):
    image=Image.open(OUT/f'browser-BD-C0{i}.png').convert('RGB')
    sx=image.width/box['viewportWidth'];sy=image.height/box['viewportHeight']
    crop=(math.floor(box['x']*sx),math.floor(box['y']*sy),math.ceil((box['x']+box['width'])*sx),math.ceil((box['y']+box['height'])*sy))
    assert crop[0]>=0 and crop[1]>=0 and crop[2]<=image.width and crop[3]<=image.height
    cards.append(image.crop(crop));measurements.append({'id':f'BD-C0{i}','screenshot':list(image.size),'css_viewport':[box['viewportWidth'],box['viewportHeight']],'crop_pixels':list(crop),'provider_scale':[sx,sy]})
w=max(c.width for c in cards);h=max(c.height for c in cards)
board=Image.new('RGB',(w*3+40,h*2+100),(14,21,29));d=ImageDraw.Draw(board)
d.text((18,15),'Project RL · Essai bande dessinée',font=font(27),fill=(235,240,244))
d.text((18,54),'Cinq personnages équipés · captures de la galerie · comparatif 30 / 32 px',font=font(15),fill=(160,182,196))
for i,card in enumerate(cards):board.paste(card,(10+(i%3)*(w+10),80+(i//3)*(h+10)))
x=10+2*(w+10);y=80+h+10
d.text((x+22,y+32),'BD-STYLE01',font=font(27),fill=(235,240,244))
for k,line in enumerate(['Contours encrés','Couleurs en aplats','Ombres simples','Armes en main','Étude visuelle à examiner']):
    d.text((x+22,y+100+k*40),line,font=font(20),fill=(160,182,196))
proof=save(OUT/'comic-style-browser-board-01.png',board)
write(OUT/'validation-01.json',{'review_id':'BD-STYLE01','references_verified':len(checked),'characters':5,'native_variants':10,'transparent_sources':True,'normalized_equipment_clipped':False,
 'shaft_approval_verified':True,'browser_card_count':5,'browser_contact_board':proof,'browser_measurements':measurements,'browser_card_resampling':'none;crop_and_paste_only',
 'browser_capture_is_exact_native_pixel_scale':'not_claimed;native_CSS_sizes_verified_separately','new_art_style_integration':'none;pending_user_review'})
print(json.dumps({'references_verified':len(checked),'native_variants':10,'browser_cards':5,'runtime_integration':'none'}))
