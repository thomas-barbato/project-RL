"""C03: lossless native engine comparisons across heavy outfits and loadouts."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont
from publish_vertical_walk_review18 import save_animation

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/vertical-walk-review-18/current/equipment-C03'
ENGINE = OUT / 'engine'
FONT = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 14)
BOLD = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Bold.ttf'), 17)
BG = (30, 40, 49)
SUBJECTS = [('bastion','01 Bastion'),('traqueuse','02 Traqueuse'),('arcaniste','03 Arcaniste'),('metamorphe','04 Métamorphe')]
CASES = [('unarmed','Sans arme'),('pistol','Pistolet'),('blades','Deux couteaux'),('pistols','Deux pistolets'),('mixed','Pistolet + couteau')]


def main():
    checks = json.loads((ENGINE/'checks.json').read_text('utf8'))
    assert checks['front_returns'] == 80 and checks['one_handed_hand_contacts'] == 3744
    verified = 0
    for case, case_label in CASES:
        frames, details, durations = [], [[],[],[],[]], []
        for step in range(2):
            for tick in range(13):
                source = Image.open(ENGINE/case/f'step{step}-tick{tick:02}.png').convert('RGB')
                board = Image.new('RGB',(610,575),BG); draw=ImageDraw.Draw(board)
                draw.text((10,5),'18-C03 - armures lourdes - natif64',font=BOLD,fill='white')
                progress=1. if tick==12 else (tick+.5)/12
                for col,(identity,label) in enumerate(SUBJECTS):
                    x=168+col*288
                    draw.text((10+col*150,27),label,font=FONT,fill='white')
                    detail=Image.new('RGB',(260,610),BG); dd=ImageDraw.Draw(detail)
                    dd.text((10,5),label+' - détail ×3',font=BOLD,fill='white')
                    for direction in range(2):
                        y0,y1=(100,310) if direction==0 else (400,610)
                        board.paste(source.crop((x-32,y0,x+96,y1)),(10+col*150,67+direction*253))
                        direction_label='Haut / dos' if direction==0 else 'Bas / face'
                        draw.text((10+col*150,47+direction*253),direction_label,font=FONT,fill='white')
                        dy=-1 if direction==0 else 1
                        y=(234 if direction==0 else 414)+(step+progress)*dy*64
                        crop=source.crop((x-8,round(y)-8,x+72,round(y)+72))
                        detail.paste(crop.resize((240,240),Image.Resampling.NEAREST),(10,60+direction*260))
                        dd.text((10,38+direction*260),direction_label,font=FONT,fill='white')
                    dd.text((10,580),'Arrêt : face habituelle' if tick==12 else case_label,font=FONT,fill='#c2cdd5')
                    details[col].append(detail)
                    if step==0 and tick==4 and case=='mixed': detail.save(OUT/f'{identity}-mixed-detail-x3.png')
                draw.text((10,540),case_label,font=FONT,fill='white')
                draw.text((10,558),'Arrêt : dessin frontal existant' if tick==12 else 'Appuis alternés / 400 ms par pas',font=FONT,fill='#c2cdd5')
                frames.append(board); durations.append(650 if tick==12 else 40 if tick%3==0 else 30)
                if step==0 and tick==4: board.save(OUT/f'{case}-marche-natif64.png')
        save_animation(frames,durations,OUT/f'{case}-natif64.webp'); verified+=len(frames)
        for (identity,_),sequence in zip(SUBJECTS,details):
            save_animation(sequence,durations,OUT/f'{case}-{identity}-detail-x3.webp'); verified+=len(sequence)
    for scene,label in [('camp-west','Bastion / Traqueuse'),('camp-east','Arcaniste / Métamorphe')]:
        check=json.loads((ENGINE/scene/'camp-checks.json').read_text('utf8'))
        assert check['front_returns']==8 and check['moving_and_idle_actor_gain']==1.48
        frames,durations=[],[]
        for step in range(4):
            for tick in range(13):
                source=Image.open(ENGINE/scene/f'camp-step{step}-tick{tick:02}.png').convert('RGB')
                board=Image.new('RGB',(596,600),BG); draw=ImageDraw.Draw(board)
                draw.text((10,5),'18-C03 - camp et lumières du jeu - natif64',font=BOLD,fill='white')
                draw.text((10,29),label+' / pistolet + couteau',font=FONT,fill='white')
                board.paste(source.crop((16,60,592,572)),(10,52))
                draw.text((10,576),'Arrêt : face habituelle' if tick==12 else 'Haut : dos' if step<2 else 'Bas : face',font=FONT,fill='white')
                frames.append(board); durations.append(650 if tick==12 else 40 if tick%3==0 else 30)
                if step==0 and tick==4: board.save(OUT/f'{scene}-marche-natif64.png')
        save_animation(frames,durations,OUT/f'{scene}-natif64.webp'); verified+=len(frames)
    (OUT/'apercu.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>18-C03 — armures et armes</title>
<style>body{margin:16px;background:#131c24;color:#eef3f7;font:16px system-ui}h1{font-size:24px;margin:0 0 10px}p{max-width:82ch;line-height:1.4;margin:9px 0}button,select{padding:7px 9px;margin:3px;background:#263744;color:#eef3f7;border:1px solid #526b7a}a{color:#91d6f3}.view{overflow:auto}img{display:block;max-width:none}</style>
<h1>18-C03 — armures et armes</h1><p>Armures les plus lourdes des quatre classes : dos en montant, face en descendant, puis pose frontale habituelle à l'arrêt. Les armes restent des dessins séparés et communs à toutes les classes.</p>
<p>C03 est validé avec les protections de tête d'origine. <a href="../headgear-C04/apercu.html">Le lot C04 présente les vraies variantes sans casque et les six casques équipables</a>. Les autres armures, palettes et prises à deux mains restent à décliner. Prototype désactivé dans le jeu normal.</p>
<label>Équipement <select id="loadout" onchange="chooseLoadout()"><option value="unarmed">Sans arme</option><option value="pistol">Pistolet</option><option value="blades">Deux couteaux</option><option value="pistols">Deux pistolets</option><option value="mixed" selected>Pistolet + couteau</option></select></label>
<label>Vue <select id="subject" onchange="showStudio()"><option value="all">Tous — natif64</option><option value="bastion">01 Bastion ×3</option><option value="traqueuse">02 Traqueuse ×3</option><option value="arcaniste">03 Arcaniste ×3</option><option value="metamorphe">04 Métamorphe ×3</option><option value="camp-west">Camp : 01 / 02</option><option value="camp-east">Camp : 03 / 04</option></select></label><button onclick="showCamp('camp-west')">Camp : 01 / 02</button><button onclick="showCamp('camp-east')">Camp : 03 / 04</button>
<p><a href="../apercu.html">C01 validé</a> · <a href="../roster-C02/apercu.html">C02 validé</a> · Captures du moteur ; animations sans perte ; pauses de lecture allongées.</p>
<div class="view"><img id="view" src="mixed-natif64.webp" alt="18-C03 : quatre armures lourdes en marche de dos et de face avec armes indépendantes"></div>
<script>function showStudio(){const c=document.getElementById('loadout'),s=document.getElementById('subject').value;if(s.startsWith('camp-')){c.value='mixed';document.getElementById('view').src=s+'-natif64.webp';}else{document.getElementById('view').src=c.value+(s==='all'?'-natif64.webp':'-'+s+'-detail-x3.webp');}}function showCamp(s){document.getElementById('subject').value=s;showStudio();}function chooseLoadout(){const s=document.getElementById('subject');if(s.value.startsWith('camp-'))s.value='all';showStudio();}</script></html>''','utf8')
    (OUT/'publication.json').write_text(json.dumps({'review':'VERT18-C03','normal_game_enabled':False,'native_tile_px':64,
        'all_animations_lossless':True,'decoded_animation_frames_verified':verified,'neutral_animations':5,'detail_animations':20,
        'camp_animations':2,'raw_capture_frames':234,'artificial_reading_pause_ms':650},indent=2)+'\n','utf8')
    print(json.dumps({'preview':str(OUT/'apercu.html'),'lossless_frames_verified':verified,'animations':27}))


if __name__ == '__main__':
    main()
