"""Publish native normal-render captures in the existing animation review."""
from pathlib import Path
import json
import re
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/animation-polish-01/class-motion-01/current'
REVIEW = ROOT / 'docs/reviews/animation-polish-01/CLASS-MOTION01-r01.json'

SECTION = '''<section id="revue-classes">
<h2>01 · Repos et marche des quatre autres classes</h2>
<p><strong>Bastion, Technomancien, Arcaniste et Métamorphe</strong> sont synchronisés. Compare leur identité et leur démarche avec les cinq types d’armure.</p>
<p class="pending"><strong>Rendu actuel du jeu à examiner.</strong> La Traqueuse est exclue de cette revue.</p>
<div class="controls">
<label>Armure à comparer<select id="classes-armor"><option value="0">01 · Tenue de base</option><option value="1">02 · Légère</option><option value="2">03 · Intermédiaire</option><option value="3" selected>04 · Lourde</option><option value="4">05 · Très lourde</option></select></label>
<label>Équipement à comparer<select id="classes-equipment"><option value="0">Sans arme</option><option value="1">Fusil d’assaut</option><option value="2">Mitrailleuse lourde</option><option value="3">Hache d’armes</option></select></label>
<label>Directions à comparer<select id="classes-axes"><option value="sides">Gauche / droite</option><option value="vertical">Haut / bas</option><option value="all">Toutes les directions</option></select></label>
<button id="classes-play">Pause</button>
<label>Lecture des classes<select id="classes-mode"><option value="loop">Marche continue</option><option value="stop">Deux pas puis repos</option></select></label>
<label>Vitesse des classes<select id="classes-speed"><option value="1">400 ms par pas</option><option value="3">Ralenti ×3</option></select></label>
</div>
<p id="classes-columns" class="note">Colonnes : repos · gauche · droite. Une ligne par classe, cases natives de 64 px, éclairage Discret.</p>
<div class="native" style="max-width:640px"><canvas id="classes-board" width="384" height="640" style="display:block;width:384px;height:640px;image-rendering:pixelated" aria-label="Bastion, Technomancien, Arcaniste et Métamorphe au repos et dans quatre directions"></canvas></div>
<div class="timeline"><label>Phase des classes<input id="classes-phase" type="range" min="0" max="6" value="0"></label><output id="classes-time">1 / 6</output></div>
<p id="classes-status" class="note" role="status">Préparation des captures…</p>
<p id="classes-findings" class="pending"></p>
<details id="classes-zoom-details"><summary>Zoom entier ×2 des quatre classes</summary><div class="native"><canvas id="classes-zoom" width="1280" height="1280" style="display:block;width:1280px;height:1280px;max-width:none;image-rendering:pixelated"></canvas></div></details>
<p class="note">Le retour au repos conserve l’image réellement utilisée par le jeu. Une vue immobile vers le haut ou le bas signale une couverture manquante. Ces quatre équipements sont des témoins ; cette revue ne couvre pas toutes les armes ni leurs palettes.</p>
<p class="note"><a href="class-motion-01/current/checks.json">Bilan des captures du moteur</a> · <a href="class-motion-01/current/classes-3-0-native.webp">Animation native des armures lourdes sans arme</a></p>
</section>'''

SCRIPT = '''let classesFrames=[],classesGeneration=0,classesPaused=false,classesPhase=0,classesElapsed=0,classesLast=0;
const classesCoverage=__COVERAGE__;
function paintClasses(){const image=classesFrames[classesPhase];if(!image?.complete)return;const mode=$('classes-axes').value,columns=mode==='all'?[0,1,2,3,4]:mode==='vertical'?[0,3,4]:[0,1,2],board=$('classes-board'),width=columns.length*128;if(board.width!==width){board.width=width;board.style.width=width+'px';}const context=board.getContext('2d');context.imageSmoothingEnabled=false;columns.forEach((col,index)=>context.drawImage(image,col*128,0,128,640,index*128,0,128,640));$('classes-columns').textContent='Colonnes : '+columns.map(col=>['repos','gauche','droite','haut','bas'][col]).join(' · ')+'. Cases natives de 64 px, éclairage Discret.';if($('classes-zoom-details').open){const zoomBoard=$('classes-zoom');if(zoomBoard.width!==width*2){zoomBoard.width=width*2;zoomBoard.style.width=width*2+'px';}const zoom=zoomBoard.getContext('2d');zoom.imageSmoothingEnabled=false;zoom.drawImage(board,0,0,width*2,1280);}$('classes-phase').value=classesPhase;$('classes-time').textContent=classesPhase===6?'Retour au repos frontal':`${classesPhase+1} / 6`;}
async function loadClasses(){const generation=++classesGeneration;classesFrames=[];$('classes-status').textContent='Préparation des 7 captures…';const tier=$('classes-armor').value,loadout=$('classes-equipment').value;const images=Array.from({length:7},(_,phase)=>{const image=new Image();image.src=`class-motion-01/current/classes-${tier}-${loadout}-${phase}.png?revision=CLASS-MOTION01-r01`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==classesGeneration)return;classesFrames=images;classesElapsed=0;classesPhase=0;paintClasses();$('classes-status').textContent='7 / 7 captures prêtes · 4 classes · taille native · rendu actuel.';const selected=classesCoverage.filter(c=>c.armor_stage===+tier&&c.loadout===+loadout);const missing=selected.filter(c=>!c.vertical_supported).map(c=>c.name);$('classes-findings').textContent=missing.length?'À reprendre : '+missing.join(', ')+' — cycle face/dos absent pour cette configuration. Les deux côtés restent à comparer au repos.':'Cycle face/dos disponible pour les quatre classes. Vérifie aussi que l’identité, le gabarit et les prises restent cohérents entre les vues.';}catch(error){if(generation===classesGeneration)$('classes-status').textContent='Capture indisponible : '+error.message;}}
$('classes-armor').onchange=loadClasses;$('classes-equipment').onchange=loadClasses;
$('classes-axes').onchange=paintClasses;
$('classes-play').onclick=()=>{classesPaused=!classesPaused;$('classes-play').textContent=classesPaused?'Lire la marche':'Pause';};
$('classes-phase').oninput=()=>{classesPaused=true;$('classes-play').textContent='Lire la marche';classesPhase=+$('classes-phase').value;classesElapsed=classesPhase*(800/6);paintClasses();};
$('classes-mode').onchange=()=>{classesElapsed=0;};$('classes-zoom-details').ontoggle=paintClasses;
function animateClasses(now){if(classesLast&&!classesPaused&&classesFrames.length){classesElapsed+=(now-classesLast)/(+$('classes-speed').value);const duration=$('classes-mode').value==='loop'?800:1320;const elapsed=classesElapsed%duration;const phase=elapsed<800?Math.min(5,Math.floor(elapsed/(800/6))):6;if(phase!==classesPhase){classesPhase=phase;paintClasses();}}classesLast=now;requestAnimationFrame(animateClasses);}loadClasses();requestAnimationFrame(animateClasses);'''


def publish_archived_motion01():
    report = json.loads((OUT / 'checks.json').read_text(encoding='utf-8-sig'))
    assert report['traqueuse_excluded'] and not report['normal_renderer_modified']
    assert len(report['cases']) == 80 and report['boards'] == 140
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 10)
    for tier in range(5):
        for loadout in range(4):
            frames = []
            for phase in range(7):
                path = OUT / f'classes-{tier}-{loadout}-{phase}.png'
                image = Image.open(path).convert('RGB')
                assert image.size == (640, 640)
                before = [image.crop((0, row*160+32, 640, (row+1)*160)).tobytes() for row in range(4)]
                draw = ImageDraw.Draw(image)
                for row, cls in enumerate(report['classes']):
                    top = row*160
                    draw.rectangle((0, top, 639, top+31), fill=(30,40,49))
                    draw.text((6, top+1), f'{row+1:02d} · {cls["name"]}', fill='white', font=font)
                    draw.text((6, top+17), 'Repos', fill='#b6cbd2', font=small)
                    for col, direction in enumerate(['Gauche','Droite','Haut','Bas'], 1):
                        label = direction + (' → repos' if phase == 6 else '')
                        draw.text((col*128+8, top+8), label, fill='white', font=font)
                assert before == [image.crop((0, row*160+32, 640, (row+1)*160)).tobytes() for row in range(4)]
                image.save(path)
                frames.append(image)
            if tier == 3 and loadout == 0:
                frames[0].save(OUT/f'classes-{tier}-{loadout}-native.webp', save_all=True, append_images=frames[1:], duration=[133,134,133,133,134,133,520], loop=0, lossless=True, method=6)
    coverage = [{k:c[k] for k in ['class','name','armor_stage','loadout','vertical_supported']} for c in report['cases']]
    script = SCRIPT.replace('__COVERAGE__', json.dumps(coverage,ensure_ascii=False))
    page_path = ROOT / 'artifacts/animation-polish-01/apercu.html'
    page = page_path.read_text(encoding='utf-8')
    if '<section id="revue-classes">' in page:
        page = re.sub(r'<section id="revue-classes">.*?</section>', lambda m: SECTION, page, count=1, flags=re.S)
        page = re.sub(r'let classesFrames=.*?loadClasses\(\);requestAnimationFrame\(animateClasses\);', lambda m: script, page, count=1, flags=re.S)
    else:
        page = page.replace('<section id="identite-laterale">', SECTION+'\n<section id="identite-laterale">',1)
        page = page.replace('</script></html>',script+'\n</script></html>')
        page = page.replace('<nav>', '<nav><a href="#revue-classes">Revue des classes</a> · ',1)
    page = re.sub(r'<h1>.*?</h1>', '<h1>Classes : cohérence du repos et de la marche</h1>', page, count=1)
    page = re.sub(r'<title>.*?</title>', '<title>Project RL · Revue des classes · CLASS-MOTION01</title>', page, count=1)
    page = re.sub(r'(<div class="tag">).*?(</div>)', r'\g<1>CLASS-MOTION01 · QUATRE CLASSES · RENDU ACTUEL\g<2>', page, count=1)
    page = page.replace('Lecture de la marche gauche/droite rétablie. Les anciennes figures latérales restent à harmoniser avec la face et le dos actuels.', 'Comparaison du repos et des déplacements : identité, proportions, appuis et équipement.')
    page = page.replace('Comparaison 02 / 03 : une foulée plus resserrée pour la Traqueuse en armure lourde. Le Bastion est conservé tel quel.', 'Quatre classes, cinq types d’armure et quatre équipements témoins : 80 configurations à examiner ensemble.')
    page = re.sub(r'<div class="pending">.*?</div>', '<div class="pending"><strong>Revue du rendu actuel.</strong> Les écarts constatés sont montrés dans les captures pour choisir les reprises.</div>', page, count=1, flags=re.S)
    page_path.write_text(page, encoding='utf-8')
    missing = [c for c in report['cases'] if not c['vertical_supported']]
    frozen = [{**{k:c[k] for k in ['class','armor_stage','loadout']},**d} for c in report['cases'] for d in c['directions'] if d['distinct_frames'] < 6]
    REVIEW.parent.mkdir(parents=True, exist_ok=True)
    durable = {k:report[k] for k in ['review','capture_status','visual_approval','normal_renderer_modified','traqueuse_excluded','native_cell_px','step_ms','boards','engine_renders','classes','weapons','armor_models','cases']}
    durable.update(date='2026-11-02',missing_vertical_cases=len(missing),incomplete_cycles=frozen,preview='http://127.0.0.1:8765/animation-polish-01/apercu.html?revision=CLASS-MOTION01-r01#revue-classes')
    REVIEW.write_text(json.dumps(durable,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(json.dumps({'cases':len(report['cases']),'engine_renders':report['engine_renders'],'missing_vertical_cases':len(missing),'incomplete_cycles':len(frozen),'page':str(page_path)},ensure_ascii=False))


def main():
    from build_class_lateral_r02_review import publish
    ready=(ROOT/'artifacts/animation-polish-01/class-lateral-02/r02/current/checks.json').exists()
    publish(pending=not ready)


if __name__ == '__main__':
    main()
