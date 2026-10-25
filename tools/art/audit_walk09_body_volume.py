"""Compare existing front and lateral whole figures, without changing assets."""
from pathlib import Path
import json
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'docs/reviews/player-walk-09/volume-audit'

def bounds(image):
    return image.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()

def measure(image):
    x0,y0,x1,y1 = bounds(image)
    height = y1-y0
    alpha = image.getchannel('A')
    area = sum(a >= 96 for a in alpha.getdata())
    a,b = round(y0+height*.27), round(y0+height*.55)
    chest_area = sum(alpha.getpixel((x,y)) >= 96 for y in range(a,b) for x in range(128))
    return dict(bounds=[x0,y0,x1,y1],height=height,width=x1-x0,opaque_area=area,
                upper_body_mean_occupied_width=round(chest_area/(b-a),2))

def main():
    OUT.mkdir(exist_ok=True)
    manifest=json.loads((ROOT/'assets/animations/walk-09/player/manifest.json').read_text())
    base=Image.open(ROOT/'assets/sprites/class-armors-01/base.png').convert('RGBA')
    groups=['Bastion','Technomancien','Traqueuse','Arcaniste','Metamorphe']
    results=[]
    for cls,name in enumerate(groups):
        board=Image.new('RGB',(1024,900),'#253136');draw=ImageDraw.Draw(board)
        draw.text((12,8),f'{name} / CURRENT FRONT vs CURRENT WALK / native64 and x2',fill='white')
        for rank in range(5):
            index=cls*5+rank;x=index%16*128;y=index//16*128
            front=base.crop((x,y,x+128,y+128));fm=measure(front)
            front.save(OUT/f'CLASS-{index:02}-front.png')
            figures=[front];labels=['front']
            comparisons=[]
            for facing,frame,label in [(-1,8,'left idle'),(-1,1,'left passing'),(1,8,'right idle'),(1,1,'right passing')]:
                e=next(e for e in manifest if e['id']==f'CLASS-{index:02}' and e['facing']==facing and e['frame']==frame)
                sheet=Image.open(ROOT/e['file']).convert('RGBA');im=Image.new('RGBA',(128,128));yy=e['sheet_row']*128
                for part in [0,1,3,2,4]:im.alpha_composite(sheet.crop((part*128,yy,(part+1)*128,yy+128)))
                figures.append(im);labels.append(label);m=measure(im)
                comparisons.append(dict(facing=facing,frame=frame,**m,
                    height_ratio=round(m['height']/fm['height'],3),area_ratio=round(m['opaque_area']/fm['opaque_area'],3),
                    upper_body_width_ratio=round(m['upper_body_mean_occupied_width']/fm['upper_body_mean_occupied_width'],3)))
                im.save(OUT/f'CLASS-{index:02}-{label.replace(" ","-")}.png')
            results.append(dict(id=f'CLASS-{index:02}',name=name,outfit=rank+1,front=fm,lateral=comparisons))
            yy=30+rank*172
            for col,(im,label) in enumerate(zip(figures,labels)):
                native=im.crop((32,32,96,96));xx=12+col*202
                draw.text((xx,yy),f'{rank+1}/5 {label}',fill='white')
                board.paste(native,(xx,yy+30),native)
                zoom=native.resize((128,128),Image.Resampling.NEAREST)
                board.paste(zoom,(xx+68,yy+24),zoom)
        board.save(OUT/f'{name}-front-vs-walk.png')
    (OUT/'metrics.json').write_text(json.dumps({'method':'alpha >=96, unarmed whole figure; upper band 27..55 percent of each silhouette height; visible 2D mass, not physical volume','runtime_assets_modified':False,'outfits':results},indent=2)+'\n')
    for r in results:
        print(r['id'],r['name'],r['outfit'],'front H',r['front']['height'],'walk H',[a['height'] for a in r['lateral']],
              'upper width ratios',[a['upper_body_width_ratio'] for a in r['lateral']])

if __name__=='__main__':main()
