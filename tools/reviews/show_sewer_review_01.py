"""Publish EG01 proposals, native engine views and the small ambient rat cycle."""
from pathlib import Path
from html import escape
from html.parser import HTMLParser
from urllib.parse import unquote,urlsplit
import json
import shutil
from PIL import Image,ImageDraw,ImageFont
from build_sewer_review_01 import ROOT,ART,DOC,OUT,ACTIVE,read,write,sha,hashes,SEEDS

def publish():
    proposal=read(DOC/'proposal.json');engine=read(OUT/'engine/checks.json')
    assert proposal['normal_map_sha256']==hashes(),'Normal map changed'
    assert engine['review']=='EG01' and not engine['normal_map_changed']
    labels=['entrance-before','entrance-proposal']
    labels += [f'seed-{i}-{v}' for i in range(1,4) for v in ['access','collector','encounter']]
    labels += [f'street-{i}-{v}' for i in range(1,3) for v in ['north','central']]
    asset_time=max(p.stat().st_mtime for p in (ART/'kit').glob('*-native.png'))
    for label in labels:
        p=OUT/'engine'/f'{label}.png'
        with Image.open(p) as im:
            assert im.width>=1552 and im.height>=894,(p.name,im.size)
            assert im.convert('RGB').getbbox(),p.name
        assert p.stat().st_mtime>=asset_time,'Recapture the current source assets before publishing'
    native=OUT/'native';native.mkdir(exist_ok=True)
    for label in labels:
        with Image.open(OUT/'engine'/f'{label}.png') as im:
            im.convert('RGB').crop((334,50,1552,894)).save(native/f'{label}.png')
    frames=[]
    for i in range(12):
        with Image.open(OUT/'engine'/f'water-{i:02}.png') as im:
            assert im.width>=1552 and im.height>=894
            frames.append(im.convert('RGB').crop((334,50,1552,762)))
    frames[0].save(native/'sewer-ambient.gif',save_all=True,append_images=frames[1:],duration=180,loop=0,optimize=True)
    # Native scale proof. No enlarged rat in a misleading 64px actor box.
    board=Image.new('RGBA',(520,165),'#28302e');d=ImageDraw.Draw(board)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
    d.line((18,112,502,112),fill='#8e9b90',width=1)
    player=Image.open(ROOT/'assets/sprites/art-finish-01/FIN-01.png').convert('RGBA')
    board.alpha_composite(player,(110-64,112-96))
    rat=Image.open(ART/'kit/rat-native.png').convert('RGBA')
    for i in range(4):board.alpha_composite(rat.crop((i*20,0,(i+1)*20,12)),(258+i*55,102))
    d.text((48,126),'Humain · taille de jeu',font=font,fill='#d7dbd3')
    d.text((235,126),'Rat · 18 px, queue comprise',font=font,fill='#d7dbd3')
    board.convert('RGB').save(native/'rat-scale.png')
    prefix='../sewer-review-01/'
    asset_prefix='../../assets/source-art/sewer-review-01/'
    def scene(label,caption):
        return f'<figure><figcaption>{escape(caption)}</figcaption><div class="native"><img loading="lazy" src="{prefix}native/{label}.png" width="1218" height="844" alt="{escape(caption)}"></div></figure>'
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>EG01 · Égouts et peuplement</title><style>*{box-sizing:border-box}html{scroll-behavior:smooth}body{background:#10171a;color:#d4d9d4;font:16px/1.6 system-ui;margin:0;padding:24px}main{max-width:1280px;margin:auto}h1,h2,h3{color:#d9c69e}h1{font-size:27px}h2{font-size:22px;margin-top:36px}h3{font-size:18px}a{color:#a8c9d0}nav{display:flex;gap:20px;flex-wrap:wrap}figure{margin:20px 0 30px}figcaption{color:#bbc4bf;margin-bottom:8px}.native{overflow:auto;max-width:100%;border:1px solid #4c595b;background:#090d10}.native img{display:block;max-width:none}.notice{border-left:3px solid #b1985d;padding:12px 18px;background:#1e2728}.tabs{display:flex;gap:10px;flex-wrap:wrap;margin:18px 0}button{background:#273336;color:#e4d7b8;border:1px solid #667475;padding:9px 18px;cursor:pointer;font:inherit}button[aria-pressed=true]{background:#48544d;border-color:#cbb77c}[hidden]{display:none!important}.overview{max-width:950px;position:relative;background:#070c0e;border:1px solid #495557}.overview img{width:100%;height:auto;display:block}.overview svg{position:absolute;inset:0;width:100%;height:100%}.kit{display:flex;gap:18px;flex-wrap:wrap;align-items:flex-start}.item{background:#27312f;border:1px solid #4b5754;padding:14px;min-width:160px}.item img{display:block;margin:10px auto;max-width:none}.item p{font-size:14px;margin:0}.small{font-size:14px;color:#b8c1ba}code{color:#e1cea6}.facts{display:flex;gap:25px;flex-wrap:wrap;padding:12px 0;color:#d9cdb1}li{margin:8px 0}.source{font-size:14px}</style><main>
<h1>EG01 · Une entrée d’égouts et un premier donjon variable</h1>
<p class="notice"><strong>Proposition à examiner, pas une activation dans la campagne.</strong> Les dessins, l’entrée, le modèle de génération et les placements peuvent être jugés séparément. Les dégâts, les protections et le gain d’XP restent inchangés.</p>
<nav><a href="#entry">Entrée sur le trottoir gauche</a><a href="#dungeon">Trois générations</a><a href="#ambience">Rats et ambiance</a><a href="#street">Peuplement de la rue</a><a href="#kit">Éléments séparés</a><a href="#rules">Règles proposées</a></nav>
<h2 id="entry">EG-ENT-01 r02 · Entrée sur le trottoir gauche · X 2, Y 14</h2>
<p>La trappe est déplacée à gauche, sur le trottoir, comme demandé après le premier aperçu. Elle laisse la chaussée et la bordure dégagées. La portion de mur effondrée, la clôture arrachée, la position détruite et les débris de l’impact sont retirés comme éléments entiers. Le sol est recomposé avec les mêmes dalles usées et la même chaussée.</p>'''
    html+=scene('entrance-proposal','Proposition dans le moteur · même caméra et même échelle que la rue')
    html+='<details><summary>Comparer avec l’emplacement actuel</summary>'+scene('entrance-before','Emplacement actuel avant retrait des débris')+'</details>'
    html+='''<h2 id="dungeon">EG-GEN-01 · Même kit, trois organisations</h2>
<p>Pour ce prototype : dix modules de salles, galeries reliées, boucles et embranchements, canaux franchis par des grilles, petites salles de maintenance et sas d’accès. Les trois essais gardent les mêmes matériaux et le même répertoire de monstres : zombies et cultistes, avec un profil de tir parmi ces derniers. Les salles, leurs liaisons et les groupes varient selon la graine.</p>
<p class="small">Format essayé : 47 × 44 cases de 64 px. C’est une dimension de prototype à juger, pas une taille imposée pour tous les futurs donjons. Les vues locales ci-dessous restent à un pixel pour un pixel. Les plans d’ensemble sont réduits et explicitement séparés des vues natives.</p><div class="tabs">'''
    for i,seed in enumerate(SEEDS,1):html+=f'<button data-seed="{i}" aria-pressed="{str(i==1).lower()}">Génération {i} · {seed}</button>'
    html+='</div>'
    for i,seed in enumerate(SEEDS,1):
        p=read(ART/f'seed-{i}/plan.json');count=sum(len(g['members']) for g in p['groups'])
        html+=f'<section data-panel="{i}"'+(' hidden' if i!=1 else '')+f'><h3>Génération {i}</h3><div class="facts"><span>{len(p["rooms"])} salles</span><span>{len(p["groups"])} groupes</span><span>{count} ennemis proposés</span><span>{p["floor_count"]} cases accessibles</span></div>'
        html+=scene(f'seed-{i}-collector','Collecteur et passerelles · rendu réel éclairé, acteurs à leur échelle')
        html+=scene(f'seed-{i}-access','Sas d’accès · escalier de retour proposé vers la rue')
        html+=scene(f'seed-{i}-encounter','Exemple de groupe placé sur une berge accessible')
        html+='<details><summary>Voir le plan d’ensemble et les groupes</summary><p class="small">Vue technique réduite, sans le cercle de vision. Les cercles numérotés indiquent les centres des groupes ; les captures locales montrent les véritables sprites.</p>'
        html+=f'<div class="overview"><img loading="lazy" src="{prefix}engine/seed-{i}-map.png" width="3008" height="2816" alt="Plan du donjon {i}"><svg viewBox="0 0 3008 2816" xmlns="http://www.w3.org/2000/svg">'
        for g in p['groups']:
            x,y=g['centre'];color='#d4a967' if g['family']=='cultist' else '#91b1a0'
            html+=f'<g><title>G{g["id"]} · {g["family"]} · {len(g["members"])} ennemis</title><circle cx="{x*64+32}" cy="{y*64+32}" r="30" fill="#11191c" stroke="{color}" stroke-width="6"/><text x="{x*64+32}" y="{y*64+44}" fill="{color}" font-size="38" text-anchor="middle">{g["id"]}</text></g>'
        html+=f'</svg></div><p><a href="{prefix}engine/seed-{i}-map.png">Ouvrir la carte entière à taille native</a></p></details></section>'
    html+='''<h2 id="ambience">EG-RAT-01 / EG-TOX-01 / EG-PIP-01 · Petits rats et égouts sales</h2>
<p>Conduites rouillées, métal terni, vannes, dépôts brunâtres et verdâtres près des sorties, eau sombre et éclairage local calculé par le moteur. Quelques lampes restent éteintes. Les rats sont de petits décors mobiles : quatre poses, quelques pauses, aucun rôle d’ennemi, aucune barre de vie ni collision ajoutée.</p>'''
    html+=f'<div class="native"><img src="{prefix}native/rat-scale.png" width="520" height="165" alt="Rat de 18 pixels comparé à un humain à taille native"></div>'
    html+=f'<figure><figcaption>Extrait animé du moteur · eau et petits rats, taille native</figcaption><div class="native"><img id="ambient" src="{prefix}native/sewer-ambient.gif" width="1218" height="712" alt="Ambiance animée des égouts"></div></figure>'
    html+=f'<button id="motion" aria-pressed="true" data-gif="{prefix}native/sewer-ambient.gif" data-still="{prefix}native/seed-1-collector.png">Mettre l’aperçu en pause</button>'
    html+='<p class="small">La boue paraît toxique, mais ce lot n’ajoute pas encore de dégâts de poison ou de règle de résistance. Le caractère dangereux du décor et ses éventuels effets de jeu restent deux décisions distinctes.</p>'
    html+='''<h2 id="street">POP-D2-02 · Les mêmes principes pour la rue</h2><p>La rue reste dessinée à la main. Ses cours et portions de voie servent de régions de peuplement : tentatives liées à la surface, choix d’une famille, puis placement d’un petit groupe sur des cases valides. Les portes, l’arrivée de la prison et le nouvel accès des égouts gardent une zone dégagée. Les profils de combat existants des niveaux 1–2 sont conservés dans ces aperçus.</p>'''
    for i in range(1,3):
        p=read(ART/f'street-population-{i}.json');count=sum(len(g['members']) for g in p['groups'])
        html+=f'<details><summary>Exemple de rue {i} · {len(p["groups"])} groupes, {count} ennemis proposés</summary>'
        html+=scene(f'street-{i}-north',f'Rue · exemple {i} · carrefour nord')+scene(f'street-{i}-central',f'Rue · exemple {i} · voie centrale')+'</details>'
    html+='<h2 id="kit">Kit séparé · sans découpe d’illustration</h2><p>Chaque objet est complet, avec son emprise propre. Les matériaux de sol, mur et eau sont des textures répétables ; les murs, angles et raccords sont construits par leur géométrie. Les objets ne sont pas arrachés à une image de salle.</p><div class="kit">'
    for key,code,label in [('entry','EG-ENT-01','Trappe de rue'),('stairs','EG-ACC-02','Accès intérieur'),('pipe','EG-PIP-01','Conduite rouillée'),('sludge','EG-TOX-01','Rejet toxique'),('grate','EG-GRI-01','Panneau de passerelle'),('rat','EG-RAT-01','Cycle du petit rat')]:
        p=ART/'kit'/f'{key}-native.png';w,h=Image.open(p).size
        html+=f'<div class="item"><b>{code}</b><p>{label} · {w} × {h} px'+(' · quatre poses de 20 × 12' if key=='rat' else '')+f'</p><img loading="lazy" src="{asset_prefix}kit/{key}-native.png" width="{w}" height="{h}" alt="{label}"></div>'
    html+='</div><h2 id="rules">Fonctionnement proposé à valider</h2><ul>'
    for text in [
        'Assembler des modules compatibles autour d’un accès fixe : salles, collecteurs, bifurcations, angles et galeries. Une autre graine change l’agencement, sans changer l’identité des égouts.',
        'Construire un niveau une fois pour la partie et le retrouver identique en revenant. Cette persistance et la transition de la rue ne sont pas encore activées dans le jeu.',
        'Traiter le peuplement par groupes : 3–5 zombies ou 2–4 cultistes dans ces propositions. La densité donne des tentatives selon la surface disponible ; une salle peut rester vide.',
        'Vérifier les collisions, l’accès, les seuils, les berges et les emprises des machines. Aucun monstre ne commence dans l’eau ni sur une passerelle. Aucun ennemi à moins de huit pas de chemin de l’arrivée ; les six cases autour du point d’arrivée restent également dégagées.',
        'Garder l’eau non franchissable et les grilles praticables. Les boues et les rats sont pour le moment purement visuels.',
        'Reporter le réglage des dégâts, de la résistance, de l’XP, des pouvoirs et des boss. Aucun boss de secteur n’est ajouté à cette proposition.'
    ]:html+='<li>'+escape(text)+'</li>'
    html+='''</ul><p>Les 300 graines contrôlées restent déterministes, avec toutes les cases praticables reliées et des placements compatibles avec les obstacles. Cela vérifie la génération ; ton jugement porte encore sur le dessin et le résultat en situation.</p>
<p class="source">Méthode étudiée : <a href="https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Common/src/Drlg/DrlgMaze.cpp">assemblage des salles et accès des égouts dans D2MOO</a>, <a href="https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Game/src/MONSTER/MonsterRegion.cpp">densité et groupes</a>, <a href="https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Game/src/MONSTER/MonsterChoose.cpp">choix pondéré des familles</a>. Il s’agit d’une adaptation à Project RL ; les dimensions et nombres proposés ne sont pas présentés comme ceux de Diablo II.</p>
<p class="small">Dessins créés avec le tool imagegen intégré. <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-01/prompts.json">Prompts et références de fabrication</a> · <a href="../street-population-review-01/index.html">Essai jouable précédent</a></p></main>
<script>document.querySelectorAll('[data-seed]').forEach(b=>b.onclick=()=>{document.querySelectorAll('[data-panel]').forEach(p=>p.hidden=p.dataset.panel!==b.dataset.seed);document.querySelectorAll('[data-seed]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});const m=document.getElementById('motion'),a=document.getElementById('ambient');m.onclick=()=>{const on=m.getAttribute('aria-pressed')!=='true';a.src=on?m.dataset.gif:m.dataset.still;a.height=on?712:844;m.setAttribute('aria-pressed',String(on));m.textContent=on?'Mettre l’aperçu en pause':'Animer l’aperçu';};if(matchMedia('(prefers-reduced-motion: reduce)').matches)m.click();</script></html>'''
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):self.refs.extend(v for k,v in attrs if k in ('href','src','data-gif','data-still'))
    parser=Refs();parser.feed(html)
    for ref in parser.refs:
        url=urlsplit(ref)
        if url.scheme or not url.path:continue
        assert (OUT/unquote(url.path)).resolve().is_file(),ref
    (OUT/'index.html').write_text(html,encoding='utf-8')
    (ROOT/'artifacts/street-defects-review-01/index.html').write_text(html,encoding='utf-8')
    proofs=DOC/'references';proofs.mkdir(exist_ok=True)
    for name in ['entrance-proposal','seed-1-collector','seed-2-collector','seed-3-collector','rat-scale']:
        shutil.copy2(native/f'{name}.png',proofs/f'{name}.png')
    write(DOC/'checks.json',dict(review='EG01',status='awaiting_user_review',engine=engine,
        normal_map_sha256_unchanged=True,simulation_balancing_changed=False,
        native_images_preflight=len(labels),native_rat_frame=[20,12],rat_silhouette_max_width=18,
        source_assets={p.name:sha(p) for p in (ART/'kit').glob('*.png')},
        prompt_set=rel_prompt(),page_local_references_checked=len(parser.refs),
        proofs={p.name:sha(p) for p in proofs.glob('*.png')}))
    print('Published EG01: native entry, three sewer generations, two street populations, small animated rats; no activation.')

def rel_prompt():return (DOC/'prompts.json').relative_to(ROOT).as_posix()
if __name__=='__main__':publish()
