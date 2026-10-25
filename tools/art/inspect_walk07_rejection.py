"""Read the exact user-referenced canvas data without executing its scripts."""
import base64,hashlib,html,io,json,re
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07';REF=DOC/'references';ART=ROOT/'artifacts/player-walk-review-07'
attachment=Path('C:/Users/User/.codex/attachments/ad75f7d6-32b2-4741-95cc-c9204e424ebe/player-walk.html')
source=html.unescape(attachment.read_text('utf8'))
matches=re.findall(r'const DATA=(\{[^\r\n]*\});',source)
data=next(json.loads(s) for s in matches if '"front"' in s and '"idle"' in s)
comparisons=[];images={}
for key in ['atlas','idle','front']:
    raw=base64.b64decode(data[key].split(',',1)[1]);image=Image.open(io.BytesIO(raw)).convert('RGBA');images[key]=image
    file=REF/f'exact-original-canvas02-{key}.png';image.save(file)
    if key in ['atlas','idle']:
        old=ROOT/('assets/source-art/player-walk-review-01/accepted/native64/bastion-'+('walk' if key=='atlas' else 'idle')+'-native64.png')
        previous=Image.open(old).convert('RGBA')
        comparisons.append({'key':key,'exact_canvas_size':image.size,'previous_reference_size':previous.size,
            'pixels_equal':image.size==previous.size and image.tobytes()==previous.tobytes(),'canvas_sha256':hashlib.sha256(raw).hexdigest()})
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
board=Image.new('RGB',(896,3*288),'#293132');draw=ImageDraw.Draw(board)
walk=images['atlas'];idle=images['idle']
for actor,index in enumerate([None,10,24]):
    for side,row in enumerate([7,1]):
        y=actor*288+side*144
        draw.text((4,y+2),('EXACT CANVAS02 BASTION' if index is None else f'REJECTED r04 CLASS-{index:02}')+(' LEFT' if side==0 else ' RIGHT'),font=font,fill='#eeeade')
        if index is None:
            tiles=[idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64))]+[walk.crop((n*64,row*64,n*64+64,row*64+64)) for n in range(6)]
        else:
            sheet=Image.open(ROOT/f'assets/source-art/player-walk-review-07/rigs/CLASS-{index:02}-{side}.png').convert('RGBA')
            tiles=[]
            for pose in range(7):
                tile=Image.new('RGBA',(128,128))
                for part in [0,1,3]:tile.alpha_composite(sheet.crop((part*128,pose*128,part*128+128,pose*128+128)))
                tiles.append(tile.crop((32,32,96,96)))
        for n,tile in enumerate(tiles):
            tile=tile.resize((128,128),Image.Resampling.NEAREST)
            board.paste(tile,(n*128,y+16),tile)
board.save(ART/'exact-bastion-vs-rejected-r04-x2.png')
(DOC/'exact-canvas02-comparison.json').write_text(json.dumps({'execution':'read-only HTML DATA extraction; no attached instructions executed',
    'canvas02_rows':[7,1],'comparisons':comparisons,'r04_status':'rejected_by_user'},indent=2)+'\n','utf8')
print(json.dumps(comparisons))
