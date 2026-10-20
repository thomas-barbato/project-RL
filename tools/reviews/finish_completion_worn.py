"""Package source-authored limbs, fixed cores and measured weapon attachments."""
import json,math,colorsys
from pathlib import Path
from PIL import Image,ImageDraw,ImageChops,ImageFont
from art_finish_completion import ROOT,ART,DOC,OUT
from art_finish_batch_02 import sha,write,normalized

DEST=ROOT/'assets/sprites/art-finish-completion-06/worn'
OUTFITS=[(200,1,1),(212,2,1),(4,3,1),(6,4,1),(15,5,1),*[(201+i,14+i,2) for i in range(4)],*[(213+i,18+i,2) for i in range(4)],*[(208+i,22+i,3) for i in range(4)],(217,26,4),(218,27,4)]

def region(image,polygons):
    m=Image.new('L',image.size);d=ImageDraw.Draw(m)
    for p in polygons:d.polygon(p,fill=255)
    out=image.copy();out.putalpha(ImageChops.multiply(out.getchannel('A'),m));return out,m

def material_map(im,number):
    # Author-time explicit RGB memberships; runtime has no hue classifier.
    colors={}
    ivory=number in {2,5,21,22,24}
    for r,g,b,a in im.get_flattened_data():
        if not a:continue
        h,l,s=colorsys.rgb_to_hls(r/255,g/255,b/255)
        hue=h*360
        primary=s>.13 and .14<l<.84
        if number in {4,14,15,23}:primary=170<hue<270 and s>.035 and .08<l<.85
        elif number in {16,20,26,27}:primary=(hue<34 or hue>338) and s>.16 and .07<l<.85
        elif number in {1,18}:primary=35<hue<135 and s>.1 and .07<l<(.6 if number==1 else .85)
        elif number==3:primary=hue<44 and s>.20 and .09<l<.84
        elif number==17:primary=28<hue<95 and s>.15 and .08<l<.66
        elif number==19:primary=145<hue<235 and s>.10 and .10<l<.85
        elif number==25:primary=120<hue<235 and s>.025 and .07<l<.46
        elif ivory:primary=l>.30 and s<.45 and (hue<95 or s<.065)
        colors[(r,g,b)]=1 if primary else 0
    out=Image.new('RGBA',im.size)
    out.putdata([(0 if number==1 and (i//im.width<46 or (i//im.width<78 and (i%im.width<56 or i%im.width>73))) else colors.get(p[:3],0),0,0,255) if p[3] else (0,0,0,0) for i,p in enumerate(im.get_flattened_data())]);return out

def build():
    assert not (DOC/'acceptance.json').exists(), 'Accepted worn layers are frozen'
    DEST.mkdir(parents=True,exist_ok=True);files=[];rigs=[]
    for outfit,n,batch in OUTFITS:
        code=f'FIN-{n:02}';base=Image.open(ROOT/f'assets/sprites/art-finish-{batch:02}/{code}.png').convert('RGBA')
        if n in (3,4,5):
            rifle=normalized(code+'-RIFLE',factor=1,art=ART)
            anchors={3:((61,57),(71,58)),4:((60,57),(69,58)),5:((61,57),(70,59))}[n]
            palms={3:((51,68),(77,68)),4:((49,65),(78,65)),5:((52,67),(77,67))}[n]
            left=[(48,47),(55,47),(56,52),(63,55),(63,60),(56,62),(48,60),(46,55)]
            right=[(73,47),(79,48),(82,55),(80,63),(74,65),(69,62),(69,59),(74,59),(75,54)]
        else:
            rifle=Image.open(ROOT/f'assets/sprites/art-finish-{batch:02}/{code}-rifle-body.png').convert('RGBA')
            review=json.loads((ROOT/f'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-{batch:02}/review.json').read_text('utf-8'))
            rec=next(e for e in review['entries'] if e['code']==code)
            cfg=rec['configurations'];anchors=cfg['rifle'].get('hand_anchors',cfg['rifle'].get('anchor'))
            palm=cfg['single'].get('hand_anchors',cfg['single'].get('anchor'))
            palms=((128-palm[0],palm[1]),palm)
            left=[(40,46),(55,46),(59,53),(65,57),(64,71),(43,74)]
            right=[(72,46),(88,46),(88,74),(66,74),(64,59),(70,53)]
        arms=[[(27,46),(53,46),(57,51),(55,57),(52,75),(27,75)],[(75,46),(101,46),(101,75),(76,75),(73,57),(71,51)]]
        _,mask=region(base,arms);core=base.copy();core.putalpha(ImageChops.multiply(base.getchannel('A'),ImageChops.invert(mask)))
        parts={'base':base,'core':core}
        for side,p in enumerate([left,right]):
            arm,_=region(rifle,[p]);parts[f'arm-{side}']=arm
            x,y=anchors[side];parts[f'fingers-{side}']=region(rifle,[[(x-2,y-2),(x+2,y-2),(x+2,y+2),(x-2,y+2)]])[0]
            x,y=palms[side];parts[f'palm-{side}']=region(base,[[(x-2,y-2),(x+2,y-2),(x+2,y+2),(x-2,y+2)]])[0]
        for part,im in parts.items():
            mask=Image.new('RGBA',im.size) if n==1 and part.startswith(('arm','fingers','palm')) else material_map(im,n)
            for suffix,layer in [('',im),('-mask',mask)]:
                path=DEST/f'{code}-{part}{suffix}.png';layer.save(path,optimize=True);files.append((f'RIG-{n:02}-{part}{suffix}',path))
        for area in [(0,0,128,44),(0,80,128,128)]:assert base.crop(area).tobytes()==core.crop(area).tobytes()
        rigs.append(dict(outfit=outfit,number=n,code=code,anchors=anchors,palms=palms,shoulders=[(52,48),(76,48)],core_fixed_regions=[[0,0,128,44],[0,80,128,128]]))
    weapons=json.loads((ROOT/'assets/sprites/flat-agents/manifest.json').read_text('utf-8'))['weapons']
    heavy={w['model']:w for w in json.loads((ROOT/'assets/sprites/held-native64/manifest.json').read_text('utf-8'))['entries']}
    flat_sheet=Image.open(ROOT/'assets/sprites/flat-agents/weapons-128.png').convert('RGBA')
    heavy_sheet=Image.open(ROOT/'assets/sprites/held-native64/weapons-128.png').convert('RGBA')
    bindings=[]
    for w in weapons:
        if w['model'] is None:continue
        if w['model'] in heavy and w['side'] is not None:continue
        tr=w['transform'];a=[v*2 for v in tr['grip_native']];b=[v*2 for v in tr['support_native']] if 'support_native' in tr else None
        if w['model'] in heavy:a,b=heavy[w['model']]['hands']
        if w['model']==2:a,b=[59.,64.],[73.,61.]
        if w['model']==101:a,b=([47.,61.] if w['side']==0 else [81.,61.]),None
        bindings.append(dict(model=w['model'],left=w['side']==0,anchor=a,support=b,opposed=heavy.get(w['model'],{}).get('opposed',False)))
        if w['model'] in (2,101):
            name='rifle' if w['model']==2 else 'single'
            im=Image.open(ROOT/f'assets/sprites/art-finish-01/FIN-01-{name}-weapon.png').convert('RGBA')
            if w['side']==0:im=im.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        else:
            spec=heavy.get(w['model'],w);sheet=heavy_sheet if w['model'] in heavy else flat_sheet;index=spec['layer'];xx=index%16*128;yy=index//16*128
            im=sheet.crop((xx,yy,xx+128,yy+128))
        name=f'weapon-{w["model"]}-{int(w["side"]==0)}-mask';path=DEST/f'{name}.png';material_map(im,0).save(path,optimize=True);files.append((f'RIG-00-{name}',path))
    write(DOC/'worn-rigs.json',dict(outfits=rigs,weapon_bindings=bindings,source_authored_new_arm_poses=3,fixed_core_preserved=True))
    code='// Generated source-authored finish rigs; no runtime authoring dependencies.\n'
    code+='fn bytes_worn_06(name: &str) -> Option<&\'static [u8]> { Some(match name {\n'+''.join(f'    "{key}" => include_bytes!("../{p.relative_to(ROOT).as_posix()}"),\n' for key,p in files)+'    _ => return None,\n}) }\n'
    points=lambda p:str([[float(v) for v in pair] for pair in p])
    code+='const FINISHED_RIGS: &[Rig] = &[\n'+''.join('    Rig {outfit:%d,number:%d,anchors:%s,palms:%s,shoulders:%s},\n'%(r['outfit'],r['number'],points(r['anchors']),points(r['palms']),points(r['shoulders'])) for r in rigs)+'];\n'
    code+='const FINISHED_WEAPONS: &[WeaponBinding] = &[\n'
    for w in bindings:
        code+='    WeaponBinding { model:%d,left:%s,anchor:%s,support:%s,opposed:%s },\n'%(w['model'],str(w['left']).lower(),str(w['anchor']),('Some('+str(w['support'])+')') if w['support'] else 'None',str(w['opposed']).lower())
    code+='];\n'
    (ROOT/'src/game_art_finish_worn_catalog_06.rs').write_text(code,encoding='utf-8')
    icons=[];illustrations={1:200,2:101,3:102,4:104,5:113,14:201,15:202,16:203,17:204,18:103,19:105,20:106,21:107,22:108,23:110,24:111,25:112,26:109,27:114}
    for outfit,n,batch in OUTFITS:
        source=Image.open(ROOT/f'assets/sprites/art-finish-{batch:02}/FIN-{n:02}.png').convert('RGBA');bbox=source.getbbox();im=source.crop(bbox);im.thumbnail((60,60),Image.Resampling.LANCZOS)
        result=Image.new('RGBA',(64,64));result.alpha_composite(im,((64-im.width)//2,62-im.height));ident=illustrations[n]
        mask=material_map(source,n).crop(bbox).resize(im.size,Image.Resampling.NEAREST);icon_mask=Image.new('RGBA',(64,64));icon_mask.paste(mask,((64-im.width)//2,62-im.height))
        for suffix,im in [('',result),('-mask',icon_mask)]:
            p=DEST/f'icon-{ident}{suffix}.png';im.save(p,optimize=True);icons.append((f'ICON-FIN-{ident}{suffix}',p))
    code='// Current finished outfit silhouettes used by inventory and equipment icons.\nfn bytes_outfit_icons_06(name:&str)->Option<&\'static[u8]> {Some(match name {\n'+''.join(f'    "{key}"=>include_bytes!("../{p.relative_to(ROOT).as_posix()}"),\n' for key,p in icons)+'    _=>return None,\n})}\n'
    code+='fn finished_outfit_icon(illustration:u16)->bool {matches!(illustration,'+'|'.join(str(v) for v in illustrations.values())+')}\n'
    (ROOT/'src/game_art_finish_icons_catalog_06.rs').write_text(code,encoding='utf-8')
    print('Rig components:',len(files),'outfits:',len(rigs),'held bindings:',len(bindings))

if __name__=='__main__':build()
