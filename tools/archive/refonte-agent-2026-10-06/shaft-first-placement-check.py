"""Sampling/composition trial; drawing comes entirely from imagegen and accepted rigs."""
from pathlib import Path
import json,math,shutil
from PIL import Image,ImageDraw
BASE=Path(__file__).resolve().parent
ROOT=BASE.parents[1]
OUT=BASE/'shaft-review-01'
OUT.mkdir(exist_ok=True)
raw_path=Path('C:/Users/User/.codex/generated_images/01a11010-433f-7fc2-ac29-ffb696dec5ea/exec-8b9d8c59-19f8-4cd2-b925-ad4cb59086a9.png')
source=OUT/'sources/EQ-H01.png';source.parent.mkdir(exist_ok=True)
shutil.copy2(raw_path,source)
raw=Image.open(source).convert('RGBA');crop=raw.crop(raw.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox())
held=crop.resize((9,34),Image.Resampling.NEAREST)
alpha=held.getchannel('A').point(lambda a:255 if a>=128 else 0)
held=held.convert('RGB').quantize(colors=12,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE).convert('RGBA');held.putalpha(alpha)
held.save(OUT/'EQ-H01-first-held.png')
grip=(4,26)
scratch=Image.new('RGBA',(96,96));scratch.alpha_composite(held,(48-grip[0],48-grip[1]))
rotated=scratch.rotate(-math.degrees(math.atan2(3,4)),resample=Image.Resampling.NEAREST,center=(48,48))
weapon=Image.new('RGBA',(64,64));weapon.paste(rotated,(32-48,38-48))
profiles=json.loads((ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json').read_text(encoding='utf-8'))['profiles']
arms=Image.open(ROOT/'assets/sprites/cyberpunk/agent-rigs/arms-64.png').convert('RGBA')
fingers=Image.open(ROOT/'assets/sprites/cyberpunk/agent-rigs/fingers-64.png').convert('RGBA')
board=Image.new('RGB',(960,300),(16,23,31));d=ImageDraw.Draw(board)
for profile in profiles:
 n=profile['number'];body=Image.open(BASE/f'weapon-silhouette-review-03/body-{n}-native32.png').convert('RGBA')
 for box in profile['forearm_masks']:body.paste((0,0,0,0),box)
 frame=Image.new('RGBA',(64,64));frame.alpha_composite(body,(16,16))
 index=(n-1)*25+2;box=(index%16*64,index//16*64,index%16*64+64,index//16*64+64)
 frame.alpha_composite(arms.crop(box));frame.alpha_composite(weapon);frame.alpha_composite(fingers.crop(box))
 frame.save(OUT/f'EQ-H01-C{n}-first-placement.png')
 board.paste(frame.resize((192,192),Image.Resampling.NEAREST),((n-1)*192,60),frame.resize((192,192),Image.Resampling.NEAREST))
 d.text(((n-1)*192+12,20),profile['class'],fill='white')
board.save(OUT/'EQ-H01-first-placement.png')
print({'grip_opacity':[weapon.getpixel(p)[3] for p in ((32,38),(35,34))]})
