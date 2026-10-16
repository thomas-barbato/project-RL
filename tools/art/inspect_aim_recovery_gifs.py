"""Inspect encoded VFX at native scale, using GIF timestamps rather than frame IDs."""
import hashlib,json
from pathlib import Path
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-52'
report=[]
board=Image.new('RGB',(960,576),(11,18,24));pen=ImageDraw.Draw(board)
def at_time(path,frame):
    time=round(frame*1000/24/10)*10;elapsed=0
    with Image.open(path)as gif:
        for i in range(gif.n_frames):
            gif.seek(i);end=elapsed+gif.info['duration']
            if elapsed<=time<end:return gif.convert('RGB').copy()
            elapsed=end
    raise AssertionError((path,frame,time))
for row,skill in enumerate(['BD04','BD06']):
    for col,direction in enumerate(['HAUT','DROITE','GAUCHE','BAS']):
        path=OUT/f'{skill}-A01-{direction}-64.gif';frame=82 if skill=='BD04'else 31
        decoded=at_time(path,frame)
        source=Image.open(OUT/'frames'/f'{skill}-A01-{direction}'/f'{frame:03}.png').convert('RGB')
        errors=[]
        for a,b in zip(source.get_flattened_data(),decoded.get_flattened_data()):
            chosen=(a[0]>a[1]*1.4 and a[0]>a[2]*1.4)if skill=='BD04'else (a[2]>a[1]+15 and a[2]>a[0]+10)
            if chosen:errors.append(max(abs(x-y)for x,y in zip(a,b)))
        mean=sum(errors)/len(errors)if errors else 0;p95=sorted(errors)[int((len(errors)-1)*.95)]if errors else 0
        assert mean<20 and p95<45,(path.name,mean,p95)
        center={'HAUT':(288,96),'DROITE':(480,288),'GAUCHE':(96,288),'BAS':(288,480)}[direction]
        x,y=center;crop=decoded.crop((x-64,y-64,x+64,y+64));base=(col*240,row*288)
        pen.text((base[0]+10,base[1]+10),f'{skill} {direction}',fill=(224,231,231))
        board.paste(crop,(base[0]+10,base[1]+32))
        board.paste(decoded.crop((304,0,576,104)).crop((0,0,224,104)),(base[0]+10,base[1]+172))
        report.append({'variant':path.name,'timestamp_frame':frame,'native_crop':True,'mean_color_error':round(mean,2),'p95_color_error':p95})
board.save(OUT/'gif-native-inspection.png')
(OUT/'gif-palette-verification.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'GIFs_checked':len(report),'palette':'passed','inspection':str(OUT/'gif-native-inspection.png')}))
