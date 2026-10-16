"""GEL-A05: foreground ice coating, preserving all previous review artifacts."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image
import prepare_agent_skill_animation_gel_status as previous

BASE = previous.OUT
OUT = previous.base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-28'
SOURCE = Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea\exec-3097f40e-6096-47e6-90e8-4e9bfb0e0ff5.png')
old_protected = previous.protected
old_write_html = previous.write_html


def protected():
    hashes = old_protected()
    for p in sorted(BASE.rglob('*')):
        if p.is_file():
            hashes[p.relative_to(previous.base.ROOT).as_posix()] = hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists()
    previous.base.dump(OUT/'review-protection.json',protected())
    (OUT/'gel-source.png').write_bytes(SOURCE.read_bytes())
    with Image.open(SOURCE) as image:
        assert image.mode == 'RGBA' and image.size == (1536,1024)
        assert image.getchannel('A').getextrema()[0] == 0
        atlas = Image.new('RGBA',(1536,256))
        for i in range(6):
            x,y = i%3*512,i//3*512
            atlas.paste(image.crop((x,y,x+512,y+512)).resize((256,256),Image.Resampling.LANCZOS),(i*256,0))
        atlas.save(OUT/'gel-atlas.png')
    previous.base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen edit','reference':'skill-animation-review-27/gel-source.png',
        'source_sha256':hashlib.sha256(SOURCE.read_bytes()).hexdigest(),'source_size':[1536,1024],'layout':[3,2],
        'cell_size':[256,256],'alpha_preserved':True,'postprocessing':'Equal-cell extraction and uniform resize only; no painting.',
        'composition':'New foreground covers torso and legs; crop excludes upper quarter to preserve face. Old rear layer keeps only bottom quarter crystals.'})
    previous.base.dump(OUT/'review.json',{'batch':'ANIM-GEL01','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':20,'id':'GEL','variant':'GEL-A05','name':'Gel au premier plan','status':'awaiting_visual_review'},
            {'number':19,'id':'FR06','variant':'FR06-A03','name':'Explosion cristalline avec Gel au premier plan','status':'awaiting_visual_review'}],
        'user_direction':'ok alors on fait ca :) following the foreground translucent torso/legs and sparse rear crystals proposal.',
        'scope':'New visual proposal only; existing status timings, walls, Burn replacement and crystalline consumption retained. No campaign or mechanics change.'})
    launch=(BASE/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--gel-status-v2'","'--gel-status-v2' '--gel-status-front'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'frames':6}))


def write_html(entries):
    old_write_html(entries)
    path=OUT/'index.html'
    html=path.read_text(encoding='utf-8').replace('GEL-A04','GEL-A05').replace('FR06-A02','FR06-A03')
    html=html.replace('Le givre enveloppe le personnage concerné et ses reflets restent animés tant que le Gel est présent.',
        'La glace translucide couvre le torse et les jambes au premier plan. Le visage reste dégagé, avec quelques cristaux derrière les pieds. Les reflets restent animés tant que le Gel est présent.')
    html=html.replace('<h1>20 · Gel animé</h1>','<h1>20 · Gel au premier plan</h1>')
    html=html.replace('<div class="controls">','<p><a href="gel-comparaison-64.gif">Comparer avec la version précédente</a></p><div class="controls">')
    path.write_text(html,encoding='utf-8')


def package():
    previous.OUT = OUT
    previous.protected = protected
    previous.write_html = write_html
    previous.package()
    # Reuse the previous verified package logic, but publish the revised IDs.
    for p in list(OUT.glob('GEL-A04*.gif'))+list(OUT.glob('FR06-A02*.gif')):
        dest=p.with_name(p.name.replace('GEL-A04','GEL-A05').replace('FR06-A02','FR06-A03'))
        assert p.resolve().is_relative_to(OUT.resolve()) and dest.resolve().is_relative_to(OUT.resolve())
        assert not dest.exists()
        p.rename(dest)
    verification=json.loads((OUT/'verification.json').read_text(encoding='utf-8'))
    for e in verification['animations']:
        for k,v in list(e.items()):
            if isinstance(v,str): e[k]=v.replace('GEL-A04','GEL-A05').replace('FR06-A02','FR06-A03')
    previous.base.dump(OUT/'verification.json',verification)
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    from PIL import ImageChops
    layer_checks=[]
    for cx in (3,4):
        with Image.open(OUT/'native/GEL/035.png') as image,Image.open(OUT/'native/GEL/control-035.png') as control:
            diff=ImageChops.difference(image.convert('RGB'),control.convert('RGB')).crop((72,88,648,664))
        head=diff.crop((cx*64+24,4*64+8,cx*64+40,4*64+13))
        assert head.getbbox() is None, 'Coating covers the upper head.'
        torso=diff.crop((cx*64+20,4*64+20,cx*64+44,4*64+38))
        legs=diff.crop((cx*64+20,4*64+39,cx*64+44,4*64+56))
        assert torso.getbbox() and legs.getbbox(), 'Missing foreground body frost.'
        layer_checks.append({'actor':'player' if cx==3 else 'enemy','head_clear':True,'torso_frost_visible':True,'leg_frost_visible':True})
    previous.base.dump(OUT/'foreground-verification.json',{'checks':layer_checks,'rear_opacity':.26,'front_opacity':.70,'upper_quarter_excluded_from_front':True,'only_bottom_quarter_behind':True})
    old_frames=BASE/'frames/GEL-A04'
    new_frames=OUT/'frames/GEL-A05'
    boards=[]
    from PIL import ImageDraw
    for i in range(96):
        board=Image.new('RGB',(1184,620),'#0b1218')
        draw=ImageDraw.Draw(board)
        for j,(folder,label) in enumerate([(old_frames,'Avant · GEL-A04'),(new_frames,'Premier plan · GEL-A05')]):
            draw.text((16+j*592,9),label,font=previous.base.font(20,True),fill='#e0e7e7')
            with Image.open(folder/f'{i:03}.png') as im: board.paste(im,(8+j*592,40))
        boards.append(board)
    previous.save_animation(OUT/'gel-comparaison-64.gif',boards,24)
    print(json.dumps({'comparison':'native 64px, no resampling','status':'awaiting_visual_review'}))


if __name__ == '__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--prepare',action='store_true')
    args=parser.parse_args()
    prepare() if args.prepare else package()
