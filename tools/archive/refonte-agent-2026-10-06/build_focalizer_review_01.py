"""Condition imagegen sources and compose fixed existing bodies for review only."""
from pathlib import Path
import base64, hashlib, json, math, shutil
from io import BytesIO
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
BASE=Path(__file__).resolve().parent
OUT=BASE/'focalizer-review-01'
BANK=BASE/'weapon-silhouette-review-03'
SHIELDS=BASE/'shield-review-07'
RIG=ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json'
BG=(12,19,27);FG=(229,236,240);SUB=(160,182,196)
NAMES=['Bastion','Technomancien','Traqueur','Arcaniste','Métamorphe']

def font(n):return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',n)
def ref(p):return {'path':p.relative_to(ROOT).as_posix(),'binary_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
def jref(p):return {'reference':p.relative_to(ROOT).as_posix(),'reference_sha256':hashlib.sha256(p.read_text(encoding='utf-8').encode()).hexdigest()}
def save(p,im):
    p.parent.mkdir(parents=True,exist_ok=True);im.save(p)
    return {**ref(p),'dimensions':list(im.size)}
def write(p,data):p.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
def url(im):
    b=BytesIO();im.save(b,format='PNG');return 'data:image/png;base64,'+base64.b64encode(b.getvalue()).decode()
def condition(crop,size,palette):
    sample=crop.resize(size,Image.Resampling.NEAREST)
    alpha=sample.getchannel('A').point(lambda v:255 if v>=128 else 0)
    im=sample.convert('RGB').quantize(palette=palette,dither=Image.Dither.NONE).convert('RGBA');im.putalpha(alpha)
    return im
def placed(profile,side,held,grip):
    # The canonical head is above the handle. Turn it outward, aligned with
    # THIS hand's elbow/wrist vector; reuse the existing arm without warping.
    weapon=Image.new('RGBA',(48,48));offset=(24-grip[0],18)
    weapon.alpha_composite(held,offset);pivot=(24,18+grip[1])
    weapon=weapon.transpose(Image.Transpose.FLIP_TOP_BOTTOM);pivot=(pivot[0],47-pivot[1])
    scratch=Image.new('RGBA',(96,96));scratch.alpha_composite(weapon,(48-pivot[0],48-pivot[1]))
    hand=profile['gloves'][side];elbow=profile['elbows'][side]
    angle=math.degrees(math.atan2(hand[0]-elbow[0],hand[1]-elbow[1]))
    rotated=scratch.rotate(angle,resample=Image.Resampling.NEAREST,center=(48,48))
    anchor=(16+hand[0],16+hand[1]);layer=Image.new('RGBA',(64,64))
    layer.paste(rotated,(anchor[0]-48,anchor[1]-48))
    count=lambda im:sum(a>0 for a in im.getchannel('A').tobytes())
    assert count(layer)==count(rotated)
    assert layer.getpixel(anchor)[3]==255,(profile['number'],side,grip)
    body=Image.open(BANK/f"body-{profile['number']}-native32.png").convert('RGBA')
    contact=sum(layer.getpixel((16+x,16+y))[3]>0 and body.getpixel((x,y))[3]>0
                for y in range(hand[1]-2,hand[1]+3) for x in range(hand[0]-2,hand[0]+3))
    assert contact>=2,(profile['number'],side,contact)
    return layer,{'native_hand_anchor':hand,'canvas_hand_anchor':list(anchor),'angle_from_vertical':angle,
                  'handle_body_contact_pixels':contact,'clipped_pixels':0,'new_forearm_layers':0,'new_hand_layers':0}

def main():
    entries=json.loads((OUT/'prompts.json').read_text(encoding='utf-8'))['assets']
    profiles=json.loads(RIG.read_text(encoding='utf-8'))['profiles']
    shield_bank=json.loads((SHIELDS/'review-shields-SHIELD-VIS01-07.json').read_text(encoding='utf-8'))
    shield_ids=[r['candidate_id'] for r in shield_bank['candidates']]
    floor_tile=Image.open(BANK/'review-floor-native32.png').convert('RGBA')
    terrain=Image.new('RGBA',(64,64))
    for y in (0,32):
        for x in (0,32):terrain.alpha_composite(floor_tile,(x,y))
    floors={'terrain':terrain,'dark':Image.new('RGBA',(64,64),(16,23,31,255)),
            'light':Image.new('RGBA',(64,64),(173,182,184,255))}
    records=[];compositions=[];images={};placements=[]
    for number,e in enumerate(entries,1):
        cid=e['candidate_id'];source=OUT/'sources'/f'{cid}.png';source.parent.mkdir(exist_ok=True)
        if not source.exists():shutil.copy2(e['source_path'],source)
        raw=Image.open(source).convert('RGBA');bbox=raw.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
        crop=raw.crop(bbox);size=tuple(e['native_dimensions']);grip=tuple(e['grip'])
        inv_size=(min(27,round(27*crop.width/crop.height)),27)
        samples=[crop.resize(s,Image.Resampling.NEAREST) for s in (size,inv_size)]
        pixels=[p[:3] for im in samples for p in im.get_flattened_data() if p[3]>=128]
        strip=Image.new('RGB',(len(pixels),1));strip.putdata(pixels)
        palette=strip.quantize(colors=12,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE)
        held=condition(crop,size,palette);assert held.getpixel(grip)[3]==255
        inv=condition(crop,inv_size,palette);inventory=Image.new('RGBA',(32,32))
        inventory.alpha_composite(inv,((32-inv.width)//2,(32-inv.height)//2))
        colors=sorted({p[:3] for p in held.get_flattened_data() if p[3]})
        assert len(colors)<=12
        assert {p[3] for p in held.get_flattened_data()}<={0,255}
        held_ref=save(OUT/f'{cid}-held.png',held);inv_ref=save(OUT/f'{cid}-inventory-native32.png',inventory)
        records.append({'candidate_id':cid,'review_number':number,'source':ref(source),'source_crop':list(bbox),
                        'held_layer':held_ref,'held_grip':list(grip),'inventory_tile':inv_ref,
                        'opaque_colors':colors,'visual_approval':'pending_explicit_user_review',
                        'conditioning':'nearest;source-derived_12-color_palette_no_dither;binary_alpha_128',
                        'source_prompt':jref(OUT/'prompts.json'),'models_or_rules_added':False})
        images[(cid,'inventory')]=inventory
        for profile in profiles:
            n=profile['number'];layers={};qa={}
            for side in ('L','R'):
                layers[side],qa[side]=placed(profile,side,held,grip)
                layer_ref=save(OUT/'layers'/f'{cid}-C{n}-{side}.png',layers[side])
                placements.append({'candidate_id':cid,'class_number':n,'hand':side,'layer':layer_ref,**qa[side]})
            for side in ('L','R'):
                opposite='L' if side=='R' else 'R'
                for other in ['none','knife','pistol','same',*shield_ids]:
                    family={'knife':'00','pistol':'06'}.get(other,'none')
                    filename=f'C{n}-L{family}-Rnone.png' if side=='R' else f'C{n}-Lnone-R{family}.png'
                    baseline_path=BANK/filename;baseline=Image.open(baseline_path).convert('RGBA')
                    combined=layers[side].copy();frame=baseline.copy();frame.alpha_composite(layers[side])
                    if other=='same':
                        combined.alpha_composite(layers[opposite]);frame.alpha_composite(layers[opposite])
                    if other in shield_ids:
                        shield=Image.open(SHIELDS/'layers'/f'{other}-C{n}-{opposite}.png').convert('RGBA')
                        combined.alpha_composite(shield);frame.alpha_composite(shield)
                    mask=combined.getchannel('A')
                    changes=sum(a!=b and not m for a,b,m in zip(baseline.get_flattened_data(),frame.get_flattened_data(),mask.get_flattened_data()))
                    assert changes==0
                    fr=save(OUT/'frames'/f'{cid}-C{n}-{side}-{other}.png',frame)
                    fr.update(candidate_id=cid,class_number=n,class_name=profile['class'],focalizer_hand=side,
                              other_hand=other,source_frame=ref(baseline_path),outside_equipment_changes=0)
                    compositions.append(fr);images[(cid,n,side,other)]=frame
    def scene(cid,n,side,other,floor='terrain'):
        im=floors[floor].copy();im.alpha_composite(images[(cid,n,side,other)]);return im
    overview=Image.new('RGB',(1110,705),BG);d=ImageDraw.Draw(overview)
    d.text((18,12),'Focaliseurs · six candidats',font=font(23),fill=FG)
    d.text((18,46),'FOCAL-VIS01 · main droite, bouclier EQ-S01 à gauche · taille native et zoom ×2',font=font(13),fill=SUB)
    d.text((35,85),'Objet',font=font(13),fill=SUB)
    for n,name in enumerate(NAMES,1):d.text((190+(n-1)*181,85),name,font=font(13),fill=SUB)
    for i,r in enumerate(records):
        cid=r['candidate_id'];y=110+i*96;d.line((18,y-7,1090,y-7),fill=(37,50,61))
        d.text((18,y+4),f'{i+1:02}',font=font(16),fill=FG)
        inv=images[(cid,'inventory')];overview.paste(inv,(53,y+8),inv)
        zoom=inv.resize((64,64),Image.Resampling.NEAREST);overview.paste(zoom,(93,y),zoom)
        for n in range(1,6):
            frame=scene(cid,n,'R','EQ-S01');x=178+(n-1)*181
            overview.paste(frame,(x,y+2));overview.paste(frame.resize((128,128),Image.Resampling.NEAREST).crop((16,16,112,112)),(x+64,y-10))
    overview_ref=save(OUT/'focalizers-native-and-zoom.png',overview)
    both=Image.new('RGB',(810,625),BG);bd=ImageDraw.Draw(both)
    bd.text((18,12),'Focaliseurs · les deux mains',font=font(23),fill=FG)
    bd.text((18,46),'60 placements · personnage natif 32 px · gauche puis droite · autre main vide',font=font(13),fill=SUB)
    for n,name in enumerate(NAMES,1):bd.text((156+(n-1)*128,82),name,font=font(12),fill=SUB)
    for i,r in enumerate(records):
        y=110+i*82;cid=r['candidate_id'];bd.text((18,y+20),f'{i+1:02} · {cid}',font=font(13),fill=FG)
        for n in range(1,6):
            for s,side in enumerate(('L','R')):both.paste(scene(cid,n,side,'none'),(145+(n-1)*128+s*64,y))
    both_ref=save(OUT/'focalizers-both-hands-native.png',both)
    contrast=Image.new('RGB',(790,625),BG);cd=ImageDraw.Draw(contrast)
    cd.text((18,12),'Focaliseurs · diagnostic 30 / 32 px',font=font(23),fill=FG)
    cd.text((18,46),'Sol sombre, clair et terrain · 30 px pour éprouver la lecture, 32 px natifs conservés',font=font(12),fill=SUB)
    for i,r in enumerate(records):
        cid=r['candidate_id'];y=102+i*83;cd.text((18,y+17),cid,font=font(13),fill=FG)
        for k,floor in enumerate(('dark','light','terrain')):
            for j,px in enumerate((30,32)):
                x=142+k*210+j*78;contrast.paste(scene(cid,4,'R','EQ-S01',floor).resize((px*2,px*2),Image.Resampling.NEAREST),(x,y))
                cd.text((x,y-14),f'{px} px',font=font(10),fill=SUB)
    contrast_ref=save(OUT/'focalizers-readability-30-32.png',contrast)
    payload={'candidates':[{'id':r['candidate_id'],'number':r['review_number'],'inventory':url(images[(r['candidate_id'],'inventory')])} for r in records],
             'classes':NAMES,'shields':shield_ids,'floors':{k:url(im) for k,im in floors.items()},
             'frames':[{**r,'url':url(images[(r['candidate_id'],r['class_number'],r['focalizer_hand'],r['other_hand'])])} for r in compositions]}
    template=BASE/'focalizer_gallery_template_01.html';gallery=OUT/'focalizer-gallery-01.html'
    gallery.write_text(template.read_text(encoding='utf-8').replace('__PAYLOAD__',json.dumps(payload,ensure_ascii=False)),encoding='utf-8',newline='\n')
    manifest={'date':'2026-10-10','review_id':'FOCAL-VIS01','revision':1,'status':'numbered_native_review_pending_pixels_and_concrete_grips',
              'direction':jref(BASE/'caster-shield-heavy-expansion-directions-01.json'),
              'series':jref(BASE/'review-equipment-series-EQUIP-SERIES01-03.json'),
              'accepted_shields':jref(BASE/'shield-visual-acceptance-01.json'),
              'source_rig':jref(RIG),'source_weapon_bank':jref(BANK/'review-weapon-silhouettes-WEAPON-SIL01-03.json'),
              'generator':ref(Path(__file__)),'template':ref(template),'prompts':jref(OUT/'prompts.json'),
              'candidates':records,'placements':placements,'compositions':compositions,
              'overview':overview_ref,'both_hands_native':both_ref,'readability':contrast_ref,'gallery':ref(gallery),
              'qa':{'candidates':6,'hand_placements':len(placements),'compositions':len(compositions),
                    'outside_equipment_changes':0,'new_forearms_or_hands':0,'body_geometry_changes':0,
                    'clipped_pixels':0,'alpha':'binary','weapon_palette_independent_from_armor':True},
              'approval':{'family_direction':'accepted','new_pixels_and_concrete_grips':'pending',
                          'item_names_RP_stats':'not_defined_here'},'runtime_integration':'not_integrated;review_only'}
    write(OUT/'review-focalizers-FOCAL-VIS01-01.json',manifest)
    print(json.dumps(manifest['qa']))

if __name__=='__main__':main()
