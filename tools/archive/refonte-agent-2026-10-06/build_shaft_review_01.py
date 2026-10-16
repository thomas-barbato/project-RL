"""Condition generated art and reuse approved two-handed arms; review only."""
from pathlib import Path
import hashlib,json,math,shutil
from PIL import Image,ImageDraw
from build_focalizer_review_01 import font,ref,jref,save,write,url,condition

ROOT=Path(__file__).resolve().parents[2]
BASE=Path(__file__).resolve().parent
OUT=BASE/'shaft-review-01'
BANK=BASE/'weapon-silhouette-review-03'
RIG=ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json'
BG=(12,19,27);FG=(229,236,240);SUB=(160,182,196)
NAMES=['Bastion','Technomancien','Traqueur','Arcaniste','Métamorphe']
TARGET_GRIPS=[(32,38),(35,34)]

def atlas_cell(atlas,index):
    x=index%16*64;y=index//16*64
    return atlas.crop((x,y,x+64,y+64))

def placed(held,grips):
    lower,upper=grips
    assert lower[0]==upper[0] and lower[1]-upper[1]==5
    scratch=Image.new('RGBA',(96,96))
    scratch.alpha_composite(held,(48-lower[0],48-lower[1]))
    angle=-math.degrees(math.atan2(3,4))
    rotated=scratch.rotate(angle,resample=Image.Resampling.NEAREST,center=(48,48))
    result=Image.new('RGBA',(64,64))
    result.paste(rotated,(TARGET_GRIPS[0][0]-48,TARGET_GRIPS[0][1]-48))
    assert sum(a>0 for a in rotated.getchannel('A').tobytes())==sum(a>0 for a in result.getchannel('A').tobytes())
    assert all(result.getpixel(p)[3]==255 for p in TARGET_GRIPS)
    return result,angle

def main():
    assert not (OUT/'review-shafts-SHAFT-VIS01-01.json').exists()
    entries=json.loads((OUT/'prompts.json').read_text(encoding='utf-8'))['assets']
    rig=json.loads(RIG.read_text(encoding='utf-8'))
    frozen={ROOT/r['path']:r['binary_sha256'] for r in rig['files']}
    assert all(hashlib.sha256(p.read_bytes()).hexdigest()==digest for p,digest in frozen.items())
    arms_path=ROOT/'assets/sprites/cyberpunk/agent-rigs/arms-64.png'
    fingers_path=ROOT/'assets/sprites/cyberpunk/agent-rigs/fingers-64.png'
    arms=Image.open(arms_path).convert('RGBA');fingers=Image.open(fingers_path).convert('RGBA')
    floor_tile=Image.open(BANK/'review-floor-native32.png').convert('RGBA')
    terrain=Image.new('RGBA',(64,64))
    for y in (0,32):
        for x in (0,32):terrain.alpha_composite(floor_tile,(x,y))
    floors={'terrain':terrain,'dark':Image.new('RGBA',(64,64),(16,23,31,255)),
            'light':Image.new('RGBA',(64,64),(173,182,184,255))}
    candidates=[];placements=[];compositions=[];images={}
    for number,e in enumerate(entries,1):
        cid=e['candidate_id'];source=OUT/'sources'/f'{cid}.png'
        source.parent.mkdir(exist_ok=True)
        if not source.exists():shutil.copy2(e['source_path'],source)
        raw=Image.open(source).convert('RGBA')
        bbox=raw.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
        crop=raw.crop(bbox);size=tuple(e['native_dimensions']);grips=[tuple(p) for p in e['grips']]
        inv_size=(round(size[0]*30/size[1]),30)
        pixels=[p[:3] for s in (size,inv_size) for p in crop.resize(s,Image.Resampling.NEAREST).get_flattened_data() if p[3]>=128]
        strip=Image.new('RGB',(len(pixels),1));strip.putdata(pixels)
        palette=strip.quantize(colors=12,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE)
        held=condition(crop,size,palette)
        assert all(held.getpixel(p)[3]==255 for p in grips)
        inv=condition(crop,inv_size,palette);inventory=Image.new('RGBA',(32,32))
        inventory.alpha_composite(inv,((32-inv.width)//2,1))
        colors=sorted({p[:3] for p in held.get_flattened_data() if p[3]})
        assert len(colors)<=12 and set(held.getchannel('A').tobytes())<={0,255}
        weapon,angle=placed(held,grips)
        candidates.append({'candidate_id':cid,'review_number':number,'source':ref(source),'source_crop':list(bbox),
                           'held_layer':save(OUT/f'{cid}-held.png',held),'inventory_tile':save(OUT/f'{cid}-inventory-native32.png',inventory),
                           'held_grips':[list(p) for p in grips],'canvas_grips':[list(p) for p in TARGET_GRIPS],
                           'opaque_colors':colors,'conditioning':'nearest;source-derived_12-color_palette_no_dither;binary_alpha_128',
                           'visual_approval':'pending_explicit_user_review','source_prompt':jref(OUT/'prompts.json')})
        images[(cid,'inventory')]=inventory
        for profile in rig['profiles']:
            n=profile['number'];index=(n-1)*25+2
            assert rig['side_family_order'][2]==['R',2]
            sleeve=atlas_cell(arms,index);glove=atlas_cell(fingers,index)
            body_path=BANK/f'body-{n}-native32.png';original=Image.open(body_path).convert('RGBA');body=original.copy()
            allowed=Image.new('L',(64,64));ad=ImageDraw.Draw(allowed)
            for x0,y0,x1,y1 in profile['forearm_masks']:
                body.paste((0,0,0,0),(x0,y0,x1,y1));ad.rectangle((16+x0,16+y0,15+x1,15+y1),fill=255)
            frame=Image.new('RGBA',(64,64));frame.alpha_composite(body,(16,16))
            for layer in (sleeve,weapon,glove):
                frame.alpha_composite(layer);allowed=Image.composite(Image.new('L',(64,64),255),allowed,layer.getchannel('A'))
            baseline=Image.new('RGBA',(64,64));baseline.alpha_composite(original,(16,16))
            outside=sum(a!=b and not m for a,b,m in zip(baseline.get_flattened_data(),frame.get_flattened_data(),allowed.get_flattened_data()))
            assert outside==0
            contacts=[]
            for gx,gy in TARGET_GRIPS:
                contact=sum(weapon.getpixel((x,y))[3]>0 and glove.getpixel((x,y))[3]>0
                            for y in range(gy-2,gy+3) for x in range(gx-2,gx+3))
                assert contact>=2,(cid,n,(gx,gy),contact)
                contacts.append(contact)
            assert set(frame.getchannel('A').tobytes())<={0,255}
            layer_ref=save(OUT/'layers'/f'{cid}-C{n}-weapon.png',weapon)
            placements.append({'candidate_id':cid,'class_number':n,'weapon_layer':layer_ref,
                               'canvas_grips':[list(p) for p in TARGET_GRIPS],'angle_from_vertical':angle,
                               'finger_weapon_contacts':contacts,'approved_arm_atlas_index':index,
                               'arm_layer':save(OUT/'layers'/f'{cid}-C{n}-arms.png',sleeve),
                               'finger_layer':save(OUT/'layers'/f'{cid}-C{n}-fingers.png',glove),
                               'new_arm_or_finger_pixels':0,'clipped_weapon_pixels':0})
            frame_ref=save(OUT/'frames'/f'{cid}-C{n}.png',frame)
            compositions.append({**frame_ref,'candidate_id':cid,'class_number':n,'class_name':profile['class'],
                                 'source_body':ref(body_path),'outside_equipment_changes':outside})
            images[(cid,n)]=frame
    def scene(cid,n,floor='terrain'):
        result=floors[floor].copy();result.alpha_composite(images[(cid,n)]);return result
    board=Image.new('RGB',(1110,705),BG);d=ImageDraw.Draw(board)
    d.text((18,12),'Hampes de canalisation · six candidats',font=font(23),fill=FG)
    d.text((18,46),'SHAFT-VIS01 · deux prises · personnages natifs 32 px et zoom ×2',font=font(13),fill=SUB)
    for n,name in enumerate(NAMES,1):d.text((190+(n-1)*181,85),name,font=font(13),fill=SUB)
    for i,r in enumerate(candidates):
        cid=r['candidate_id'];y=110+i*96;d.line((18,y-7,1090,y-7),fill=(37,50,61))
        d.text((18,y+6),f'{i+1:02}',font=font(16),fill=FG)
        inv=images[(cid,'inventory')];board.paste(inv,(55,y+10),inv)
        zoom=inv.resize((64,64),Image.Resampling.NEAREST);board.paste(zoom,(94,y+4),zoom)
        for n in range(1,6):
            frame=scene(cid,n);x=178+(n-1)*181
            board.paste(frame,(x,y+3));board.paste(frame.resize((128,128),Image.Resampling.NEAREST).crop((16,16,112,112)),(x+64,y-10))
    overview=save(OUT/'shafts-native-and-zoom.png',board)
    diag=Image.new('RGB',(790,625),BG);d=ImageDraw.Draw(diag)
    d.text((18,12),'Hampes · diagnostic 30 / 32 px',font=font(23),fill=FG)
    d.text((18,46),'Sombre, clair et terrain · les sources natives restent à 32 pixels',font=font(12),fill=SUB)
    for i,r in enumerate(candidates):
        cid=r['candidate_id'];y=102+i*83;d.text((18,y+17),cid,font=font(13),fill=FG)
        for k,floor in enumerate(('dark','light','terrain')):
            for j,px in enumerate((30,32)):
                x=142+k*210+j*78;diag.paste(scene(cid,4,floor).resize((px*2,px*2),Image.Resampling.NEAREST),(x,y))
                d.text((x,y-14),f'{px} px',font=font(10),fill=SUB)
    readability=save(OUT/'shafts-readability-30-32.png',diag)
    payload={'candidates':[{'id':r['candidate_id'],'number':r['review_number'],'inventory':url(images[(r['candidate_id'],'inventory')])} for r in candidates],
             'classes':NAMES,'floors':{k:url(im) for k,im in floors.items()},
             'frames':[{**r,'url':url(images[(r['candidate_id'],r['class_number'])])} for r in compositions]}
    template=BASE/'shaft_gallery_template_01.html';gallery=OUT/'shaft-gallery-01.html'
    gallery.write_text(template.read_text(encoding='utf-8').replace('__PAYLOAD__',json.dumps(payload,ensure_ascii=False)),encoding='utf-8',newline='\n')
    manifest={'date':'2026-10-10','review_id':'SHAFT-VIS01','revision':1,'status':'numbered_native_review_pending_pixels_and_concrete_grips',
              'direction':jref(BASE/'caster-shield-heavy-expansion-directions-01.json'),
              'series':jref(BASE/'review-equipment-series-EQUIP-SERIES01-05.json'),
              'source_rig':jref(RIG),'source_weapon_bank':jref(BANK/'review-weapon-silhouettes-WEAPON-SIL01-03.json'),
              'arm_source':ref(arms_path),'finger_source':ref(fingers_path),'reused_pose_family':2,
              'generator':ref(Path(__file__)),'template':ref(template),'prompts':jref(OUT/'prompts.json'),
              'candidates':candidates,'placements':placements,'compositions':compositions,
              'overview':overview,'readability':readability,'gallery':ref(gallery),
              'qa':{'candidates':6,'two_hand_placements':len(placements),'compositions':len(compositions),
                    'outside_equipment_changes':0,'new_arm_or_finger_pixels':0,'body_geometry_changes':0,
                    'clipped_pixels':0,'alpha':'binary','weapon_palette_independent_from_armor':True},
              'approval':{'family_direction':'accepted','new_pixels_and_concrete_grips':'pending','item_names_RP_stats':'not_defined_here'},
              'runtime_integration':'not_integrated;review_only'}
    assert all(hashlib.sha256(p.read_bytes()).hexdigest()==digest for p,digest in frozen.items())
    write(OUT/'review-shafts-SHAFT-VIS01-01.json',manifest)
    print(json.dumps(manifest['qa']))

if __name__=='__main__':main()
