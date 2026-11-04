"""Publish the ANIM-POLISH01 native engine captures in the current preview."""
from pathlib import Path
import json
from PIL import Image
from build_armor_outfit_review import build as build_armor_outfits
from build_player_exposure_review import build as build_exposure
from build_firearm_polish_review import build as build_firearms
from build_rifle_size_review import build as build_rifles
from build_lateral_identity_review import build as build_lateral_identity

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/animation-polish-01"
CURRENT = OUT / "current"

HTML = r'''<!doctype html>
<html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · Reprise des animations · ANIM-POLISH01</title><link rel="icon" href="data:,">
<style>
:root{color-scheme:dark;font:15px system-ui,sans-serif;background:#11191e;color:#e4ebee}body{margin:24px;max-width:1160px}h1{font-size:25px}p{line-height:1.55;max-width:1000px}a{color:#a8deca}.tag{color:#91cfb9;font-size:13px;letter-spacing:.08em}.controls{display:flex;gap:14px;flex-wrap:wrap;align-items:end;margin:22px 0}label{display:grid;gap:5px;font-size:13px}select,button{font:inherit;color:inherit;background:#23343e;border:1px solid #59707b;border-radius:5px;padding:9px}button{cursor:pointer}.native{overflow:auto;border:1px solid #3c505b;max-width:1100px;background:#1e2831}#frame{display:block;width:1100px;height:760px;image-rendering:pixelated}.note{color:#acbdc6;font-size:14px}.pending{border-left:3px solid #edbd77;padding:9px 16px;background:#1e292e}.timeline{display:flex;align-items:center;gap:15px;margin:15px 0}.timeline input{width:320px}.camp{width:596px;height:552px;overflow:hidden;border:1px solid #3c505b}.camp img{display:block;width:1100px;height:760px;max-width:none;transform:translate(0,-36px);image-rendering:pixelated}#failure{color:#ffb5a2}
</style>
<div class="tag">ANIM-POLISH01 · PREMIER LOT À VALIDER</div>
<h1>Tenues et armures conservées</h1>
<p>Compare le rendu actuel avec deux propositions. Les personnages restent à <strong>64 px</strong>, accompagnés d’un zoom ×2. Le portrait « Visée » des propositions reprend le même repos frontal que le personnage sur la carte.</p>
<div class="pending"><strong>Propositions de revue.</strong> Le jeu normal conserve son rendu actuel. Cette première passe couvre quatre personnages en armure lourde. Les autres tenues et les positions de mains restantes demandent encore leur contrôle.</div>
<div class="controls">
<label>Personnage<select id="body"><option value="3">Bastion</option><option value="5">Traqueuse</option><option value="6">Arcaniste</option><option value="7">Métamorphe</option></select></label>
<label>Arme<select id="weapon"><option value="31">Mitrailleuse lourde</option><option value="51">Lance-roquettes</option><option value="23">Lance-grenades</option><option value="13">Lance-flammes</option><option value="2">Fusil d’assaut · témoin</option></select></label>
<input type="hidden" id="head" value="0">
<button id="pause">Pause</button><label>Vitesse<select id="speed"><option value="1">400 ms par pas</option><option value="3">Ralenti ×3</option></select></label>
</div>
<p class="note">Colonnes : actuel, <strong>01 / réduction modérée</strong>, <strong>02 / réduction accentuée</strong>. Les propositions conservent l’écartement des deux prises. La tête, le portrait et les icônes utilisent le même essai dans 01 et 02. Le fusil témoin conserve son dessin.</p>
<div class="native" style="max-width:540px"><canvas id="compact" width="540" height="636" style="display:block;width:540px;height:636px;image-rendering:pixelated" aria-label="Comparaison côte à côte à taille native, actuel puis 01 puis 02"></canvas></div>
<details><summary>Ouvrir aussi la planche complète avec le zoom ×2</summary><div class="native"><img id="frame" width="1100" height="760" alt="Comparaison native du rendu actuel et des deux propositions, dans les quatre directions"></div></details>
<div class="timeline"><label for="phase">Phase</label><input id="phase" type="range" min="0" max="6" value="0"><output id="time"></output></div>
<p id="failure" role="status"></p>
<p class="note">C01 : petits déplacements de la figure entière pour réduire les oscillations, sans déplacement isolé de la tête. C02-A/B : encombrement des armes lourdes. C03 : icônes au sol de 64 à 40 px. C04 : portrait frontal harmonisé. Les prises restent à juger visuellement : un contact géométrique ne suffit pas à valider l’anatomie.</p>
<h2>Comparaison dans le camp accepté</h2>
<p class="note">De gauche à droite : actuel, 01, 02. Scène rendue dans le moteur avec les matériaux et l’éclairage du camp, cases de 64 px.</p>
<label>Instant<select id="camp"><option value="camp-step0-tick00.png">Début du pas vers le haut</option><option value="camp-step0-tick06.png">Milieu du pas vers le haut</option><option value="camp-step0-tick12.png">Retour au repos</option><option value="camp-step2-tick00.png">Début du pas vers le bas</option><option value="camp-step2-tick06.png">Milieu du pas vers le bas</option><option value="camp-step2-tick12.png">Retour au repos</option></select></label>
<div class="camp"><img id="camp-image" alt="Armes lourdes comparées dans le décor du camp"></div>
<h2>Objets au sol, avec le personnage</h2><p class="note">Extrait du sol et des structures du camp accepté, sans mobilier : même case de 64 px, icône actuelle à gauche et proposition à 40 px à droite.</p><div class="native" style="max-width:640px"><img src="current/ground-native.png" width="640" height="610" style="display:block;max-width:none;image-rendering:pixelated" alt="Taille des objets au sol comparée au personnage dans le camp"></div>
<p class="note"><a href="current/checks.json">Bilan des captures</a> · <a href="current/native-summary.webp">Comparaison native animée compacte</a></p>
<script>
const $=id=>document.getElementById(id);let frames=[],generation=0,paused=false,phase=0,started=performance.now();
function show(){if(frames[phase]?.complete){const im=frames[phase];$('frame').src=im.src;const c=$('compact').getContext('2d');c.imageSmoothingEnabled=false;c.fillStyle='#1e2831';c.fillRect(0,0,540,636);c.font='14px system-ui';c.fillStyle='#e4ebee';for(let s=0;s<3;s++){const x=s*180;c.fillText(['Actuel','01 / modéré','02 / accentué'][s],x+14,22);for(let r=0;r<4;r++){c.fillStyle='#aebfc8';c.fillText(['Face','Dos','Gauche','Droite'][r],x+14,44+r*90);c.drawImage(im,145+s*310,96+r*126,115,80,x+22,46+r*90,115,80);}c.fillText('Portrait Visée',x+14,424);c.drawImage(im,170+s*310,596,96,96,x+24,432,96,96);c.fillText('Objets au sol',x+14,553);[0,2].forEach((i,j)=>c.drawImage(im,158+s*310+i*78,695,64,64,x+14+j*78,565,64,64));}}$('phase').value=phase;$('time').textContent=phase===6?'Repos frontal':`Phase ${phase+1} / 6`;}
async function load(){const g=++generation;$('failure').textContent='';const list=Array.from({length:7},(_,i)=>{const im=new Image();im.src=`current/review-${$('body').value}-${$('weapon').value}-H${$('head').value}-${i}.png`;return im;});try{await Promise.all(list.map(im=>im.decode()));if(g!==generation)return;frames=list;phase=0;started=performance.now();show();}catch(e){$('failure').textContent='Capture manquante : '+e.message;}}
for(const id of ['body','weapon','head'])$(id).onchange=load;
$('pause').onclick=()=>{paused=!paused;$('pause').textContent=paused?'Lecture':'Pause';started=performance.now();};
$('phase').oninput=()=>{paused=true;$('pause').textContent='Lecture';phase=+$('phase').value;show();};
function animate(now){if(!paused){const elapsed=(now-started)/(+$('speed').value);phase=elapsed<800?Math.min(5,Math.floor(elapsed/(800/6))):6;if(elapsed>=1350)started=now;show();}requestAnimationFrame(animate);}
$('camp').onchange=()=>{$('camp-image').src='current/camp/'+$('camp').value;};$('camp').onchange();load();requestAnimationFrame(animate);
</script></html>'''

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    page_html = HTML
    if (ROOT / 'docs/reviews/animation-polish-01/acceptance-partial.json').exists():
        page_html = page_html.replace('ANIM-POLISH01 · PREMIER LOT À VALIDER', 'ANIM-POLISH01 · ACCORD REÇU · LANCE-GRENADES EN REPRISE')
        page_html = page_html.replace('<div class="pending"><strong>Propositions de revue.</strong>', '<div class="pending"><strong>Premier accord reçu.</strong> La stabilisation, les icônes, le portrait et la réduction modérée 01 des armes lourdes sont acceptés. Le lance-grenades reçoit une nouvelle proposition 03. ')
        page_html = page_html.replace('Le jeu normal conserve son rendu actuel. Cette première passe couvre quatre personnages en armure lourde.', 'La revue reste distincte de l’intégration. Elle couvre quatre personnages en armure lourde.')
    grenade = OUT / 'grenade-r01/current'
    if (grenade / 'checks.json').exists():
        section = '''<section><h2>03 · Nouveau modèle de lance-grenades</h2>
<p>À gauche : le modèle actuel. Au centre : sa réduction précédente 02. À droite : <strong>03 / GL01</strong>, redessiné avec un corps plus fin et un canon court. Les personnages et la mitrailleuse témoin sont à taille native.</p>
<div class="native" style="max-width:540px"><img id="gl-frame" width="540" height="490" style="display:block;width:540px;height:490px;image-rendering:pixelated" alt="Lance-grenades actuel, réduction précédente et nouveau dessin 03, avec prises en marche"></div>
<p class="note">Le dessin source passe de 52 × 24 à 44 × 18 px. Cette revue porte sur le modèle et ses prises latérales ; les poses face/dos et les couleurs demanderont leur adaptation à l’écartement des nouvelles prises. Pour les autres armes lourdes, la version 01 — réduction modérée — est retenue.</p>
</section>'''
        page_html = page_html.replace('<p class="note">Colonnes :', section+'\n<p class="note">Colonnes :', 1)
        gl_script = '''let glFrames=[],glGeneration=0;async function loadGL(){let g=++glGeneration;let list=Array.from({length:6},(_,i)=>{let im=new Image();im.src=`grenade-r01/current/gl-${$('body').value}-H${$('head').value}-${i}.png`;return im;});await Promise.all(list.map(im=>im.decode()));if(g===glGeneration){glFrames=list;$('gl-frame').src=list[0].src;}}for(let id of ['body','head'])$(id).addEventListener('change',loadGL);$('phase').addEventListener('input',()=>{if(glFrames.length)$('gl-frame').src=glFrames[(+$('phase').value)%6].src;});function glAnimate(now){const i=Math.floor(now/(133*(+$('speed').value)))%6;if(glFrames[i]?.complete&&!paused)$('gl-frame').src=glFrames[i].src;requestAnimationFrame(glAnimate);}loadGL();requestAnimationFrame(glAnimate);'''
        page_html = page_html.replace('</script></html>', gl_script+'\n</script></html>')
        frames = [Image.open(grenade / f'gl-3-H0-{n}.png').convert('RGB') for n in range(6)]
        frames[0].save(grenade / 'GL01-native.webp', save_all=True, append_images=frames[1:], duration=133, loop=0, lossless=True, method=6)
    armor_section,armor_script=build_armor_outfits()
    page_html=page_html.replace('<div class="controls">',armor_section+'\n<div class="controls">',1)
    page_html=page_html.replace('</script></html>',armor_script+'\n</script></html>')
    page_html=page_html.replace('ANIM-POLISH01 · ACCORD REÇU · LANCE-GRENADES EN REPRISE','ANIM-POLISH01 · APPARENCE DES ARMURES CONSERVÉE')
    if (ROOT/'docs/reviews/animation-polish-01/C03-INTEGRATION.json').exists():
        page_html=page_html.replace('C03 : icônes au sol de 64 à 40 px.', 'C03 : icônes au sol à 40 px, intégrées au jeu le 1er novembre 2026.')
        page_html=page_html.replace('icône actuelle à gauche et proposition à 40 px à droite.', 'référence avant intégration à 64 px à gauche et taille de 40 px maintenant active en jeu à droite.')
    exposure_section,exposure_script=build_exposure()
    if exposure_section:
        page_html=page_html.replace('<section id="armures">',exposure_section+'\n<section id="armures">',1)
        page_html=page_html.replace('</script></html>',exposure_script+'\n</script></html>')
        page_html=page_html.replace('<h1>Tenues et armures conservées</h1>','<h1>Éclairage du joueur : comparatif</h1>')
    firearm_section,firearm_script=build_firearms()
    if firearm_section:
        page_html=page_html.replace('<section id="eclairage">',firearm_section+'\n<section id="eclairage">',1)
        page_html=page_html.replace('</script></html>',firearm_script+'\n</script></html>')
        page_html=page_html.replace('<h1>Éclairage du joueur : comparatif</h1>','<h1>Armes de tir : taille et lisibilité</h1>')
        page_html=page_html.replace('<h1>Armes de tir : taille et lisibilité</h1>','<h1>Armes de tir : taille et lisibilité</h1><nav><a href="#armes-tir">Armes de tir</a> · <a href="#eclairage">Éclairage</a> · <a href="#armures">Armures</a></nav>')
        page_html=page_html.replace('ANIM-POLISH01 · APPARENCE DES ARMURES CONSERVÉE','FIREARM01 · ARMES DE TIR · NOUVEAU CANDIDAT')
        page_html=page_html.replace('Compare le rendu actuel avec deux propositions. Les personnages restent à <strong>64 px</strong>, accompagnés d’un zoom ×2. Le portrait « Visée » des propositions reprend le même repos frontal que le personnage sur la carte.','Compare les armes actuellement en jeu avec le candidat <strong>01 / Compact et lisible</strong>. Les personnages sont montrés à taille native de <strong>64 px</strong>, avec un zoom entier ×2 disponible.')
        start=page_html.index('<div class="pending">')
        end=page_html.index('</div>',start)+6
        page_html=page_html[:start]+'<div class="pending"><strong>Nouvelle réduction à examiner.</strong> L’éclairage Discret, le visuel complet des armures et les icônes au sol à 40 px sont déjà intégrés. Le candidat d’armes ci-dessous reste en revue ; les anciens comparatifs sont conservés plus bas.</div>'+page_html[end:]
    rifle_section,rifle_script=build_rifles()
    if rifle_section:
        page_html=page_html.replace('<section id="armes-tir">',rifle_section+'\n<section id="armes-tir">',1)
        page_html=page_html.replace('</script></html>',rifle_script+'\n</script></html>')
        page_html=page_html.replace('<h1>Armes de tir : taille et lisibilité</h1>','<h1>Fusils : léger ajustement de taille</h1>')
        page_html=page_html.replace('<nav>','<nav><a href="#fusils">Fusils</a> · ',1)
        page_html=page_html.replace('FIREARM01 · ARMES DE TIR · NOUVEAU CANDIDAT','FIREARM02 · FUSILS · AJUSTEMENT 02')
        page_html=page_html.replace('Compare les armes actuellement en jeu avec le candidat <strong>01 / Compact et lisible</strong>.','Compare le précédent essai de fusils avec le candidat <strong>02 / Légèrement agrandi</strong>.')
        page_html=page_html.replace('<strong>Nouvelle réduction à examiner.</strong>','<strong>Léger agrandissement des fusils à examiner.</strong>')
        page_html=page_html.replace('<a href="#armes-tir">Armes de tir</a>','<a href="#armes-tir">Armes lourdes</a>')
        start=page_html.index('<section id="armes-tir">')
        end=page_html.index('</section>',start)+10
        heavy=page_html[start:end]
        heavy=heavy.replace('Armes de tir à deux mains : taille et lisibilité','Armes lourdes : comparaison séparée')
        for value,label in [('2','Fusil d’assaut'),('7','Neral E12'),('35','Fusil de guetteur'),('57','Fusil à pompe')]:
            heavy=heavy.replace(f'<option value="{value}">{label}</option>','')
        heavy=heavy.replace('<option value="31">','<option value="31" selected>')
        first=heavy.index('<p>')
        last=heavy.index('</p>',first)+4
        heavy=heavy[:first]+'<p>Le candidat <strong>01 / Compact et lisible</strong> réduit le gabarit des armes lourdes. Leur comparaison est conservée séparément pendant l’ajustement des fusils.</p>'+heavy[last:]
        heavy=heavy.replace('Les huit armes sont montrées sur les quatre classes en armure lourde. Les cinq degrés d’armure sont aussi comparés sur la Traqueuse, avec le fusil d’assaut et la mitrailleuse.','Les quatre armes lourdes sont montrées sur les quatre classes en armure lourde. Les cinq degrés d’armure sont aussi comparés sur la Traqueuse avec la mitrailleuse.')
        page_html=page_html[:start]+heavy+page_html[end:]
        integration=ROOT/'docs/reviews/animation-polish-01/FIREARM02-INTEGRATION.json'
        if integration.exists() and json.loads(integration.read_text(encoding='utf-8'))['normal_game_enabled']:
            page_html=page_html.replace('FIREARM02 · FUSILS · AJUSTEMENT 02','FIREARM02 · FUSILS · VERSION 02 INTÉGRÉE')
            page_html=page_html.replace('<strong>Léger agrandissement des fusils à examiner.</strong>','<strong>Version 02 des fusils validée et intégrée.</strong>')
            page_html=page_html.replace('Le candidat d’armes ci-dessous reste en revue ; les anciens comparatifs sont conservés plus bas.','Les fusils 02 sont actifs en jeu. Les armes lourdes conservent leur revue séparée ; les anciens comparatifs sont conservés plus bas.')
    correction=ROOT/'docs/reviews/animation-polish-01/LATERAL01-CORRECTION.json'
    if correction.exists() and json.loads(correction.read_text(encoding='utf-8'))['normal_game_enabled']:
        page_html=page_html.replace('.png`','.png?motion=LATERAL01-r01`')
        page_html=page_html.replace('</h1>','</h1><p class="note">Lecture de la marche gauche/droite rétablie. Les anciennes figures latérales restent à harmoniser avec la face et le dos actuels.</p>',1)
    identity_section,identity_script=build_lateral_identity()
    if identity_section:
        page_html=page_html.replace('<section id="fusils">',identity_section+'\n<section id="fusils">',1)
        page_html=page_html.replace('</script></html>',identity_script+'\n</script></html>')
        page_html=page_html.replace('<h1>Fusils : léger ajustement de taille</h1>','<h1>Marche latérale : retrouver la même identité</h1>')
        page_html=page_html.replace('<nav>','<nav><a href="#identite-laterale">Identité latérale</a> · ',1)
        lateral_tag='LATERAL02-r02 · TRAQUEUSE · NOUVEL ESSAI 02' if (OUT/'lateral-identity-02/r02/current/checks.json').exists() else 'LATERAL02-r01 · PREMIER CANDIDAT À EXAMINER'
        page_html=page_html.replace('FIREARM02 · FUSILS · VERSION 02 INTÉGRÉE',lateral_tag)
        if 'LATERAL02-r02' in lateral_tag:
            page_html=page_html.replace('<strong>LATERAL02-r01 en revue.</strong>','<strong>LATERAL02-r02 en revue.</strong>')
        if (OUT/'lateral-identity-02/r03/current/checks.json').exists():
            page_html=page_html.replace(lateral_tag,'LATERAL02-r03 · TRAQUEUSE · FOULÉE RESSERRÉE')
            page_html=page_html.replace('<strong>LATERAL02-r02 en revue.</strong>','<strong>LATERAL02-r03 en revue.</strong>')
        page_html=page_html.replace('Compare le précédent essai de fusils avec le candidat <strong>02 / Légèrement agrandi</strong>. Les personnages sont montrés à taille native de <strong>64 px</strong>, avec un zoom entier ×2 disponible.','La reprise commence par la Traqueuse en armure lourde : nouveau dessin latéral comparé aux vues face/dos à taille native. Les revues précédentes restent disponibles plus bas.')
        page_html=page_html.replace('<strong>Version 02 des fusils validée et intégrée.</strong> L’éclairage Discret, le visuel complet des armures et les icônes au sol à 40 px sont déjà intégrés. Les fusils 02 sont actifs en jeu. Les armes lourdes conservent leur revue séparée ; les anciens comparatifs sont conservés plus bas.','<strong>LATERAL02-r01 en revue.</strong> La lecture de l’animation est corrigée ; ces nouvelles figures latérales attendent leur validation visuelle. Les réglages de fusils, de lumière et d’objets au sol déjà intégrés sont conservés.')
    (OUT / "apercu.html").write_text(page_html, encoding="utf-8")
    sources = [CURRENT / f"native-summary-{n}.png" for n in range(7)]
    if all(p.exists() for p in sources):
        frames = [Image.open(p).convert("RGB") for p in sources]
        frames[0].save(CURRENT / "native-summary.webp", save_all=True,
                       append_images=frames[1:], duration=[133]*6+[550],
                       loop=0, lossless=True, method=6)
    main_preview = ROOT / "artifacts/refonte-agent-2026-10-06/apercu-animations.html"
    page = main_preview.read_text(encoding="utf-8")
    if 'animation-polish-01/apercu.html' not in page:
        page = page.replace('<h1>', '<p><a href="../animation-polish-01/apercu.html">ANIM-POLISH01 · Reprise tête, armes lourdes, objets au sol et portrait Visée · à valider</a></p>\n<h1>', 1)
        main_preview.write_text(page, encoding="utf-8")
    print(json.dumps({"preview": str(OUT / "apercu.html"),
                      "capture_count": len(list(CURRENT.glob("review-*-H0-*.png"))),
                      "native_animation": (CURRENT / "native-summary.webp").exists()}))

if __name__ == "__main__":
    main()
