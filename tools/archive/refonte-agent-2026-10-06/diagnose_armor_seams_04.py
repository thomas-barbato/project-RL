"""Read-only source/compositor comparison, not an artwork generator."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont, ImageChops
import build_armor_flat_review_03 as rev

art=rev.art
out=art.BASE/'armor-flat-review-04'/'audit'
out.mkdir(parents=True,exist_ok=True)
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',20)
ids=['AR-N14','AR-08','AR-10','AR-11','AR-12']
board=Image.new('RGB',(1500,len(ids)*450),'#111c25')
d=ImageDraw.Draw(board)
stats={}
for row,ident in enumerate(ids):
    ref=art.rig.normalize(art.source(ident,'independent'))
    composed=art.body(ident,'independent')
    lost=ImageChops.subtract(ref.getchannel('A'),composed.getchannel('A'))
    stats[ident]={'alpha_loss_bbox':lost.getbbox(),'registered_joints':art.joints(ident,'independent')}
    for col,(im,label) in enumerate([(ref,'Source normalisée'),(composed,'Composition actuelle'),(lost.convert('RGBA'),'Perte alpha (blanc)')]):
        x=col*500;y=row*450
        d.text((x+10,y+6),ident+' · '+label,fill='white',font=font)
        bg=Image.new('RGBA',im.size,'#afb9bd') if col<2 else Image.new('RGBA',im.size,'black')
        bg.alpha_composite(im)
        view=bg.crop((15*art.F,17*art.F,49*art.F,43*art.F)).resize((470,360),Image.Resampling.LANCZOS)
        board.paste(view,(x+15,y+38))
board.save(out/'source-vs-compositor.png')
(out/'diagnosis.json').write_text(json.dumps(stats,indent=2),'utf-8')
print(json.dumps(stats))
