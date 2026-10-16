"""Review-only armor catalogue using imagegen paintings and the accepted rig.

Source conditioning extracts source-authored sleeves/gloves. The torso, head
and lower body come from one fixed reference. Weapons have no outfit parameter.
The editable material-index maps/palettes are separate from the weapon layers.
"""
from pathlib import Path
from functools import lru_cache
import json, math, hashlib, shutil, argparse
from PIL import Image, ImageChops, ImageDraw, ImageFont
import build_flat_rig_review_02 as rig

BASE=Path(__file__).resolve().parent; ROOT=BASE.parents[1]
OUT=BASE/'armor-flat-review-01'; MS=rig.MS; F=rig.F
CAT=json.loads((OUT/'catalogue.json').read_text('utf-8'))
ENTRIES={e['id']:e for e in CAT['entries']}
POSES=['independent','rifle','heavy','pole']

def save(im,p):p.parent.mkdir(parents=True,exist_ok=True);im.save(p)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

@lru_cache(maxsize=100)
def source(ident,pose):
    e=ENTRIES[ident]
    if e.get('canonical'):return rig.source(e['canonical'],pose)
    sheet=Image.open(OUT/e['source']).convert('RGBA');i=POSES.index(pose)
    w,h=sheet.size;box=(i%2*w//2,i//2*h//2,(i%2+1)*w//2,(i//2+1)*h//2)
    return sheet.crop(box).resize((1254,1254),Image.Resampling.LANCZOS)

@lru_cache(maxsize=100)
def joints(ident,pose):
    e=ENTRIES[ident]
    if e.get('canonical'):return rig.JOINTS[pose][e['canonical']]
    # Measure the glove's neutral grey interior near the template grip.
    # This only registers existing source anatomy; it is not a glove drawing.
    src=source(ident,pose).resize((314,314),Image.Resampling.LANCZOS);px=src.load();result=[]
    for shoulder,elbow,hand in rig.JOINTS[pose][1]:
        x,y=[v/4 for v in hand];samples=[]
        for py in range(max(0,round(y-22)),min(314,round(y+23))):
            for pxx in range(max(0,round(x-25)),min(314,round(x+26))):
                r,g,b,a=px[pxx,py];v=max(r,g,b)
                if a>210 and 16<v<105 and max(r,g,b)-min(r,g,b)<22:
                    weight=math.exp(-((pxx-x)**2+(py-y)**2)/140)
                    samples.append((pxx,py,weight))
        if samples:
            total=sum(p[2] for p in samples)
            measured=[4*sum(p[k]*p[2] for p in samples)/total for k in (0,1)]
        else:measured=list(hand)
        result.append([list(shoulder),list(elbow),measured])
    return result

def affine_points(points,src_pts,dst_pts):
    rows=[[x,y,1] for x,y in src_pts]
    coeff=rig.solve3(rows,[p[0] for p in dst_pts])+rig.solve3(rows,[p[1] for p in dst_pts])
    return [[coeff[0]*x+coeff[1]*y+coeff[2],coeff[3]*x+coeff[4]*y+coeff[5]] for x,y in points]

def arm_polys(ident,pose):
    return [affine_points(rig.POLYS[pose][s],rig.JOINTS[pose][1][s],joints(ident,pose)[s]) for s in (0,1)]

def registered(im,ident,pose,side):
    target=[rig.master(p) for p in rig.JOINTS[pose][1][side]];src=joints(ident,pose)[side]
    rows=[[x,y,1] for x,y in target]
    coeff=rig.solve3(rows,[p[0] for p in src])+rig.solve3(rows,[p[1] for p in src])
    return im.transform((MS,MS),Image.Transform.AFFINE,coeff,Image.Resampling.BICUBIC)

@lru_cache(maxsize=100)
def body(ident,pose):
    if ENTRIES[ident].get('canonical'):return rig.body(ENTRIES[ident]['canonical'],pose)
    ref=source(ident,'independent');src=source(ident,pose)
    oldmask=rig.mask(ref.size,arm_polys(ident,'independent'))
    core=ref.copy();core.putalpha(ImageChops.multiply(core.getchannel('A'),ImageChops.invert(oldmask)))
    newmask=rig.mask(src.size,arm_polys(ident,pose))
    fill=src.copy();fill.putalpha(ImageChops.multiply(src.getchannel('A'),ImageChops.multiply(oldmask,ImageChops.invert(newmask))))
    core.alpha_composite(fill);result=rig.normalize(core)
    for s in (0,1):result.alpha_composite(registered(rig.selected(src,[arm_polys(ident,pose)[s]]),ident,pose,s))
    return result

@lru_cache(maxsize=184)
def fingers(ident,pose,side):
    if ENTRIES[ident].get('canonical'):return rig.fingers(ENTRIES[ident]['canonical'],pose,side)
    x,y=joints(ident,pose)[side][2]
    if pose=='independent':
        lo,hi=(-50,40) if side==0 else (-40,50)
        poly=[[x+lo+8,y-52],[x+hi-15,y-56],[x+hi,y-34],[x+hi,y+33],[x+hi-12,y+54],[x+lo+8,y+52],[x+lo,y+25],[x+lo,y-32]]
    elif pose=='pole':
        poly=([[x-70,y-38],[x-40,y-52],[x+17,y-40],[x+48,y-12],[x+45,y+37],[x+14,y+66],[x-38,y+68],[x-75,y+31]] if side==0 else
              [[x-69,y-14],[x-54,y-51],[x-11,y-72],[x+26,y-55],[x+45,y-16],[x+43,y+30],[x+23,y+55],[x-29,y+57],[x-68,y+31]])
    else:poly=[[x-42,y-38],[x-15,y-53],[x+29,y-48],[x+49,y-23],[x+45,y+27],[x+19,y+49],[x-29,y+42],[x-46,y+10]]
    return registered(rig.selected(source(ident,pose),[poly]),ident,pose,side)

def outputs(im,key):
    small=im.resize((64,64),Image.Resampling.LANCZOS)
    save(small,OUT/'native32'/f'{key}.png')
    save(small.resize((128,128),Image.Resampling.NEAREST),OUT/'display64'/f'{key}.png')
    save(im.resize((512,512),Image.Resampling.LANCZOS),OUT/'normal'/f'{key}.png')
    save(im.crop((8*F,8*F,58*F,50*F)).resize((600,504),Image.Resampling.LANCZOS),OUT/'portraits'/f'{key}.png')

def conditioning():
    registrations={}
    for e in CAT['entries']:
        if e['kind']!='outfit' or not (e.get('canonical') or (OUT/e['source']).exists()):continue
        ident=e['id'];registrations[ident]={p:joints(ident,p) for p in POSES}
        for p in POSES:save(body(ident,p),OUT/'conditioned'/f'{ident}-{p}.png')
    (OUT/'source-registration.json').write_text(json.dumps(registrations,indent=2),'utf-8')
    board(registrations)

def board(ids):
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',19)
    ids=list(ids)
    for start in range(0,len(ids),6):
        group=ids[start:start+6];im=Image.new('RGB',(1500,360*len(group)),(14,21,29));d=ImageDraw.Draw(im)
        for row,ident in enumerate(group):
            y=row*360;d.text((10,y+4),ident+' · '+ENTRIES[ident]['name'],fill='white',font=font)
            for col,p in enumerate(POSES):
                b=body(ident,p);x=col*370
                crop=b.crop((15*F,14*F,49*F,49*F)).resize((295,303),Image.Resampling.LANCZOS)
                im.paste(crop,(x,y+36),crop)
                n=b.resize((64,64),Image.Resampling.LANCZOS)
                d.rectangle((x+296,y+220,x+361,y+285),fill='#aeb8bc');im.paste(n,(x+297,y+221),n)
        save(im,OUT/'audit'/f'conditioned-{start//6+1}.png')

# Per-source material authoring. These palettes describe the actual paint
# materials, rather than a global hue filter or a rectangle over a character.
# The resulting index -> material assignment is persisted and reused unchanged
# for every pose. Neutral leather, outlines, steel and skin remain source RGB.
RAMPS={
 'ivory':['#a49e88','#d8ceb3','#fff3d5'],
 'blue':['#032155','#064cd5','#3888ff'],
 'orange':['#783006','#d95a0a','#ff9527'],
 'violet':['#371267','#7725b5','#c55fe8'],
 'cyan':['#065654','#0ac7c9','#70f8ea'],
 'red':['#661515','#ce2827','#ff5950'],
 'olive':['#384414','#76942f','#bdd36d'],
 'yellow':['#755306','#d7a505','#ffe557'],
 'petrol':['#142f3e','#245a70','#4798a9'],
 'steelblue':['#233748','#4d6b86','#8fabc1'],
 'bone':['#82755b','#ccbda0','#f0e4c7'],
 'organic_violet':['#2c2338','#514261','#78668e'],
 'organic_cyan':['#264c4e','#417d80','#71b6b8'],
}
# Source material keys; new replacement colors for each of the two extra
# variants. No material assignment is inferred at composition time.
MATERIALS={
 'AR-01':('ivory','blue','olive','bone','steelblue','red'),
 'AR-02':('orange','blue','petrol','bone','olive','orange'),
 'AR-03':('violet','cyan','olive','bone','red','steelblue'),
 'AR-04':('blue','ivory','petrol','orange','red','bone'),
 'AR-05':('cyan',None,'orange',None,'violet',None),
 'AR-06':('orange','ivory','blue','bone','olive','ivory'),
 'AR-07':('ivory',None,'olive',None,'steelblue',None),
 'AR-08':('blue','ivory','red','bone','olive','ivory'),
 'AR-09':('red',None,'blue',None,'olive',None),
 'AR-10':('olive','blue','red','bone','petrol','orange'),
 'AR-11':('red','cyan','olive','bone','blue','orange'),
 'AR-12':('yellow','violet','petrol','bone','olive','orange'),
 'AR-13':('blue','ivory','red','bone','olive','ivory'),
 'AR-14':('red','ivory','blue','bone','olive','ivory'),
 'AR-15':('red','ivory','petrol','bone','olive','ivory'),
 'AR-16':('blue','ivory','red','bone','olive','ivory'),
 'AR-17':('olive','blue','petrol','bone','red','steelblue'),
 'AR-N01':('blue','ivory','red','bone','olive','ivory'),
 'AR-N02':('red',None,'blue',None,'olive',None),
 'AR-N03':('steelblue','orange','olive','bone','red','steelblue'),
 'AR-N04':('petrol','bone','olive','ivory','red','bone'),
 'AR-N05':('organic_violet','organic_cyan','olive','bone','red','petrol'),
 'AR-N06':('olive','bone','petrol','ivory','violet','bone'),
}

def rgb(s):return tuple(bytes.fromhex(s[1:]))
def distance(a,b):return sum((x-y)**2 for x,y in zip(a,b))

def author_materials(ident):
    e=ENTRIES[ident];file=OUT/'palettes'/f'{ident}.json'
    assert e['kind']=='outfit','Inventory pieces deliberately have one illustration'
    spec=MATERIALS[ident]
    if file.exists():
        previous=json.loads(file.read_text('utf-8'))
        if previous['source_materials']==list(spec[:2]):return previous
    im=Image.new('RGBA',(1254,1254))
    for j,p in enumerate(POSES):im.alpha_composite(source(ident,p).resize((627,627)),(j%2*627,j//2*627))
    im=im.resize((900,900),Image.Resampling.LANCZOS)
    q=im.convert('RGB').quantize(colors=64,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE)
    palette=q.getpalette()[:64*3];cols=[palette[i:i+3] for i in range(0,len(palette),3)]
    neutral=[(n,n,n) for n in (0,12,24,40,60,90,125,170,220,250)]
    # Protect flesh/leather brown and bone wherever it is not a declared cloth
    # material. Source color exemplars are specific to this asset's materials.
    protected=neutral+[(90,59,51),(147,103,85)]
    if spec[0] not in ('ivory','bone') and spec[1] not in ('ivory','bone'):protected += [rgb(s) for s in RAMPS['bone']]
    prototypes={'protected':protected,'principal':[rgb(s) for s in RAMPS[spec[0]]]}
    if spec[1]:prototypes['secondary']=[rgb(s) for s in RAMPS[spec[1]]]
    indices={key:[] for key in prototypes}
    for i,c in enumerate(cols):
        k=min(prototypes,key=lambda k:min(distance(c,t) for t in prototypes[k]))
        indices[k].append(i)
    data={'id':ident,'method':'explicit indexed material palette, source RGB protected outside declared materials','source_palette':cols,'material_indices':indices,'source_materials':list(spec[:2]),'variants':[list(spec[:2]),list(spec[2:4]),list(spec[4:6])],'variant_names':['Couleurs de référence','Variante 2','Variante 3']}
    file.write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
    save(q,OUT/'palettes'/f'{ident}-indexed-reference.png')
    return data

def material_variant(im,data,v):
    if v==0:return im.copy()
    pal=Image.new('P',(1,1));flat=[n for c in data['source_palette'] for n in c];pal.putpalette(flat+[0]*(768-len(flat)))
    indexed=im.convert('RGB').quantize(palette=pal,dither=Image.Dither.NONE)
    replacement=flat[:];mask_lut=[0]*256
    for material,slot in (('principal',0),('secondary',1)):
        dest=data['variants'][v][slot]
        if not dest:continue
        src_ramp=[rgb(c) for c in RAMPS[data['source_materials'][slot]]]
        dst_ramp=[rgb(c) for c in RAMPS[dest]]
        for i in data['material_indices'].get(material,[]):
            c=data['source_palette'][i]
            # Preserve the cel-shade position in its material's ramp.
            # Interpolate between source ramp stops instead of flattening
            # every antialiased edge to one of only three RGB values.
            best=None
            for shade in (0,1):
                lo,hi=src_ramp[shade:shade+2];axis=[hi[k]-lo[k] for k in range(3)]
                t=max(0,min(1,sum((c[k]-lo[k])*axis[k] for k in range(3))/max(1,sum(a*a for a in axis))))
                estimate=[lo[k]+axis[k]*t for k in range(3)];error=distance(c,estimate)
                if best is None or error<best[0]:best=(error,shade,t)
            _,shade,t=best;lo,hi=dst_ramp[shade:shade+2]
            replacement[3*i:3*i+3]=[round(lo[k]+(hi[k]-lo[k])*t) for k in range(3)];mask_lut[i]=255
    colored=indexed.copy();colored.putpalette(replacement+[0]*(768-len(replacement)))
    change=indexed.point(mask_lut,'L');result=Image.composite(colored.convert('RGB'),im.convert('RGB'),change).convert('RGBA')
    result.putalpha(im.getchannel('A'));return result

def build(only=None):
    configs=[{'number':n,'label':label,'pose':p,'items':items} for n,label,p,items in rig.CONFIGS]
    weapons={c['number']:[rig.item_layer(i,c['pose'],s) for i,s in c['items']] for c in configs}
    manifests=[];checked=0
    for e in CAT['entries']:
        ident=e['id'];data=author_materials(ident) if e['kind']=='outfit' else None;entry=dict(e);entry['views']={}
        if only and ident not in only:
            prior=json.loads((OUT/'review.json').read_text('utf-8'))
            manifests.append(next(p for p in prior['entries'] if p['id']==ident));continue
        if e['kind']=='outfit':
            for v in range(3):
                for c in configs:
                    pose=c['pose'];im=material_variant(body(ident,pose),data,v)
                    for layer,meta in weapons[c['number']]:im.alpha_composite(layer)
                    for side in (0,1):
                        shown=any(s==side or s is None for _,s in c['items'])
                        shield=any(s==side and rig.old.BY_ID[i]['kind']=='shield' for i,s in c['items'])
                        if shown and not shield:
                            glove=material_variant(fingers(ident,pose,side),data,v);im.alpha_composite(glove)
                            layers=[l for (i,s),(l,m) in zip(c['items'],weapons[c['number']]) if s==side or s is None]
                            assert any(ImageChops.multiply(glove.getchannel('A'),l.getchannel('A')).getbbox() for l in layers),(ident,pose,side)
                            checked+=1
                    key=f'{ident}-V{v}-{c["number"]}';outputs(im,key)
                    entry['views'][f'{v}-{c["number"]}']={'native32':f'native32/{key}.png','display64':f'display64/{key}.png','normal':f'normal/{key}.png','portrait':f'portraits/{key}.png'}
            if e['source_reference']:
                prev=Image.open(ROOT/e['source_reference']).convert('RGBA');prev=prev.crop(prev.getchannel('A').getbbox());prev.thumbnail((400,400),Image.Resampling.LANCZOS)
                save(prev,OUT/'previous'/f'{ident}.png');entry['previous']=f'previous/{ident}.png'
        else:
            src=Image.open(ROOT/e['source']).convert('RGBA');src=src.crop(src.getchannel('A').getbbox());src.thumbnail((700,700),Image.Resampling.LANCZOS)
            entry['variants']=['Illustration d’inventaire']
            for v in (0,):
                im=src;can=Image.new('RGBA',(900,900));can.alpha_composite(im,((900-im.width)//2,(900-im.height)//2))
                key=f'{ident}-V{v}';small=can.resize((32,32),Image.Resampling.LANCZOS);save(small,OUT/'native32'/f'{key}.png');save(small.resize((64,64),Image.Resampling.NEAREST),OUT/'display64'/f'{key}.png');save(can.resize((400,400),Image.Resampling.LANCZOS),OUT/'portraits'/f'{key}.png')
                entry['views'][str(v)]={'native32':f'native32/{key}.png','display64':f'display64/{key}.png','portrait':f'portraits/{key}.png'}
        manifests.append(entry);print(ident+' ready',flush=True)
    if only and not any(ENTRIES[i]['kind']=='outfit' for i in only):checked=json.loads((OUT/'review.json').read_text('utf-8'))['checked_grip_overlaps']
    result={**CAT,'entries':manifests,'configurations':configs,'weapon_transforms':{n:[m for _,m in ls] for n,ls in weapons.items()},'common_rig':{p:[[rig.native(q) for q in s] for s in j[1]] for p,j in rig.JOINTS.items()},'checked_grip_overlaps':checked}
    (OUT/'review.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),'utf-8')
    (OUT/'review-data.js').write_text('window.armorReview='+json.dumps(result,ensure_ascii=False)+';','utf-8')
    shutil.copy2(BASE/'comic64-reference-review-04/images/floor32.png',OUT/'floor32.png')
    validate(result)

def validate(data):
    count=0;clipped=[]
    for e in data['entries']:
        for k,v in e['views'].items():
            n=Image.open(OUT/v['native32']).convert('RGBA');b=Image.open(OUT/v['display64']).convert('RGBA')
            assert b.tobytes()==n.resize((2*n.width,2*n.height),Image.Resampling.NEAREST).tobytes()
            box=n.getchannel('A').getbbox();assert box
            if box[0]==0 or box[1]==0 or box[2]==n.width or box[3]==n.height:clipped.append((e['id'],k))
            count+=1
        if e['kind']=='outfit':
            for pose in POSES:
                base=body(e['id'],pose)
                for variant in (1,2):assert material_variant(base,author_materials(e['id']),variant).getchannel('A').tobytes()==base.getchannel('A').tobytes()
            for pose in POSES[1:]:
                assert body(e['id'],pose).crop((0,0,MS,24*F)).tobytes()==body(e['id'],'independent').crop((0,0,MS,24*F)).tobytes(),(e['id'],pose,'head')
                assert body(e['id'],pose).crop((0,38*F,MS,MS)).tobytes()==body(e['id'],'independent').crop((0,38*F,MS,MS)).tobytes(),(e['id'],pose,'lower body')
    assert not clipped,clipped
    assert all(sha(ROOT/p)==h for p,h in data['protected_active_files'].items())
    for ident,c in [('AR-07',1),('AR-09',4)]:
        for cfg in data['configurations']:
            a=OUT/'native32'/f'{ident}-V0-{cfg["number"]}.png';b=rig.OUT/'native32'/f'BD-FLAT01-{cfg["number"]}-C{c:02}.png'
            assert a.read_bytes()==b.read_bytes(),(ident,cfg['number'],'approved image preservation')
    report={'review_id':'ARMOR-FLAT01','outfits':23,'existing_outfits':17,'new_proposals':6,'inventory_pieces':18,'variants_per_outfit':3,'variants_per_inventory_piece':1,'configurations_per_outfit':16,'exact_nearest_doubling':count,'clipped_views':clipped,'common_weapon_transforms_for_all_outfits':True,'fixed_heads_and_lower_bodies_across_poses':True,'palette_alpha_preserved':True,'unchanged_approved_outfit_compositions':32,'checked_grip_overlaps':data['checked_grip_overlaps'],'active_files_unchanged':len(data['protected_active_files']),'no_deletions':True,'visual_acceptance_pending':True}
    (OUT/'validation.json').write_text(json.dumps(report,indent=2),'utf-8');print(json.dumps(report),flush=True)

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--build',action='store_true');parser.add_argument('--only');args=parser.parse_args()
    build(args.only.split(',') if args.only else None) if args.build else conditioning()
