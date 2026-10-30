"""C05: compare every authored body, then all reviewed head/loadout variants."""
from pathlib import Path
import json
from PIL import Image, ImageDraw
from publish_vertical_walk_review18 import save_animation, FONT, BOLD, BG
from publish_vertical_walk_headgear18 import NAMES

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/vertical-walk-review-18/current/size-C05'
ENGINE=OUT/'engine'
LABELS=['01 Arcaniste','02 PNJ du camp','03 Bastion','04 Traqueuse','05 Métamorphe',
        '06 Bastion lourd','07 Traqueuse lourde','08 Arcaniste lourd','09 Métamorphe lourd']


def engine_crop(folder,col,step,tick,direction):
    source=Image.open(folder/f'step{step}-tick{tick:02}.png').convert('RGB')
    progress=1 if tick==12 else (tick+.5)/12
    x=168+col*288
    y=(234 if direction==0 else 414)+(step+progress)*(-1 if direction==0 else 1)*64
    return source.crop((x-8,round(y)-8,x+72,round(y)+72))


def old_crop(profile,step,tick,direction):
    if tick==12:
        im=Image.open(ROOT/profile['standing']).convert('RGBA')
    else:
        phase=step*3+min(2,tick//4)
        back=direction==0
        sheet=Image.open(ROOT/profile['file']).convert('RGBA')
        im=sheet.crop((phase*128,back*128,(phase+1)*128,(back+1)*128))
    result=Image.new('RGB',(80,80),BG)
    c=im.crop((24,24,104,104));result.paste(c,(0,0),c)
    return result


def duration(tick):
    return 650 if tick==12 else 40 if tick%3==0 else 30


def main():
    assert json.loads((ENGINE/'size-checks.json').read_text())['body_profiles']==9
    assert json.loads((ENGINE/'equipment/checks.json').read_text())['one_handed_hand_contacts']==4056
    old=[]
    for folder in ('candidate-r01','candidate-C02','candidate-C03'):
        old+=json.loads((ROOT/'assets/source-art/vertical-walk-review-18'/folder/'manifest.json').read_text())['profiles']
    locations=[('base',0),('base',2),('roster',0),('roster',1),('roster',2)]+[('equipment/unarmed',i) for i in range(4)]
    total=0
    all_frames=[];all_duration=[]
    sequences=[[] for _ in range(9)]
    for step in range(2):
        for tick in range(13):
            combined=Image.new('RGB',(380,9*125+32),BG);cd=ImageDraw.Draw(combined)
            for n,(profile,(folder,col)) in enumerate(zip(old,locations)):
                board=Image.new('RGB',(380,290),BG);d=ImageDraw.Draw(board)
                d.text((5,4),LABELS[n]+' — gabarit C05',font=BOLD,fill='white')
                d.text((5,30),'Avant',font=FONT,fill='#e6bd93');d.text((5,155),'C05',font=FONT,fill='#a7e3bd')
                cd.text((5,n*125+4),LABELS[n],font=BOLD,fill='white')
                for direction in range(2):
                    x=115+direction*130
                    d.text((x,30),'Haut / dos' if direction==0 else 'Bas / face',font=FONT,fill='white')
                    before=old_crop(profile,step,tick,direction)
                    after=engine_crop(ENGINE/folder,col,step,tick,direction)
                    board.paste(before,(x,51));board.paste(after,(x,176))
                    combined.paste(after,(x,n*125+28))
                caption='Repos frontal — même cycle' if tick==12 else 'Marche — 400 ms / pas'
                d.text((5,269),caption,font=FONT,fill='#c2cdd5')
                sequences[n].append(board)
                if step==0 and tick in (4,12):board.save(OUT/f'body-{n}-{"idle" if tick==12 else "moving"}.png')
            cd.text((5,9*125+4),'Repos frontal' if tick==12 else 'Neuf silhouettes — natif64',font=FONT,fill='#c2cdd5')
            all_frames.append(combined);all_duration.append(duration(tick))
    save_animation(all_frames,all_duration,OUT/'all-bodies-native64.webp');total+=26
    for n,sequence in enumerate(sequences):
        save_animation(sequence,all_duration,OUT/f'body-{n}-native64.webp');total+=26
    # All seven head choices, both loadouts, four classes, two complete steps.
    for head,name in enumerate(NAMES):
        for loadout in ('unarmed','mixed'):
            key=f'head-{head}-{loadout}';frames=[]
            for step in range(2):
                for tick in range(13):
                    board=Image.new('RGB',(380,315),BG);d=ImageDraw.Draw(board)
                    for col,label in enumerate(['06 Bastion','07 Traqueuse','08 Arcaniste','09 Métamorphe']):
                        cx=(col%2)*190;cy=(col//2)*145
                        d.text((cx+5,cy+4),label,font=BOLD,fill='white')
                        for direction in range(2):
                            d.text((cx+5+95*direction,cy+25),'Haut / dos' if direction==0 else 'Bas / face',font=FONT,fill='white')
                            c=engine_crop(ENGINE/'heads'/key,col,step,tick,direction)
                            board.paste(c,(cx+5+95*direction,cy+45))
                    d.text((5,294),'Repos frontal' if tick==12 else 'C05 — '+name,font=FONT,fill='#c2cdd5')
                    frames.append(board)
                    if step==0 and tick in (4,12) and head in (0,4):
                        board.save(OUT/f'{key}-{"idle" if tick==12 else "moving"}.png')
            save_animation(frames,all_duration,OUT/f'{key}-native64.webp');total+=26
            print(json.dumps({'completed':key,'decoded_frames_verified':total}),flush=True)
    # Preserve review of the other already presented weapon configurations.
    for loadout in ('unarmed','pistol','blades','pistols','mixed'):
        frames=[]
        for step in range(2):
            for tick in range(13):
                board=Image.new('RGB',(380,315),BG);d=ImageDraw.Draw(board)
                for col,label in enumerate(['06 Bastion','07 Traqueuse','08 Arcaniste','09 Métamorphe']):
                    cx=(col%2)*190;cy=(col//2)*145;d.text((cx+5,cy+4),label,font=BOLD,fill='white')
                    for direction in range(2):
                        d.text((cx+5+95*direction,cy+25),'Haut / dos' if direction==0 else 'Bas / face',font=FONT,fill='white')
                        board.paste(engine_crop(ENGINE/'equipment'/loadout,col,step,tick,direction),(cx+5+95*direction,cy+45))
                d.text((5,294),'Repos frontal' if tick==12 else 'C05 — tête propre à la tenue',font=FONT,fill='#c2cdd5');frames.append(board)
        save_animation(frames,all_duration,OUT/f'gear-{loadout}-native64.webp');total+=26
    # Armed base Arcaniste also needs a coherent idle, independently of heavy armor.
    frames=[]
    for step in range(2):
        for tick in range(13):
            board=Image.new('RGB',(380,155),BG);d=ImageDraw.Draw(board)
            d.text((5,4),'01 Arcaniste — pistolet + couteau',font=BOLD,fill='white')
            for direction in range(2):
                x=115+130*direction;d.text((x,30),'Haut / dos' if direction==0 else 'Bas / face',font=FONT,fill='white')
                board.paste(engine_crop(ENGINE/'base',1,step,tick,direction),(x,51))
            d.text((5,135),'Repos frontal' if tick==12 else 'Marche',font=FONT,fill='#c2cdd5');frames.append(board)
    save_animation(frames,all_duration,OUT/'base-mixed-native64.webp');total+=26
    camps=[]
    for name,folder,prefix in [('base','base','camp-'),('roster','roster','camp-'),
        ('heavy-west','equipment/camp-west','camp-'),('heavy-east','equipment/camp-east','camp-'),
        ('bare-west','heads/camp-bare-west','camp-'),('bare-east','heads/camp-bare-east','camp-'),
        ('helmet-west','heads/camp-helmet-west','camp-'),('helmet-east','heads/camp-helmet-east','camp-')]:
        frames=[];durations=[]
        for step in range(4):
            for tick in range(13):
                im=Image.open(ENGINE/folder/f'{prefix}step{step}-tick{tick:02}.png').convert('RGB')
                board=Image.new('RGB',(596,570),BG);d=ImageDraw.Draw(board)
                d.text((10,5),'C05 — camp / '+name,font=BOLD,fill='white')
                board.paste(im.crop((16,60,592,572)),(10,35))
                d.text((10,550),'Repos frontal' if tick==12 else 'Même éclairage / natif64',font=FONT,fill='white')
                frames.append(board);durations.append(duration(tick))
        save_animation(frames,durations,OUT/f'camp-{name}-native64.webp');total+=52;camps.append(name)
    body_options=''.join(f'<option value="{n}"'+(' selected' if n==6 else '')+f'>{label}</option>' for n,label in enumerate(LABELS))
    head_options=''.join(f'<option value="{n}">{name}</option>' for n,name in enumerate(NAMES))
    camp_labels={'base':'Arcaniste + PNJ du camp','roster':'Trois autres classes de base',
        'heavy-west':'Armures lourdes : Bastion / Traqueuse','heavy-east':'Armures lourdes : Arcaniste / Métamorphe',
        'bare-west':'Sans casque : Bastion / Traqueuse','bare-east':'Sans casque : Arcaniste / Métamorphe',
        'helmet-west':'Casqués : Bastion / Traqueuse','helmet-east':'Casqués : Arcaniste / Métamorphe'}
    camp_options=''.join(f'<option value="{name}">{camp_labels[name]}</option>' for name in camps)
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>18-C05 — cohérence des gabarits</title>
<style>body{margin:12px;max-width:380px;background:#131c24;color:#eef3f7;font:14px system-ui}h1{font-size:22px;margin:0 0 7px}p{line-height:1.35;margin:7px 0}select{display:block;width:100%;padding:5px;margin:4px 0 8px;background:#263744;color:#eef3f7;border:1px solid #526b7a;box-sizing:border-box}.view{overflow:auto}img{display:block;max-width:none}a{color:#91d6f3}summary{cursor:pointer;color:#bed5e3}[hidden]{display:none!important}</style>
<h1>18-C05 — correction des gabarits</h1><p>Neuf silhouettes reprises : repos frontal du même cycle et échelle calibrée entre face et dos.</p>
<details><summary>Ce qui change et portée du comparatif</summary><p>Les anciens repos utilisaient un autre corps. C05 utilise une vraie pose neutre du cycle, sans changement de corps à l'arrêt. Les dessins entiers sont calibrés avant affichage ; aucun zoom pendant la marche. Le dos de la Traqueuse lourde est redessiné pour corriger ses proportions.</p><p>Neuf corps présentés, quatre armures lourdes, sept choix de tête, cinq configurations d'armes déjà montrées. Les autres armures et prises à deux mains restent à dessiner. Proposition de revue, désactivée dans le jeu normal.</p><p>Avant : sources précédentes. C05 : captures du moteur recentrées à taille native. Les variations d'appui et de flexion restent visibles.</p></details>
<label>Comparaison <select id="mode" onchange="show()"><option value="body">Avant / C05 — chaque silhouette</option><option value="all">C05 — les neuf silhouettes</option><option value="heads">C05 — casques et armes</option><option value="gear">C05 — cinq ports d'armes</option><option value="base">C05 — Arcaniste de base armé</option><option value="camp">C05 — dans le camp</option></select></label>
<label id="body-controls">Silhouette <select id="body" onchange="show()">BODY_OPTIONS</select></label>
<div id="head-controls" hidden><label>Tête <select id="helmet" onchange="show()">HEAD_OPTIONS</select></label><label>Armes <select id="loadout" onchange="show()"><option value="mixed">Pistolet + couteau</option><option value="unarmed">Sans arme</option></select></label></div>
<label id="gear-controls" hidden>Armes <select id="gear" onchange="show()"><option value="unarmed">Sans arme</option><option value="pistol">Pistolet</option><option value="blades">Deux couteaux</option><option value="pistols">Deux pistolets</option><option value="mixed">Pistolet + couteau</option></select></label>
<label id="camp-controls" hidden>Scène <select id="camp" onchange="show()">CAMP_OPTIONS</select></label>
<p>Natif64 · animations sans perte · pauses de lecture.</p><div class="view"><img id="view" src="body-6-native64.webp" alt="Comparaison des gabarits avant et après correction C05"></div>
<script>function show(){const g=id=>document.getElementById(id),m=g('mode').value;for(const [id,value] of [['body-controls','body'],['head-controls','heads'],['gear-controls','gear'],['camp-controls','camp']])g(id).hidden=m!==value;g('view').src=m==='body'?'body-'+g('body').value+'-native64.webp':m==='all'?'all-bodies-native64.webp':m==='heads'?'head-'+g('helmet').value+'-'+g('loadout').value+'-native64.webp':m==='gear'?'gear-'+g('gear').value+'-native64.webp':m==='base'?'base-mixed-native64.webp':'camp-'+g('camp').value+'-native64.webp';}</script></html>'''
    html=html.replace('BODY_OPTIONS',body_options).replace('HEAD_OPTIONS',head_options).replace('CAMP_OPTIONS',camp_options)
    if (ROOT/'docs/reviews/vertical-walk-18/acceptance-C05.json').exists():
        html=html.replace('18-C05 — correction des gabarits</h1>', '18-C05 — gabarits validés</h1>')
        html=html.replace('Proposition de revue, désactivée dans le jeu normal.',
                          'Lot validé le 29 octobre 2026 ; intégration dans le jeu normal encore à réaliser.')
    (OUT/'apercu.html').write_text(html,'utf8')
    (OUT/'publication.json').write_text(json.dumps({'review':'VERT18-C05','body_profiles':9,'headgear_combinations':56,
        'five_original_loadouts':True,'all_animations_lossless':True,'decoded_frames_verified':total,
        'normal_game_enabled':False},indent=2)+'\n','utf8')
    print(json.dumps({'preview':str(OUT/'apercu.html'),'decoded_frames_verified':total}))


if __name__=='__main__':main()
