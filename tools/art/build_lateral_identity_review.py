"""Publish LATERAL02 without enabling new figures in the normal renderer."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/animation-polish-01/lateral-identity-02/current'


def build():
    revision = next((n for n in [3,2] if (OUT.parent/f'r0{n}/current/checks.json').exists()),1)
    out = OUT.parent/f'r0{revision}/current' if revision>1 else OUT
    if not (out / 'checks.json').exists():
        return '', ''
    checks = json.loads((out / 'checks.json').read_text(encoding='utf-8'))
    assert checks['boards'] == 14 and not checks['normal_game_enabled']
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    for direction in ['left', 'right']:
        frames = []
        for phase in range(7):
            path = out / f'identity-0{revision}-{direction}-{phase}.png'
            image = Image.open(path).convert('RGB')
            pixels = image.crop((0, 32, 512, 160)).tobytes()
            draw = ImageDraw.Draw(image)
            draw.rectangle((0, 0, 511, 31), fill=(30, 40, 49))
            labels = ['Face validée', f'0{revision-1} / Précédent', f'0{revision} / Nouvel essai', 'Dos validé'] if revision>1 else ['Face validée', 'Côté actuel', '01 / Redessiné', 'Dos validé']
            for col, label in enumerate(labels):
                draw.text((col * 128 + 8, 8), label, font=font, fill=(235, 241, 243))
            assert pixels == image.crop((0, 32, 512, 160)).tobytes()
            image.save(path)
            frames.append(image)
        animated = frames[:6] if revision>1 else frames
        prefix = f'LATERAL02-r0{revision}' if revision>1 else 'LATERAL02'
        animated[0].save(out / f'{prefix}-{direction}-native.webp', save_all=True,
                       append_images=animated[1:], duration=[133, 134, 133, 133, 134, 133] + ([] if revision>1 else [520]),
                       loop=0, lossless=True, method=6)
    section = '''<section id="identite-laterale"><h2>01 · Même personnage de face, de dos et de côté</h2>
<p><strong>Premier candidat : Traqueuse, armure lourde.</strong> Les nouveaux côtés reprennent les cheveux attachés, le masque, l’armure claire et la silhouette de la face et du dos validés. L’ancien côté, encore casqué, reste visible pour comparer.</p>
<p class="pending">Dessin en revue, sans intégration au jeu. Ce lot couvre le corps et la tenue, sans arme : deux directions, six phases par cycle et un repos distinct. Les autres classes et armures restent à reprendre après cette première revue.</p>
<div class="controls"><label>Direction<select id="identity-direction"><option value="left">Gauche</option><option value="right">Droite</option></select></label><button id="identity-play">Pause</button><label>Vitesse<select id="identity-speed"><option value="1">400 ms par pas</option><option value="3">Ralenti ×3</option></select></label></div>
<div class="native" style="max-width:512px"><canvas id="identity-board" width="512" height="160" style="display:block;max-width:none;image-rendering:pixelated" aria-label="Face validée, ancien côté, candidat 01 et dos validé dans le camp à taille native"></canvas></div>
<div class="timeline"><label>Phase<input id="identity-phase" type="range" min="0" max="6" value="0"></label><output id="identity-time">1 / 6</output></div><p id="identity-status" class="note" role="status"></p>
<details><summary>Zoom entier ×4 de la comparaison</summary><div class="native"><canvas id="identity-zoom" width="2048" height="640" style="display:block;width:2048px;height:640px;max-width:none;image-rendering:pixelated"></canvas></div></details>
<p class="note">Captures du moteur dans le camp accepté, cases de 64 px et éclairage Discret. Face et dos conservés comme repères fixes. Un cycle comporte deux pas de 400 ms ; la pause finale de démonstration n’ajoute aucun délai en jeu.</p>
<p class="note"><a href="lateral-identity-02/current/LATERAL02-left-native.webp">Animation native gauche</a> · <a href="lateral-identity-02/current/LATERAL02-right-native.webp">Animation native droite</a> · <a href="lateral-identity-02/current/checks.json">Contrôles du moteur</a></p></section>'''
    script = '''let identityFrames=[],identityGeneration=0,identityPaused=false,identityPhase=0,identityStart=performance.now();
function paintIdentity(){const image=identityFrames[identityPhase];if(!image?.complete)return;const columns=window.matchMedia('(max-width:600px)').matches?2:4,board=$('identity-board'),width=columns*128,height=columns===2?320:160;if(board.width!==width||board.height!==height){board.width=width;board.height=height;}const context=board.getContext('2d');context.imageSmoothingEnabled=false;for(let col=0;col<4;col++)context.drawImage(image,col*128,0,128,160,(col%columns)*128,Math.floor(col/columns)*160,128,160);const zoom=$('identity-zoom').getContext('2d');zoom.imageSmoothingEnabled=false;zoom.drawImage(image,0,0,2048,640);$('identity-phase').value=identityPhase;$('identity-time').textContent=identityPhase===6?'Repos latéral':`${identityPhase+1} / 6`;}
async function loadIdentity(){const generation=++identityGeneration;$('identity-status').textContent='Préparation du cycle…';identityFrames=[];const direction=$('identity-direction').value;const images=Array.from({length:7},(_,n)=>{const image=new Image();image.src=`lateral-identity-02/current/identity-01-${direction}-${n}.png?revision=LATERAL02-r01`;return image;});try{await Promise.all(images.map(image=>image.decode()));if(generation!==identityGeneration)return;identityFrames=images;identityStart=performance.now();paintIdentity();$('identity-status').textContent='LATERAL02-r01 · 01 en revue · corps et tenue sans arme · jeu inchangé.';}catch(error){if(generation===identityGeneration)$('identity-status').textContent='Capture manquante : '+error.message;}}
$('identity-direction').onchange=loadIdentity;$('identity-play').onclick=()=>{identityPaused=!identityPaused;$('identity-play').textContent=identityPaused?'Lire la marche':'Pause';identityStart=performance.now();};$('identity-phase').oninput=()=>{identityPaused=true;$('identity-play').textContent='Lire la marche';identityPhase=+$('identity-phase').value;paintIdentity();};
function animateIdentity(now){if(!identityPaused&&identityFrames.length){const elapsed=(now-identityStart)/(+$('identity-speed').value),loop=$('identity-mode')?.value==='loop',duration=loop?800:1320,t=elapsed%duration;const phase=t<800?Math.min(5,Math.floor(t/(800/6))):6;if(phase!==identityPhase){identityPhase=phase;paintIdentity();}}requestAnimationFrame(animateIdentity);}window.addEventListener('resize',paintIdentity);if($('identity-mode'))$('identity-mode').onchange=()=>{identityStart=performance.now();};loadIdentity();requestAnimationFrame(animateIdentity);'''
    if revision>1:
        section = section.replace('01 · Même personnage de face, de dos et de côté', '02 · Traqueuse : deux appuis successifs')
        section = section.replace('<strong>Premier candidat : Traqueuse, armure lourde.</strong> Les nouveaux côtés reprennent les cheveux attachés, le masque, l’armure claire et la silhouette de la face et du dos validés. L’ancien côté, encore casqué, reste visible pour comparer.', '<strong>Traqueuse en armure lourde uniquement.</strong> Le nouvel essai 02 reprend les deux appuis opposés, le passage de la jambe et sa reprise de contact. L’essai 01 reste à côté pour comparer. <strong>Le Bastion est conservé tel quel.</strong>')
        section = section.replace('Dessin en revue, sans intégration au jeu. Ce lot couvre le corps et la tenue, sans arme : deux directions, six phases par cycle et un repos distinct. Les autres classes et armures restent à reprendre après cette première revue.', 'Candidat 02 en revue, sans intégration au jeu. Corps et tenue sans arme : gauche et droite, six phases par cycle et un repos distinct. Cette reprise ne concerne que la Traqueuse montrée ici.')
        section = section.replace('<button id="identity-play">Pause</button>', '<button id="identity-play">Pause</button><label>Lecture<select id="identity-mode"><option value="loop">Marche continue</option><option value="stop">Deux pas puis repos</option></select></label>')
        section = section.replace('candidat 01 et dos validé', 'candidat 02 et dos validé')
        section = section.replace('Un cycle comporte deux pas de 400 ms ; la pause finale de démonstration n’ajoute aucun délai en jeu.', 'Marche continue par défaut : deux pas de 400 ms. « Deux pas puis repos » montre aussi l’arrêt ; la phase 6 permet de regarder le repos séparément.')
        section = section.replace('LATERAL02-left-native.webp', 'LATERAL02-r02-left-native.webp').replace('LATERAL02-right-native.webp', 'LATERAL02-r02-right-native.webp')
        section = section.replace('lateral-identity-02/current/', 'lateral-identity-02/r02/current/')
        script = script.replace('identity-01-${direction}', 'identity-02-${direction}').replace('lateral-identity-02/current/', 'lateral-identity-02/r02/current/').replace('LATERAL02-r01', 'LATERAL02-r02').replace('01 en revue', '02 en revue')
    if revision==3:
        section = section.replace('02 · Traqueuse : deux appuis successifs','03 · Traqueuse : foulée resserrée')
        section = section.replace('Le nouvel essai 02 reprend les deux appuis opposés, le passage de la jambe et sa reprise de contact. L’essai 01 reste à côté pour comparer.','Le nouvel essai 03 réduit l’écartement des jambes en gardant les appuis alternés. Le précédent 02 reste à côté pour comparer ; la pose de repos est conservée à l’identique.')
        section = section.replace('Candidat 02 en revue','Candidat 03 en revue').replace('ancien côté, candidat 02','essai 02, candidat 03')
        section = section.replace('lateral-identity-02/r02/current/','lateral-identity-02/r03/current/').replace('LATERAL02-r02-','LATERAL02-r03-')
        script = script.replace('identity-02-${direction}','identity-03-${direction}').replace('lateral-identity-02/r02/current/','lateral-identity-02/r03/current/').replace('LATERAL02-r02','LATERAL02-r03').replace('02 en revue','03 en revue')
    return section, script


def publish_current():
    """Refresh only this review in the shared page; keep other captures intact."""
    section, script = build()
    assert section and script
    path = ROOT / 'artifacts/animation-polish-01/apercu.html'
    page = path.read_text(encoding='utf-8')
    start = page.index('<section id="identite-laterale">')
    end = page.index('</section>', start) + len('</section>')
    page = page[:start] + section + page[end:]
    start = page.index('let identityFrames=')
    marker = 'loadIdentity();requestAnimationFrame(animateIdentity);'
    end = page.index(marker, start) + len(marker)
    page = page[:start] + script + page[end:]
    if (OUT.parent / 'r02/current/checks.json').exists():
        page = page.replace('LATERAL02-r01 · PREMIER CANDIDAT À EXAMINER', 'LATERAL02-r02 · TRAQUEUSE · NOUVEL ESSAI 02')
        page = page.replace('<strong>LATERAL02-r01 en revue.</strong>', '<strong>LATERAL02-r02 en revue.</strong>')
        page = page.replace('La reprise commence par la Traqueuse en armure lourde : nouveau dessin latéral comparé aux vues face/dos à taille native. Les revues précédentes restent disponibles plus bas.', 'Reprise de la marche de la Traqueuse en armure lourde : essai précédent et nouvel essai 02 à taille native. Le Bastion est conservé tel quel.')
    if (OUT.parent / 'r03/current/checks.json').exists():
        page = page.replace('LATERAL02-r02 · TRAQUEUSE · NOUVEL ESSAI 02','LATERAL02-r03 · TRAQUEUSE · FOULÉE RESSERRÉE')
        page = page.replace('<strong>LATERAL02-r02 en revue.</strong>','<strong>LATERAL02-r03 en revue.</strong>')
        page = page.replace('Reprise de la marche de la Traqueuse en armure lourde : essai précédent et nouvel essai 02 à taille native. Le Bastion est conservé tel quel.','Comparaison 02 / 03 : une foulée plus resserrée pour la Traqueuse en armure lourde. Le Bastion est conservé tel quel.')
    path.write_text(page, encoding='utf-8')
    print(path)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--publish-current', action='store_true')
    args = parser.parse_args()
    if args.publish_current:
        publish_current()
    else:
        build()
