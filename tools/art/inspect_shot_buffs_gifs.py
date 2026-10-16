"""Native-scale GIF phase inspection and color preservation for ANIM-S24."""
import json
from pathlib import Path
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-53'
board=Image.new('RGB',(960,552),(11,18,24));pen=ImageDraw.Draw(board);report=[]
def at_time(path,frame):
    time=round(frame*1000/24/10)*10;elapsed=0
    with Image.open(path)as gif:
        for i in range(gif.n_frames):
            gif.seek(i);end=elapsed+gif.info['duration']
            if elapsed<=time<end:return gif.convert('RGB').copy()
            elapsed=end
    raise AssertionError((path,frame))
for row,skill in enumerate(['T01','T03']):
    for col,direction in enumerate(['HAUT','DROITE','GAUCHE','BAS']):
        path=OUT/f'{skill}-A01-{direction}-64.gif';frame=20 if skill=='T01'else 28
        decoded=at_time(path,frame);source=Image.open(OUT/'frames'/f'{skill}-A01-{direction}'/f'{frame:03}.png').convert('RGB')
        errors=[]
        for a,b in zip(source.get_flattened_data(),decoded.get_flattened_data()):
            selected=(a[0]>a[1]*1.4 and a[0]>a[2]*1.4)if skill=='T01'else(a[0]>210 and a[1]>210 and a[2]>210)
            if selected:errors.append(max(abs(x-y)for x,y in zip(a,b)))
        mean=sum(errors)/len(errors)if errors else 0;p95=sorted(errors)[int((len(errors)-1)*.95)]if errors else 0
        assert mean<20 and p95<45,(path.name,mean,p95)
        x,y={'HAUT':(288,96),'DROITE':(480,288),'GAUCHE':(96,288),'BAS':(288,480)}[direction]
        bx,by=col*240,row*276;pen.text((bx+10,by+10),f'{skill} {direction}',fill=(224,231,231))
        board.paste(decoded.crop((x-64,y-64,x+64,y+64)),(bx+10,by+32))
        board.paste(decoded.crop((344,0,568,96)),(bx+10,by+172))
        report.append({'variant':path.name,'timestamp_frame':frame,'native_crop':True,'mean_color_error':round(mean,2),'p95_color_error':p95})
board.save(OUT/'gif-native-inspection.png')
(OUT/'gif-palette-verification.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'GIFs_checked':len(report),'palette':'passed','inspection':str(OUT/'gif-native-inspection.png')}))
