"""Package approved native64 art and explicit limb ownership; no repainting."""
import json
import math
from pathlib import Path
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import ROOT, REVIEW, sha, write_json

DEST = ROOT/'assets/sprites/class-armors-01'
DOC = ROOT/'docs/reviews/class-armor-integration-01'
CLASSES = ['bastion-copper-wave-01','technomancien-no-robe-heads-02',
           'traqueuse-sniper-wave-01','arcaniste-red-robe-wave-05','metamorphe-biodruid-wave-03']
PALMS = [
    [[(18,35),(45,35)],[(18,35),(45,35)],[(19,35),(45,35)],[(18,35),(46,35)],[(16,35),(48,35)]],
    [[(19,35),(45,35)]]*5,
    [[(21,36),(43,36)]]*5,
    [[(17,27),(47,27)],[(17,28),(47,28)],[(17,28),(47,28)],[(17,29),(47,29)],[(17,29),(47,29)]],
    [[(20,34),(44,35)],[(21,35),(44,35)],[(21,35),(44,35)],[(20,35),(44,35)],[(19,35),(45,35)]],
]


def blank():
    return Image.new('RGBA',(128,128))


def selection(image, polygon):
    mask = Image.new('L',image.size)
    ImageDraw.Draw(mask).polygon(polygon,fill=255)
    part = Image.new('RGBA',image.size)
    part.paste(image,(0,0),mask)
    return part,mask


def padded(image):
    result = blank()
    result.paste(image,(32,32))
    return result


def ranged_ownership(base, palm, pivot, side, seam_width):
    """Follow the transparent gap beside the torso, never carry a thigh."""
    x,y=palm;px,py=pivot
    mask=Image.new('L',(128,128))
    boundary=(x+32)/2
    for row in range(py-1,y+6):
        runs=[];start=None
        for col in range(64):
            opaque=base.getpixel((col+32,row+32))[3]>=160
            if opaque and start is None:start=col
            if start is not None and (not opaque or col==63):
                runs.append((start,col if not opaque else col+1));start=None
        for lo,hi in runs:
            exterior=hi<=boundary+1 if side==0 else lo>=boundary-1
            if exterior and abs((lo+hi-1)/2-x)<=10:
                for col in range(max(0,lo-1),min(64,hi+1)):mask.putpixel((col+32,row+32),255)
        if row<=py+1:
            # At the elbow the sleeve can join the torso. Its joint keeps the
            # existing inner seam; rows below use separate silhouette runs.
            lo,hi=(0,round(px+3)) if side==0 else (round(px-3),64)
            for col in range(lo,hi):mask.putpixel((col+32,row+32),255)
        elif row<=y+1:
            # Broad gauntlets can touch the hip. Follow their inner cuff seam
            # instead of treating a joined armor/torso run as one movable limb.
            lo,hi=(0,x+seam_width+1) if side==0 else (x-seam_width,64)
            for col in range(lo,hi):mask.putpixel((col+32,row+32),255)
    # The last antialiased fingertip can be below the solid run threshold.
    # Its known palm axis belongs to the hand, including those last three rows.
    for row in range(y,y+4):mask.putpixel((x+32,row+32),255)
    image=blank();image.paste(base,(0,0),mask)
    return image,mask


def snap(image, point):
    x,y = point
    pixels = [(xx,yy,image.getpixel((xx,yy))[3])
              for yy in range(y-1,y+2) for xx in range(x-2,x+3)
              if image.getpixel((xx,yy))[3]>=96]
    assert pixels, point
    total = sum(a for _,_,a in pixels)
    return [round(sum(p[i]*p[2] for p in pixels)/total) for i in range(2)]


def atlas(images, name):
    sheet = Image.new('RGBA',(16*128,math.ceil(len(images)/16)*128))
    for i,im in enumerate(images):
        sheet.paste(im,((i%16)*128,(i//16)*128))
    p = DEST/(name+'.png')
    sheet.save(p,optimize=True)
    return {'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}


def main():
    DEST.mkdir(parents=True,exist_ok=True)
    DOC.mkdir(parents=True,exist_ok=True)
    banks = {n:[] for n in ['base','core','arm-0','arm-1','fingers-0','fingers-1',
                           'ranged-core','ranged-arm-0','ranged-arm-1',
                           'grip-0','grip-1','hand-mask-0','hand-mask-1','head-mask',
                           'raised-arm-0','raised-arm-1','raised-erase-0','raised-erase-1',
                           'raised-fingers-0','raised-fingers-1']}
    pose_path = ROOT/'assets/source-art/class-armor-pose-fixes-03/production.json'
    poses = json.loads(pose_path.read_text('utf-8')) if pose_path.exists() else {'entries':[]}
    pose_by_index = {p['index']:p for p in poses['entries']}
    rigs, approvals, identities = [], [], []
    for ci,name in enumerate(CLASSES):
        number = [1,14,15,16,17][ci]
        batch = 1 if ci==0 else 2
        closed_body = Image.open(ROOT/f'assets/sprites/art-finish-{batch:02}/FIN-{number:02}-single-body.png').convert('RGBA')
        closed_fingers = Image.open(ROOT/f'assets/sprites/art-finish-{batch:02}/FIN-{number:02}-single-fingers.png').convert('RGBA')
        # The visible knuckle centre, rather than the old sleeve endpoint, is
        # the point that must meet the weapon handle. Different original
        # paintings place that centre one or two pixels apart.
        points = [(x,y,closed_fingers.getpixel((x,y))[3]) for y in range(128)
                  for x in range(128) if closed_fingers.getpixel((x,y))[3]>=16]
        total = sum(a for _,_,a in points)
        old_palm = [round(sum(p[k]*p[2] for p in points)/total) for k in (0,1)]
        cx,cy = old_palm
        closed_hand,_ = selection(closed_body,[(cx-2,cy-2),(cx+3,cy-2),(cx+3,cy+3),(cx-2,cy+3)])
        if ci==2:
            # FIN-15's separate foreground contains only a thin finger edge.
            # Its closed knuckle plane is already painted on the hand layer;
            # give that small plane foreground ownership as well, never cuff.
            knuckles,_ = selection(closed_hand,[(cx-1,cy-1),(cx+1,cy-1),
                                               (cx+1,cy+2),(cx-1,cy+2)])
            closed_fingers = Image.alpha_composite(closed_fingers,knuckles)
        decision = REVIEW/name/'acceptance.json'
        approval = json.loads(decision.read_text('utf-8'))
        approvals.append({'path':decision.relative_to(ROOT).as_posix(),'sha256':sha(decision)})
        for ri,e in enumerate(approval['entries']):
            native = ROOT/e['native']['path']
            assert sha(native)==e['native']['sha256']
            image = Image.open(native).convert('RGBA')
            assert image.size==(64,64)
            outfit_index = ci*5+ri
            override = poses.get('metamorphe') if outfit_index==20 else None
            if override:
                assert sha(ROOT/override['native']['path'])==override['native']['sha256']
                image = Image.open(ROOT/override['native']['path']).convert('RGBA')
            base = padded(image)
            core = base.copy()
            palms = [snap(image,p) for p in PALMS[ci][ri]]
            pivots = []
            original_arms = []
            ranged_core = base.copy()
            for side,(x,y) in enumerate(palms):
                if ci==3:
                    pivot = [24 if side==0 else 40,20]
                    px,py = pivot
                    points = [(px-3,py-1),(px+3,py-1),(x+4,y+3),(x-4,y+3)]
                else:
                    pivot = [x+(1 if side==0 else -1),y-8]
                    px,py = pivot
                    points = [(px-2.5,py-1),(px+2.5,py-1),(x+2.5,y+3),(x-2.5,y+3)]
                # Explicit per-arm polygons end at the palm and never select legs.
                points = [(a+32,b+32) for a,b in points]
                arm,mask = selection(base,points)
                original_arms.append(arm.copy())
                core.paste((0,0,0,0),(0,0,128,128),mask)
                _,handmask = selection(base,[(x+29.5,y+30.5),(x+34.5,y+30.5),
                                             (x+34.5,y+36),(x+29.5,y+36)])
                from PIL import ImageChops
                handmask = ImageChops.multiply(handmask,mask)
                arm.paste((0,0,0,0),(0,0,128,128),handmask)
                handmask_rgba = blank();handmask_rgba.putalpha(handmask)
                banks[f'hand-mask-{side}'].append(handmask_rgba)
                hand = closed_hand if side==1 else closed_hand.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
                fg = closed_fingers if side==1 else closed_fingers.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
                anchor = old_palm if side==1 else [127-old_palm[0],old_palm[1]]
                offset = (round(x+32-anchor[0]),round(y+32-anchor[1]))
                grip = blank();grip.paste(hand,offset)
                fingers = blank();fingers.paste(fg,offset)
                banks[f'grip-{side}'].append(grip)
                banks[f'arm-{side}'].append(arm)
                banks[f'fingers-{side}'].append(fingers)
                pivots.append([v+32 for v in pivot])
                # Own the full exterior of the forearm, including the old
                # hanging fingers. The former narrow strip left cuffs behind.
                # Keep its existing inner seam so torso/thigh ownership stays fixed.
                seam_width=5 if ci==0 and ri==4 else [3,3,2,4,2][ci]
                ranged_arm,owned=ranged_ownership(base,(x,y),pivot,side,seam_width)
                ranged_core.paste((0,0,0,0),(0,0,128,128),owned)
                # Closed gripping hands are separate layers. Do not move the
                # original open fingers or pixels below the wrist into the gun.
                ranged_arm.paste((0,0,0,0),(0,y+31,128,128))
                banks[f'ranged-arm-{side}'].append(ranged_arm)
            head = Image.open(ROOT/e['components']['head-mask']['path']).convert('L')
            # Ownership is constrained to the head; shoulder fur/collars stay on the torso.
            top = image.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()[1]
            cap = Image.new('L',(64,64))
            ImageDraw.Draw(cap).rectangle((24,0,40,top+10),fill=255)
            from PIL import ImageChops
            head = ImageChops.multiply(head,cap)
            head_rgba = Image.new('RGBA',(64,64))
            head_rgba.putalpha(head)
            banks['head-mask'].append(padded(head_rgba))
            if ri==0:
                identity,_ = selection(base,[(56,32),(72,32),(72,32+top+12),(56,32+top+12)])
                identities.append(identity)
            banks['base'].append(base)
            banks['core'].append(core)
            banks['ranged-core'].append(ranged_core)
            rebuilt = core.copy()
            for side in range(2):
                arm = original_arms[side]
                mask = arm.getchannel('A').point(lambda a:255 if a else 0)
                rebuilt.paste(arm,(0,0),mask)
            assert all(rebuilt.getpixel((x,y))==base.getpixel((x,y)) for y in range(128)
                       for x in range(128) if base.getpixel((x,y))[3])
            pose_fix = pose_by_index.get(outfit_index)
            raised_palms = pose_fix['palms'] if pose_fix else [
                [min(palms[0][0]-3,14)+32,top+45],
                [max(palms[1][0]+3,50)+32,top+45]]
            for side in range(2):
                for name in ['arm','erase','fingers']:
                    pose_image = blank()
                    if pose_fix:
                        ref = pose_fix['parts'][f'{name}-{side}']
                        assert sha(ROOT/ref['path'])==ref['sha256']
                        pose_image = padded(Image.open(ROOT/ref['path']).convert('RGBA'))
                    banks[f'raised-{name}-{side}'].append(pose_image)
            rigs.append({'index':outfit_index,'class_id':approval['class_id'],
                'candidate_id':e['candidate_id'],'rank':ri+1,'native':e['native'],
                'runtime_native':override['native'] if override else e['native'],
                'pose_revision':pose_fix.get('candidate_id') if pose_fix else None,
                'raised_palms':raised_palms,
                'palms':[[v+32 for v in p] for p in palms],'pivots':pivots,
                'head_center':[64,32+top+5.0],'head_top':top,
                'limb_masks_are_source_pixels':True})
    files = {n:atlas(ims,n) for n,ims in banks.items()}
    files['identity-heads'] = atlas(identities,'identity-heads')
    # Reuse the six existing worn helmets; these are not newly designed helmets.
    source = Image.open(ROOT/'assets/sprites/cyberpunk/player-worn-128.png').convert('RGBA')
    helmets, openings = [], []
    for index in range(6):
        cell = index+14
        head = source.crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128))
        box = head.getchannel('A').getbbox()
        opening = Image.new('L',(128,128))
        if index in (0,1):
            ImageDraw.Draw(opening).polygon([(51,18),(77,18),(78,35),(65,44),(51,35)],fill=255)
            head.paste((0,0,0,0),(0,0,128,128),opening)
        cropped = head.crop(box)
        scale = 11/(box[3]-box[1])
        size = (round(cropped.width*scale),11)
        reduced = cropped.resize(size,Image.Resampling.LANCZOS)
        result = blank()
        result.paste(reduced,(64-size[0]//2,35))
        hole = blank()
        hm = opening.crop(box).resize(size,Image.Resampling.NEAREST)
        h = Image.new('RGBA',size);h.putalpha(hm)
        hole.paste(h,(64-size[0]//2,35))
        helmets.append(result);openings.append(hole)
    files['helmets'] = atlas(helmets,'helmets')
    files['helmet-openings'] = atlas(openings,'helmet-openings')
    write_json(DEST/'manifest.json',{'review_id':'CLASS-ARMOR-INTEGRATION01',
        'native_tile':[64,64],'layer_canvas':[128,128],'native_origin':[32,32],
        'outfit_count':25,'source_acceptances':approvals,'outfits':rigs,'atlases':files,
        'weapon_designs':'unchanged current 113-model catalogue; original grip bindings reused',
        'colors':'approved original materials; new torso variants remain deferred'})
    code = '// Generated from approved native64 compositions; no artifacts dependency.\n'
    code += 'fn armor_bank_bytes(part: &str) -> &\'static [u8] { match part {\n'
    code += ''.join(f'    "{n}" => include_bytes!("../{r["path"]}"),\n' for n,r in files.items())
    code += '    _ => unreachable!("known armor component"),\n}}\n'
    code += 'const ARMOR_RIGS: &[ArmorRig] = &[\n'
    for r in rigs:
        palms = [[float(v) for v in p] for p in r['palms']]
        pivots = [[float(v) for v in p] for p in r['pivots']]
        center = [float(v) for v in r['head_center']]
        raised = [[float(v) for v in p] for p in r['raised_palms']]
        corrected = 'true' if r['pose_revision'] else 'false'
        code += f'    ArmorRig {{ palms: {palms}, pivots: {pivots}, head_center: {center}, raised_palms: {raised}, corrected_arms: {corrected} }},\n'
    code += '];\n'
    (ROOT/'src/game_art_class_armor_catalog.rs').write_text(code,encoding='utf-8')
    print(json.dumps({'outfits':len(rigs),'atlases':len(files),'native_sources_unchanged':True}))


if __name__=='__main__':
    main()
