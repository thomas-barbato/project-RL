"""Review evidence: real renderer fixtures, native boards, rigid-weapon bounds."""
import base64,json,gzip
from pathlib import Path
import numpy as np
from PIL import Image,ImageOps,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-07';DOC=ROOT/'docs/reviews/player-walk-07'
SLUGS=['bastion','technomancien','traqueuse','arcaniste','metamorphe']

def over(dest,src):
    d=dest.astype(np.uint32);s=src.astype(np.uint32)
    a=s[:,:,3:4];old=d[:,:,3:4]*(255-a);total=a*255+old;safe=np.maximum(total,1)
    rgb=(s[:,:,:3]*a*255+d[:,:,:3]*old+total//2)//safe
    alpha=(total+127)//255
    return np.concatenate([rgb,alpha],axis=2).astype(np.uint8)

def rust_pose(folder,data,cfg,pose):
    result=np.zeros((128,128,4),dtype=np.uint8)
    for i,x,y in data['configs'][cfg]['frames'][pose]:
        sprite=ImageOps.flip(Image.open(folder/data['sprites'][i]['file']).convert('RGBA'))
        layer=Image.new('RGBA',(128,128));layer.paste(sprite,(x,y))
        result=over(result,np.array(layer))
    return result

fixtures=[];sources={};bounds=[];outfit_count=0;font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
board=Image.new('RGB',(5*256,5*152),'#293132');draw=ImageDraw.Draw(board)
repair_board=Image.new('RGB',(1024,432),'#293132');repair_draw=ImageDraw.Draw(repair_board)
family_board=Image.new('RGB',(512,5*432),'#293132');family_draw=ImageDraw.Draw(family_board)
meta_board=Image.new('RGB',(512,5*432),'#293132');meta_draw=ImageDraw.Draw(meta_board)
for ci,slug in enumerate(SLUGS):
    packed=json.loads((ART/f'preview-data/{slug}.json').read_text('utf8'))
    frames=json.loads(gzip.decompress(base64.b64decode(packed['framesGzip'])))
    atlas=Image.open(ART/f'preview-data/{slug}.webp').convert('RGBA')
    for rank,index in enumerate(packed.get('class_indices',list(range(ci*5,ci*5+5)))):
        outfit_count+=1;folder=ART/f'layered-final/CLASS-{index:02}'
        data=json.loads((folder/'layers.json').read_text('utf8'))
        if 10<=index<=14:
            for group,(cfg,name) in enumerate([(1,'pistolet'),(3,'fusil'),(4,'hache')]):
                for position,pose in enumerate([2,5,9,12]):
                    x=position*128;y=(index-10)*432+group*144
                    im=Image.fromarray(rust_pose(folder,data,cfg,pose));family_board.paste(im,(x,y+16),im)
                    family_draw.text((x+3,y+2),f'{index+1:02} / '+name+(' G' if pose<7 else ' D'),font=font,fill='#eeeade')
        if 20<=index<=24:
            for group,(cfg,name) in enumerate([(1,'pistolet'),(3,'fusil'),(4,'hache')]):
                for position,pose in enumerate([2,5,9,12]):
                    x=position*128;y=(index-20)*432+group*144
                    im=Image.fromarray(rust_pose(folder,data,cfg,pose));meta_board.paste(im,(x,y+16),im)
                    meta_draw.text((x+3,y+2),f'{index+1:02} / '+name+(' G' if pose<7 else ' D'),font=font,fill='#eeeade')
        if index in [10,24]:
            for group,(cfg,name) in enumerate([(1,'pistolet'),(3,'fusil'),(4,'hache')]):
                for position,pose in enumerate([2,5,9,12]):
                    x=([10,24].index(index)*4+position)*128;y=group*144
                    im=Image.fromarray(rust_pose(folder,data,cfg,pose))
                    repair_board.paste(im,(x,y+16),im)
                    repair_draw.text((x+3,y+2),f'{index+1:02} / '+name+(' G' if pose<7 else ' D'),font=font,fill='#eeeade')
        for side in range(2):
            rgba=rust_pose(folder,data,4,side*7+2)
            image=Image.fromarray(rgba);board.paste(image,(ci*256+side*128,rank*152+20),image)
        draw.text((ci*256+8,rank*152+3),f'{index+1:02} / CLASS-{index:02} / hache',font=font,fill='#eeeade')
        for cfg,poses in [(1,[0,3,7,10]),(4,[0,2,7,9]),(8,[1,6,8,13])]:
            for pose in poses:
                commands=frames[rank][cfg][pose]
                for group in commands:
                    for cmd in group:
                        key=f'{slug}:{cmd[0]}'
                        if key not in sources:
                            x,y,w,h=packed['rects'][cmd[0]]
                            sources[key]={'width':w,'height':h,'data':base64.b64encode(atlas.crop((x,y,x+w,y+h)).tobytes()).decode()}
                fixtures.append({'key':f'CLASS-{index:02}/{cfg}/{pose}','slug':slug,'commands':commands,
                    'expected':base64.b64encode(rust_pose(folder,data,cfg,pose).tobytes()).decode()})
        for cfg,poses in enumerate(frames[rank]):
            for pose,groups in enumerate(poses):
                for cmd in groups[3]:
                    ident,a,b,c,d,e,f=cmd;_,_,w,h=packed['rects'][ident]
                    corners=[(a*x+b*y+e,c*x+d*y+f) for x in [0,w] for y in [0,h]]
                    bb=[min(p[0] for p in corners),min(p[1] for p in corners),max(p[0] for p in corners),max(p[1] for p in corners)]
                    if bb[0]<0 or bb[1]<0 or bb[2]>128 or bb[3]>128:
                        bounds.append({'class':index,'configuration':cfg,'pose':pose,'bounds':bb})
board.save(ART/'all25-axe-native.png')
repair_board.save(ART/'cycle-repair-equipment-native.png')
family_board.save(ART/'r08-traqueuse-five-outfits-equipment-native.png')
meta_board.save(ART/'r09-metamorphe-five-outfits-equipment-native.png')
(ART/'raster-fixtures.json').write_text(json.dumps({'sources':sources,'fixtures':fixtures},separators=(',',':')),'utf8')
# Rebuild the complete native overview, not a stale partial generation.
overview=Image.new('RGB',(5*560,5*224),'#293132')
for index in range(25):overview.paste(Image.open(ART/f'CLASS-{index:02}-unarmed-native.png'),((index//5)*560,(index%5)*224))
overview.save(ART/'all25-unarmed-native-overview.png')
report={'raster_fixtures':len(fixtures),'source_crops':len(sources),'outfits':outfit_count,'rigid_weapon_pose_checks':outfit_count*117*14,
    'weapon_bounds_outside_128_canvas':bounds,'runtime_integration':False}
(DOC/'raster-audit.json').write_text(json.dumps(report,indent=2)+'\n','utf8')
print(json.dumps({**report,'weapon_bounds_outside_128_canvas':len(bounds)}))
