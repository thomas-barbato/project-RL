"""Compose source-authored BD layers for review; never paint or touch game assets.

All shape/palette information comes from the generated source drawings. Masks
select those pixels only. All transforms are uniform similarities; the old pixel
rig, its hands, material stencils and palettes are deliberately not imported.
"""
from pathlib import Path
from functools import lru_cache
from collections import Counter
import argparse, json, math, hashlib, shutil
from PIL import Image, ImageChops, ImageDraw, ImageFont

BASE=Path(__file__).resolve().parent; ROOT=BASE.parents[1]
OLD=BASE/'comic-catalogue-review-01'; OUT=BASE/'comic-catalogue-review-03'
SIDE=64; F=16; MS=SIDE*F
NAMES=['Bastion','Technomancien','Traqueur','Arcaniste','Métamorphe']
CAT=json.loads((OLD/'catalogue.json').read_text('utf-8'))
ENTRIES=CAT['entries']; BY_ID={e['review_id']:e for e in ENTRIES}

# Hand centres and source-authored selection polygons measured on the new BD
# drawings. Only the front strip of fingers can be composited over a weapon.
# Source coordinates are retained in the manifest to allow independent review.
R={
 1:dict(cx=640,top=204,bottom=1202,main='R',
   hands={'L':[424,728],'R':[946,624]}, elbows={'L':[405,604],'R':[854,597]},
   fingers={'L':[[400,682],[444,697],[448,741],[427,770],[398,753]],'R':[[913,577],[956,590],[960,643],[945,666],[914,655]]},
   old_arms=[[[319,500],[510,500],[504,599],[486,688],[472,786],[312,790]],[[790,483],[998,480],[1020,710],[791,710],[797,622]]],
   gun_hands=[[671,618],[811,561]],
   gun_arms=[[[429,500],[554,500],[566,545],[654,558],[705,572],[706,671],[576,679],[449,641]],[[756,490],[856,490],[893,545],[888,623],[818,646],[749,610]]]),
 2:dict(cx=650,top=87,bottom=1223,main='L',mirror=True,
   hands={'L':[334,704],'R':[935,754]}, elbows={'L':[388,561],'R':[855,579]},
   fingers={'L':[[300,662],[341,675],[353,715],[330,749],[300,731]],'R':[[919,710],[953,731],[950,776],[928,790],[912,759]]},
   old_arms=[[[260,451],[436,468],[460,564],[418,636],[396,787],[254,799]],[[815,451],[972,445],[1020,795],[861,798],[824,606]]],
   gun_hands=[[697,578],[809,517]],
   gun_arms=[[[442,442],[558,443],[570,490],[638,504],[735,520],[739,635],[599,635],[467,612]],[[748,439],[849,440],[903,543],[898,611],[789,617],[751,566]]]),
 3:dict(cx=590,top=100,bottom=1205,main='L',
   hands={'L':[512,596],'R':[860,453]}, elbows={'L':[357,548],'R':[832,534]},
   fingers={'L':[[493,553],[534,559],[548,593],[525,635],[496,625]],'R':[[842,411],[878,411],[902,446],[884,478],[850,484]]},
   old_arms=[[[265,442],[420,450],[439,513],[527,523],[576,554],[574,665],[425,681],[264,600]],[[784,397],[935,395],[940,608],[812,634],[776,561]]],
   gun_hands=[[602,572],[793,494]],
   gun_arms=[[[327,433],[471,435],[484,495],[582,512],[648,535],[646,633],[469,631],[332,602]],[[711,416],[813,418],[855,522],[842,585],[752,572],[699,514]]]),
 4:dict(cx=625,top=137,bottom=1252,main='R',
   hands={'L':[621,681],'R':[839,489]}, elbows={'L':[452,580],'R':[813,618]},
   fingers={'L':[[593,645],[635,650],[670,684],[640,719],[610,714]],'R':[[816,450],[850,457],[880,484],[860,521],[821,529]]},
   old_arms=[[[389,431],[534,431],[563,532],[640,576],[718,640],[718,770],[535,806],[390,661]],[[743,437],[903,437],[917,677],[750,675],[717,558]]],
   gun_hands=[[683,560],[796,525]],
   gun_arms=[[[431,438],[540,440],[560,485],[630,504],[712,526],[714,608],[620,644],[485,592]],[[731,440],[782,441],[833,496],[837,577],[790,601],[723,557]]]),
 5:dict(cx=650,top=85,bottom=1196,main='L',
   hands={'L':[372,556],'R':[900,704]}, elbows={'L':[485,550],'R':[842,581]},
   fingers={'L':[[331,516],[368,511],[389,538],[390,584],[361,601],[333,578]],'R':[[869,679],[917,669],[945,701],[932,736],[900,745],[868,723]]},
   old_arms=[[[306,488],[571,444],[573,520],[529,596],[472,671],[306,671]],[[772,445],[927,433],[1005,776],[808,801],[789,605]]],
   gun_hands=[[669,556],[782,505]],
   gun_arms=[[[464,444],[562,440],[570,482],[657,500],[712,515],[715,603],[592,615],[486,582]],[[752,451],[855,442],[866,556],[812,575],[743,552]]]),
}
POLE={
 1:([[668,688],[763,514]],[[[429,511],[548,516],[573,568],[640,613],[695,633],[708,734],[595,740],[450,683]],[[732,461],[815,465],[895,577],[895,648],[806,660],[736,550]]]),
 2:([[641,635],[775,477]],[[[437,443],[556,443],[570,509],[638,552],[692,599],[693,680],[598,688],[466,627]],[[733,422],[850,423],[919,532],[918,580],[802,590],[728,517]]]),
 3:([[606,623],[725,454]],[[[350,436],[471,437],[484,510],[565,549],[639,574],[647,670],[540,680],[398,627]],[[689,392],[794,401],[850,513],[844,568],[765,581],[695,502]]]),
 4:([[677,656],[746,488]],[[[430,438],[538,445],[550,514],[645,580],[714,620],[721,715],[621,750],[474,647]],[[707,434],[774,437],[862,534],[853,648],[796,655],[727,548]]]),
 5:([[646,640],[787,455]],[[[451,440],[555,440],[565,511],[637,564],[689,606],[699,682],[612,699],[481,625]],[[733,389],[833,389],[909,537],[904,586],[814,603],[749,512]]]),
}
for i,r in R.items():
    r['firearm_hands']=r['gun_hands'];r['firearm_arms']=r['gun_arms']
    r['pole_hands'],r['pole_arms']=POLE[i]
HEAVY={
 1:([[606,686],[875,692]],[[[428,510],[552,510],[570,569],[637,610],[657,642],[657,739],[511,747],[430,669]],[[790,499],[871,500],[944,618],[955,747],[803,755],[784,652]]]),
 2:([[630,654],[934,655]],[[[426,444],[553,443],[566,516],[638,572],[682,617],[684,707],[564,721],[432,638]],[[809,437],[886,438],[972,563],[1004,716],[874,739],[809,619]]]),
 3:([[592,651],[858,643]],[[[333,434],[474,435],[489,511],[546,565],[643,602],[652,704],[521,726],[337,638]],[[763,421],[844,416],[897,547],[924,724],[813,737],[762,612]]]),
 4:([[635,647],[876,635]],[[[429,438],[539,439],[559,510],[628,562],[684,605],[698,685],[640,717],[463,713]],[[728,434],[778,437],[879,535],[928,589],[925,713],[814,741],[745,636]]]),
 5:([[597,658],[850,647]],[[[438,448],[557,446],[571,532],[642,588],[675,651],[652,708],[549,719],[444,645]],[[752,443],[856,443],[891,552],[934,701],[807,741],[745,606]]]),
}
for i,r in R.items():r['heavy_hands'],r['heavy_arms']=HEAVY[i]
SIDEARM={
 1:([[1030,619]], [856,619], [[[790,483],[880,480],[921,534],[1100,538],[1110,693],[913,709],[792,644]]]),
 4:([[976,548]], [835,548], [[[743,437],[904,437],[1040,486],[1050,642],[891,699],[778,617],[717,558]]]),
}
for i,(hands,elbow,arms) in SIDEARM.items():
    R[i]['sidearm_hands']=hands;R[i]['sidearm_elbow']=elbow;R[i]['sidearm_arms']=arms
HEAVY_JOINTS={1:([819,520],[833,620]),2:([865,460],[904,583]),3:([804,460],[807,575]),4:([769,475],[792,566]),5:([807,455],[812,562])}
# Front support points on the actual receiver, foregrip or barrel of each model.
HEAVY_SUPPORT={
 14:(.43,.59),16:(.58,.38),17:(.59,.48),18:(.59,.47),19:(.59,.48),20:(.61,.43),21:(.61,.39),
 22:(.72,.65),23:(.62,.62),24:(.70,.66),25:(.62,.60),26:(.67,.63),27:(.70,.61),28:(.63,.60),
 29:(.55,.41),30:(.61,.56),31:(.62,.45),32:(.62,.45),33:(.59,.46),34:(.63,.43),35:(.62,.40),
 48:(.65,.61),49:(.52,.53),50:(.61,.55),51:(.62,.63),52:(.60,.57),53:(.65,.59),54:(.58,.64),
}
EXTRA_SUPPORT={'heavy_machinegun':[(.58,.51),(.58,.47),(.46,.58),(.55,.53),(.55,.53),(.55,.47)],'plasma':[(.55,.57),(.55,.60),(.55,.58),(.61,.56),(.57,.56),(.58,.55)]}

# Individually inspected grips, as fractions of each alpha-cropped source.
# Firearm handles differ by model. Their envelope is independent of the
# separation of the two hands, so swapping models never enlarges the body/gun.
GUN_GRIPS={
 1:(.28,.67),2:(.27,.69),3:(.28,.73),4:(.33,.68),5:(.29,.67),6:(.29,.70),7:(.29,.66),
 8:(.23,.71),9:(.30,.74),10:(.24,.76),11:(.31,.79),12:(.29,.73),13:(.35,.76),
 14:(.26,.72),15:(.25,.76),16:(.25,.56),17:(.22,.61),18:(.32,.69),19:(.28,.72),20:(.30,.69),21:(.30,.70),
 22:(.24,.70),23:(.30,.74),24:(.25,.75),25:(.22,.71),26:(.18,.75),27:(.24,.69),28:(.25,.73),
 29:(.26,.76),30:(.24,.66),31:(.26,.72),32:(.21,.44),33:(.26,.65),34:(.26,.73),35:(.31,.65),
 36:(.25,.72),37:(.24,.72),38:(.24,.73),39:(.23,.74),40:(.26,.76),41:(.26,.75),42:(.29,.70),43:(.30,.74),44:(.32,.72),45:(.30,.73),46:(.27,.75),47:(.26,.75),
 48:(.40,.80),49:(.26,.78),50:(.28,.78),51:(.30,.76),52:(.29,.71),53:(.30,.72),54:(.29,.76),
 55:(.26,.72),56:(.24,.75),57:(.25,.71),58:(.21,.74),59:(.25,.71),60:(.28,.68),61:(.30,.75),
 102:(.19,.67),103:(.17,.68),104:(.17,.71),105:(.17,.71),106:(.15,.69),107:(.17,.74),
 108:(.13,.67),109:(.24,.74),110:(.16,.65),111:(.19,.70),112:(.22,.69),113:(.24,.70),
}
MELEE_GRIPS={
 62:(.32,.85),63:(.34,.87),64:(.25,.86),65:(.26,.86),66:(.33,.87),67:(.48,.90),68:(.29,.88),
 69:(.43,.87),70:(.35,.84),71:(.22,.85),72:(.33,.85),73:(.35,.89),74:(.46,.86),75:(.30,.88),76:(.40,.82),77:(.43,.84),78:(.50,.83),79:(.50,.82),80:(.43,.82),81:(.49,.84),
 82:(.24,.79),83:(.18,.84),84:(.20,.84),85:(.17,.85),86:(.16,.84),87:(.20,.83),88:(.16,.84),
 89:(.47,.84),90:(.38,.84),91:(.32,.85),92:(.48,.85),93:(.48,.81),94:(.44,.83),
 95:(.18,.85),96:(.17,.85),97:(.18,.85),98:(.14,.87),99:(.19,.85),100:(.17,.85),101:(.18,.85),
}
FAMILY={0:'Couteaux',1:'Lames',2:'Épées à deux mains',3:'Haches',4:'Masses',5:'Armes d’hast',6:'Pistolets',7:'Pistolets-mitrailleurs',8:'Fusils d’assaut',9:'Fusils à pompe',10:'Mitrailleuses lourdes',11:'Fusils à énergie',12:'Lance-flammes',13:'Lance-grenades',14:'Lance-roquettes',15:'Lance-aiguilles',16:'Marteaux',17:'Fusils de précision','focalizer':'Focaliseurs occultes','shaft':'Hampes de canalisation','shield':'Boucliers','heavy_machinegun':'Mitrailleuses lourdes','plasma':'Lance-plasma'}

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(im,p):p.parent.mkdir(parents=True,exist_ok=True);im.save(p)
def bounds(im):return im.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
@lru_cache(maxsize=200)
def raw(ident):return Image.open(OLD/'sources'/f'{ident}.png').convert('RGBA')
def select(im,polys):
    m=Image.new('L',im.size);d=ImageDraw.Draw(m)
    for p in polys:d.polygon(p,fill=255)
    a=im.copy();a.putalpha(ImageChops.multiply(im.getchannel('A'),m));return a,m
def affine(im,point,target,scale,angle=0):
    a=math.cos(angle)/scale;b=math.sin(angle)/scale
    d=-b;f=a
    return im.transform((MS,MS),Image.Transform.AFFINE,(a,b,point[0]-a*target[0]-b*target[1],d,f,point[1]-d*target[0]-f*target[1]),resample=Image.Resampling.BICUBIC)
def coordinate(i,pt):
    r=R[i];s=29/(r['bottom']-r['top']);x,y=pt
    x=32+(x-r['cx'])*s;y=47+(y-r['bottom'])*s
    if r.get('mirror'):x=64-x
    return [x*F,y*F]
def normalize(i,im):
    r=R[i];s=29*F/(r['bottom']-r['top'])
    o=affine(im,[r['cx'],r['bottom']],[32*F,47*F],s)
    return o.transpose(Image.Transpose.FLIP_LEFT_RIGHT) if r.get('mirror') else o

@lru_cache(maxsize=24)
def body(i,pose='side'):
    src=Image.open(OUT/'sources'/f'BD-C{i:02}-unarmed.png').convert('RGBA')
    if pose=='side':return normalize(i,src)
    r=R[i];p=Image.open(OUT/'sources'/f'BD-C{i:02}-{pose}.png').convert('RGBA')
    # Keep every visible fixed body pixel; replace source-authored old arms and
    # place only source-authored new arms. A new arm may naturally occlude it.
    old_regions=[r['old_arms'][1]] if pose=='sidearm' else r['old_arms']
    old,mask=select(src,old_regions); fixed=src.copy()
    fixed.putalpha(ImageChops.multiply(src.getchannel('A'),ImageChops.invert(mask)))
    fill=p.copy();fill.putalpha(ImageChops.multiply(p.getchannel('A'),mask))
    fixed.alpha_composite(fill)
    arm,_=select(p,r[f'{pose}_arms']);fixed.alpha_composite(arm)
    return normalize(i,fixed)
def glove_strip(i,pose,side=None):
    if pose=='side':
        src=Image.open(OUT/'sources'/f'BD-C{i:02}-unarmed.png').convert('RGBA')
        return normalize(i,select(src,[R[i]['fingers'][side]])[0])
    src=Image.open(OUT/'sources'/f'BD-C{i:02}-{pose}.png').convert('RGBA')
    layers=[]
    for x,y in R[i][f'{pose}_hands']:
        # Front fingers only, not the sleeve, full palm or a black rectangle.
        layers.append([[x-8,y-42],[x+20,y-37],[x+24,y+28],[x+10,y+41],[x-10,y+34]])
    return normalize(i,select(src,layers)[0])

def rigid_support_arm(i,base,target):
    """Two-bone IK rotates existing BD arm pixels; neither bone is scaled."""
    r=R[i];src=Image.open(OUT/'sources'/f'BD-C{i:02}-heavy.png').convert('RGBA')
    arm,arm_mask=select(src,[r['heavy_arms'][1]])
    shoulder,elbow=HEAVY_JOINTS[i];hand=r['heavy_hands'][1]
    # Split existing arm pixels at its measured elbow, with a small overlap.
    axis=[hand[j]-shoulder[j] for j in (0,1)];length=math.hypot(*axis);axis=[v/length for v in axis]
    lo=Image.new('L',src.size);hi=Image.new('L',src.size);lp=lo.load();hp=hi.load();am=arm_mask.load()
    for y in range(max(0,int(min(p[1] for p in r['heavy_arms'][1]))),min(src.height,int(max(p[1] for p in r['heavy_arms'][1]))+1)):
        for x in range(max(0,int(min(p[0] for p in r['heavy_arms'][1]))),min(src.width,int(max(p[0] for p in r['heavy_arms'][1]))+1)):
            if not am[x,y]:continue
            projection=(x-elbow[0])*axis[0]+(y-elbow[1])*axis[1]
            if projection<=12:lp[x,y]=255
            if projection>=-12:hp[x,y]=255
    upper=arm.copy();upper.putalpha(ImageChops.multiply(arm.getchannel('A'),lo))
    lower=arm.copy();lower.putalpha(ImageChops.multiply(arm.getchannel('A'),hi))
    a,b,c=[coordinate(i,q) for q in (shoulder,elbow,hand)]
    l1=math.dist(a,b);l2=math.dist(b,c);dist=math.dist(a,target)
    if dist>l1+l2-1:
        # Keep a real reach when a source grip is beyond the arm's full span.
        ratio=(l1+l2-1)/dist;target=[a[j]+(target[j]-a[j])*ratio for j in (0,1)]
    dist=max(abs(l1-l2)+1,math.dist(a,target));vx=(target[0]-a[0])/dist;vy=(target[1]-a[1])/dist
    along=(l1*l1-l2*l2+dist*dist)/(2*dist);height=math.sqrt(max(0,l1*l1-along*along))
    choices=[[a[0]+vx*along-vy*height,a[1]+vy*along+vx*height],[a[0]+vx*along+vy*height,a[1]+vy*along-vx*height]]
    outward=-1 if r.get('mirror') else 1
    joint=max(choices,key=lambda q:q[0]*outward)
    upangle=math.atan2(joint[1]-a[1],joint[0]-a[0])-math.atan2(b[1]-a[1],b[0]-a[0])
    lowangle=math.atan2(target[1]-joint[1],target[0]-joint[0])-math.atan2(c[1]-b[1],c[0]-b[0])
    old_arm=normalize(i,arm);result=base.copy()
    result.putalpha(ImageChops.multiply(result.getchannel('A'),ImageChops.invert(old_arm.getchannel('A'))))
    original=Image.open(OUT/'sources'/f'BD-C{i:02}-unarmed.png').convert('RGBA');_,om=select(original,r['old_arms'])
    original.putalpha(ImageChops.multiply(original.getchannel('A'),ImageChops.invert(om)))
    fixed=normalize(i,original);fixed.alpha_composite(result);result=fixed
    result.alpha_composite(affine(normalize(i,upper),a,a,1,upangle))
    result.alpha_composite(affine(normalize(i,lower),b,joint,1,lowangle))
    x,y=hand;poly=[[x-8,y-42],[x+20,y-37],[x+24,y+28],[x+10,y+41],[x-10,y+34]]
    front=normalize(i,select(src,[poly])[0]);front=affine(front,b,joint,1,lowangle)
    x,y=r['heavy_hands'][0];poly=[[x-8,y-42],[x+20,y-37],[x+24,y+28],[x+10,y+41],[x-10,y+34]]
    fingers=normalize(i,select(src,[poly])[0]);fingers.alpha_composite(front)
    return result,fingers,[v/F for v in target]

def gear_spec(e):
    im=raw(e['review_id']);bb=bounds(im);obj=im.crop(bb);w,h=obj.size
    fam=e['family'];n=int(e['review_id'][4:]) if e['review_id'].startswith('BD-W') else None
    if e['kind']=='shield':return obj,[w*.5,h*.5],17/h,'side',0
    if fam in (6,7):
        u,v=GUN_GRIPS[n];return obj,[u*w,v*h],(7.5 if fam==6 else 11.5)/w,'side',0
    if n in GUN_GRIPS or fam in ('heavy_machinegun','plasma'):
        if n in GUN_GRIPS:u,v=GUN_GRIPS[n]
        else:
            number=int(e['review_id'][-2:]);grips={'heavy_machinegun':[(.19,.62),(.23,.67),(.18,.46),(.13,.62),(.17,.66),(.17,.62)],'plasma':[(.18,.68),(.16,.72),(.17,.71),(.23,.77),(.22,.70),(.22,.72)]}
            u,v=grips[fam][number-1]
        heavy=fam in (10,12,13,14,'heavy_machinegun','plasma')
        extent=32 if heavy else 24 if fam==17 else 21
        scale=min(extent/w,11/h) if heavy else extent/w
        return obj,[u*w,v*h],scale,'heavy' if heavy else 'firearm',0 if heavy else -math.radians(9)
    if fam=='focalizer':return obj,[w*.5,h*.83],10/h,'side',0
    if fam=='shaft':return obj,[w*.5,h*.77],32/h,'pole',0
    u,v=MELEE_GRIPS[n]
    if fam==0:extent=9.5;pose='side'
    elif fam==1:extent=21;pose='side'
    elif fam in (2,3):extent=22;pose='pole'
    elif fam==5:
        # The source shaft is diagonal: its length is the diagonal, not its
        # bounding-box height. Using the latter clipped the tips in the canvas.
        return obj,[w*u,h*v],28/math.hypot(w,h),'pole',0
    else:extent=17;pose='side'
    # The diagonal source handles have a stable grip-to-tip direction.
    return obj,[w*u,h*v],extent/max(w,h),pose,0

def compose_weapon(e,i):
    obj,grip,s,pose,angle=gear_spec(e);r=R[i];fam=e['family']
    if fam in (6,7) and i in SIDEARM:pose='sidearm'
    if pose in ('side','sidearm'):
        side=('R' if r['main']=='L' else 'L') if fam in ('shield','focalizer') else r['main']
        target=coordinate(i,r['sidearm_hands'][0] if pose=='sidearm' else r['hands'][side])
        fore=coordinate(i,r['sidearm_elbow'] if pose=='sidearm' else r['elbows'][side])
        if fam in (4,16) and target[0]<32*F:
            # Diagonal clubs and hammers point outward from a left-hand grip,
            # rather than across the wearer's helmet. Their top stays up.
            obj=obj.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            grip=[obj.width-grip[0],grip[1]]
        if fam in (6,7):
            angle=math.atan2(target[1]-fore[1],target[0]-fore[0])
            if target[0]<fore[0]:
                obj=obj.transpose(Image.Transpose.FLIP_LEFT_RIGHT);grip=[obj.width-grip[0],grip[1]];angle-=math.pi
        if fam=='focalizer':angle=math.atan2(target[1]-fore[1],target[0]-fore[0])+math.pi/2
        fingers=glove_strip(i,pose,side)
    else:
        target=coordinate(i,r[f'{pose}_hands'][0]);fingers=glove_strip(i,pose)
        if pose=='pole':
            q=coordinate(i,r['pole_hands'][1]);a=math.atan2(q[1]-target[1],q[0]-target[0])
            # Source swords/hampes run upward; axes of diagonal polearms
            # and hammers run from their measured grip towards top-right.
            source_angle=-math.pi/4 if fam==5 else -math.pi/2
            angle=a-source_angle
        elif r.get('mirror'):
            # Facing left is a horizontal reflection, never a 180-degree
            # rotation: receiver/sights stay up and the magazine stays down.
            obj=obj.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            grip=[obj.width-grip[0],grip[1]];angle=-angle
    base=body(i,pose);support=None
    if pose=='heavy':
        n=int(e['review_id'][4:]) if e['review_id'].startswith('BD-W') else None
        uv=HEAVY_SUPPORT[n] if n else EXTRA_SUPPORT[fam][int(e['review_id'][-2:])-1]
        point=[obj.width*uv[0],obj.height*uv[1]]
        if r.get('mirror'):point[0]=obj.width-point[0]
        dx,dy=[point[j]-grip[j] for j in (0,1)]
        co,si=math.cos(angle),math.sin(angle)
        projected=[target[0]+s*F*(co*dx-si*dy),target[1]+s*F*(si*dx+co*dy)]
        base,fingers,support=rigid_support_arm(i,base,projected)
    weapon=affine(obj,grip,target,s*F,angle)
    composed=base.copy();composed.alpha_composite(weapon)
    if e['kind']!='shield':composed.alpha_composite(fingers)
    return composed,dict(pose=pose,source_grip=grip,source_object_bbox=bounds(raw(e['review_id'])),scale_native=s,angle_degrees=math.degrees(angle),target_grip_native=[v/F for v in target],support_grip_native=support,arm_bone_scale=1,layer_order=['body','BD arms','weapon','front fingers'] if e['kind']!='shield' else ['body','shield'],uniform_scale=True)

def armor_on_body(e,i):
    # Torso alone defines the COMPLETE appearance, including helmet, gloves
    # and boots. These items keep their individual inventory art and rules.
    return body(i).copy(),dict(pose='side',slot=e['kind'],changes_character_appearance=False,appearance_source='torso_complete_outfit',uniform_scale=True)

def standalone_outfit(e):
    # A complete torso equipment drawing defines its fixed full outfit. It is
    # sampled once with no old-body stencil, and never stretched to a tile box.
    canonical={'BD-A007':1,'BD-B003':2,'BD-B002':3,'BD-A009':4,'BD-A003':5}
    if e['review_id'] in canonical:return body(canonical[e['review_id']]).copy()
    ob=raw(e['review_id']).crop(bounds(raw(e['review_id'])));s=29*F/ob.height
    return affine(ob,[ob.width/2,ob.height],[32*F,47*F],s)
def outputs(master,name):
    native=master.resize((64,64),Image.Resampling.LANCZOS)
    display=native.resize((128,128),Image.Resampling.NEAREST)
    normal=master.resize((512,512),Image.Resampling.LANCZOS)
    save(native,OUT/'native32'/f'{name}.png');save(display,OUT/'display64'/f'{name}.png');save(normal,OUT/'normal'/f'{name}.png')
    return native,normal
def font(n):return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',n)
def contact(entries):
    for offset in range(0,len(entries),30):
        group=entries[offset:offset+30];w,h=240,208
        board=Image.new('RGB',(1200,((len(group)+4)//5)*h),(14,21,29));d=ImageDraw.Draw(board)
        for n,e in enumerate(group):
            x=n%5*w;y=n//5*h;d.text((x+7,y+3),e['review_id'],font=font(13),fill='white');d.text((x+7,y+22),e['name'][:29],font=font(13),fill=(155,181,198))
            p=e['compositions'][0]['display64'];im=Image.open(OUT/p).convert('RGBA');board.paste(im,(x+54,y+58),im)
        save(board,OUT/'boards'/f'catalogue-{offset//30+1:02}.png')
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--pilot',action='store_true');args=ap.parse_args()
    active=json.loads((ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json').read_text('utf-8'))
    before={p['path']:sha(ROOT/p['path']) for p in active['files']}
    records=[];ids=['BD-W069','BD-W102','BD-W001','BD-W036','BD-EQ-GA03','BD-W048','BD-W089','BD-EQ-H01','BD-EQ-S02','BD-A018','BD-A022','BD-A032']
    chosen=[BY_ID[k] for k in ids] if args.pilot else ENTRIES
    for n,e in enumerate(chosen):
        item={k:v for k,v in e.items() if k not in ('prompt',)};item['group']=FAMILY.get(e.get('family'),{'outfit':'Tenues complètes','head':'Casques et capuches','hands':'Gants et gantelets','feet':'Bottes'}.get(e['kind'],e['kind']));item['compositions']=[]
        for i in ([1] if e['kind']=='outfit' else range(1,6)):
            master,meta=(standalone_outfit(e),dict(pose='unarmed',uniform_scale=True)) if e['kind']=='outfit' else compose_weapon(e,i) if e['kind'] in ('weapon','shield') else armor_on_body(e,i)
            key=e['review_id']+(f'-C{i:02}' if e['kind']!='outfit' else '')
            native,normal=outputs(master,key)
            item['compositions'].append(dict(character=i,name=NAMES[i-1],native32=f'native32/{key}.png',display64=f'display64/{key}.png',normal=f'normal/{key}.png',registration=meta))
        ob=standalone_outfit(e) if e['kind']=='outfit' else raw(e['review_id'])
        ob=ob.crop(bounds(ob));ob.thumbnail((280,280),Image.Resampling.LANCZOS);save(ob,OUT/'objects'/f"{e['review_id']}.png");item['object']=f"objects/{e['review_id']}.png"
        small=ob.copy();small.thumbnail((28,28),Image.Resampling.LANCZOS)
        icon=Image.new('RGBA',(32,32));icon.alpha_composite(small,((32-small.width)//2,(32-small.height)//2))
        save(icon,OUT/'icons32'/f"{e['review_id']}.png");save(icon.resize((64,64),Image.Resampling.NEAREST),OUT/'icons64'/f"{e['review_id']}.png")
        item['icon32']=f"icons32/{e['review_id']}.png";item['icon64']=f"icons64/{e['review_id']}.png"
        records.append(item)
        if n%20==0:print(n+1,'/',len(chosen),flush=True)
    contact(records)
    assert all(sha(ROOT/p)==h for p,h in before.items())
    manifest=dict(review_id='BD-CATALOGUE03',status='review_only_no_new_candidates_approved',logical_tile=32,display_tile=64,display_method='nearest_x2_of_same_native32',entries=records,classes=NAMES,counts=dict(Counter(e['kind'] for e in records)),composition_count=sum(len(e['compositions']) for e in records),active_files_before_after=before,source_methods='existing individual generated drawings + source-authored BD unarmed/arm poses; similarity transforms only; no old pixel art hands, stencils, shadows or palettes')
    (OUT/('pilot.json' if args.pilot else 'catalogue.json')).write_text(json.dumps(manifest,ensure_ascii=False,indent=2),'utf-8')
    if not args.pilot:
        (OUT/'review-data.js').write_text('window.reviewData='+json.dumps(manifest,ensure_ascii=False)+';','utf-8')
        shutil.copy2(BASE/'comic_catalogue_template_03.html',OUT/'index.html')
        shutil.copy2(BASE/'comic64-reference-review-04/images/floor32.png',OUT/'floor32.png')
    print('complete',len(records),'objects',manifest['composition_count'],'compositions')
if __name__=='__main__':main()
