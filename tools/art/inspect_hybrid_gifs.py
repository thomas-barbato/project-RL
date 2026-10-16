"""Decode representative GIF timestamps into native-size inspection crops."""
from pathlib import Path
from PIL import Image, ImageDraw
import prepare_agent_skill_animation_review as base
OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-48'
board=Image.new('RGB',(1200,860),'#0b1218');draw=ImageDraw.Draw(board)
samples=[('M02-A01-POSSEDE-64.gif',20,'Décharge · projectile'),('M02-A01-POSSEDE-64.gif',25,'Décharge · impact'),
    ('M02-A01-POSSEDE-64.gif',35,'Décharge · terminé'),('HY07-A01-HAUT-64.gif',18,'Drone · cœur'),
    ('HY07-A01-HAUT-64.gif',23,'Drone · explosion'),('HY07-A01-HAUT-64.gif',35,'Drone · terminé')]
for index,(name,frame,label) in enumerate(samples):
    timestamp=round(frame*1000/24/10)*10;elapsed=0
    with Image.open(OUT/name) as gif:
        for part in range(gif.n_frames):
            gif.seek(part);end=elapsed+gif.info['duration']
            if elapsed<=timestamp<end:
                crop=gif.convert('RGB').crop((96,112,496,512));break
            elapsed=end
        else:raise AssertionError((name,timestamp))
    x,y=(index%3)*400,(index//3)*430
    draw.text((x+12,y+6),label+' · 64 px',font=base.font(16),fill='#e0e7e7')
    board.paste(crop,(x,y+30))
board.save(OUT/'decoded-animation-check.png')
print('Six GIF phases decoded and cropped without resampling.')
