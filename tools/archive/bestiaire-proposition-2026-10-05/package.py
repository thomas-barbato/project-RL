"""Non-destructive review packaging: alpha cutoff, crop, nearest fit, closed palette.

All drawing comes from imagegen. No repainting or procedural replacement art.
Active gameplay/art assets are read only and fingerprinted before/after.
"""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import json, hashlib, math, shutil

ROOT=Path(__file__).resolve().parents[2]; OUT=Path(__file__).resolve().parent
CAT=json.loads((OUT/'catalog.json').read_text(encoding='utf-8'))
PREVIOUS={e['number']:e for e in json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))} if (OUT/'manifest.json').exists() else {}
F=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',14)
SMALL=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',11)

def encoded(im,budget=24):
    alpha=im.getchannel('A')
    # Quantize occupied pixels only: a black background must not consume colors.
    colors=[p[:3] for p in im.get_flattened_data() if p[3]]
    sample=Image.new('RGB',(len(colors),1));sample.putdata(colors)
    palette=sample.quantize(budget,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE)
    result=im.convert('RGB').quantize(palette=palette,dither=Image.Dither.NONE).convert('RGBA')
    result.putalpha(alpha);return result

def condition(im,side,maximum,corpse=False):
    im=im.convert('RGBA')
    im.putalpha(im.getchannel('A').point(lambda a:255 if a>=128 else 0))
    bbox=im.getbbox()
    if not bbox:raise ValueError('Empty source')
    im=im.crop(bbox)
    scale=min(maximum[0]/im.width,maximum[1]/im.height)
    size=(max(1,round(im.width*scale)),max(1,round(im.height*scale)))
    image=im.resize(size,Image.Resampling.NEAREST)
    tile=Image.new('RGBA',(side,side)); anchor=(side//2,side-2)
    tile.alpha_composite(image,((side-size[0])//2,side-size[1]-1))
    return encoded(tile,24 if side==32 else 24),dict(source_bounds=list(bbox),native_bounds=list(tile.getbbox()),foot_anchor=list(anchor),resampling='nearest',alpha='binary cutoff 128',palette_limit=24)

def pair_subjects(raw):
    """Assign WHOLE connected source subjects before cropping the two panels.

    Generated silhouettes can cross the nominal midpoint. Cropping first admits
    a piece of the neighboring corpse into the living tile (seen on #26).
    Preserve intentional empty holes and disconnected pool clusters. Only tiny
    source cutout debris is excluded; no pixels are painted or recolored.
    """
    raw=raw.convert('RGBA');w,h=raw.size
    remaining=bytearray(a>=128 for a in raw.getchannel('A').get_flattened_data())
    groups=[[],[]]
    for start in range(w*h):
        if not remaining[start]:continue
        remaining[start]=0;stack=[start];points=[];sx=0
        while stack:
            point=stack.pop();points.append(point);x,y=point%w,point//w;sx+=x
            for yy in range(max(0,y-1),min(h,y+2)):
                for xx in range(max(0,x-1),min(w,x+2)):
                    p=yy*w+xx
                    if remaining[p]:remaining[p]=0;stack.append(p)
        if len(points)>=32:groups[int(sx/len(points)>=w/2)].append(points)
    result=[];source=raw.load()
    for parts in groups:
        largest=max(map(len,parts));parts=[part for part in parts if len(part)>=max(32,largest*.002)]
        points=[p for part in parts for p in part]
        l=min(p%w for p in points);r=max(p%w for p in points)+1
        t=min(p//w for p in points);b=max(p//w for p in points)+1
        im=Image.new('RGBA',(r-l,b-t));target=im.load()
        for p in points:
            x,y=p%w,p//w;target[x-l,y-t]=source[x,y][:3]+(255,)
        result.append(im)
    return result

def stamp(canvas,im,xy):canvas.paste(im,xy,im)
def board(entries,name):
    cols=4;cellw=304;cellh=234
    board=Image.new('RGB',(cols*cellw,72+math.ceil(len(entries)/cols)*cellh),'#121b22');d=ImageDraw.Draw(board)
    d.text((14,12),name,font=F,fill='#e5eee9')
    d.text((14,37),'Petit = taille native ; grand = zoom entier. Vivant / cadavre avec sang.',font=SMALL,fill='#a2b8c3')
    for i,e in enumerate(entries):
        x=i%cols*cellw;y=72+i//cols*cellh
        d.rectangle((x+4,y+4,x+cellw-5,y+cellh-5),fill='#202b33')
        d.text((x+12,y+12),f'{e["number"]:02}  {e["name"]}',font=F,fill='#9ed7cd')
        d.text((x+12,y+34),e['family']+' / '+str(e['tile_size'])+' px',font=SMALL,fill='#9db4c0')
        a=Image.open(OUT/e['living']).convert('RGBA');b=Image.open(OUT/e['corpse']).convert('RGBA')
        side=e['tile_size'];zoom=3 if side==32 else 2
        stamp(board,a,(x+20,y+67));stamp(board,b,(x+96,y+67))
        stamp(board,a.resize((side*zoom,side*zoom),Image.Resampling.NEAREST),(x+12,y+103))
        stamp(board,b.resize((side*zoom,side*zoom),Image.Resampling.NEAREST),(x+158,y+103))
    board.save(OUT/f'review/{name}.png')

def run():
    complete=[]
    for e in CAT:
        if e['number'] in (14,16):
            for revision in (2,3):
                revised=f'sources/{e["number"]:03}-r{revision}.png'
                if (OUT/revised).exists():e['source']=revised;e['revision']=revision
        path=OUT/e['source']
        if not path.exists():continue
        source_hash=hashlib.sha256(path.read_bytes()).hexdigest()
        cached=PREVIOUS.get(e['number'],{})
        if cached.get('packaging_version')==2 and cached.get('source_sha256')==source_hash and cached.get('source')==e['source'] and all((OUT/cached[key]).exists() for key in ('living','corpse')):
            complete.append(cached);continue
        raw=Image.open(path).convert('RGBA');side=e['tile_size'];n=e['number']
        if n<=32:
            left,right=pair_subjects(raw)
            living,lmeta=condition(left,side,e['maximum'])
            living.save(OUT/e['living']);e['living_encoding']=lmeta
            maximum=(side-2,round(side*.50))
        else:right=raw;maximum=e['maximum']
        corpse,cmeta=condition(right,side,maximum,True)
        corpse.save(OUT/e['corpse']);e['corpse_encoding']=cmeta
        e['corpse_includes_blood']=True
        e['packaging_version']=2;e['source_sha256']=source_hash
        e['blood_layer_status']='Authored pool included in review composite; separate reusable pool layer is reserved for integration after art approval.'
        e.pop('blood',None)
        for key in ('living','corpse'):
            im=Image.open(OUT/e[key]).convert('RGBA')
            opaque=[p for p in im.get_flattened_data() if p[3]]
            assert im.size==(side,side) and opaque
            assert set(im.getchannel('A').get_flattened_data())<={0,255}
            assert len({p[:3] for p in opaque})<=24
            e[key+'_colors']=len({p[:3] for p in opaque})
        if n>=33:
            assert hashlib.sha256((OUT/e['living']).read_bytes()).hexdigest()==e['living_sha256']
        complete.append(e)
    (OUT/'manifest.json').write_text(json.dumps(complete,ensure_ascii=False,indent=2),encoding='utf-8')
    for family in dict.fromkeys(e['family'] for e in complete):
        entries=[e for e in complete if e['family']==family]
        board(entries,f'{entries[0]["number"]:02}-{entries[-1]["number"]:02}')
    ready={e['number'] for e in complete}
    print(json.dumps({'complete':len(complete),'missing':[e['number'] for e in CAT if e['number'] not in ready]},ensure_ascii=False))

if __name__=='__main__':run()
