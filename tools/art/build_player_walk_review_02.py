"""Package proposed carry poses and reuse existing walk legs for visual review."""
import base64
import hashlib
import json
import shutil
from pathlib import Path
from PIL import Image, ImageDraw
from build_player_walk_review_04 import animated

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-02'
DOC=ROOT/'docs/reviews/player-walk-02'
SOURCE=ROOT/'assets/source-art/player-walk-review-02'
VIS=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/player-walk-armed.html')
ROWS=[7,0,1]


def uri(path):
    return 'data:image/png;base64,'+base64.b64encode(path.read_bytes()).decode('ascii')


def normalized(im):
    box=im.getchannel('A').point(lambda a:255 if a>=48 else 0).getbbox()
    scale=60/(box[3]-box[1])
    # The copper stripe at the top of the helmet registers the body centre;
    # centering the weapon's overall bounds would shift the actor sideways.
    points=[]
    for y in range(box[1],round(box[1]+(box[3]-box[1])*.16)):
        for x in range(round(im.width*.25),round(im.width*.75)):
            r,g,b,a=im.getpixel((x,y))
            if a>=128 and r>90 and r>g*1.45 and r>b*1.8:
                points.append((x,a))
    center=sum(x*a for x,a in points)/sum(a for _,a in points) if points else im.width/2
    reduced=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.LANCZOS)
    out=Image.new('RGBA',(128,128))
    out.alpha_composite(reduced,(round(64-center*scale),round(95-box[3]*scale)))
    return out


def main():
    for directory in [ART,DOC,SOURCE,VIS.parent]:directory.mkdir(parents=True,exist_ok=True)
    walk=Image.open(ROOT/'artifacts/player-walk-review-01/bastion-walk-native64.png').convert('RGBA')
    idle=Image.open(ROOT/'artifacts/player-walk-review-01/bastion-idle-native64.png').convert('RGBA')
    originals=[]
    for row in ROWS:
        poses=[idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64))]
        poses += [walk.crop((frame*64,row*64,frame*64+64,row*64+64)) for frame in range(6)]
        originals.append(poses)
    entries=[{'id':'none','number':0,'label':'Sans arme · référence'}]+json.loads((DOC/'inputs.json').read_text('utf-8'))
    all_data=[]
    native_board=Image.new('RGB',(3*160,len(entries)*130),'#263134')
    draw=ImageDraw.Draw(native_board)
    for n,entry in enumerate(entries):
        if entry['id']=='none':
            bases=[]
            for poses in originals:
                im=Image.new('RGBA',(128,128));im.alpha_composite(poses[0],(32,32));bases.append(im)
        else:
            path=SOURCE/f"WA02-{entry['number']:02}-{entry['id']}-source.png"
            if not path.exists():shutil.copy2(entry['generated'],path)
            source=Image.open(path).convert('RGBA')
            bases=[normalized(source.crop((round(col*source.width/3),0,round((col+1)*source.width/3),source.height))) for col in range(3)]
        atlas=Image.new('RGBA',(7*128,3*128))
        for direction,base in enumerate(bases):
            for frame in range(7):
                if entry['id']=='none':
                    out=Image.new('RGBA',(128,128));out.alpha_composite(originals[direction][frame],(32,32))
                else:
                    # Preserve the complete painted arms, grips, shaft and
                    # blade. No rectangular crop through the carried object.
                    frames=animated(base,'heavy',(64,95),equipped=True)
                    out=frames[8 if frame==0 else round((frame-1)*7/5)].copy()
                assert out.getbbox() is not None
                strong=out.getchannel('A').point(lambda a:255 if a>=48 else 0).getbbox()
                assert strong[0]>0 and strong[1]>0 and strong[2]<128 and strong[3]<128
                atlas.alpha_composite(out,(frame*128,direction*128))
            native_board.paste(bases[direction],(direction*160+16,n*130),bases[direction])
        draw.text((5,n*130+112),entry['label'],fill='white')
        dest=ART/f"{entry['id']}-atlas-native.png";atlas.save(dest,optimize=True)
        all_data.append({'id':entry['id'],'number':entry['number'],'label':entry['label'],'image':uri(dest)})
    native_board.save(ART/'equipped-poses-native.png',optimize=True)
    payload=json.dumps({'equipment':all_data},ensure_ascii=False,separators=(',',':'))
    template=(ROOT/'tools/art/player_walk_review_02.html').read_text('utf-8')
    html=template.replace('__ARMED_DATA__',payload)
    assert len(html.encode('utf-8'))<1_000_000
    VIS.write_text(html,encoding='utf-8')
    (ART/'fragment.html').write_text(html,encoding='utf-8')
    report={'review_id':'PLAYER-WALK-02','status':'awaiting_visual_review','default_transition_ms':400,
            'choices_ms':[120,200,400],'directions':['left','front','right'],'equipment_count':5,
            'frames_per_direction':7,'native_tile':64,'composition_canvas':128,
            'fragment_bytes':len(html.encode('utf-8')),'game_integrated':False,
            'limits':['Proposed carry drawings, not engine equipment layers','Bastion heavy copper only','No gameplay change']}
    (DOC/'packaging.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report,indent=2));print(VIS)


if __name__=='__main__':main()
