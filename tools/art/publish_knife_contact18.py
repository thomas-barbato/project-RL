"""C11: put independent knife handles in actual fingers, not inherited cuffs.

Preserves C10 body pixels and all accepted C06/C08/C09 resources. The new
foreground ownership is selected only for a hand holding a knife.
"""
from pathlib import Path
import json,math,hashlib,shutil
from PIL import Image,ImageDraw,ImageChops,ImageFont
from finalize_knife_grip18 import cell,posed,angle as knife_angle
from finalize_pistol_tilt18 import angle as pistol_angle,helmet

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'knife-contact-C11'
DOC=ROOT/'docs/reviews/vertical-walk-18'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/knife-contact-C11'
# Explicitly inspected points INSIDE the fingers, in native128 source coordinates.
# Each pair is screen-left/screen-right, for front idle and first back phase.
PALMS={
 'bastion-armor1':{'front':[[48.5,72],[78.5,72]],'back':[[49,70.5],[77,70.8]]},
 'bastion-armor2':{'front':[[49,71],[79,71]],'back':[[49,69.5],[79,69.5]]},
 'bastion-armor3':{'front':[[48.8,69.5],[78.3,69.5]],'back':[[49,69],[79,70]]},
 'traqueuse-armor1':{'front':[[52,72],[75.8,72.2]],'back':[[51,72.5],[75.6,72.3]]},
 'traqueuse-armor2':{'front':[[52.7,74.8],[75.7,74]],'back':[[51.5,71.5],[76,72.5]]},
 'traqueuse-armor3':{'front':[[51.5,73.5],[76.5,73.5]],'back':[[51,74],[77.5,73]]},
 'arcaniste-armor1':{'front':[[50.5,68],[76.5,68.5]],'back':[[50.5,68.3],[76.8,69.2]]},
 'arcaniste-armor2':{'front':[[50.8,67.5],[77.3,68.2]],'back':[[50.5,69],[77,69]]},
 'arcaniste-armor3':{'front':[[50.5,68.7],[77,69.5]],'back':[[50.5,70],[77,70.5]]},
 'metamorphe-armor1':{'front':[[52.4,74],[76.3,74]],'back':[[51.5,74],[77,74]]},
 'metamorphe-armor2':{'front':[[51.3,72.5],[76,73]],'back':[[51.5,73],[77,73]]},
 'metamorphe-armor3':{'front':[[52.5,72.4],[76.5,72.5]],'back':[[51.5,71.5],[75.5,71.5]]},
}
LOADS={'dual-knives':[(70,False),(70,True)],'pistol-knife':[(104,False),(70,True)]}

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(p):return p.relative_to(ROOT).as_posix()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def side_of(left,back):return int(not left) if not back else int(left)
def rect(point,radius):
    x,y=[round(v)+64 for v in point]
    return (x-radius,y-radius,x+radius+1,y+radius+1)

def follow(reference,frame,point):
    x,y=point;ix,iy=round(x),round(y)
    sample=list(reference.crop((ix-3,iy-3,ix+4,iy+4)).get_flattened_data());scores=[]
    for dy in range(-2,3):
        for dx in range(-2,3):
            xx,yy=round(x+dx),round(y+dy)
            if frame.getpixel((xx,yy))[3]<128:continue
            patch=frame.crop((xx-3,yy-3,xx+4,yy+4))
            score=0.
            for a,b in zip(sample,patch.get_flattened_data()):
                score+=abs(a[3]-b[3])
                score+=sum(abs(a[c]*a[3]/255-b[c]*b[3]/255) for c in range(3))
            scores.append((score/(49*4)+2*(dx*dx+dy*dy),dx,dy))
    _,dx,dy=min(scores)
    return [round(x+dx,2),round(y+dy,2)]

def main():
    OUT.mkdir(exist_ok=True);PREVIEW.mkdir(exist_ok=True)
    composite=OUT/'composites';composite.mkdir(exist_ok=True)
    resources=PREVIEW/'resources';resources.mkdir(exist_ok=True)
    poses=read(ART/'armor-C10/conditioned/manifest.json');old=read(ART/'armor-C10/review-catalogue.json')
    recipes=read(ART/'extension-C06/conditioned-weapons/recipes.json')
    knife=read(ART/'candidate-C08/knife-angles.json');pistol=read(ART/'candidate-C09/pistol-angles.json')
    images={}
    def image(path):
        if path not in images:images[path]=Image.open(ROOT/path).convert('RGBA')
        return images[path]
    gear={k:image(recipes['headgear'][k]) for k in ['front','back','opening','ownership']}
    profiles=[];cases=[];hand_contact_checks=0;failures=[];reassembly=0
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    overview=Image.new('RGB',(960,680),'#1e2831');draw=ImageDraw.Draw(overview)
    for index,p in enumerate(poses['profiles']):
        oldp=next(q for q in old['profiles'] if q['id']==p['id']);pose=p['family_poses']['one']
        raw=[cell(image(pose['complete']),n) for n in range(13)];native=[im.crop((64,64,192,192)) for im in raw]
        frames=[]
        for n,frame in enumerate(native):
            back=6<=n<12;reference=6 if back else 12;direction='back' if back else 'front'
            grips=[list(v) if n==reference else follow(native[reference],frame,v) for v in PALMS[p['id']][direction]]
            assert all(frame.getpixel(tuple(round(v) for v in xy))[3]>=64 for xy in grips),(p['id'],n,grips)
            frames.append({**pose['frames'][n],'knife_grips':grips,'knife_foreground_rects':[rect(v,3) for v in grips]})
        # The complete original hand pixels move only between ownership layers.
        # Weapons in the other hand retain both their old target and old mask.
        layers={};hilts={}
        for model in range(68,80):
            for logical_left in [False,True]:
                weapon_frames=[]
                for n,f in enumerate(frames):
                    side=side_of(logical_left,6<=n<12);b=next(b for b in recipes['bindings'] if b['model']==model and b['left']==(side==0));source=image(b['palettes'][0]['file'])
                    r={'pivot':b['anchor'],'target':[v+64 for v in f['knife_grips'][side]],'scale':1.,'flip':False,'left_source':b['left'],'angle':0.}
                    r['angle']=knife_angle(model,r,knife);weapon=posed(source,r)
                    hilt=source.copy();box=source.getchannel('A').getbbox();hilt.paste((0,0,0,0),(0,0,hilt.width,box[3]-9))
                    hilt=posed(hilt,r);owned=Image.new('L',(256,256));owned.paste(raw[n].getchannel('A').crop(rect(f['knife_grips'][side],3)),rect(f['knife_grips'][side],3))
                    if not ImageChops.multiply(hilt.getchannel('A'),owned).getbbox():failures.append({'profile':p['id'],'model':model,'left':logical_left,'pose':n})
                    hand_contact_checks+=1;weapon_frames.append(weapon)
                if model==70:layers[(model,logical_left)]=weapon_frames
        # Pistol transform and contact are copied byte-for-byte from the C10 recipe.
        pistol_frames=[]
        for n,f in enumerate(frames):
            side=side_of(False,6<=n<12);b=next(b for b in recipes['bindings'] if b['model']==104 and b['left']==(side==0))
            r={'pivot':b['anchor'],'target':[v+64 for v in f['grips'][side]],'scale':1.,'flip':False,'left_source':b['left'],'angle':-math.pi/2 if side==0 else math.pi/2};r['angle']=pistol_angle(104,r,pistol)
            pistol_frames.append(posed(image(b['palettes'][0]['file']),r))
        layers[(104,False)]=pistol_frames
        entries=[]
        for loadout,load in LOADS.items():
            cores=[];hands=[]
            for n,f in enumerate(frames):
                core=raw[n].copy();foreground=Image.new('RGBA',(256,256))
                for model,left in load:
                    side=side_of(left,6<=n<12);point=f['knife_grips'][side] if model==70 else f['grips'][side];box=rect(point,3 if model==70 else 2)
                    foreground.paste(raw[n].crop(box),box[:2]);core.paste((0,0,0,0),box)
                assert Image.alpha_composite(core,foreground).tobytes()==raw[n].tobytes()
                reassembly+=1;cores.append(core);hands.append(foreground)
            for head in [0,115,116,117,118,119,120]:
                atlas=Image.new('RGBA',(1536,768))
                for n in range(13):
                    frame=cores[n].copy() if not head else helmet(cores[n],gear,pose['frames'][n]['head_anchor'],6<=n<12,head)
                    for binding in load:frame.alpha_composite(layers[binding][n])
                    frame.alpha_composite(hands[n]);atlas.paste(frame,(n%6*256,n//6*256))
                file=composite/f'{p["id"]}-{loadout}-H{head}-B.png';atlas.save(file);shutil.copy2(file,resources/file.name)
                a=next(c for c in oldp['cases'] if c['loadout']==loadout and c['head']==head)
                entries.append({'loadout':loadout,'head':head,'A':'../armor-C10/resources/'+Path(a['file']).name,'B':'resources/'+file.name,'case_A':a['case'],'case_B':file.stem})
                cases.append({'id':file.stem,'file':rel(file),'kind':'C11-corrected-knife-contact','side':256,'foot':[128,159],'save':head in [0,120],**({'camp_group':p['id'].split('-')[0]} if head==0 and loadout=='pistol-knife' else {})})
                if head==0:cases.append({'id':a['case']+'-A','file':a['file'],'kind':'C10-original-knife-contact','side':256,'foot':[128,159],'save':True})
        profiles.append({'id':p['id'],'number':p['number'],'label':oldp['label'],'frames':frames,'cases':entries})
        # One native-size side-by-side row for the current simple knife pair.
        a=image(next(c for c in oldp['cases'] if c['loadout']=='dual-knives' and c['head']==0)['file']);b=image(rel(composite/f'{p["id"]}-dual-knives-H0-B.png'))
        x=index%3*320;y=index//3*165+20;draw.text((x+8,y-16),oldp['label'],font=font,fill='white')
        for kind,sheet in enumerate([a,b]):
            draw.text((x+kind*160+10,y+4),'A · actuelle' if kind==0 else 'B · manche en main',font=font,fill='#b7c8d5' if kind==0 else '#ffcb7a')
            for slot,n in enumerate([12,6]):
                im=cell(sheet,n).crop((64,64,192,192));overview.paste(im,(x+kind*160-24+slot*72,y+24),im)
    assert not failures,failures
    write(OUT/'knife-attachments.json',{'review':'VERT18-C11','normal_game_enabled':False,'body_pixels_changed':False,'knife_models':list(range(68,80)),'below_horizontal_degrees':20,'profiles':[{k:p[k] for k in ['id','number','frames']} for p in profiles]})
    write(OUT/'engine-cases.json',{'review':'VERT18-C11','cases':cases})
    write(PREVIEW/'catalogue.json',{'review':'VERT18-C11','normal_game_enabled':False,'profiles':profiles,'source_art_unchanged':True})
    overview.save(PREVIEW/'C11-native64.png')
    for ext in ['html','js']:shutil.copy2(ROOT/f'tools/art/knife_contact18.{ext}',PREVIEW/('apercu.html' if ext=='html' else 'app.js'))
    write(DOC/'ANALYSE-C11.json',{'review':'VERT18-C11','status':'prepared_pending_engine_and_visual_review','normal_game_enabled':False,'body_profiles':12,'knife_models_contact_checked':12,'hilt_finger_contact_checks':hand_contact_checks,'hilt_finger_contact_failures':failures,'body_fingers_reassemblies_exact':reassembly,'original_body_pixels_changed':False,'knife_source_pixels_angles_and_scale_changed':False,'pistol_pixels_angle_and_target_changed':False,'candidate_compositions':168,'baseline_configurations':24,'engine_configurations':len(cases),'authored_hand_experiments':'Two builtin_imagegen sources retained for diagnosis; not consumed. C11-B corrects landmark and foreground ownership using the original C10 fingers.'})
    write(DOC/'candidate-resources-C11.json',{'review':'VERT18-C11','status':'unapproved_candidates','sha256':{rel(p):digest(p) for p in OUT.rglob('*') if p.is_file()}})
    print(json.dumps({'profiles':len(profiles),'hilt_contacts':hand_contact_checks,'candidate_atlases':168,'engine_configurations':len(cases),'body_pixels_unchanged':True}))

if __name__=='__main__':main()
