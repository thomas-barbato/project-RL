"""Publish the current morphology batch and a separately scoped rifle study."""
from pathlib import Path
import argparse
import json
import re
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/animation-polish-01/biped-lateral-01/r01/current'
PAGE=ROOT/'artifacts/animation-polish-01/apercu.html'

SECTION='''<section id="bipedes"><h2>01 · Bipèdes : appuis et port d’arme</h2>
<p>Premier lot : <strong>01 Possédé · 02 Cultiste · 03 Démon massif · 04 Démon ailé au sol</strong>. Chaque corps conserve son anatomie et ses accessoires. <strong>05 Bastion au fusil</strong> examine séparément les deux prises pendant la marche.</p>
<p class="pending">Nouveaux dessins à examiner : huit poses, 650 ms par pas, puis vrai repos frontal. Le reste du bestiaire et les autres configurations d’armes restent à reprendre.</p>
<div class="controls"><label>Créature ou port d’arme<select id="biped-item">__OPTIONS__</select></label><label>Trajet des bipèdes<select id="biped-direction"><option value="both">Aller-retour</option><option value="left">Vers la gauche</option><option value="right">Vers la droite</option></select></label><button id="biped-play">Pause</button><button id="biped-next">Pose suivante</button><label>Cadence des bipèdes<select id="biped-pace"><option value="650">650 ms par pas</option><option value="800">800 ms par pas</option><option value="400">400 ms · ancien rythme</option></select></label></div>
<label>Comparer les bipèdes<select id="biped-pair"><option value="walk">Marche actuelle / nouveau pas</option><option value="idle">Repos actuel / nouveau pas</option><option value="all">Les trois colonnes</option></select></label>
<p id="biped-columns" class="note">Marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px ; mêmes décor et éclairage.</p>
<div class="native" style="max-width:480px"><canvas id="biped-board" width="320" height="192" style="display:block;width:320px;height:192px;image-rendering:pixelated" aria-label="Bipède au repos, marche actuelle et nouvelle démarche"></canvas></div>
<div class="timeline" style="display:flex;flex-wrap:wrap;gap:8px"><label>Phase des bipèdes<input id="biped-phase" type="range" min="0" max="8" value="0"></label><output id="biped-time" style="flex-basis:100%;white-space:normal">Contact A</output></div><p id="biped-status" class="note" role="status">Préparation des captures…</p><p id="biped-scope" class="note"></p>
<details id="biped-zoom-details"><summary>Zoom entier ×2 des bipèdes</summary><div class="native"><canvas id="biped-zoom" width="640" height="384" style="display:block;width:640px;height:384px;max-width:none;image-rendering:pixelated"></canvas></div></details>
<p class="note">Le Bastion utilise le dessin partagé du fusil d’assaut et des bras dessinés pour les deux prises. Cet essai couvre sa tenue lourde et cette arme uniquement. Il ne valide pas encore toutes les armes ni toutes les classes.</p>
<p class="note">87 sources déclarées à deux appuis ont été repérées ; certaines demandent une correction de classement avant dessin. Les oiseaux, corps rampants, appuis multiples et formes flottantes suivront leur anatomie propre.</p>
<p class="note"><a href="biped-lateral-01/r01/current/checks.json">Bilan natif</a> · <a href="biped-lateral-01/r01/coverage.json">Couverture du bestiaire</a> · <a id="biped-animation" href="biped-lateral-01/r01/current/MON-005-0/native.webp">Animation native sélectionnée</a></p>
</section>'''

SCRIPT='''let bipedCache=new Map(),bipedBank=null,bipedReady=false,bipedPaused=false,bipedElapsed=0,bipedLast=0,bipedManual=null,bipedFacing='left',bipedPainted='';
const bipedItems=__ITEMS__,bipedDurations=[180,180,170,120,180,180,170,120],bipedSamples=[0,5,9,14,16,21,25,30,32],bipedNames=['Contact A','Appui A','Passage du pied B','Reprise du pied B','Contact B','Appui B','Passage du pied A','Reprise du pied A','Repos frontal'];
function bipedPhaseAt(t){let end=0;for(let p=0;p<8;p++){end+=bipedDurations[p];if(t<end)return p;}return 7;}
function bipedSelection(){const direction=$('biped-direction').value;let side=direction==='right'?'right':'left',sample=0,phase=0;if(bipedManual!==null){side=bipedFacing;phase=bipedManual;sample=bipedSamples[phase];}else{let t=bipedElapsed%(direction==='both'?3900:1950);if(direction==='both')side=t<1950?'left':'right';t%=1950;phase=t>=1300?8:bipedPhaseAt(t);sample=t>=1300?32:Math.min(31,Math.floor(t/(1300/32)));}return {file:side+'-'+String(sample).padStart(2,'0')+'.png',side,phase};}
function blitBiped(canvas,image,scale){const pair=$('biped-pair').value,columns=pair==='all'?[0,1,2]:pair==='idle'?[0,2]:[1,2],width=columns.length*160*scale;if(canvas.width!==width){canvas.width=width;canvas.style.width=width+'px';}const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;columns.forEach((source,col)=>ctx.drawImage(image,source*160,0,160,192,col*160*scale,0,160*scale,192*scale));}
function paintBiped(){if(!bipedReady)return;const s=bipedSelection();bipedFacing=s.side;const key=s.file+$('biped-pair').value;if(key!==bipedPainted){const image=bipedBank.get(s.file);blitBiped($('biped-board'),image,1);bipedPainted=key;if($('biped-zoom-details').open)blitBiped($('biped-zoom'),image,2);}$('biped-phase').value=s.phase;$('biped-time').textContent=(s.side==='left'?'Gauche · ':'Droite · ')+bipedNames[s.phase];}
async function loadBiped(){bipedReady=false;bipedBank=null;const id=$('biped-item').value,item=bipedItems.find(i=>i.id===id);for(const name of ['biped-item','biped-play','biped-next','biped-phase'])$(name).disabled=true;$('biped-status').textContent='Préparation des captures…';try{if(bipedCache.has(id)){bipedBank=bipedCache.get(id);bipedCache.delete(id);bipedCache.set(id,bipedBank);}else{while(bipedCache.size>=2)bipedCache.delete(bipedCache.keys().next().value);const images=item.captures.map(file=>{const image=new Image();image.src='biped-lateral-01/r01/current/'+id+'/'+file+'?revision=BIPED-LATERAL01-r01';return {file,image};});await Promise.all(images.map(({image})=>image.decode()));bipedBank=new Map(images.map(({file,image})=>[file,image]));bipedCache.set(id,bipedBank);}bipedReady=true;bipedPainted='';bipedElapsed=0;bipedManual=null;paintBiped();$('biped-status').textContent='66 / 66 captures prêtes · '+item.name+' · taille native.';$('biped-scope').textContent=item.player?'05 · Étude des prises : Bastion, tenue lourde, fusil d’assaut partagé, palette d’origine.':'Créature '+item.id+' · palette d’origine · huit poses adaptées à sa morphologie.';$('biped-animation').href='biped-lateral-01/r01/current/'+id+'/native.webp';}catch(error){$('biped-status').textContent='Capture indisponible : '+error.message;}finally{for(const name of ['biped-item','biped-play','biped-next','biped-phase'])$(name).disabled=false;}}
function freezeBiped(phase){bipedPaused=true;bipedManual=phase;$('biped-play').textContent='Reprendre les bipèdes';paintBiped();}
$('biped-item').onchange=loadBiped;$('biped-phase').oninput=()=>freezeBiped(+$('biped-phase').value);$('biped-next').onclick=()=>freezeBiped((+$('biped-phase').value+1)%9);
$('biped-play').onclick=()=>{if(bipedPaused&&bipedManual!==null){bipedElapsed=bipedDurations.slice(0,Math.min(8,bipedManual)).reduce((a,b)=>a+b,0)+($('biped-direction').value==='both'&&bipedFacing==='right'?1950:0);bipedManual=null;}bipedPaused=!bipedPaused;$('biped-play').textContent=bipedPaused?'Reprendre les bipèdes':'Pause';};
$('biped-direction').onchange=()=>{bipedElapsed=0;bipedManual=null;bipedFacing=$('biped-direction').value==='right'?'right':'left';bipedPainted='';paintBiped();};$('biped-zoom-details').ontoggle=()=>{bipedPainted='';paintBiped();};
$('biped-pair').onchange=()=>{$('biped-columns').textContent=$('biped-pair').value==='all'?'Repos actuel · marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px.':$('biped-pair').value==='idle'?'Repos actuel · nouveau cycle. Taille native, cases de 64 px.':'Marche actuelle au même rythme · nouveau cycle. Taille native, cases de 64 px.';bipedPainted='';paintBiped();};
function animateBiped(now){if(bipedLast&&bipedReady&&!bipedPaused)bipedElapsed+=(now-bipedLast)*(650/(+$('biped-pace').value));bipedLast=now;paintBiped();requestAnimationFrame(animateBiped);}loadBiped();requestAnimationFrame(animateBiped);'''


def main(page_only=False, labels=None, headers=None, step_ms=650):
    data=json.loads((OUT/'checks.json').read_text(encoding='utf-8-sig'))
    panel_width=data.get('panel_width',160); board_width=3*panel_width
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',10)
    # Evenly distribute integer frame milliseconds over the two-step cycle.
    short=[round((i+1)*2*step_ms/32)-round(i*2*step_ms/32) for i in range(32)]
    assert sum(short)==2*step_ms
    labels=labels or ['01 · Possédé','02 · Cultiste','03 · Démon massif','04 · Démon ailé au sol','05 · Bastion au fusil']
    headers=headers or ['Repos actuel','Marche actuelle','Nouveau pas']
    options=[]
    for item,label in zip(data['items'],labels):
        options.append(f'<option value="{item["id"]}">{label}</option>')
        if page_only:continue
        for name in item['captures']:
            path=OUT/item['id']/name;im=Image.open(path).convert('RGB');assert im.size==(board_width,192)
            actor=im.crop((0,32,board_width,192)).tobytes();d=ImageDraw.Draw(im);d.rectangle((0,0,board_width-1,31),fill='#1e2831')
            for col,text in enumerate(item.get('headers', headers)):d.text((col*panel_width+5,3),text,font=font,fill='#a5dfba' if col==2 else '#d2dde1')
            d.text((5,17),label,font=small,fill='#b6cbd2')
            assert actor==im.crop((0,32,board_width,192)).tobytes();im.save(path)
        frames=[Image.open(OUT/item['id']/name).convert('RGB') for name in item['captures']]
        frames[0].save(OUT/item['id']/'native.webp',save_all=True,append_images=frames[1:],duration=(short+[step_ms])*2,loop=0,lossless=True,method=4)
    coverage=ROOT/'docs/reviews/animation-polish-01/BIPED-LATERAL01-COVERAGE.json'
    (OUT.parent/'coverage.json').write_bytes(coverage.read_bytes())
    html=PAGE.read_text(encoding='utf-8')
    section=SECTION.replace('__OPTIONS__',''.join(options))
    if '<section id="bipedes">' in html:html=re.sub(r'<section id="bipedes">.*?</section>',lambda _:section,html,count=1,flags=re.S)
    else:html=html.replace('<section id="reprise-laterale">',section+'\n<section id="reprise-laterale">',1)
    script=SCRIPT.replace('__ITEMS__',json.dumps(data['items'],ensure_ascii=False))
    if 'let bipedCache=' in html:html=re.sub(r'let bipedCache=.*?loadBiped\(\);requestAnimationFrame\(animateBiped\);',lambda _:script,html,count=1,flags=re.S)
    else:html=html.replace('</script></html>',script+'\n</script></html>')
    if '<a href="#bipedes">' not in html:html=html.replace('<a href="#reprise-laterale">','<a href="#bipedes">Monstres et prises</a> · <a href="#reprise-laterale">',1)
    html=re.sub(r'<title>.*?</title>','<title>Project RL · Bipèdes et prises · BIPED-LATERAL01-r01</title>',html,count=1)
    html=re.sub(r'(<div class="tag">).*?(</div>)',r'\g<1>BIPED-LATERAL01-r01 · MONSTRES ET PORT DU FUSIL\g<2>',html,count=1)
    html=html.replace('Bastion, Arcaniste et Métamorphe : cadence et appuis repris.','Premier lot de monstres bipèdes et essai de port du fusil ; les trois classes restent comparables plus bas.')
    assert 'Technomancien' not in html
    PAGE.write_text(html,encoding='utf-8')
    print(f'Published {len(data["items"])} native motion studies')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--page-only',action='store_true');args=parser.parse_args()
    if (ROOT/'artifacts/animation-polish-01/biped-lateral-03/r01/current/checks.json').exists():
        from build_biped_armed03_review import main as current
        current(page_only=args.page_only)
    elif (ROOT/'artifacts/animation-polish-01/biped-lateral-02/r01/current/checks.json').exists():
        from build_biped_next02_review import main as current
        current(page_only=args.page_only)
    elif (ROOT/'artifacts/animation-polish-01/biped-lateral-01/r03/current/checks.json').exists():
        from build_biped_player03_review import main as current
        current(page_only=args.page_only)
    elif (ROOT/'artifacts/animation-polish-01/biped-lateral-01/r02/current/checks.json').exists():
        from build_biped_low02_review import main as current
        current(page_only=args.page_only)
    else:main(page_only=args.page_only)
