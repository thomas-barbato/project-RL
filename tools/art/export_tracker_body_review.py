"""107 / D58-BODY01: condition the new source for a logical 64px tile."""
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
REVIEW=BASE/'skill-animation-review-96'
OLD=BASE/'skill-animation-review-93/traqueur-palette02/D58-PAL02-native64.png'

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def uniform_blocks(im):
    same=count=0
    for y in range(0,im.height-1,2):
        for x in range(0,im.width-1,2):
            pixels=[im.getpixel((x+dx,y+dy)) for dx,dy in [(0,0),(1,0),(0,1),(1,1)]]
            if any(p[3]>=8 for p in pixels):count+=1;same+=int(all(p==pixels[0] for p in pixels))
    return {'occupied':count,'uniform':same,'uniform_percent':round(100*same/max(1,count),2)}

def main():
    assert not (REVIEW/'visual-acceptance-01.json').exists()
    source_path=REVIEW/'D58-BODY01-source.png';source=Image.open(source_path).convert('RGBA')
    bounds=source.getchannel('A').point(lambda a:255 if a>=8 else 0).getbbox()
    assert bounds and source.getpixel((0,0))[3]==0
    crop=source.crop(bounds)
    height=50;width=round(crop.width*height/crop.height)
    assert 50<=width<=76, (width,height)
    body=crop.resize((width,height),Image.Resampling.LANCZOS)
    native=Image.new('RGBA',(128,128));native.alpha_composite(body,((128-width)//2,96-height))
    dest=REVIEW/'D58-BODY01-native128.png';native.save(dest)
    old=Image.open(OLD).convert('RGBA')
    before=Image.new('RGBA',(128,128));before.alpha_composite(old,(32,32))
    proof={'date':'2026-10-15','candidate':'D58-BODY01','number':107,'status':'visual_proposal_not_accepted',
        'source_size':source.size,'source_alpha8_bounds':bounds,'native_canvas':[128,128],'logical_tile':[64,64],
        'body_size':[width,height],'body_alpha8_bounds':native.getchannel('A').point(lambda a:255 if a>=8 else 0).getbbox(),
        'direct_high_resolution_reduction':True,'source_sha256':sha(source_path),'native_sha256':sha(dest),'old_sha256':sha(OLD),
        'previous_repeated_2x2_blocks':uniform_blocks(old),'new_repeated_2x2_blocks':uniform_blocks(native),
        'contact_shadow':'Separate layer in the review scene; not baked into the new body.',
        'gameplay_or_collision_changed':False,'active_asset_or_animation_changed':False}
    assert proof['previous_repeated_2x2_blocks']['uniform_percent']==100
    assert proof['new_repeated_2x2_blocks']['uniform_percent']<25
    board=Image.new('RGB',(720,280),'#131a20');d=ImageDraw.Draw(board)
    d.text((14,10),'107 / Traqueur - comparaison native, case 64 px',fill='#e6e8e9')
    for col,(title,bg) in enumerate([('Sombre','#22282d'),('Clair','#c4c2b4'),('Texture','#514d46')]):
        x=14+col*236;d.text((x,34),title,fill='#c2cbd0')
        for row,(label,im) in enumerate([('Actuel / 32 px agrandi',before),('107 / dessin direct',native)]):
            y=62+row*108;d.text((x,y-16),label,fill='#dce2e5');d.rectangle((x,y,x+213,y+71),fill=bg)
            if col==2:
                for yy in range(y,y+72,7):
                    for xx in range(x,x+214,11):d.rectangle((xx,yy,xx+2,yy+1),fill='#737065')
            d.rectangle((x+72,y+4,x+135,y+67),outline='#9b8b6d')
            board.paste(im,(x+104-64,y+36-64),im)
    board.save(REVIEW/'D58-BODY01-comparison-native.png')
    # Optional enlarged views use nearest, so native pixel information stays visible.
    zoom=Image.new('RGB',(1024,512),'#131a20')
    zoom.paste(before.resize((512,512),Image.Resampling.NEAREST),(0,0),before.resize((512,512),Image.Resampling.NEAREST))
    zoom.paste(native.resize((512,512),Image.Resampling.NEAREST),(512,0),native.resize((512,512),Image.Resampling.NEAREST))
    zoom.save(REVIEW/'D58-BODY01-comparison-zoom4.png')
    proof['comparison_native_sha256']=sha(REVIEW/'D58-BODY01-comparison-native.png')
    (REVIEW/'export-verification.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'source':source.size,'bounds':bounds,'body':[width,height],'new_block_uniform_percent':proof['new_repeated_2x2_blocks']['uniform_percent']}))

if __name__=='__main__':main()
