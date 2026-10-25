"""Extract review references from existing art; no creative repainting."""
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/player-walk-review-02'
OUT.mkdir(parents=True,exist_ok=True)
source=Image.open(ROOT/'assets/source-art/player-walk-review-01/bastion-idle-eight-directions-source.png').convert('RGBA')
board=Image.new('RGBA',(1536,512))
for col,row in enumerate([7,0,1]):
    im=source.crop(tuple(round(v) for v in (row%4*source.width/4,row//4*source.height/2,(row%4+1)*source.width/4,(row//4+1)*source.height/2)))
    board.alpha_composite(im.resize((512,512),Image.Resampling.LANCZOS),(col*512,0))
board.save(OUT/'body-three-views-reference.png')
weapons={
    'pistol':Image.open(ROOT/'assets/sprites/art-finish-01/FIN-01-single-weapon.png').convert('RGBA'),
    'rifle':Image.open(ROOT/'assets/sprites/weapon-readability-03/WR-RF01.png').convert('RGBA'),
    'axe':Image.open(ROOT/'assets/sprites/weapon-readability-03/WR-AX01.png').convert('RGBA'),
}
bank=Image.open(ROOT/'assets/sprites/flat-agents/weapons-128.png').convert('RGBA')
cell=121
weapons['hammer']=bank.crop((cell%16*128,cell//16*128,cell%16*128+128,cell//16*128+128))
for name,im in weapons.items():
    box=im.getbbox()
    crop=im.crop(box)
    scale=max(1,min(16,480//max(crop.size)))
    zoom=crop.resize((crop.width*scale,crop.height*scale),Image.Resampling.NEAREST)
    out=Image.new('RGBA',(512,512))
    out.alpha_composite(zoom,((512-zoom.width)//2,(512-zoom.height)//2))
    out.save(OUT/f'{name}-reference.png')
    im.save(OUT/f'{name}-native-source.png')
print(OUT)
