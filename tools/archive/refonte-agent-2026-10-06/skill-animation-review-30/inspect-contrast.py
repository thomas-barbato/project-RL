import json
from pathlib import Path
from PIL import Image,ImageChops
root=Path(__file__).resolve().parent
old=root.parent/'skill-animation-review-29'
for e in json.loads((root/'native/capture.json').read_text(encoding='utf-8')):
    for v in e['views']:
        x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h)
        for phase in e['control_frames']:
            with Image.open(root/'native'/e['folder']/f'control-{phase:03}.png') as a,Image.open(old/'native'/e['folder']/f'control-{phase:03}.png') as b:
                a=a.crop(bounds).convert('RGB');b=b.crop(bounds).convert('RGB');diff=ImageChops.difference(a,b)
            if diff.getbbox():
                before=diff.getbbox()
                if 17<=phase<34:
                    px,py=v['player_cell'];px=(px+.5)*64;py=(py+.5)*64
                    baseline=py-43-(phase-17)/24*7
                    diff.paste((0,0,0),(int(px-59),int(baseline-19),int(px+160),int(baseline+4)))
                print(e['folder'],v['id'],phase,'original',before,'outside floating label',diff.getbbox())
                assert diff.getbbox() is None
print('All terrain and actor pixels identical; only transient text edge variations found.')
