"""Check modular coverage and prepare helmet/dual native-engine witnesses."""
from pathlib import Path
import json,hashlib
from PIL import Image,ImageChops
from compose_vertical_extension18 import cell,transform,compose,ART,ROOT

def read(p):return json.loads(Path(p).read_text('utf8'))
def write(p,v):Path(p).write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n','utf8')

def helmet(image,gear,anchor,back,model):
    image=image.copy();x,y=[round(v)+64-16 for v in anchor]
    original=image.crop((x,y,x+32,y+32));mask=gear['ownership'].getchannel('A').point(lambda a:255 if a else 0)
    image.paste((0,0,0,0),(x,y,x+32,y+32),mask)
    cover=gear['back' if back else 'front'].crop(((model-115)*32,0,(model-114)*32,32))
    opening=gear['opening'].crop(((model-115)*32,0,(model-114)*32,32)).getchannel('A').point(lambda a:255 if a else 0)
    restore=ImageChops.multiply(mask,cover.getchannel('A').point(lambda a:255 if a<220 else 0)) if back and model in [115,119] else Image.new('L',(32,32)) if back else opening
    patch=image.crop((x,y,x+32,y+32));patch.paste(original,(0,0),restore);image.paste(patch,(x,y));image.alpha_composite(cover,(x,y))
    return image

def main():
    recipes=read(ART/'conditioned-weapons/recipes.json');poses=read(ART/'conditioned-weapons/manifest.json')
    assert len(recipes['profiles'])==8 and not recipes['checks']['contact_failures']
    bindings=recipes['bindings'];one=sorted({b['model'] for b in bindings if not b['two_handed']});assert len(one)==39
    palette_checks=0
    for b in bindings:
        alpha=None
        for p in b['palettes']:
            current=Image.open(ROOT/p['file']).getchannel('A').tobytes()
            if alpha is None:alpha=current
            assert current==alpha;palette_checks+=1
    gear={k:Image.open(ROOT/recipes['headgear'][k]).convert('RGBA') for k in ['front','back','opening','ownership']}
    out=ART/'conditioned-weapons/composites-variants';out.mkdir(exist_ok=True)
    extra=[];head_checks=0;dual_choices=0
    for profile in recipes['profiles']:
        pose=next(p for p in poses['profiles'] if p['id']==profile['profile'])
        assert len(profile['cases'])==152
        for main in one:
            for off in one:
                a=next(c for c in profile['cases'] if c['model']==main and not c['left'])
                b=next(c for c in profile['cases'] if c['model']==off and c['left'])
                assert a['family']==b['family']=='one' and not a['two_handed'] and not b['two_handed'];dual_choices+=1
        for model in [2,66,88,104]:
            case=next(c for c in profile['cases'] if c['model']==model and not c['left']);family=pose['family_poses'][case['family']]
            body=Image.open(ROOT/family['body']).convert('RGBA');hands=Image.open(ROOT/family['hands']).convert('RGBA')
            layers=[]
            for i,r in enumerate(case['frames']):
                b=next(b for b in bindings if b['model']==model and b['left']==r['left_source']);layers.append(transform(Image.open(ROOT/b['palettes'][0]['file']).convert('RGBA'),r))
            for head in range(115,121):
                atlas=Image.new('RGBA',(1536,768))
                for i,r in enumerate(case['frames']):
                    bare=cell(body,i);covered=helmet(bare,gear,family['frames'][i]['head_anchor'],6<=i<12,head)
                    # The head overlay must never create or erase any hand,
                    # sleeve, weapon or pixel below the shared shoulder row.
                    assert covered.crop((0,124,256,256)).tobytes()==bare.crop((0,124,256,256)).tobytes()
                    full=compose(covered,cell(hands,i),layers[i],r);atlas.paste(full,((i%6)*256,(i//6)*256));head_checks+=1
                path=out/f'{pose["id"]}-W{model:03}-H{head}.png';atlas.save(path)
                extra.append({'id':path.stem,'file':path.relative_to(ROOT).as_posix(),'kind':'helmet','side':256,'foot':[128,pose['family_poses']['one']['frames'][12]['bounds'][3]+64],'save':model==104})
        family=pose['family_poses']['one'];body=Image.open(ROOT/family['body']).convert('RGBA');hands=Image.open(ROOT/family['hands']).convert('RGBA')
        for pair in [(104,70),(104,104),(68,79)]:
            atlas=Image.new('RGBA',(1536,768))
            for i in range(13):
                full=cell(body,i)
                for left,model in [(False,pair[0]),(True,pair[1])]:
                    c=next(c for c in profile['cases'] if c['model']==model and c['left']==left);r=c['frames'][i]
                    b=next(b for b in bindings if b['model']==model and b['left']==r['left_source']);full.alpha_composite(transform(Image.open(ROOT/b['palettes'][0]['file']).convert('RGBA'),r))
                full.alpha_composite(cell(hands,i));atlas.paste(full,((i%6)*256,(i//6)*256))
            path=out/f'{pose["id"]}-DUAL-{pair[0]}-{pair[1]}.png';atlas.save(path)
            extra.append({'id':path.stem,'file':path.relative_to(ROOT).as_posix(),'kind':'dual','side':256,'foot':[128,family['frames'][12]['bounds'][3]+64],'save':True})
    write(ART/'extra-engine-cases.json',extra)
    report={'review':'VERT18-C06','normal_game_enabled':False,'weapon_binding_cases':1216,'binding_pose_cases':1216*13,'all_palette_alpha_checks':palette_checks,'dual_pair_choices_checked':dual_choices,'dual_coverage_method':'compositional: each two-layer choice references independently verified left/right whole-pose contacts; no extra body or arm pass','helmet_pose_checks_lower_body_unchanged':head_checks,'extra_engine_cases':len(extra),'normal_game_integration':False}
    write(ART/'modular-checks.json',report);print(json.dumps(report))

if __name__=='__main__':main()
