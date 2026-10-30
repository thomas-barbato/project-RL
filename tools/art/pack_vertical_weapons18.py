"""C06 complete arm poses, independent shared weapons and native grip layers."""
from pathlib import Path
import json, math, re, shutil, time
import numpy as np
from PIL import Image, ImageDraw, ImageFont
from pack_vertical_walk_equipment18 import complete_figures, palm
from pack_vertical_walk_size18 import structural_box, condition
from pack_vertical_walk_headgear18 import replace_head

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'extension-C06/conditioned-weapons'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06'
S=256
# Native grip contacts inspected on the neutral authored figures. These are
# points inside each clenched hand, not the centers of the bulky forearm armor.
CONTACTS={
 'arcaniste':{'ranged':[[58,60],[70,60]],'melee-wide':[[56,62],[72,62]],'melee-close':[[58,62],[62,60]]},
 'bastion':{'ranged':[[58,62],[71,62]],'melee-wide':[[56,64],[74,64]],'melee-close':[[58,65],[61,62]]},
 'traqueuse':{'ranged':[[58,63],[72,63]],'melee-wide':[[58,64],[72,64]],'melee-close':[[59,64],[62,61]]},
 'metamorphe':{'ranged':[[58,63],[72,63]],'melee-wide':[[57,66],[73,66]],'melee-close':[[58,65],[61,62]]},
 'bastion-lourd':{'ranged':[[57,62],[73,62]],'melee-wide':[[57,64],[75,64]],'melee-close':[[60,64],[64,62]]},
 'traqueuse-lourde':{'ranged':[[58,62],[72,62]],'melee-wide':[[57,63],[72,63]],'melee-close':[[59,64],[63,61]]},
 'arcaniste-lourd':{'ranged':[[58,61],[72,61]],'melee-wide':[[56,62],[73,62]],'melee-close':[[64,66],[64,62]]},
 'metamorphe-lourd':{'ranged':[[59,63],[71,63]],'melee-wide':[[58,66],[74,66]],'melee-close':[[60,66],[64,63]]},
}


def follow_hand(reference,image,point,dy):
    x,y=point;sample=np.asarray(reference.crop((x-3,y-3,x+4,y+4)),np.float32)
    candidates=[]
    for oy in range(-2,3):
        for ox in range(-2,3):
            xx,yy=x+ox,y+round(dy)+oy
            patch=np.asarray(image.crop((xx-3,yy-3,xx+4,yy+4)),np.float32)
            score=float(np.mean(np.abs(patch-sample)))+1.5*(ox*ox+oy*oy)
            candidates.append((score,[xx,yy]))
    return min(candidates,key=lambda p:p[0])[1]


def posed(source,pivot,target,scale,angle):
    """Same pixel-center, premultiplied bilinear convention as held_geometry."""
    src=np.asarray(source,dtype=np.float32);y,x=np.mgrid[0:S,0:S].astype(np.float32)
    sn,cs=np.float32(math.sin(angle)),np.float32(math.cos(angle));sc=np.float32(scale)
    dx=x+.5-target[0];dy=y+.5-target[1]
    sx=pivot[0]+(cs*dx+sn*dy)/sc-.5;sy=pivot[1]+(-sn*dx+cs*dy)/sc-.5
    ix=np.floor(sx).astype(np.int32);iy=np.floor(sy).astype(np.int32);fx=sx-ix;fy=sy-iy
    acc=np.zeros((S,S,4),np.float32)
    for xx,yy,w in [(ix,iy,(1-fx)*(1-fy)),(ix+1,iy,fx*(1-fy)),(ix,iy+1,(1-fx)*fy),(ix+1,iy+1,fx*fy)]:
        valid=(xx>=0)&(yy>=0)&(xx<source.width)&(yy<source.height)
        p=src[np.clip(yy,0,source.height-1),np.clip(xx,0,source.width-1)]
        a=p[:,:,3]*w*valid;acc[:,:,3]+=a;acc[:,:,:3]+=p[:,:,:3]*a[:,:,None]
    rgb=np.divide(acc[:,:,:3],acc[:,:,3,None],out=np.zeros_like(acc[:,:,:3]),where=acc[:,:,3,None]>0)
    rgba=np.concatenate([rgb,acc[:,:,3,None]],axis=2)
    return Image.fromarray(np.clip(np.floor(rgba+.5),0,255).astype(np.uint8))


def cells_of(profile):
    sheet=Image.open(ROOT/profile['file']).convert('RGBA');idle=Image.open(ROOT/profile['standing']).convert('RGBA')
    return [sheet.crop((f['phase']*128,int(f['direction']=='back')*128,(f['phase']+1)*128,(int(f['direction']=='back')+1)*128)) for f in profile['frames']]+[idle]


def hands_parts(image,points,occluded=False):
    core=image.copy();fingers=Image.new('RGBA',image.size)
    if not occluded:
        for x,y in points:
            x,y=round(x),round(y);box=(x-2,y-2,x+3,y+3)
            fingers.paste(image.crop(box),(x-2,y-2));core.paste((0,0,0,0),box)
    return core,fingers


def atlas(images):
    out=Image.new('RGBA',(6*S,3*S))
    for i,im in enumerate(images):out.paste(im,((i%6)*S,(i//6)*S))
    return out


def main():
    OUT.mkdir(parents=True,exist_ok=True);PREVIEW.mkdir(parents=True,exist_ok=True)
    old=json.loads((ART/'candidate-C05/manifest.json').read_text('utf8'))['profiles']
    profiles=[p for p in old if p['selector']['kind']=='player' and not p.get('head_variants')]
    requests=json.loads((ROOT/'docs/reviews/vertical-walk-18/prompts-C06-weapons.json').read_text('utf8'))['jobs']
    refs=json.loads((ROOT/'artifacts/vertical-walk-review-18/current/extension-catalog/catalog.json').read_text('utf8'))
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    entries=[];failures=[]
    for p in profiles:
        identity=p['id'].removesuffix('-size05');variants={}
        original=cells_of(p);original_frames=p['frames']+[{'direction':'idle','phase':0,'palms':p['standing_palms']}]
        candidates={'one':(original,original_frames)}
        for req in [r for r in requests if r['profile']==p['id']]:
            path=ROOT/req['output']
            if not path.exists() or time.time()-path.stat().st_mtime<2:continue
            try:
                figures=complete_figures(Image.open(path).convert('RGBA'))
                height=p['target_structural_height'];foot=p['original_foot_y']
                normalized=[]
                for row in range(4):
                    b=structural_box(figures[row*4]);sc=height/(b[3]-b[1])
                    normalized += [condition(im,sc,foot)[0] for im in figures[row*4:row*4+4]]
                images=[];frames=[]
                for i,n in enumerate([1,2,3,5,6,7,9,10,11,13,14,15,0]):
                    im=normalized[n];back=6<=i<12;top=structural_box(im)[1]
                    neutral=normalized[0];neutral_top=structural_box(neutral)[1]
                    points=CONTACTS[identity][req['family']]
                    if back:
                        front=frames[i-6];front_top=structural_box(images[i-6])[1]
                        grips=[[128-x,y+top-front_top] for x,y in front['grips']]
                    else:grips=[follow_hand(neutral,im,pt,top-neutral_top) for pt in points]
                    images.append(im);frames.append({'direction':'back' if back else 'front' if i<12 else 'idle','phase':i%6,'grips':grips,'grips_occluded':back})
                candidates[req['family']]=(images,frames)
            except (AssertionError,ValueError,OSError) as e:failures.append({'id':req['id'],'error':str(e)})
        # Keep exactly the same canonical class head between weapon families.
        heavy=next((h for h in old if h['id']==identity+'-headgear-size05'),None)
        if heavy:
            headparts=heavy['head_parts'];mask=Image.open(ROOT/headparts['replace_mask']).convert('RGBA')
            heads=[Image.open(ROOT/headparts[d]).convert('RGBA') for d in ('front','back')]
        else:
            mask=Image.open(ART/'candidate-C04/head-ownership.png').convert('RGBA');heads=[]
            for im in [original[12],original[6]]:
                anchor=[64,structural_box(im)[1]+6];x,y=48,round(anchor[1])-16
                h=im.crop((x,y,x+32,y+32));h.putalpha(Image.composite(h.getchannel('A'),Image.new('L',(32,32)),mask.getchannel('A')));heads.append(h)
            headparts={}
        board=Image.new('RGB',(len(candidates)*256,330),'#1e2831');draw=ImageDraw.Draw(board)
        for col,(family,(images,frames)) in enumerate(candidates.items()):
            cores=[];fingers=[];complete=[]
            for i,(im,f) in enumerate(zip(images,frames)):
                back=f['direction']=='back';anchor=[64,structural_box(im)[1]+6]
                im=replace_head(im,heads[int(back)],mask,anchor)
                # C05 one-handed landmarks remain the accepted authored points.
                grips=f['palms'] if family=='one' else f['grips']
                occluded=family!='one' and back
                core,hand=hands_parts(im,grips,occluded)
                canvas=Image.new('RGBA',(S,S));canvas.alpha_composite(core,(64,64));cores.append(canvas)
                canvas=Image.new('RGBA',(S,S));canvas.alpha_composite(hand,(64,64));fingers.append(canvas)
                f['head_anchor']=anchor;f['grips']=grips;f['grips_occluded']=occluded
                f['bounds']=list(structural_box(im))
                complete.append(im)
            core_path=OUT/f'{identity}-{family}-body.png';finger_path=OUT/f'{identity}-{family}-hands.png'
            atlas(cores).save(core_path);atlas(fingers).save(finger_path)
            variants[family]={'body':core_path.relative_to(ROOT).as_posix(),'hands':finger_path.relative_to(ROOT).as_posix(),'frames':frames}
            draw.text((col*256+4,4),identity+' / '+family,font=font,fill='white')
            for row,n in enumerate([12,0,6]):
                im=complete[n].copy();d=ImageDraw.Draw(im)
                for x,y in frames[n]['grips']:d.point((round(x),round(y)),fill='lime')
                board.paste(im.resize((256,256),Image.Resampling.NEAREST).crop((64,48,192,180)),(col*256+45,24+row*100),im.resize((256,256),Image.Resampling.NEAREST).crop((64,48,192,180)))
        board.save(PREVIEW/f'{identity}-grips-audit-x2.png')
        entries.append({'id':identity,'selector':p['selector'],'family_poses':variants,'head_parts':headparts,'head_bank_front':heads[0].getbbox(),'head_bank_back':heads[1].getbbox()})
    out={'review':'VERT18-C06-WEAPONS','normal_game_enabled':False,'native_tile_px':64,'side':S,'profiles':entries,'conditioning_errors':failures,
         'weapon_references':refs['weapons'],'scope_models':113,'source_art_shared':True,'all_grip_landmarks_require_native_review':True}
    (OUT/'manifest.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'profiles':len(entries),'whole_pose_families':sum(len(p['family_poses']) for p in entries),'errors':failures}))


if __name__=='__main__':main()
