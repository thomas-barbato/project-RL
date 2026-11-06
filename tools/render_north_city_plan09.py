"""Render the editable city plan as a schematic, never as final game art."""
from pathlib import Path
from html import escape
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'artifacts/north-city-01'
S=12
COLORS={'house':'#8c8172','apartment':'#747e83','shop':'#aa9365','workshop':'#77746a','ruin':'#72565a','school':'#6d98ac','police':'#728bb0','fire':'#af7465','military':'#7d895f'}

def main():
    p=json.loads((ROOT/'assets/maps/north-city-01/city-plan09.json').read_text(encoding='utf-8'))
    size=p['width']*S
    out=Image.new('RGB',(size,size),'#404943');d=ImageDraw.Draw(out)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',15)
    bold=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',17)
    svg=[f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" role="group" aria-label="Plan complet de la nouvelle ville, 112 cases de côté">', '<style>text{font-family:Segoe UI,sans-serif}.building{cursor:pointer}.building:hover .roof,.building.selected .roof{stroke:#ffe5a0;stroke-width:3}.label{font-size:17px;font-weight:700;fill:#f1e5c7;paint-order:stroke;stroke:#17222d;stroke-width:5;stroke-linejoin:round}</style>',f'<rect width="{size}" height="{size}" fill="#404943"/>']
    def box(rect,color,stroke=None,width=1,attrs=''):
        l,t,r,b=rect;xy=(l*S,t*S,(r+1)*S,(b+1)*S)
        d.rectangle(xy,fill=color,outline=stroke,width=width)
        svg.append(f'<rect x="{xy[0]}" y="{xy[1]}" width="{xy[2]-xy[0]}" height="{xy[3]-xy[1]}" fill="{color}" stroke="{stroke or "none"}" stroke-width="{width}" {attrs}/>')
    def line(points,color,width,dash=False):
        pts=[(round((x+.5)*S),round((y+.5)*S)) for x,y in points]
        if not dash:d.line(pts,fill=color,width=max(1,round(width)),joint='curve')
        else:
            for a,b in zip(pts,pts[1:]):
                length=((b[0]-a[0])**2+(b[1]-a[1])**2)**.5
                for k in range(0,int(length),14):
                    q=min(k+7,length)
                    d.line([(round(a[0]+(b[0]-a[0])*k/length),round(a[1]+(b[1]-a[1])*k/length)),(round(a[0]+(b[0]-a[0])*q/length),round(a[1]+(b[1]-a[1])*q/length))],fill=color,width=max(1,round(width)))
        svg.append(f'<polyline points="{" ".join(str(x)+","+str(y) for x,y in pts)}" fill="none" stroke="{color}" stroke-width="{width}" stroke-linejoin="round" stroke-linecap="round"'+(' stroke-dasharray="7 7"' if dash else '')+'/>')
    def label(x,y,text,big=True):
        px,py=(x+.5)*S,(y+.5)*S
        d.text((px,py),text,font=bold if big else font,anchor='mm',fill='#f1e5c7',stroke_width=2,stroke_fill='#17222d')
        svg.append(f'<text class="label" x="{px}" y="{py}" text-anchor="middle" dominant-baseline="middle">{escape(text)}</text>')
    for c in p['campuses']:box(c['rect'],'#585b51','#77786b')
    for g in p['green_spaces']:box(g['rect'],'#4c6550','#364939')
    for q in p['plazas']:box(q['rect'],'#888778','#6f7068')
    for color,extra in [('#242c31',2.2),('#8a8a7b',1.7),('#9b9b88',1.35),('#303b43',0)]:
        for r in p['roads']:
            if r['kind']!='path':line(r['points'],color,(r['width']+extra)*S)
    for r in p['roads']:
        if r['kind']=='avenue':line(r['points'],'#a98e59',1.3,True)
    for r in p['roads']:
        if r['kind']=='path':line(r['points'],'#b0a58b',max(3,r['width']*S*.65))
    for b in p['buildings']:
        if len(b['access_path'])>1:line(b['access_path'],'#a5a18a',4)
    for b in p['buildings']:
        svg.append(f'<g class="building" data-building="{b["id"]}" role="button" aria-label="{escape(b["id"]+" · "+b["name"])}" tabindex="0"><title>{escape(b["id"]+" · "+b["name"])}</title>')
        for rect in b['rects']:
            l,t,r,bb=rect
            box([l,t+.25,r,bb+.25],'#232c31')
            box(rect,COLORS[b['kind']],'#242c31',2,'class="roof"')
            # Schematic ridge, not a new roof texture or a native-size drawing.
            if b['kind'] not in ['ruin','police','school','fire','military']:
                line([[l+.2,(t+bb)/2],[r-.2,(t+bb)/2]],'#4c5353',1)
        x,y=b['door'];d.ellipse((x*S+3,y*S+3,x*S+9,y*S+9),fill='#e2b770')
        svg.append(f'<circle cx="{(x+.5)*S}" cy="{(y+.5)*S}" r="3" fill="#e2b770"/>')
        svg.append('</g>')
    for t in p['trees']:
        x,y=t['at'];px,py=(x+.5)*S,(y+.5)*S
        d.ellipse((px-6,py-5,px+7,py+8),fill='#263b2d')
        d.ellipse((px-6,py-7,px+6,py+5),fill='#6b8050',outline='#3e583b')
        svg.append(f'<circle cx="{px}" cy="{py+2}" r="7" fill="#263b2d"/><circle cx="{px}" cy="{py-1}" r="6" fill="#6b8050" stroke="#3e583b"/>')
    for e in p['aftermath']:
        x,y=e['at'];px,py=(x+.5)*S,(y+.5)*S
        d.ellipse((px-5,py-3,px+5,py+3),fill='#9d4c48')
        svg.append(f'<ellipse cx="{px}" cy="{py}" rx="5" ry="3" fill="#9d4c48"><title>{escape(e["name"])}</title></ellipse>')
    for x,y,text in [(28,8,'COMMISSARIAT'),(16,32,'ÉCOLE'),(80,27,'POMPIERS'),(92,13,'POSTE MILITAIRE'),(77,67,'JARDIN PUBLIC'),(54,69,'MARCHÉ'),(20,108,'FAUBOURG RÉSIDENTIEL'),(19,60,'QUARTIER ANCIEN'),(94,87,'ATELIERS')]:label(x,y,text)
    line([[41,111],[41,106]],'#e8c678',3)
    line([[99,4],[99,0]],'#e8c678',3)
    label(96,2,'CASERNE ↑')
    label(43,109,'ARRIVÉE')
    # The outer closure is explicit; the arrows are transfers, not holes in a map.
    d.rectangle((0,0,size-1,size-1),outline='#202a2e',width=7)
    svg.append(f'<rect x="3" y="3" width="{size-6}" height="{size-6}" fill="none" stroke="#202a2e" stroke-width="6"/>')
    svg.append('</svg>')
    OUT.mkdir(parents=True,exist_ok=True)
    (OUT/'city-plan09.svg').write_text(''.join(svg),encoding='utf-8')
    canvas=Image.new('RGB',(size,size+128),'#111820');canvas.paste(out,(0,64));d=ImageDraw.Draw(canvas)
    d.text((22,18),'PLAN09 · NOUVELLE VILLE · 112 × 112 CASES',font=bold,fill='#e1c593')
    c=p['checks'];d.text((22,size+78),f"{c['building_count']} bâtiments · {c['enterable_candidates']} intérieurs proposés · {c['tree_count']} arbres · schéma d’aménagement, dessins à produire",font=font,fill='#bdc7c9')
    canvas.save(OUT/'city-plan09.png')
    print(OUT/'city-plan09.png')

if __name__=='__main__':main()
