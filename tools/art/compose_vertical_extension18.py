"""C06 rigid shared-weapon recipes and reproducible complete native figures.

No body deformation, per-frame scaling or runtime drawing-file dependency.
The browser and the isolated engine fixture consume these same recipes.
"""
from pathlib import Path
import json,math,re,shutil
import numpy as np
from PIL import Image,ImageDraw,ImageFont
from pack_vertical_weapons18 import posed

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18/extension-C06'
OUT=ART/'conditioned-weapons'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06'
S=256

def read(path):return json.loads(Path(path).read_text('utf8'))
def write(path,value):Path(path).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n','utf8')

def fixed_scale(profile,model):
    kind=profile.removesuffix('-lourde').removesuffix('-lourd')
    index={'bastion':0,'traqueuse':2,'arcaniste':3,'metamorphe':4}[kind]
    source=(ROOT/'src/game_art_weapon_readability_catalog.rs').read_text('utf8')
    for block in source.split('ReviewedWeapon {')[1:]:
        if int(re.search(r'model: (\d+)',block)[1])==model:
            return float(re.search(r'scales: \[([^]]+)',block)[1].split(',')[index])
    if model==67:return [1,.95,.88,.92,.96][index]
    return [0.78,0.70,0.64,0.68,0.72][index] if model<=60 else 1.

def recipe(profile,binding,frame,family):
    back=frame['direction']=='back';model=binding['model']
    pivot=binding['anchor'][:];support=binding['support'];flip=False
    if binding['two_handed']:
        scale=fixed_scale(profile,model)
        if back:flip=True;pivot[0]=128-pivot[0];support=[128-support[0],support[1]]
        a,b=np.array(frame['grips'],float)
        delta=np.array(support)-pivot;target_delta=b-a
        angle=math.atan2(target_delta[1],target_delta[0])-math.atan2(delta[1],delta[0])
        center=(a+b)/2+64
        target=center-(np.array(support)-pivot)*scale/2 @ np.array([[math.cos(angle),math.sin(angle)],[-math.sin(angle),math.cos(angle)]])
        # Symmetric placement shares a subpixel mismatch between both fists;
        # weapon length never follows the changing palm separation.
    else:
        screen_side=int(not binding['left']);screen_side=1-screen_side if back else screen_side
        target=np.array(frame['grips'][screen_side])+64;scale=1.
        angle=(-math.pi/2 if screen_side==0 else math.pi/2) if model>=101 else 0.
    return {'pivot':pivot,'target':list(target),'scale':scale,'angle':angle,'flip':flip,'behind_body':binding['two_handed'] and back}

def transform(source,r):
    if r['flip']:source=source.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
    return posed(source,r['pivot'],r['target'],r['scale'],r['angle'])

def cell(sheet,i):return sheet.crop(((i%6)*S,(i//6)*S,(i%6+1)*S,(i//6+1)*S))

def compose(body,hands,weapon,r):
    out=Image.new('RGBA',(S,S))
    for im in ([weapon,body,hands] if r['behind_body'] else [body,weapon,hands]):out.alpha_composite(im)
    return out

def contact_distance(source,r,fingers,point):
    # Compare actual opaque weapon pixels and the owned fist pixels, not just
    # mathematical control points. Hidden rear grips are reported separately.
    xy=np.argwhere(np.asarray(source)[:,:,3]>=32)[:,::-1].astype(float)+.5
    if r['flip']:xy[:,0]=128-xy[:,0]
    xy-=r['pivot'];a=r['angle'];xy=xy@np.array([[math.cos(a),math.sin(a)],[-math.sin(a),math.cos(a)]])*r['scale']+r['target']-.5
    px,py=[round(v+64) for v in point]
    owned=np.argwhere(np.asarray(fingers)[py-2:py+3,px-2:px+3,3]>=32)[:,::-1]+[px-2,py-2]
    if not len(owned):return None
    return round(float(np.sqrt(((xy[:,None,:]-owned[None,:,:])**2).sum(2)).min()),3)

def main():
    data=read(OUT/'manifest.json');export=read(ROOT/'artifacts/vertical-walk-review-18/current/extension-catalog/weapon-palettes/manifest.json')
    refs=ART/'references/weapon-palettes';refs.mkdir(parents=True,exist_ok=True)
    for b in export['bindings']:
        for c in b['palettes']:
            p=Path(c['file']);dest=refs/p.name;shutil.copy2(ROOT/p,dest);c['file']=dest.relative_to(ROOT).as_posix()
    write(refs/'manifest.json',export)
    # Human-readable model names are taken from the current content catalogue.
    catalog=(ROOT/'src/game_art_catalog.rs').read_text('utf8').split('pub(super) fn held_weapon')[1].split('pub(super) fn weapon_hold')[0]
    id_models={ident:int(n) for ident,n in re.findall(r'"([^"]+)"\s*=>\s*(\d+)',catalog)}
    locale={}
    for p in (ROOT/'content/core/locales').rglob('*.json5'):locale.update(dict(re.findall(r'"([^"]+)"\s*:\s*"([^"\n]+)"',p.read_text('utf8'))))
    names={}
    for p in (ROOT/'content/core/weapons').glob('*.json5'):
        text=p.read_text('utf8');ident=re.search(r'\bid"?\s*:\s*"([^"]+)"',text);name=re.search(r'\bname_key"?\s*:\s*"([^"]+)"',text)
        if ident and name and ident[1] in id_models:names[id_models[ident[1]]]=locale.get(name[1],ident[1].removeprefix('core:'))
    entries=[];checks=[];max_distance=0;missing=[]
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    capture=PREVIEW/'prepared-engine';capture.mkdir(exist_ok=True)
    for p in data['profiles']:
        cache={fam:(Image.open(ROOT/v['body']).convert('RGBA'),Image.open(ROOT/v['hands']).convert('RGBA')) for fam,v in p['family_poses'].items()}
        cases=[]
        for b in export['bindings']:
            model=b['model'];family='one' if not b['two_handed'] else 'ranged' if model<=60 else 'melee-close' if 88<=model<=93 else 'melee-wide'
            source=Image.open(ROOT/b['palettes'][0]['file']).convert('RGBA')
            frames=p['family_poses'][family]['frames'];recipes=[]
            for i,f in enumerate(frames):
                used=b
                if not b['two_handed']:
                    screen_side=int(not b['left']);screen_side=1-screen_side if f['direction']=='back' else screen_side
                    used=next(q for q in export['bindings'] if q['model']==model and q['left']==(screen_side==0))
                r=recipe(p['id'],b,f,family)
                if not b['two_handed']:r['pivot']=used['anchor']
                r['left_source']=used['left'];recipes.append(r)
                if not r['behind_body']:
                    hand=cell(cache[family][1],i)
                    src=Image.open(ROOT/used['palettes'][0]['file']).convert('RGBA')
                    points=f['grips'] if b['two_handed'] else [f['grips'][(1-int(not b['left'])) if f['direction']=='back' else int(not b['left'])]]
                    for point in points:
                        d=contact_distance(src,r,hand,point)
                        if d is None or d>2.25:missing.append({'profile':p['id'],'model':model,'left':b['left'],'frame':i,'distance':d})
                        if d is not None:max_distance=max(max_distance,d)
            scales={r['scale'] for r in recipes};assert len(scales)==1
            c={'model':model,'left':b['left'],'family':family,'name':names.get(model,f'Arme {model:03}'),'two_handed':b['two_handed'],'frames':recipes}
            cases.append(c)
            # One complete original-color atlas per representative model for
            # the native engine fixture; not read by the ordinary game.
            if not b['left']:
                file=capture/f'{p["id"]}-W{model:03}.png'
                if not file.exists():
                    atlas=Image.new('RGBA',(S*6,S*3))
                    for i,r in enumerate(recipes):
                        used=next(q for q in export['bindings'] if q['model']==model and q['left']==r['left_source'])
                        src=Image.open(ROOT/used['palettes'][0]['file']).convert('RGBA')
                        im=compose(cell(cache[family][0],i),cell(cache[family][1],i),transform(src,r),r)
                        atlas.paste(im,((i%6)*S,(i//6)*S))
                    atlas.save(file)
                foot=p['family_poses']['one']['frames'][12]['bounds'][3]+64
                checks.append({'id':p['id']+f'-W{model:03}','file':file.relative_to(ROOT).as_posix(),'kind':'weapon','model':model,'side':S,'foot':[128,foot]})
        entries.append({'profile':p['id'],'cases':cases})
        # Contact sheets cover every weapon model on each accepted C05 body.
        for start in range(0,113,16):
            group=[c for c in cases if not c['left']][start:start+16]
            board=Image.new('RGB',(768,len(group)*112//2+36),'#1e2831');draw=ImageDraw.Draw(board)
            for k,c in enumerate(group):
                atlas=Image.open(capture/f'{p["id"]}-W{c["model"]:03}.png').convert('RGBA');x=(k%2)*384;y=(k//2)*112
                draw.text((x+3,y+3),f'W{c["model"]:03} / '+c['name'],font=font,fill='white')
                for col,n in enumerate([12,1,8]):
                    im=cell(atlas,n).crop((64,75,192,171));board.paste(im,(x+col*128,y+17),im)
            board.save(PREVIEW/f'weapons-{p["id"]}-{start//16:02}.png')
    manifest={'review':'VERT18-C06','native_tile_px':64,'normal_game_enabled':False,'bindings':export['bindings'],'profiles':entries,'fixed_weapon_scale_through_cycle':True,'headgear':read(ROOT/'assets/source-art/vertical-walk-review-18/candidate-C04/headgear.json'),'checks':{'weapon_models':113,'profile_count':len(entries),'binding_cases':sum(len(p['cases']) for p in entries),'original_color_contact_max_px':max_distance,'contact_failures':missing,'contact_test_limit_px':2.25,'ordered_dual_pairs_per_profile':39*39},'engine_cases':checks}
    write(OUT/'recipes.json',manifest)
    print(json.dumps(manifest['checks']))

if __name__=='__main__':main()
