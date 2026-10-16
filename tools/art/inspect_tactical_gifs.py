"""Decode selected native GIF timestamps for visual inspection, without resampling."""
from PIL import Image,ImageDraw
import prepare_agent_skill_animation_review as base
OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-49'
samples=[('HY04-A01-HAUT-64.gif',14,'Échange · avant'),('HY04-A01-HAUT-64.gif',20,'Échange · fragments'),
    ('HY04-A01-HAUT-64.gif',25,'Échange · après'),('HY06-A01-HAUT-64.gif',35,'Domination · active'),
    ('HY06-A01-DEGATS-64.gif',51,'Domination · dégâts reçus'),('HY06-A01-HAUT-64.gif',84,'Domination · terminée')]
board=Image.new('RGB',(1200,860),'#0b1218');draw=ImageDraw.Draw(board)
for index,(name,frame,label) in enumerate(samples):
    timestamp=round(frame*1000/24/10)*10;elapsed=0
    with Image.open(OUT/name) as gif:
        for part in range(gif.n_frames):
            gif.seek(part);end=elapsed+gif.info['duration']
            if elapsed<=timestamp<end:crop=gif.convert('RGB').crop((96,112,496,512));break
            elapsed=end
        else:raise AssertionError((name,timestamp))
    x,y=index%3*400,index//3*430
    draw.text((x+12,y+6),label+' · 64 px',font=base.font(16),fill='#e0e7e7');board.paste(crop,(x,y+30))
board.save(OUT/'decoded-animation-check.png')
print('Six GIF phases decoded and cropped without resampling.')
