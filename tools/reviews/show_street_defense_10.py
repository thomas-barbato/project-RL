"""Publish native engine views and a short animated proposal in the current review."""
from pathlib import Path
from html.parser import HTMLParser
from urllib.parse import urlsplit,unquote
import json
import hashlib
import shutil
from PIL import Image, ImageDraw, ImageFont
from build_street_defense_10 import ROOT,ART,DOC,OUT,MAP,read,write,sha

PREFIX='../street-defense-review-10/'
def publish():
    proposal=read(ART/'defense.json');checks=read(OUT/'engine/checks.json')
    assert checks['review']==proposal['review'], 'Engine captures belong to another revision'
    assert checks['central_lane_clear'] and not checks['production_map_changed']
    assert (OUT/'engine/gate-proposal.png').stat().st_mtime>=max((ROOT/a['file']).stat().st_mtime for a in proposal['assets']), 'Capture the current native assets before publishing'
    assert proposal['production_sha256']=={p.relative_to(ROOT).as_posix():sha(p) for p in MAP.rglob('*') if p.is_file()}
    # Reject minimized/empty frames before overwriting any review references.
    frames=list((OUT/'engine').glob('*.png'))
    for frame in frames:
        if frame.name=='map-proposal.png':continue
        with Image.open(frame) as im:
            assert im.width>=1552 and im.height>=894, f'Invalid capture dimensions: {frame.name} {im.size}'
            assert im.convert('RGB').getbbox(), f'Empty capture: {frame.name}'
    stage=OUT/'native';stage.mkdir(exist_ok=True)
    proofs=DOC/'references';proofs.mkdir(exist_ok=True)
    scenes=[('gate','Le poste de garde depuis la porte'),('approach','Le poste et les traces du combat'),
        ('south-fire','Incendie dans la ruine près du portail'),('van-fire','Épave incendiée au carrefour'),
        ('west-fire','Façade effondrée à l’ouest'),('court-fire','Gravats de la cour')]
    for key,_ in scenes:
        for suffix in ['before','proposal']:
            im=Image.open(OUT/'engine'/f'{key}-{suffix}.png').convert('RGB').crop((334,50,1552,894))
            im.save(stage/f'{key}-{suffix}.png');shutil.copy2(stage/f'{key}-{suffix}.png',proofs/f'{key}-{suffix}.png')
    for key,box in [('gate',(540,120,1360,640)),('south-fire',(440,100,1450,750))]:
        frames=[Image.open(OUT/'engine'/f'{key}-motion-{i:02}.png').convert('RGB').crop(box) for i in range(12)]
        frames[0].save(stage/f'{key}-animated.gif',save_all=True,append_images=frames[1:],duration=125,loop=0,optimize=False)
        shutil.copy2(stage/f'{key}-animated.gif',proofs/f'{key}-animated.gif')
    Image.open(OUT/'engine/gate-proposal.png').crop((540,120,1360,640)).save(OUT/'entree-native.png')
    # Technical contact sheet: existing sprites only, one source pixel per
    # display pixel, aligned at the real foot anchors. No artwork is stretched.
    comparison=Image.new('RGBA',(540,172),'#303b40')
    draw=ImageDraw.Draw(comparison)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
    small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    draw.text((14,10),'Même ligne de sol · taille réelle, sans agrandissement',font=font,fill='#dfc89e')
    draw.line((20,112,520,112),fill='#acb7b8',width=1)
    stature=[]
    for center,code,file,anchor,label in [
        (90,'FIN-01',ROOT/'assets/sprites/art-finish-01/FIN-01.png',[64,96],'Joueur'),
        (270,'GARD-U01-01',ROOT/proposal['guards'][0]['file'],proposal['guards'][0]['foot_anchor'],'Garde 01'),
        (450,'GARD-U01-02',ROOT/proposal['guards'][1]['file'],proposal['guards'][1]['foot_anchor'],'Garde 02')]:
        sprite=Image.open(file).convert('RGBA')
        box=sprite.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
        height=box[3]-box[1]
        comparison.alpha_composite(sprite,(center-anchor[0],112-anchor[1]))
        draw.text((center-52,126),label,font=font,fill='#e4e7e3')
        draw.text((center-52,148),f'Silhouette : {height} px',font=small,fill='#bcc9ca')
        stature.append(dict(id=code,canvas=list(sprite.size),foot_anchor=anchor,
            opaque_height_px=height,alpha_threshold=128,source_sha256=sha(file)))
    comparison.save(stage/'stature-native.png')
    shutil.copy2(stage/'stature-native.png',proofs/'stature-native.png')
    shutil.copy2(OUT/'engine/map-proposal.png',proofs/'map-proposal.png')
    parts=['''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>DEFENSE-U01-10 · poste de garde et incendies</title><style>body{background:#11181c;color:#d3d9d7;margin:24px;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#dfc89e}p{max-width:1100px}a{color:#ddbd84}nav,.buttons{display:flex;gap:12px;flex-wrap:wrap}button{background:#28352f;color:#dfd4bd;border:1px solid #687466;padding:8px 14px;font:inherit;cursor:pointer}button[aria-pressed=true]{border-color:#dec694;background:#474c36}.scroll{overflow:auto;max-width:100%;border:1px solid #46514b;margin:18px 0}img{display:block;max-width:none}.map{height:760px}.cards{display:flex;gap:20px;flex-wrap:wrap}.card{border:1px solid #46514b;padding:14px;min-width:190px}.tile{display:flex;gap:20px;align-items:end;min-height:90px;background:#77766e;padding:16px}.dark{background:#192126}.note{color:#aab6b0}section{margin:36px 0}</style>
<h1>DEFENSE-U01-10 r05 · cadavres conservés, obstacles supplémentaires retirés</h1>
<p><strong>Prévisualisation uniquement.</strong> Deux sentinelles en acier clair, avec une Gatling sombre plus compacte et lisible, occupent deux positions de sacs de sable en L reliées aux montants du portail. Le passage central reste libre. Les pieds des gardes sont placés sur la chaussée, à l’intérieur des protections et en avant des piliers.</p>
<p>Le bloc de béton et les deux obstacles antichars ajoutés en r04 sont retirés. Les deux protections en sacs de sable des gardes sont conservées. L’approche reste dégagée et le portail conserve son ouverture actuelle. Les parcours vers la rue, les deux flancs et la sortie nord sont contrôlés dans le moteur.</p>
<p>Les neuf nouveaux cadavres sont validés et conservés aux positions montrées en r04. Le lot garde ses vingt et un corps ajoutés à la carte actuelle : sept civils, sept cultistes et sept militaires, avec poses et orientations variées. Les onze traces de sang restent aux mêmes endroits. Les corps sont des décors franchissables. Les quatre foyers animés et leurs vrais éclairages sont conservés.</p>
<p class="note">Même carte, mêmes cases de 64 px, mêmes personnages et vision de six cases. Les gardes sont ici des figurants animés pour juger la scène ; aucun tir, dégât, nouveau dialogue ou comportement de combat n’est activé par cet aperçu.</p>
<nav><a href="#entree">Entrée animée</a><a href="#incendie">Incendie animé</a><a href="#gate">Vues comparées</a><a href="#map">Carte entière</a><a href="#dessins">Dessins numérotés</a></nav>
<section id="entree"><h2>À la sortie de la prison · taille réelle</h2><div class="scroll"><img src="../street-defense-review-10/native/gate-animated.gif" width="820" height="520" alt="Poste de garde animé dans le moteur"></div><h3>Stature comparée au joueur</h3><p>Les gardes sont légèrement renforcés à partir de leurs grands dessins sources : silhouette de 62 px contre 60 px pour le corps du joueur, hors franges d’alpha très faibles. Leurs pieds sont alignés sur la comparaison ci-dessous ; tous les dessins gardent leur échelle native. Les cases du jeu restent de 64 px.</p><div class="scroll"><img src="../street-defense-review-10/native/stature-native.png" width="540" height="172" alt="Joueur et deux gardes alignés sur le même sol, sans zoom"></div><details><summary>Précédente proposition r02 · même recadrage natif</summary><div class="scroll"><img src="../../docs/reviews/refonte-histoire-2026-10-16/street-defense-review-10/rejected-r02/entry-native.png" width="820" height="520" alt="Ancienne position et armures plus sombres"></div></details></section>
<section id="incendie"><h2>Flammes et éclairage · taille réelle</h2><div class="scroll"><img src="../street-defense-review-10/native/south-fire-animated.gif" width="1010" height="650" alt="Incendie animé dans le moteur"></div></section>''']
    parts.append('<details><summary>Comparer avec la proposition r03 précédente</summary><div class="scroll"><img src="../../docs/reviews/refonte-histoire-2026-10-16/street-defense-review-10/previous-r03/gate-proposal.png" width="1218" height="844" loading="lazy" alt="Poste de garde avant les nouveaux obstacles"></div><div class="scroll"><img src="../../docs/reviews/refonte-histoire-2026-10-16/street-defense-review-10/previous-r03/approach-proposal.png" width="1218" height="844" loading="lazy" alt="Approche avant les nouveaux obstacles et corps"></div></details>')
    for key,title in scenes:
        parts.append(f'<section id="{key}"><h2>{title}</h2><div class="buttons"><button data-img="{key}" data-src="{PREFIX}native/{key}-before.png" aria-pressed="false">Carte actuelle</button><button data-img="{key}" data-src="{PREFIX}native/{key}-proposal.png" aria-pressed="true">Proposition</button></div><div class="scroll"><img id="{key}-image" src="{PREFIX}native/{key}-proposal.png" width="1218" height="844" loading="lazy" alt="Vue du moteur sans agrandissement"></div></section>')
    parts.append(f'<section id="map"><h2>Répartition sur la carte entière</h2><p>Vue technique du décor sans masque de vision ni joueur, pour examiner les placements. La vision de six cases reste active dans les scènes ci-dessus.</p><div class="buttons"><button data-focus="0">Nord</button><button data-focus="1700">Centre</button><button data-focus="3250">Entrée au sud</button></div><div class="scroll map" id="map-window"><img src="{PREFIX}engine/map-proposal.png" width="1664" height="4224" alt="Carte entière avec la proposition"></div></section>')
    parts.append('<section id="dessins"><h2>Dessins candidats · aucun zoom</h2><div class="cards">')
    for code,title,w,h in [('GARD-U01-01','Acier clair · Gatling',64,64),('GARD-U01-02','Acier clair · Gatling',64,64),('SACS-U01-02','Barricade en L',80,72)]:
        item=next(a for a in proposal['assets'] if a['id']==code)
        file='../../'+item['file']
        title+=' · '+item['revision'].lstrip('-')
        parts.append(f'<div class="card"><h3>{code} · {title}</h3><div class="tile"><img src="{file}" width="{w}" height="{h}" alt="Dessin natif sur fond clair"></div><div class="tile dark"><img src="{file}" width="{w}" height="{h}" alt="Dessin natif sur fond sombre"></div></div>')
    parts.append('''</div></section><p><a href="../../docs/reviews/refonte-histoire-2026-10-16/street-defense-review-10/README.md">Composition et limites de cette proposition</a> · <a href="../../assets/source-art/first-sector-review-01/u01-defense-r10/prompts-r03.json">Sources et prompts des sentinelles · imagegen intégré</a></p><script>document.querySelectorAll('[data-img]').forEach(b=>b.onclick=()=>{document.getElementById(b.dataset.img+'-image').src=b.dataset.src;document.querySelectorAll('[data-img="'+b.dataset.img+'"]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});document.querySelectorAll('[data-focus]').forEach(b=>b.onclick=()=>document.getElementById('map-window').scrollTop=+b.dataset.focus);document.getElementById('map-window').scrollTop=3250;</script></html>''')
    html='\n'.join(parts)
    for page in [OUT/'index.html',ROOT/'artifacts/street-defects-review-01/index.html']:page.write_text(html,encoding='utf-8')
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):self.refs.extend(v for k,v in attrs if k in ['href','src','data-src'])
    parser=Refs();parser.feed(html);checked=0
    for ref in parser.refs:
        path=urlsplit(ref).path
        if not path:continue
        assert ((OUT/unquote(path)).resolve()).is_file(),ref
        checked+=1
    write(DOC/'checks.json',dict(engine=checks,production_assets_unchanged=True,page_local_references_checked=checked,
        native_source_assets=3,master_images_generated_with='built-in imagegen',runtime_integrated=False,
        circulation=proposal['circulation'],
        stature_same_native_scale=stature,
        immutable_reference_sha256={p.name:sha(p) for p in proofs.iterdir() if p.is_file()}))
    print('Published DEFENSE-U01-10; native scenes, animated fires and',checked,'local references checked; no activation.')

if __name__=='__main__':publish()
