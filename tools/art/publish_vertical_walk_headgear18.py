"""C04: native engine review of seven head choices with/without held weapons."""
from pathlib import Path
import json
from PIL import Image, ImageDraw
from publish_vertical_walk_equipment18 import SUBJECTS, FONT, BOLD, BG
from publish_vertical_walk_review18 import save_animation

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/vertical-walk-review-18/current/headgear-C04'
ENGINE=OUT/'engine'
NAMES=['Sans casque','115 Calotte Cognefer','116 Capuche de mue','117 Casque Cerbère','118 Heaume Bastion','119 Masque Chrysalide','120 Visière Fantôme']


def main():
    checks=json.loads((ENGINE/'checks.json').read_text('utf8'))
    assert checks['front_returns']==224 and checks['sampled_weapon_contacts']==672
    verified=0
    for head,name in enumerate(NAMES):
        for loadout in ['unarmed','mixed']:
            key=f'head-{head}-{loadout}';frames=[];compact=[];details=[[],[],[],[]];durations=[]
            for step in range(2):
                for tick in range(13):
                    source=Image.open(ENGINE/key/f'step{step}-tick{tick:02}.png').convert('RGB')
                    board=Image.new('RGB',(610,575),BG);draw=ImageDraw.Draw(board)
                    card=Image.new('RGB',(380,310),BG);cd=ImageDraw.Draw(card)
                    draw.text((10,5),'18-C04 - '+name+' - natif64',font=BOLD,fill='white')
                    progress=1. if tick==12 else (tick+.5)/12
                    for col,(identity,label) in enumerate(SUBJECTS):
                        x=168+col*288;draw.text((10+col*150,27),label,font=FONT,fill='white')
                        cx=(col%2)*190;cy=(col//2)*145
                        cd.text((cx+5,cy+4),label,font=BOLD,fill='white')
                        detail=Image.new('RGB',(260,625),BG);dd=ImageDraw.Draw(detail)
                        dd.text((10,5),label+' - détail ×3',font=BOLD,fill='white');dd.text((10,28),name,font=FONT,fill='white')
                        for direction in range(2):
                            y0,y1=(100,310) if direction==0 else (400,610)
                            board.paste(source.crop((x-32,y0,x+96,y1)),(10+col*150,67+direction*253))
                            label_dir='Haut / dos' if direction==0 else 'Bas / face'
                            draw.text((10+col*150,47+direction*253),label_dir,font=FONT,fill='white')
                            dy=-1 if direction==0 else 1;y=(234 if direction==0 else 414)+(step+progress)*dy*64
                            crop=source.crop((x-8,round(y)-8,x+72,round(y)+72))
                            card.paste(crop,(cx+5+direction*95,cy+43))
                            cd.text((cx+5+direction*95,cy+23),label_dir,font=FONT,fill='white')
                            detail.paste(crop.resize((240,240),Image.Resampling.NEAREST),(10,75+direction*260))
                            dd.text((10,52+direction*260),label_dir,font=FONT,fill='white')
                        dd.text((10,600),'Arrêt : tête équipée conservée' if tick==12 else 'Pistolet + couteau' if loadout=='mixed' else 'Sans arme',font=FONT,fill='#c2cdd5')
                        details[col].append(detail)
                        if step==0 and tick==4 and head in [0,1,4]: detail.save(OUT/f'{key}-{identity}-detail-x3.png')
                    draw.text((10,540),'Pistolet + couteau' if loadout=='mixed' else 'Sans arme',font=FONT,fill='white')
                    draw.text((10,558),'Arrêt : retour frontal avec la même tête' if tick==12 else 'Appuis alternés / 400 ms par pas',font=FONT,fill='#c2cdd5')
                    cd.text((5,291),'Repos : face' if tick==12 else 'Cycles recentrés — natif64',font=FONT,fill='#c2cdd5')
                    frames.append(board);compact.append(card);durations.append(650 if tick==12 else 40 if tick%3==0 else 30)
                    if step==0 and tick in [4,12] and head in [0,1,4]: board.save(OUT/f'{key}-{"repos" if tick==12 else "marche"}-natif64.png')
            save_animation(frames,durations,OUT/f'{key}-natif64.webp');verified+=len(frames)
            save_animation(compact,durations,OUT/f'{key}-cycles-natif64.webp');verified+=len(compact)
            for (identity,_),sequence in zip(SUBJECTS,details):
                save_animation(sequence,durations,OUT/f'{key}-{identity}-detail-x3.webp');verified+=len(sequence)
            print(json.dumps({'completed':key,'frames_verified':verified}),flush=True)
    for kind,head in [('bare',0),('helmet',4)]:
        for side,label in [('west','Bastion / Traqueuse'),('east','Arcaniste / Métamorphe')]:
            key=f'camp-{kind}-{side}';check=json.loads((ENGINE/key/'camp-checks.json').read_text('utf8'))
            assert check['front_returns']==8 and check['moving_and_idle_actor_gain']==1.48
            frames=[];durations=[]
            for step in range(4):
                for tick in range(13):
                    source=Image.open(ENGINE/key/f'camp-step{step}-tick{tick:02}.png').convert('RGB')
                    board=Image.new('RGB',(596,600),BG);draw=ImageDraw.Draw(board)
                    draw.text((10,5),'18-C04 - camp - '+NAMES[head],font=BOLD,fill='white');draw.text((10,29),label+' / pistolet + couteau',font=FONT,fill='white')
                    board.paste(source.crop((16,60,592,572)),(10,52))
                    draw.text((10,576),'Arrêt : tête équipée conservée' if tick==12 else 'Haut : dos' if step<2 else 'Bas : face',font=FONT,fill='white')
                    frames.append(board);durations.append(650 if tick==12 else 40 if tick%3==0 else 30)
                    if step==0 and tick==4:board.save(OUT/f'{key}-marche-natif64.png')
            save_animation(frames,durations,OUT/f'{key}-natif64.webp');verified+=len(frames)
    (OUT/'apercu.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>18-C04 — avec et sans casque</title>
<style>body{margin:12px;background:#131c24;color:#eef3f7;font:14px system-ui}h1{font-size:22px;margin:0 0 7px}p{max-width:82ch;line-height:1.35;margin:7px 0}select{display:block;width:100%;box-sizing:border-box;padding:5px;margin-top:3px;background:#263744;color:#eef3f7;border:1px solid #526b7a}.controls{display:grid;grid-template-columns:1fr 1fr;gap:7px;margin-top:8px}.wide{grid-column:1/-1}a{color:#91d6f3}.view{overflow:auto}img{display:block;max-width:none}summary{cursor:pointer;color:#bed5e3}</style>
<h1>18-C04 — avec / sans casque</h1><p>Quatre armures : dos en montant, face en descendant, puis face à l'arrêt.</p>
<details><summary>C04 en revue — portée du lot</summary><p>Les quatre armures lourdes validées, tête découverte ou avec l'une des six protections existantes. Le même équipement reste visible pendant la marche et au retour de face. Sans casque, l'Arcaniste retrouve son visage et sa capuche abaissée. Les corps et les prises d'armes de C03 sont conservés.</p><p>Prototype désactivé dans le jeu normal. Les autres armures, palettes et prises à deux mains restent à décliner. <a href="../equipment-C03/apercu.html">Revoir C03 validé</a>.</p></details><div class="controls">
<label>Tête <select id="helmet" onchange="choose()"><option value="0">Sans casque</option><option value="1">115 Calotte Cognefer</option><option value="2">116 Capuche de mue</option><option value="3">117 Casque Cerbère</option><option value="4">118 Heaume Bastion</option><option value="5">119 Masque Chrysalide</option><option value="6">120 Visière Fantôme</option></select></label>
<label>Armes <select id="loadout" onchange="choose()"><option value="mixed">Pistolet + couteau</option><option value="unarmed">Sans arme</option></select></label>
<label class="wide">Vue <select id="subject" onchange="show()"><option value="all">Tous — cycles natif64</option><option value="travel">Tous — déplacements natif64</option><option value="bastion">01 Bastion ×3</option><option value="traqueuse">02 Traqueuse ×3</option><option value="arcaniste">03 Arcaniste ×3</option><option value="metamorphe">04 Métamorphe ×3</option><option value="camp-bare-west">Camp sans casque : 01 / 02</option><option value="camp-bare-east">Camp sans casque : 03 / 04</option><option value="camp-helmet-west">Camp casqué : 01 / 02</option><option value="camp-helmet-east">Camp casqué : 03 / 04</option></select></label></div>
<p>Images du moteur · sans perte · pauses de lecture.</p>
<div class="view"><img id="view" src="head-0-mixed-cycles-natif64.webp" alt="18-C04 : personnages avec et sans casque, de face et de dos"></div>
<script>function show(){const h=document.getElementById('helmet'),l=document.getElementById('loadout'),s=document.getElementById('subject').value;if(s.startsWith('camp-')){h.value=s.includes('-bare-')?'0':'4';l.value='mixed';document.getElementById('view').src=s+'-natif64.webp';}else{const k='head-'+h.value+'-'+l.value;document.getElementById('view').src=k+(s==='all'?'-cycles-natif64.webp':s==='travel'?'-natif64.webp':'-'+s+'-detail-x3.webp');}}function choose(){const s=document.getElementById('subject');if(s.value.startsWith('camp-'))s.value='all';show();}</script></html>''','utf8')
    status=json.loads((ROOT/'docs/reviews/vertical-walk-18/ANALYSE-C04.json').read_text('utf8'))['status']
    if status=='needs_size_rework':
        page=OUT/'apercu.html'
        text=page.read_text('utf8').replace('<h1>18-C04 — avec / sans casque</h1>',
            '<h1>18-C04 — gabarits à corriger</h1><p style="color:#ffd591">Le changement de taille signalé est confirmé. '
            '<a href="../size-C05/apercu.html">Ouvrir le correctif complet C05.</a></p>')
        page.write_text(text,'utf8')
    (OUT/'publication.json').write_text(json.dumps({'review':'VERT18-C04','normal_game_enabled':False,'native_tile_px':64,'all_animations_lossless':True,
        'decoded_animation_frames_verified':verified,'animations':88,'raw_capture_frames':572,'head_choices':7,'loadouts':2,'compact_cycles_keep_native_pixels':True},indent=2)+'\n','utf8')
    print(json.dumps({'preview':str(OUT/'apercu.html'),'frames_verified':verified,'animations':88}))


if __name__=='__main__':main()
