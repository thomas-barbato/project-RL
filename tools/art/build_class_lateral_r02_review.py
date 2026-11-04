"""Current-roster motion study: eight key poses and a slower moving comparison."""
from pathlib import Path
import argparse
import json
import re
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/class-lateral-02/r02/current'
PAGE=ROOT/'artifacts/animation-polish-01/apercu.html'

SECTION='''<section id="reprise-laterale"><h2>02 · Des pas plus posés</h2>
<p><strong>Bastion, Arcaniste et Métamorphe.</strong> Armures lourdes, sans arme. La revue suit les classes encore proposées à la création.</p>
<p class="pending"><strong>Nouvelle étude : 650 ms par pas, huit poses par cycle.</strong> Contact, prise d’appui, passage du pied puis repose. L’aller-retour permet de juger les appuis sur le sol.</p>
<div class="controls"><label>Vue de la marche<select id="walk2-view"><option value="travel">Aller-retour</option><option value="left">Détail gauche sur place</option><option value="right">Détail droite sur place</option></select></label><button id="walk2-play">Pause</button><button id="walk2-next">Pose suivante</button><label>Cadence proposée<select id="walk2-pace"><option value="650">650 ms par pas</option><option value="800">800 ms par pas</option><option value="400">400 ms · ancien rythme</option></select></label></div>
<p class="note">Colonnes : repos actuel · précédent au même rythme · nouveau cycle. Cases natives de 64 px, éclairage Discret.</p>
<div class="native" style="max-width:384px"><canvas id="walk2-board" width="384" height="480" style="display:block;width:384px;height:480px;image-rendering:pixelated" aria-label="Bastion, Arcaniste et Métamorphe : repos, cycle précédent et nouveau pas"></canvas></div>
<div class="timeline"><label>Pose du pas<input id="walk2-phase" type="range" min="0" max="8" value="0"></label><output id="walk2-time">Contact A</output></div><p id="walk2-status" class="note" role="status">Préparation de la marche…</p>
<details id="walk2-zoom-details"><summary>Zoom entier ×2 des appuis</summary><div class="native"><canvas id="walk2-zoom" width="768" height="960" style="display:block;width:768px;height:960px;max-width:none;image-rendering:pixelated"></canvas></div></details>
<p class="note">Le retour au repos utilise la tenue actuelle. Cet aller-retour est une étude d’animation ; les nouveaux dessins et la cadence restent à examiner avant intégration. La Traqueuse conserve son cycle.</p>
<p class="note"><a href="class-lateral-02/r02/current/CLASS-LATERAL02-r02-native.webp">Aller-retour natif animé</a> · <a href="class-lateral-02/r02/current/checks.json">Bilan des captures</a></p>
</section>'''

SCRIPT='''let walk2Bank=new Map(),walk2Ready=false,walk2Paused=false,walk2Elapsed=0,walk2Last=0,walk2Manual=null,walk2Direction='left',walk2Painted='';
const walk2Files=__FILES__,walk2Durations=__DURATIONS__,walk2Samples=__SAMPLES__,walk2Names=['Contact A','Appui A','Passage du pied B','Reprise du pied B','Contact B','Appui B','Passage du pied A','Reprise du pied A','Repos frontal'];
function walk2PhaseAt(time){let end=0;for(let p=0;p<8;p++){end+=walk2Durations[p];if(time<end)return p;}return 7;}
function walk2Selection(){const view=$('walk2-view').value;let direction=view==='right'?'right':'left',phase=0,sample=0,kind=view==='travel'?'travel':'keys';if(walk2Manual!==null){direction=walk2Direction;phase=walk2Manual;sample=kind==='travel'?walk2Samples[phase]:phase;}else{const full=1300+650;const t=walk2Elapsed%(view==='travel'?full*2:full);if(view==='travel')direction=t<full?'left':'right';const local=t%full;phase=local>=1300?8:walk2PhaseAt(local);sample=kind==='travel'?(local>=1300?32:Math.min(31,Math.floor(local/(1300/32)))):phase;}return {file:`${kind}-${direction}-${String(sample).padStart(2,'0')}.png`,direction,phase};}
function paintWalk2(){if(!walk2Ready)return;const selected=walk2Selection();walk2Direction=selected.direction;if(selected.file!==walk2Painted){const image=walk2Bank.get(selected.file),context=$('walk2-board').getContext('2d');context.imageSmoothingEnabled=false;context.drawImage(image,0,0);walk2Painted=selected.file;if($('walk2-zoom-details').open){const zoom=$('walk2-zoom').getContext('2d');zoom.imageSmoothingEnabled=false;zoom.drawImage(image,0,0,768,960);}}$('walk2-phase').value=selected.phase;$('walk2-time').textContent=(selected.direction==='left'?'Gauche · ':'Droite · ')+walk2Names[selected.phase];}
async function loadWalk2(){try{const images=walk2Files.map(file=>{const image=new Image();image.src='class-lateral-02/r02/current/'+file+'?revision=CLASS-LATERAL02-r02';return {file,image};});await Promise.all(images.map(({image})=>image.decode()));walk2Bank=new Map(images.map(({file,image})=>[file,image]));walk2Ready=true;paintWalk2();$('walk2-status').textContent='84 / 84 captures prêtes · trois classes actuelles · étude de marche.';}catch(error){$('walk2-status').textContent='Capture indisponible : '+error.message;}}
function freezeWalk2(phase){walk2Paused=true;walk2Manual=phase;$('walk2-play').textContent='Reprendre la marche';paintWalk2();}
$('walk2-phase').oninput=()=>freezeWalk2(+$('walk2-phase').value);$('walk2-next').onclick=()=>freezeWalk2((+$('walk2-phase').value+1)%9);
$('walk2-play').onclick=()=>{if(walk2Paused){if(walk2Manual!==null){const start=walk2Durations.slice(0,Math.min(8,walk2Manual)).reduce((a,b)=>a+b,0);walk2Elapsed=start+($('walk2-view').value==='travel'&&walk2Direction==='right'?1950:0);}walk2Manual=null;}walk2Paused=!walk2Paused;$('walk2-play').textContent=walk2Paused?'Reprendre la marche':'Pause';};
$('walk2-view').onchange=()=>{walk2Elapsed=0;walk2Manual=null;walk2Direction=$('walk2-view').value==='right'?'right':'left';walk2Painted='';paintWalk2();};$('walk2-zoom-details').ontoggle=()=>{walk2Painted='';paintWalk2();};
function animateWalk2(now){if(walk2Last&&walk2Ready&&!walk2Paused)walk2Elapsed+=(now-walk2Last)*(650/(+$('walk2-pace').value));walk2Last=now;paintWalk2();requestAnimationFrame(animateWalk2);}loadWalk2();requestAnimationFrame(animateWalk2);'''


def clean_legacy(html):
    # Preserve historical evidence on disk; remove the superseded current UI and timers.
    html=re.sub(r'<section id="revue-classes">.*?</section>','',html,count=1,flags=re.S)
    html=re.sub(r'let classesFrames=.*?loadClasses\(\);requestAnimationFrame\(animateClasses\);','',html,count=1,flags=re.S)
    html=re.sub(r'let lat2Frames=.*?loadLat2\(\);requestAnimationFrame\(animateLat2\);','',html,count=1,flags=re.S)
    html=re.sub(r'<a href="#revue-classes">.*?</a>\s*·\s*','',html,count=1)
    html=html.replace('Bastion, Technomancien, Arcaniste et Métamorphe : premiers candidats à comparer.','Bastion, Arcaniste et Métamorphe : cadence et appuis repris.')
    html=html.replace('Premier lot : quatre nouvelles démarches en armure lourde, sans arme, comparées aux côtés actuels.','Trois classes actuelles, armures lourdes sans arme : étude du contact au sol et du passage de la jambe.')
    html=re.sub(r'<title>.*?</title>','<title>Project RL · Marche posée · CLASS-LATERAL02-r02</title>',html,count=1)
    html=re.sub(r'(<div class="tag">).*?(</div>)',r'\g<1>CLASS-LATERAL02-r02 · TROIS CLASSES ACTUELLES · PAS POSÉS\g<2>',html,count=1)
    return html


def publish(pending=False):
    html=clean_legacy(PAGE.read_text(encoding='utf-8'))
    if pending:
        html=re.sub(r'let walk2Bank=.*?loadWalk2\(\);requestAnimationFrame\(animateWalk2\);','',html,count=1,flags=re.S)
    section='<section id="reprise-laterale"><h2>Reprise de la démarche</h2><p>Bastion, Arcaniste et Métamorphe : les appuis et la cadence sont en cours de reprise.</p></section>' if pending else SECTION
    html=re.sub(r'<section id="reprise-laterale">.*?</section>',lambda m:section,html,count=1,flags=re.S)
    if not pending:
        report=json.loads((OUT/'checks.json').read_text(encoding='utf-8-sig'))
        assert [c['slug'] for c in report['classes']]==['bastion','arcaniste','metamorphe']
        assert len(report['captures'])==84
        font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12);small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',10)
        for file in report['captures']:
            path=OUT/file;im=Image.open(path).convert('RGB');assert im.size==(384,480)
            before=[im.crop((0,row*160+32,384,(row+1)*160)).tobytes() for row in range(3)]
            draw=ImageDraw.Draw(im)
            for row,cls in enumerate(report['classes']):
                y=row*160;draw.rectangle((0,y,383,y+31),fill=(30,40,49))
                draw.text((5,y+1),f'{row+1:02d} · {cls["name"]}',font=font,fill='white');draw.text((5,y+17),'Repos actuel',font=small,fill='#b6cbd2')
                draw.text((136,y+1),'Précédent',font=font,fill='#c1c6ca');draw.text((136,y+17),'au même rythme',font=small,fill='#b6cbd2')
                draw.text((264,y+1),'Nouveau pas',font=font,fill='#a5dfba');draw.text((264,y+17),'8 poses clés',font=small,fill='#b6cbd2')
            assert before==[im.crop((0,row*160+32,384,(row+1)*160)).tobytes() for row in range(3)]
            im.save(path)
        animation=[];durations=[]
        short=[41 if i%8<5 else 40 for i in range(32)]
        assert sum(short)==1300
        for direction in ['left','right']:
            animation.extend([Image.open(OUT/f'travel-{direction}-{n:02}.png').convert('RGB') for n in range(33)])
            durations.extend(short+[650])
        animation[0].save(OUT/'CLASS-LATERAL02-r02-native.webp',save_all=True,append_images=animation[1:],duration=durations,loop=0,lossless=True,method=4)
        script=SCRIPT.replace('__FILES__',json.dumps(report['captures'])).replace('__DURATIONS__',json.dumps(report['phase_durations_ms'])).replace('__SAMPLES__',json.dumps(report['phase_samples']))
        if 'let walk2Bank=' in html:html=re.sub(r'let walk2Bank=.*?loadWalk2\(\);requestAnimationFrame\(animateWalk2\);',lambda m:script,html,count=1,flags=re.S)
        else:html=html.replace('</script></html>',script+'\n</script></html>')
    assert 'Technomancien' not in html
    PAGE.write_text(html,encoding='utf-8')
    print('Published active-roster study' if not pending else 'Removed retired class and superseded active comparisons')


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--pending',action='store_true');args=p.parse_args();publish(args.pending)
