"""Compact native-size working boards; never changes a sprite or a game rule."""
import json,math
from PIL import Image,ImageDraw,ImageFont
from art_finish_completion import ROOT,ART,DOC,OUT,EXISTING,EXISTING_BATCH

def font(size):return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)

def build():
    data=json.loads((ART/'prompts.json').read_text('utf-8'))['jobs']
    ready=[j for j in data if (ART/'native'/f'{j["number"]:03}-0.png').exists()]
    for start in range(0,len(ready),12):
        jobs=ready[start:start+12]
        out=Image.new('RGB',(1440,80+math.ceil(len(jobs)/3)*240),'#11191d');d=ImageDraw.Draw(out)
        d.text((20,13),f'Finition en cours · {start+1}–{start+len(jobs)} · échelle native',font=font(23),fill='#decca7')
        d.text((20,46),'Ancien à gauche · nouveau à droite · case de 64 px · aucune normalisation des tailles',font=font(16),fill='#b9c8c0')
        for i,j in enumerate(jobs):
            x,y=i%3*480,80+i//3*240
            d.text((x+12,y+3),f'{j["id"]} · {j["number"]:03} · {j["label"]}',font=font(15),fill='#decca7')
            for c,file in enumerate([ART/'references'/f'MON-{j["number"]:03}-A.png',ART/'native'/f'{j["number"]:03}-0.png']):
                xx=x+12+c*230;d.rectangle((xx,y+30,xx+218,y+220),fill='#363f3d')
                im=Image.open(file).convert('RGBA');foot=j['foot_anchor']
                out.paste(im,(xx+109-foot[0],y+202-foot[1]),im)
        path=OUT/f'completion-masters-{start//12+1:02}.png';out.save(path)
    # Existing icons are inspected to separate already coherent art from necessary redraws.
    spec=json.loads((ROOT/'assets/sprites/equipment-native64/manifest.json').read_text('utf-8'))
    sheet=Image.open(ROOT/'assets/sprites/equipment-native64/icons-64.png').convert('RGBA')
    for start in range(0,len(spec['icon_bindings']),48):
        entries=list(spec['icon_bindings'].items())[start:start+48]
        out=Image.new('RGB',(1280,60+math.ceil(len(entries)/8)*120),'#11191d');d=ImageDraw.Draw(out)
        d.text((20,12),'Icônes existantes · contrôle à taille native',font=font(22),fill='#decca7')
        for i,(illustration,(index,dual)) in enumerate(entries):
            x=i%8*160;y=60+i//8*120
            im=sheet.crop((index%16*64,index//16*64,index%16*64+64,index//16*64+64))
            out.paste(im,(x+48,y+30),im);d.text((x+12,y+4),f'Objet {illustration}',font=font(14),fill='#c7d0c8')
        out.save(OUT/f'completion-icons-audit-{start//48+1:02}.png')
    print(json.dumps(dict(master_boards=math.ceil(len(ready)/12),ready=len(ready),pending=len(data)-len(ready))))

def source_boards(start,count):
    jobs=json.loads((ART/'prompts.json').read_text('utf-8'))['jobs'][start:start+count]
    paths=[]
    for offset in range(0,len(jobs),12):
        part=jobs[offset:offset+12]
        page=Image.new('RGB',(1536,60+math.ceil(len(part)/3)*500),'#11191d');d=ImageDraw.Draw(page)
        d.text((20,12),'Dessins sources · inspection avant reprise',font=font(22),fill='#decca7')
        for i,j in enumerate(part):
            x=i%3*512;y=60+i//3*500
            im=Image.open(j['original_master']).convert('RGBA');im.thumbnail((480,450),Image.Resampling.LANCZOS)
            page.paste(im,(x+(512-im.width)//2,y+35+(450-im.height)//2),im)
            d.text((x+10,y+4),f'{j["id"]} · {j["number"]:03} · {j["label"]}',font=font(16),fill='#decca7')
        path=OUT/f'completion-source-inspection-{start+offset:03}.png';page.save(path);paths.append(str(path))
    print(json.dumps(paths))

if __name__=='__main__':
    import sys
    source_boards(int(sys.argv[2]),int(sys.argv[3])) if len(sys.argv)>1 and sys.argv[1]=='--sources' else build()
