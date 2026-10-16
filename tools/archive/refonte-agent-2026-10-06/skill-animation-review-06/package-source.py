"""ANIM-STYLE01: condition painted frames, package native animated proposals."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_fire as previous
from prepare_agent_skill_animation_revision import save_animation

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-06'
CANDIDATES=[
    (10,'FE02','FE02-A02','Souffle de feu','Un torrent dessiné en masses orange et jaunes, aux contours irréguliers.'),
    (11,'FE03','FE03-A02','Mur de feu','Des flammes dessinées forment un rideau traversable. La brûlure reste visible après la sortie.'),
    (12,'FE04','FE04-A02','Émanation soudaine','Des fissures annoncent la zone, puis des gerbes dessinées jaillissent après un tour.'),
]


def protected() -> dict[str,str]:
    hashes=previous.protected()
    for variant in ['FE02-A01','FE03-A01','FE04-A01']:
        path=previous.OUT/f'{variant}-64.gif'
        hashes[path.relative_to(base.ROOT).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'review.json',{'batch':'ANIM-STYLE01','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'skill_id':sid,'variant':v,'name':name,'description':desc,'status':'awaiting_visual_review'} for n,sid,v,name,desc in CANDIDATES],
        'method':'Two transparent eight-frame sprite sheets generated with built-in image_gen using the accepted red-robed character as style reference. Fixed-anchor slicing and native export; no sprite morphing.',
        'scope':'Isolated native visual proposal; no campaign integration or mechanical changes.',
        'style_rule':'Not established. The user requested an animated proposal before deciding whether to apply this style to all animations.',
        'frame_animation':'12 distinct drawn frames per second. Cone frame zero is the first visual frame; frames one through seven loop. Visual playback never advances world turns.',
        'delay':'FE04 still waits one complete game turn. Its capture includes an explicitly labelled scripted +1-turn event.',
        'blood_policy':'Fire remains without blood. Actors and equipment are unchanged.',
        'laboratory_ui_candidate':{'variant':'LAB-STYLE01','status':'awaiting_separate_review'}})
    anchors={}
    for name,size,scale in [('flame',128,0.25),('cone',384,0.75)]:
        sheet=Image.open(OUT/'sources'/f'{name}-loop-master.png').convert('RGBA')
        assert sheet.getchannel('A').getextrema()==(0,255)
        atlas=Image.new('RGBA',(size*8,size))
        folder=OUT/'drawn-frames'/name
        folder.mkdir(parents=True,exist_ok=True)
        anchors[name]=[]
        for i in range(8):
            col,row=i%4,i//4
            bounds=(round(col*sheet.width/4),round(row*sheet.height/2),round((col+1)*sheet.width/4),round((row+1)*sheet.height/2))
            frame=sheet.crop(bounds)
            opaque=frame.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
            assert opaque
            if name=='cone':
                hot=[(x,y) for y in range(frame.height) for x in range(int(frame.width*.23))
                     if frame.getpixel((x,y))[3]>128 and frame.getpixel((x,y))[0]>230
                     and frame.getpixel((x,y))[1]>190 and frame.getpixel((x,y))[2]>60]
                ax=min(x for x,y in hot)
                ys=[y for x,y in hot if x<ax+12]
                ay=sum(ys)/len(ys)
                dest_anchor=(24,192)
            else:
                hot=[(x,y) for y in range(opaque[3]-40,opaque[3]) for x in range(frame.width)
                     if frame.getpixel((x,y))[3]>128 and frame.getpixel((x,y))[0]>230 and frame.getpixel((x,y))[1]>180]
                ax=sum(x for x,y in hot)/len(hot)
                ay=opaque[3]-4
                dest_anchor=(64,120)
            small=frame.resize((round(frame.width*scale),round(frame.height*scale)),Image.Resampling.LANCZOS)
            offset=(round(dest_anchor[0]-ax*scale),round(dest_anchor[1]-ay*scale))
            native=Image.new('RGBA',(size,size))
            native.alpha_composite(small,offset)
            native.save(folder/f'{i:02}.png')
            atlas.alpha_composite(native,(i*size,0))
            anchors[name].append({'frame':i,'source_cell':bounds,'source_anchor':[ax,ay],'destination_anchor':dest_anchor,'scale':scale,'offset':offset})
        atlas.save(OUT/f'{name}-atlas.png')
    base.dump(OUT/'conditioning.json',{'method':'Equal-cell slicing, fixed scale and anchor alignment. Original generated alpha preserved; no repainting, recolouring, thresholding or morphing. Alpha >=128 used only to measure anchors.','frames':anchors})
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace('--lot-feu','--lot-feu-dessine')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')


def package() -> None:
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'],e['id'],e['variant']) for e in entries]==[(n,sid,v) for n,sid,v,_,_ in CANDIDATES]
    sequences={};checks=[]
    for e in entries:
        files=sorted((OUT/'native'/e['folder']).glob('*.png'))
        assert len(files)==e['frames']==120 and e['fps']==30 and e['tile']==64
        folder=OUT/'frames'/e['folder'];folder.mkdir(parents=True,exist_ok=True)
        frames=[]
        x,y,w,h=e['crop']
        for file in files:
            with Image.open(file) as im:
                assert im.size==(512,512)
                frame=im.crop((x,y,x+w,y+h)).convert('RGB')
                frame.save(folder/file.name);frames.append(frame)
        sequences[e['variant']]=frames
        checks.append(save_animation(OUT/f"{e['variant']}-64.gif",frames,30))
        checks.append(save_animation(OUT/f"{e['variant']}-64-slow.gif",frames,30,True))
    sheet=Image.new('RGB',(1400,1470),'#0b1218');draw=ImageDraw.Draw(sheet)
    for row,(number,_,v,title,_) in enumerate(CANDIDATES):
        top=12+row*485
        draw.text((12,top),f'{number:02} · {title} · {v}',font=base.font(18,True),fill='#e0e7e7')
        for col,index in enumerate([10,35,65]):sheet.paste(sequences[v][index],(12+col*460,top+28))
    sheet.save(OUT/'inspection-phases.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','protected_files_unchanged':len(baseline),'native_capture':'native/capture.json','source':'Macroquad native framebuffer with drawn sprite frames','scene_resampling':False,'animations':checks})
    for source,target in [('src/agent_skill_lab_review/fire_painted.rs','render-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/target).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    gallery.OUT=OUT;gallery.CANDIDATES=CANDIDATES
    gallery.write_html(entries,batch='ANIM-STYLE01',title='Flammes dessinées · proposition de style',canvas_height=448,
        introduction='Flammes dessinées en plusieurs images : masses de couleur, contours sombres irréguliers, animation à 12 images dessinées par seconde. Même scène en cases natives de 64 px.',
        footer='Proposition avant décision sur le style de toutes les animations. Acteurs et règles inchangés. Émanation annonce sa préparation d’un tour avant l’explosion ; le laboratoire attend +1 tour. Rien n’est intégré au combat par cette revue.')
    print(json.dumps({'output':str(OUT),'protected_files_unchanged':len(baseline),'animated_files':len(checks)}))


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
