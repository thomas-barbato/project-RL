"""Freeze accepted C08-B geometry and independent reference compositions.

Uses a scalar, premultiplied bilinear sampler; no source artwork is repainted.
"""
from pathlib import Path
import hashlib,json,math
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'candidate-C08'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def angle(model,frame,definition):
    if not 68<=model<=79:return frame['angle']
    axis=definition['knife_axes'][f'{model}-{int(frame["left_source"])}']
    tilt=math.radians(definition['below_horizontal_degrees'])
    return (math.pi-tilt if frame['left_source'] else tilt)-axis

def apply(manifest,definition):
    changed=0
    for p in manifest['profiles']:
        for c in p['cases']:
            if not 68<=c['model']<=79:continue
            for f in c['frames']:f['angle']=angle(c['model'],f,definition);changed+=1
    assert changed==2496
    manifest['accepted_overrides']=['VERT18-C08-B']
    return changed

def posed(source,recipe):
    """Independent scalar renderer of the approved rigid transform."""
    if recipe['flip']:source=source.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
    box=source.getchannel('A').getbbox();out=Image.new('RGBA',(256,256))
    if not box:return out
    sn,cs=math.sin(recipe['angle']),math.cos(recipe['angle']);scale=recipe['scale'];px,py=recipe['pivot'];tx,ty=recipe['target']
    corners=[(tx+(cs*(x-px)-sn*(y-py))*scale,ty+(sn*(x-px)+cs*(y-py))*scale) for x in [box[0]-1,box[2]+1] for y in [box[1]-1,box[3]+1]]
    source_pixels=source.load();dest=out.load()
    for y in range(max(0,math.floor(min(p[1] for p in corners))),min(256,math.ceil(max(p[1] for p in corners))+1)):
        for x in range(max(0,math.floor(min(p[0] for p in corners))),min(256,math.ceil(max(p[0] for p in corners))+1)):
            dx,dy=x+.5-tx,y+.5-ty;sx=px+(cs*dx+sn*dy)/scale-.5;sy=py+(-sn*dx+cs*dy)/scale-.5
            ix,iy=math.floor(sx),math.floor(sy);fx,fy=sx-ix,sy-iy;a=0.;rgb=[0.,0.,0.]
            for xx,yy,w in [(ix,iy,(1-fx)*(1-fy)),(ix+1,iy,fx*(1-fy)),(ix,iy+1,(1-fx)*fy),(ix+1,iy+1,fx*fy)]:
                if 0<=xx<source.width and 0<=yy<source.height:
                    p=source_pixels[xx,yy];alpha=p[3]*w;a+=alpha
                    for c in range(3):rgb[c]+=p[c]*alpha
            if a:dest[x,y]=tuple(min(255,int(v/a+.5)) for v in rgb)+(min(255,int(a+.5)),)
    return out

def cell(sheet,n):return sheet.crop((n%6*256,n//6*256,(n%6+1)*256,(n//6+1)*256))

def main():
    accepted=read(DOC/'acceptance-C08.json');assert accepted['selected']=='B'
    for name,sha in read(DOC/'candidate-resources-C06.json')['sha256'].items():assert digest(ROOT/name)==sha,name
    bank=DOC/'candidate-resources-C08.json'
    if bank.exists():
        for name,sha in read(bank)['sha256'].items():assert digest(ROOT/name)==sha,name
    OUT.mkdir(parents=True,exist_ok=True)
    definition_path=OUT/'knife-angles.json'
    if definition_path.exists():
        definition=read(definition_path)
        assert digest(definition_path)==accepted['approved_geometry_sha256']
    else:
        preview=read(ROOT/'artifacts/vertical-walk-review-18/current/knife-C08/catalogue.json')
        definition={'review':'VERT18-C08','selected':'B','below_horizontal_degrees':20,'knife_axes':preview['knife_axes'],'knife_models':list(range(68,80)),'body_profiles':[p['id'] for p in preview['poses']['profiles']],'source_bank':'candidate-resources-C06.json'}
        write(definition_path,definition)
    recipes=read(ART/'extension-C06/conditioned-weapons/recipes.json');poses=read(ART/'extension-C06/conditioned-weapons/manifest.json')
    refs=OUT/'composites-original';refs.mkdir(exist_ok=True);images={}
    def image(path):
        if path not in images:images[path]=Image.open(ROOT/path).convert('RGBA')
        return images[path]
    count=0
    for p in poses['profiles']:
        source=p['family_poses']['one'];body,hands=image(source['body']),image(source['hands']);cases=next(q for q in recipes['profiles'] if q['profile']==p['id'])['cases']
        for main,left in [(n,None) for n in range(68,80)]+[(104,70),(68,79)]:
            name=f'{p["id"]}-W{main:03}' if left is None else f'{p["id"]}-DUAL-{main}-{left}'
            file=refs/(name+'.png')
            if not file.exists():
                atlas=Image.new('RGBA',(1536,768))
                for n in range(13):
                    frame=cell(body,n)
                    for model,is_left in [(main,False)]+([] if left is None else [(left,True)]):
                        case=next(c for c in cases if c['model']==model and c['left']==is_left);r=dict(case['frames'][n]);r['angle']=angle(model,r,definition)
                        binding=next(b for b in recipes['bindings'] if b['model']==model and b['left']==r['left_source'])
                        frame.alpha_composite(posed(image(binding['palettes'][0]['file']),r))
                    frame.alpha_composite(cell(hands,n));atlas.paste(frame,(n%6*256,n//6*256))
                atlas.save(file)
            count+=1
    assert count==112
    sha={p.relative_to(ROOT).as_posix():digest(p) for p in sorted(OUT.rglob('*')) if p.is_file()}
    write(DOC/'candidate-resources-C08.json',{'review':'VERT18-C08','selected':'B','source_art_redrawn':False,'reference_atlases':count,'sha256':sha})
    accepted.update(approved_geometry=definition_path.relative_to(ROOT).as_posix(),approved_geometry_sha256=digest(definition_path),candidate_resources='candidate-resources-C08.json');write(DOC/'acceptance-C08.json',accepted)
    print(json.dumps({'review':'VERT18-C08','selected':'B','reference_atlases':count,'whole_reference_poses':count*13,'frozen_files':len(sha)}))

if __name__=='__main__':main()
