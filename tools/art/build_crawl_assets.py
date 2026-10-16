"""Deterministic packaging of the accepted native Crawl-style sources.

Only source extraction, nearest scaling, declared layer masks and atlas packing.
No generation or painting. Content/item indices and gameplay data are preserved.
Run --check to verify the embedded assets and catalog.
"""
import argparse
import colorsys
import io
import json
import math
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'artifacts/crawl-methode-2026-10-05'
REVIEW = ROOT / 'artifacts/crawl-integration-2026-10-05'
GRIP_REVIEW = ROOT / 'artifacts/prises-audit-2026-10-05'
OUTPUT = ROOT / 'assets/sprites/cyberpunk'
COLS = 16
NAMES = ['Couteau', 'Épée', 'Épée à deux mains', 'Hache', 'Matraque',
         'Arme d’hast', 'Pistolet', 'Pistolet mitrailleur', 'Fusil d’assaut',
         'Fusil à pompe', 'Mitrailleuse', 'Fusil à énergie', 'Lance-flammes',
         'Lance-grenades', 'Lance-roquettes', 'Lance-aiguilles', 'Marteau',
         'Fusil de précision']
# Native scale and placement guides of each source silhouette. Actual handle
# contacts are authored separately in GRIPS, not inferred from these bounds.
WEAPONS = [
    (7, 18, .50, .84, 35, 29), (8, 28, .50, .86, 35, 29),
    (22, 31, .13, .87, 25, 28), (14, 30, .23, .87, 25, 29),
    (6, 23, .50, .84, 35, 28), (7, 35, .50, .72, 35, 28),
    (9, 16, .22, .84, 35, 22), (11, 19, .24, .84, 35, 22),
    (32, 23, .44, .73, 23, 28), (32, 22, .42, .72, 23, 28),
    (33, 24, .45, .72, 23, 28), (33, 24, .43, .73, 23, 28),
    (32, 23, .38, .76, 23, 28), (29, 22, .40, .76, 23, 28),
    (34, 23, .43, .77, 23, 28), (33, 24, .43, .74, 23, 28),
    (15, 23, .50, .84, 35, 28), (34, 24, .44, .73, 23, 28),
]
# Wrist centers in the native composition canvas. The second point is the
# support hand, never a guessed continuation of a generated weapon fragment.
GRIPS = [
    [(35, 29)], [(35, 29)], [(24, 30), (27, 26)], [(26, 27), (26, 23)],
    [(35, 28)], [(35, 28)], [(35, 22)], [(35, 22)],
    [(18, 29), (33, 22)], [(24, 28), (33, 22)],
    [(20, 30), (33, 22)], [(21, 29), (33, 22)],
    [(19, 29), (31, 21)], [(21, 29), (31, 21)],
    [(24, 29), (32, 22)], [(22, 30), (31, 23)],
    [(35, 26)], [(19, 30), (31, 23)],
]
# Per-outfit authored color identities: primary/secondary hue, ivory roles.
MATERIALS = [
    (0, None, 2), (.62, None, 2), (.27, .61, 0), (None, .62, 1),
    (.07, .61, 0), (.77, .49, 0), (.62, None, 2), (None, .53, 1),
    (.07, None, 2), (None, .62, 1), (.53, None, 2), (.78, None, 2),
    (.18, .61, 0), (0, .50, 0), (.14, .78, 0), (.62, None, 2),
    (0, None, 2),
]

def png(im):
    stream = io.BytesIO(); im.save(stream, 'PNG', optimize=True)
    return stream.getvalue()

def atlas(images, side):
    result = Image.new('RGBA', (COLS*side, math.ceil(len(images)/COLS)*side))
    for i, im in enumerate(images):
        assert im.size == (side, side)
        result.paste(im, (i%COLS*side, i//COLS*side))
    return png(result)

def indexed(im, budget=16):
    alpha = im.getchannel('A')
    rgb = im.convert('RGB').quantize(budget, method=Image.Quantize.MEDIANCUT,
                                   dither=Image.Dither.NONE).convert('RGBA')
    rgb.putalpha(alpha); return rgb

def connected(im):
    """Keep the actual source silhouette, discarding detached sheet debris.

    Done before bounding-box reduction: stray bars must not shrink the weapon.
    Eight-neighbor connectivity preserves diagonal native pixel-art edges.
    """
    alpha=im.getchannel('A').point(lambda a: 255 if a>=128 else 0)
    remaining={i for i,a in enumerate(alpha.tobytes()) if a}
    components=[]; w,h=im.size
    while remaining:
        seed=min(remaining); remaining.remove(seed); stack=[seed]; group=[seed]
        while stack:
            point=stack.pop();x,y=point%w,point//w
            for yy in range(max(0,y-1),min(h,y+2)):
                for xx in range(max(0,x-1),min(w,x+2)):
                    n=yy*w+xx
                    if n in remaining:
                        remaining.remove(n);stack.append(n);group.append(n)
        components.append(group)
    assert components, 'Empty source silhouette'
    main=max(components,key=len)
    clean=bytearray(w*h)
    for i in main: clean[i]=255
    im=im.copy();im.putalpha(Image.frombytes('L',im.size,bytes(clean)))
    return im

def source_cells():
    source = Image.open(REVIEW/'held-source.png').convert('RGBA')
    images = []
    for i in range(18):
        x,y = i%6,i//6
        im = source.crop((round(x*source.width/6), round(y*source.height/3),
                          round((x+1)*source.width/6), round((y+1)*source.height/3)))
        if i==3:
            im=Image.open(REVIEW/'axe-outward-source.png').convert('RGBA')
        if i==2:
            im=Image.open(GRIP_REVIEW/'greatsword-outward-source.png').convert('RGBA')
        im=connected(im)
        if i==0:
            # Its sharpened side is outward, just like the axe/greatsword.
            im=im.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        im = im.crop(im.getbbox())
        w,h,px,py,ax,ay = WEAPONS[i]
        layer = Image.new('RGBA',(48,48))
        layer.alpha_composite(im.resize((w,h), Image.Resampling.NEAREST),
                              (round(ax-px*w), round(ay-py*h)))
        assert all(layer.getpixel((j,k))[3]==0 for j in range(48) for k in (0,47)), i
        images.append(indexed(layer))
    return images

def hue_distance(a,b):
    return min(abs(a-b),1-abs(a-b)) if b is not None else 10

def body_mask(im, i):
    """Write a source-specific material map once; runtime reads labels only.

    Source palette identities above and protected head pixels are deliberately
    independent of pose. The PNG is the persistent per-pixel material authority.
    """
    result=Image.new('RGBA',im.size)
    primary,secondary,ivory=MATERIALS[i]
    for y in range(im.height):
        for x in range(im.width):
            r,g,b,a=im.getpixel((x,y)); high=max(r,g,b); low=min(r,g,b)
            if not a or high<48: continue
            hue,saturation,value=colorsys.rgb_to_hsv(r/255,g/255,b/255)
            # Skin/eye windows in the heads, not orange costume pixels.
            skin_window=(i==4 and 14<=x<18 and 8<=y<10) or (i==14 and 14<=x<18 and 9<=y<11)
            if skin_window and (r>g>b and r-g>15 and g-b>10):
                continue
            neutral_threshold=48 if i==7 else 110
            label=ivory if saturation<.23 and high>=neutral_threshold else 0
            if saturation>=.23:
                label=1 if hue_distance(hue,primary)<hue_distance(hue,secondary) else 2
            if label: result.putpixel((x,y),(label,0,0,255))
    return result

def weapon_mask(im):
    result=Image.new('RGBA',im.size)
    for y in range(im.height):
        for x in range(im.width):
            r,g,b,a=im.getpixel((x,y)); high=max(r,g,b);low=min(r,g,b)
            if a and high>45 and high-low>high*.25:
                result.putpixel((x,y),(1,0,0,255))
    return result

def arms():
    """Extract the weapon-free authored hands, then align them to each grip.

    The former color filter also admitted steel/old handles. This source contains
    no weapons, and declared glove regions provide exact foreground occlusion.
    Side fists are reflected vertically so their cuffs lead UP to the elbow.
    """
    source=Image.open(GRIP_REVIEW/'grips-clean-source.png').convert('RGBA')
    poses=[]
    # Source-native glove bounds and wrist centers, before canvas placement.
    parts=[[(27,23,40,35,(36,27),(32,24,39,30))],
           [(26,19,35,35,(31,23),(29,20,34,25))],
           [(7,22,21,31,(17,26),(15,23,20,30)),
            (25,17,37,28,(29,20),(26,18,32,22))],
           [(9,23,22,32,(19,29),(16,27,22,32)),
            (19,22,30,33,(23,25),(20,23,25,28))]]
    for pose in range(4):
        x,y=pose%2,pose//2
        cell=source.crop((x*source.width//2,y*source.height//2,
                          (x+1)*source.width//2,(y+1)*source.height//2))
        cell=cell.resize((40,40),Image.Resampling.NEAREST)
        cell.putalpha(cell.getchannel('A').point(lambda a:255 if a>=128 else 0))
        poses.append(cell)
    arm_layers=[];finger_layers=[];maps=[]
    for family,grips in enumerate(GRIPS):
        pose=0 if family in (0,1,4,5,16) else 1 if family in (6,7) else 3 if family in (2,3) else 2
        layer=Image.new('RGBA',(48,48));fingers=layer.copy();mask=layer.copy()
        for hand,(spec,(ax,ay)) in enumerate(zip(parts[pose],grips)):
            l,t,r,b,(cx,cy),glove=spec
            cuff=Image.new('RGBA',(40,40))
            for y in range(t,b):
                for x in range(l,r):
                    p=poses[pose].getpixel((x,y))
                    if not p[3]: continue
                    if pose==3:
                        belongs_left=x<20 or (y>=28 and x<22)
                        if belongs_left!=(hand==0): continue
                    xx=ax+x-cx;yy=ay+(cy-y if pose==0 else y-cy)
                    if not (0<=xx<48 and 0<=yy<48): continue
                    is_glove=glove[0]<=x<glove[2] and glove[1]<=y<glove[3]
                    if is_glove:
                        fingers.putpixel((xx,yy),p)
                    else:
                        cuff.putpixel((x,y),p)
            # Align the authored cuff's wrist and elbow with the fixed body.
            # This is nearest affine placement, not new raster drawing.
            joints=[((33,29),(30,32)), ((31,26),(30,31)),
                    ((14,25),(10,25)), ((32,23),(34,25)),
                    ((16,27),(11,26)), ((25,28),(27,30))]
            joint=joints[pose if pose<2 else 2+hand if pose==2 else 4+hand]
            (wx,wy),(ex,ey)=joint
            if pose==0:
                target_w=(ax-2,ay-2);target_e=(33,25)
            elif pose==1:
                target_w=(ax,ay+3);target_e=(34,28)
            elif hand==0:
                target_w=(ax-2,ay-1);target_e=(15,24)
            else:
                target_w=(ax+2,ay+2);target_e=(33,24)
            ux,uy=target_e[0]-target_w[0],target_e[1]-target_w[1]
            vx,vy=ex-wx,ey-wy;den=ux*ux+uy*uy
            assert den
            # Stretch along the forearm only. Its thickness stays at the native
            # body scale instead of ballooning into the torso on long reaches.
            vl=math.hypot(vx,vy);ul=math.sqrt(den);thickness=.7
            a=vx*ux/den+vy*uy/(vl*ul*thickness)
            b=vx*uy/den-vy*ux/(vl*ul*thickness)
            d=vy*ux/den-vx*uy/(vl*ul*thickness)
            e=vy*uy/den+vx*ux/(vl*ul*thickness)
            c=wx-a*target_w[0]-b*target_w[1]
            f=wy-d*target_w[0]-e*target_w[1]
            cuff=connected(cuff)
            warped=cuff.transform((48,48),Image.Transform.AFFINE,
                                  (a,b,c,d,e,f),Image.Resampling.NEAREST)
            warped=connected(warped)
            layer.alpha_composite(warped)
        for y in range(48):
            for x in range(48):
                p=layer.getpixel((x,y))
                if p[3] and max(p[:3])>100:mask.putpixel((x,y),(2,0,0,255))
        arm_layers.append(indexed(layer));finger_layers.append(indexed(fingers));maps.append(mask)
    return arm_layers,finger_layers,maps

def family_for(model):
    category=model['category']; ident=model['id']
    if category=='knives': return 0
    if category=='melee': return 1
    if category=='swords': return 2
    if category=='axes': return 3
    if category=='spears': return 5
    if category=='war_hammers':
        return 4 if any(k in ident for k in ('gourdin','masse')) else 16
    if category=='rifles': return 17
    mapping={'assault_rifles':8,'energy_rifles':11,'shotguns':9,'machine_guns':10,
             'flamethrowers':12,'grenade_launchers':13,'rocket_launchers':14}
    if category in mapping: return mapping[category]
    if ident=='core:flamethrower': return 12
    if ident=='core:needle_launcher': return 15
    raise ValueError('Unmapped weapon '+ident)

def build():
    catalog=json.loads((REVIEW/'catalog-source.json').read_text(encoding='utf-8'))
    models=json.loads((ROOT/'artifacts/sprites-cyberpunk-crawl-review-2026-10-04/equipment-catalog.json').read_text(encoding='utf-8'))['models']
    model_by_id={m['id']:m for m in models}
    held=catalog['held_weapons']
    for m in held:
        m['family']=family_for(model_by_id[m['id']]) if m['id'] in model_by_id else (7 if 'crache_fer_pm' in m['id'] else 6)
    outfits=[Image.open(SOURCE/f'tiles/armor-{i+1:02}.png').convert('RGBA') for i in range(17)]
    monsters=[Image.open(SOURCE/f'tiles/monsters-{i+1:02}.png').convert('RGBA') for i in range(24)]
    actors=outfits[:3]+monsters
    # Six civilian/combat roles reuse their own fixed bases and the same pose layers.
    bodies=outfits+monsters[17:23]
    masks=[body_mask(im,i) for i,im in enumerate(outfits)]+[Image.new('RGBA',(32,32)) for _ in range(6)]
    weapons=source_cells(); weapon_maps=[weapon_mask(im) for im in weapons]
    arm_layers,fingers,arm_maps=arms()
    entries=catalog['items']
    item_families={m['id']:m['family'] for m in held}
    # Native item drawing follows the held family, even in a large tooltip.
    icons=Image.open(OUTPUT/'ground-32.png').convert('RGBA')
    legacy_items=Image.open(OUTPUT/'items-128.png').convert('RGBA')
    item_images=[];item_maps=[]
    for entry in entries:
        index=entry['illustration'];family=item_families.get(entry['id'])
        if family is not None:
            cut=weapons[family].crop(weapons[family].getbbox());cut.thumbnail((28,28),Image.Resampling.NEAREST)
            im=Image.new('RGBA',(32,32));im.alpha_composite(cut,((32-cut.width)//2,(32-cut.height)//2))
            material=weapon_mask(im)
        elif 101<=index<=114:
            im=outfits[index-98];material=masks[index-98]
        elif entry['icon']==2:
            # Keep the accepted helmet/glove/boot identities at native size.
            # Their separate illustration indices must also mean separate art.
            im=legacy_items.crop((index%16*128,index//16*128,index%16*128+128,index//16*128+128))
            im=im.resize((32,32),Image.Resampling.NEAREST)
            im.putalpha(im.getchannel('A').point(lambda a: 255 if a>=128 else 0))
            im=indexed(im);material=weapon_mask(im)
        else:
            # Preserve already approved supplies.
            icon=entry['icon'];im=icons.crop((icon%16*32,icon//16*32,icon%16*32+32,icon//16*32+32))
            material=weapon_mask(im) if entry['icon']==2 else Image.new('RGBA',(32,32))
        item_images.append(im);item_maps.append(material)
    catalog.update(version=5, method='native fixed bodies + authored per-family grips + weapons + fingers',
                   native_tile=32,composition_canvas=48, source_review='artifacts/crawl-methode-2026-10-05',
                   held_source='artifacts/crawl-integration-2026-10-05/held-source.png',
                   axe_source='artifacts/crawl-integration-2026-10-05/axe-outward-source.png',
                   grip_source='artifacts/prises-audit-2026-10-05/grips-clean-source.png',
                   greatsword_source='artifacts/prises-audit-2026-10-05/greatsword-outward-source.png',
                   weapon_families=[{'index':i,'name':n,'anchor':GRIPS[i][0],
                                    'placement_guide':WEAPONS[i][4:], 'hands':GRIPS[i]}
                                   for i,n in enumerate(NAMES)],
                   body_count=len(bodies), material_maps='per-pixel labels: 0 protected, 1 primary, 2 secondary')
    contacts=[]
    for im in actors:
        l,t,r,b=im.getbbox();contacts.append([(l+r)//2,min(31,b-1),max(8,min(20,r-l))])
    catalog['actor_contacts']=contacts
    catalog['actor_bounds']=[list(im.getbbox()) for im in actors]
    outputs={'crawl-bodies-32.png':atlas(bodies,32),'crawl-body-materials-32.png':atlas(masks,32),
             'crawl-weapons-48.png':atlas(weapons,48),'crawl-weapon-materials-48.png':atlas(weapon_maps,48),
             'crawl-arms-48.png':atlas(arm_layers,48),'crawl-fingers-48.png':atlas(fingers,48),
             'crawl-arm-materials-48.png':atlas(arm_maps,48),'crawl-items-32.png':atlas(item_images,32),
             'crawl-item-materials-32.png':atlas(item_maps,32),
             'actors-32.png':atlas(actors,32),
             'actors-128.png':atlas([im.resize((128,128),Image.Resampling.NEAREST) for im in actors],128),
             'manifest.json':(json.dumps(catalog,ensure_ascii=False,indent=2)+'\n').encode('utf-8')}
    lines=['// Generated by tools/art/build_crawl_assets.py; do not edit by hand.',
           '#[rustfmt::skip]','pub(super) fn legacy_item(id: &str) -> Option<ItemArt> {','    Some(match id {']
    lines += [f'        "{m["id"]}" => ItemArt {{ icon: {m["icon"]}, illustration: {m["illustration"]} }},' for m in entries]
    lines += ['        _ => return None,','    })','}','#[rustfmt::skip]',
              'pub(super) fn held_weapon(id: &str) -> Option<u16> {','    Some(match id {']
    lines += [f'        "{m["id"]}" => {m["index"]},' for m in held]
    lines += ['        _ => return None,','    })','}','#[rustfmt::skip]',
              'pub(super) fn weapon_hold(index: u16) -> WeaponHold {','    match index {']
    lines += [f'        {m["index"]} => WeaponHold::{m["hold"]},' for m in held]
    lines += ['        _ => WeaponHold::Side,','    }','}',
              f'pub(super) const HELD_WEAPON_COUNT: u16 = {len(held)};',
              f'pub(super) const WEAPON_FAMILY_COUNT: u16 = {len(weapons)};',
              'pub(super) const WEAPON_FAMILIES: [u8; HELD_WEAPON_COUNT as usize] = [',
              '    '+', '.join(str(m['family']) for m in held)+',','];',
              '#[cfg(test)]',
              'pub(super) const WEAPON_GRIPS: [&[(u8, u8)]; WEAPON_FAMILY_COUNT as usize] = [']
    lines += ['    &['+', '.join(f'({x}, {y})' for x,y in grips)+'],' for grips in GRIPS]
    lines += ['];',
              '#[cfg(any(test, debug_assertions))]',
              f'pub(crate) const ITEM_COUNT: usize = {len(entries)};',
              f'pub(crate) const ACTOR_COUNT: usize = {len(actors)};',
              'pub(super) const CONTACTS: [[u8; 3]; ACTOR_COUNT] = [']
    lines += [f'    [{x}, {y}, {w}],' for x,y,w in contacts]
    lines += ['];']
    return outputs, ('\n'.join(lines)+'\n').encode('utf-8')

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--check',action='store_true');args=parser.parse_args()
    outputs,catalog=build();targets={OUTPUT/name:data for name,data in outputs.items()}
    targets[ROOT/'src/game_art_catalog.rs']=catalog
    if args.check:
        mismatches=[str(p.relative_to(ROOT)) for p,data in targets.items() if not p.exists() or p.read_bytes()!=data]
        if mismatches: raise SystemExit('Out of date: '+', '.join(mismatches))
    else:
        OUTPUT.mkdir(parents=True,exist_ok=True)
        for p,data in targets.items(): p.write_bytes(data)
    print('Crawl: 17 outfits, 24 bestiary images (23 active), 18 weapon families, 113 models, 162 items; '+('reproducible' if args.check else 'packaged'))

if __name__=='__main__': main()
