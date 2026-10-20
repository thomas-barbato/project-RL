"""Package north-extension review: overview plus actual native engine captures."""
import hashlib
import json
from pathlib import Path
import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r06'
REVISION='r06'
OUT=ROOT/'artifacts/street-expansion-review-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'


def overview(scene):
    ink=Image.open(ROOT/scene['background']).convert('RGBA')
    correction=Image.open(ROOT/scene['atmosphere']['correction']).convert('RGBA')
    a=np.array(ink,dtype=float)/255
    c=np.array(correction,dtype=float)/255
    mask=c[:,:,3:4]
    a[:,:,:3]=a[:,:,:3]*(c[:,:,:3]*2)*(1-mask)+c[:,:,:3]*mask
    painted=Image.fromarray(np.clip(a*255,0,255).astype('uint8'))
    # Reproduce UV-source patches before the world light composite.
    surface=painted.copy()
    entries=[scene['doorway'],*(d for d in scene['extra_doorways'])]
    def patch(source,at):
        x,y,w,h=source
        surface.alpha_composite(painted.crop((x,y,x+w,y+h)),at)
    for entry in entries:
        patch(entry['floor_source'],entry['opening_at'])
        patch(entry['lamp_source'],entry['lamp_at'])
        patch(entry['closed_source'],entry['closed_at'])
    car=Image.open(ROOT/scene['car_file']).convert('RGBA')
    surface.alpha_composite(car,(scene['car_at'][0]*64,scene['car_at'][1]*64))
    player=Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png').convert('RGBA')
    surface.alpha_composite(player,(scene['entrance'][0]*64-32,scene['entrance'][1]*64-32))
    irradiance=Image.open(ROOT/scene['atmosphere']['irradiance']).resize(ink.size,Image.Resampling.BILINEAR)
    result=np.array(surface,dtype=float)
    result[:,:,:3]*=np.array(irradiance,dtype=float)/127.5
    result=Image.fromarray(np.clip(result,0,255).astype('uint8'))
    ambient=np.array(scene['atmosphere']['ambient'])
    for roof in scene['roofs']:
        im=Image.open(ROOT/roof['file']).convert('RGBA')
        data=np.array(im,dtype=float)
        data[:,:,:3]*=ambient
        result.alpha_composite(Image.fromarray(np.clip(data,0,255).astype('uint8')),roof['at_px'])
    result.convert('RGB').save(OUT/'overview-native.png')
    # A single continuous crop. Never juxtapose remote map fragments without
    # separation: that would falsely suggest broken streets and buildings.
    result.convert('RGB').crop((96,3136,1536,4064)).save(OUT/'departure-continuous-native.png')
    if REVISION=='r06':
        result.convert('RGB').crop((128,224,848,704)).save(OUT/'warehouse-continuous-native.png')
    # Explicit reduced overview; detailed images below retain one pixel per pixel.
    small=result.convert('RGB').resize((520,1320),Image.Resampling.LANCZOS)
    board=Image.new('RGB',(860,1372),'#10191e')
    board.paste(small,(16,36))
    d=ImageDraw.Draw(board)
    bold=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',20)
    normal=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)
    d.text((16,5),'U01 · décors indépendants · proposition '+REVISION,font=bold,fill='#dec898')
    for y,title,lines in [(54,'SORTIE NORD',['Passage prévu vers U02.','La carte suivante reste à créer.']),
        (286,'HABITATIONS DÉTRUITES',['Six bâtiments supplémentaires.','Toits et façades effondrés.']),
        (536,'RUE RAVAGÉE',['Impacts et épaves.','Positions défensives détruites.']),
        (906,'ACCESSOIRES REPRIS',['Objets transparents indépendants.','Luminaires espacés et certains éteints.']),
        (1000,'ANCIENNE RUE CONSERVÉE',['Même sortie de prison,','même disposition au départ.']),
        (1224,'DÉPART DU JOUEUR',['Aucun sol derrière la prison.','Progression vers le haut.'])]:
        d.text((552,y),title,font=bold,fill='#dec898')
        for i,line in enumerate(lines):d.text((552,y+32+i*25),line,font=normal,fill='#d4dfdc')
    d.text((552,640),'26 × 66 cases',font=bold,fill='#dec898')
    d.text((552,676),f"{scene['walkable_cells']} cases praticables",font=normal,fill='#d4dfdc')
    d.text((552,706),'Cases inchangées : 64 px',font=normal,fill='#d4dfdc')
    d.text((552,756),'Jaune : porte ouvrable',font=normal,fill='#ecc674')
    d.text((552,786),'Rouge : accès condamné',font=normal,fill='#ef7365')
    board.save(OUT/'plan-corrige.png')


def build():
    scene=json.loads((ART/'street.json').read_text(encoding='utf-8'))
    candidate=json.loads((DOC/('candidate-'+REVISION+'.json')).read_text(encoding='utf-8'))
    active=ROOT/'assets/maps/prison-street-01'
    assert candidate['active_hashes']=={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in active.iterdir() if p.is_file()}
    overview(scene)
    cap=OUT/('engine-'+REVISION)
    checks=json.loads((cap/'checks.json').read_text(encoding='utf-8'))
    assert checks['candidate_only'] and checks['native_cell']==64 and checks['camp_round_trip']
    assert checks['dimensions']==[26,66]
    scenes={r['scene']:r for r in checks['scenes']}
    for r in scenes.values():assert r['render_read_only'] and r['cell'][2:]==[64,64]
    for prefix,index in [('house-a',2),('house-b',3)]:
        assert not scenes[prefix+'-closed']['roofs_hidden'][index]
        assert scenes[prefix+'-open']['roofs_hidden'][index]
    for name,r in scenes.items():
        frame=Image.open(cap/(name+'.png')).convert('RGB')
        x,y,_,_=r['cell']
        left=max(0,min(frame.width-768,round(x-352)))
        top=max(0,min(frame.height-512,round(y-320)))
        frame.crop((left,top,left+768,top+512)).save(OUT/(name+'-native.png'))
    for name in ('fan','steam','embers'):
        frames=[Image.open(cap/f'{name}-{i:02}.png').convert('RGB') for i in range(24)]
        assert all(f.size==(640,384) for f in frames)
        assert any(f.tobytes()!=frames[0].tobytes() for f in frames[1:]),name+' must visibly animate'
        frames[0].save(OUT/(name+'.gif'),save_all=True,append_images=frames[1:],duration=125,loop=0,optimize=False)
    report=dict(checks,overview='Numerical overview rendered from the same scene data; details are actual GPU captures',
                native_details=[768,512],native_animations=[640,384],active_map_unchanged=True)
    (DOC/('render-'+REVISION+'.json')).write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>U01 · extension vers le nord</title>
<style>body{background:#10191e;color:#d8e2de;font:17px Segoe UI,sans-serif;margin:24px}h1,h2{color:#dec898}p{max-width:1080px;line-height:1.5}a{color:#d8b77c}.scroll{overflow:auto;border:1px solid #45545b;padding:8px;margin:18px 0}img{display:block;max-width:none}.nav{display:flex;gap:18px;flex-wrap:wrap}button{background:#25373d;color:#e1c38e;border:1px solid #62777e;padding:9px 16px;cursor:pointer}.yellow{color:#ecc674}.red{color:#ef7365}</style>
<h1>U01 · ressources indépendantes et raccords repris · r05</h1>
<p><a href="vision-04/index.html">VISION-U01-04 · proposition courante pour le champ de vision</a></p>
<p>Les éléments repris sont dessinés séparément sur fond transparent puis placés dans le repère commun de la carte. Bâtiments, toits, accessoires, végétation et luminaires ont leurs propres fichiers. Les ressources déjà propres sont réutilisées. Le départ approuvé conserve sa disposition.</p>
<p>Les quatre coins de l’enceinte sont raccordés. Le sol derrière la prison est supprimé dans l’image et la grille de déplacement. Deux luminaires redondants sont retirés et six autres sont éteints, avec une lentille sombre sans émission. Les marqueurs rouges et jaunes restent des lumières du moteur.</p>
<p>Le tracé continu des rues et les seuils utilisables sont conservés. Les gros débris, épaves et impacts encombrants sont pris en compte dans les déplacements ; les petites traces restent traversables. Les bâtiments ajoutés sont condamnés et leurs lampes rouges sont calculées par le moteur. Cette proposition ne crée aucun nouveau combat ni dégâts de terrain.</p>
<p>L’ancienne sortie de prison et la disposition du départ restent au sud. La carte se prolonge vers le haut : <b>26 × 66 cases</b>, contre 26 × 18 ; <b>WALKABLE_COUNT cases praticables reliées</b>, contre 173. Les cases restent à 64 px. Le périmètre est fermé, avec un seul passage prévu au nord vers U02. Cette prochaine carte reste à créer.</p>
<p><span class="yellow">Jaune : porte ouvrable, même lorsqu’elle est fermée.</span> <span class="red">Rouge : bâtiment condamné.</span> Trois bâtiments visitables au total, avec masquage du toit à l’ouverture ou quand le joueur est dedans.</p>
<p>Proposition en revue ; la carte de la partie normale est conservée jusqu’à validation.</p>
<div class="nav"><a href="#departure">Vue continue du départ</a><a href="#plan">Plan d’ensemble</a><a href="#native">Carte à taille réelle</a><a href="#details">Vues du moteur</a><a href="bibliotheque.html">Bibliothèque de ressources</a><a href="#animations">Animations</a></div>
<h2 id="departure">Vue continue du départ · taille native</h2><p>Une seule zone de la carte, sans montage de fragments. Cette composition utilise les mêmes ressources et lumières, avec le brouillard de visibilité retiré pour examiner l’architecture. Les captures du moteur, plus bas, conservent sa visibilité normale.</p><div class="scroll"><img src="departure-continuous-native.png" width="1440" height="928" alt="Vue continue du départ sans assemblage de coins éloignés"></div>
<h2 id="plan">Ancien départ en bas, extension continue au-dessus</h2><div class="scroll"><img src="plan-corrige.png" alt="Vue d’ensemble réduite du quartier continu"></div>
<h2 id="native">Carte à taille réelle</h2><p>Vue d’ensemble calculée depuis les données du décor. Les gros plans suivants proviennent du moteur du jeu, avec sa visibilité normale.</p>
<button id="start">Aller au départ</button> <button id="north">Aller à la sortie nord</button><div id="map" class="scroll" style="height:760px"><img src="overview-native.png" width="1664" height="4224" alt="Carte complète à un pixel pour un pixel"></div>
<h2 id="details">Dans le moteur · 64 px par case</h2>
'''
    page=page.replace('WALKABLE_COUNT',str(scene['walkable_cells']))
    if REVISION=='r06':
        page=page.replace('r05','r06').replace(
            '<p>Les éléments repris sont dessinés séparément',
            '<p>Correction r06 : les accessoires tronqués autour de l’entrepôt et des deux intérieurs sont retirés des bâtiments. Les murs possèdent leurs propres contours complets ; les accessoires extérieurs sont placés entiers depuis des fichiers séparés.</p><p>Les éléments repris sont dessinés séparément')
        page=page.replace('<h2 id="departure">',
            '<h2>Entrepôt · hauteur de façade corrigée · MUR-SUD-03</h2><p>La façade sud reprend la hauteur apparente de celle du garage. Le prolongement est conservé, les coins restent aux extrémités et les gravats inutiles sont retirés. Vue continue à taille native, sans brouillard pour examiner les contours et les objets entiers.</p><div class="scroll"><img src="warehouse-continuous-native.png" width="720" height="480" alt="Entrepôt et sa façade sud à hauteur corrigée"></div><h2 id="departure">')
    for key,title in [('gate','Départ conservé'),('join-south','Jonction avec l’ancienne rue'),
                      ('join-middle','Avenue · carrefour central'),('join-north','Avenue · partie nord'),
                      ('red-door','Maison voisine · accès condamné'),('yellow-door','Première maison · porte ouvrable'),
                      ('house-a-closed','Nouvelle maison · porte fermée'),('house-a-open','Nouvelle maison · intérieur accessible'),
                      ('house-b-closed','Nouvelle boutique · porte fermée'),('house-b-open','Nouvelle boutique · intérieur accessible'),
                      ('war-houses','Bande résidentielle · maisons détruites'),
                      ('war-annex','Nouveau bâtiment effondré et rue de l’entrepôt'),
                      ('war-blast','Atelier · impacts et décombres'),
                      ('exit','Passage nord réservé pour la suite'),
                      ('lighting-off','Luminaires espacés · appareil éteint'),
                      ('court-clean','Cour · accessoires sans fragments de sol'),
                      ('west-corner','Angle nord-ouest de l’enceinte'),
                      ('east-corner','Angle nord-est de l’enceinte')]:
        page+=f'<h3>{title}</h3><div class="scroll"><img src="{key}-native.png" width="768" height="512" alt="{title} à taille native"></div>'
    page+='<h2 id="decors">Ressources · taille native</h2><p><a href="bibliotheque.html">Voir chaque élément séparément, son code et ses dimensions</a>. Les images sont présentées à un pixel pour un pixel, avec les acteurs de référence. Les sources de fabrication et les placements sont conservés dans le projet.</p><div class="scroll"><img src="modules-native.png" alt="Éléments repris à taille native"></div>'
    page+='<h2 id="animations">Animations discrètes · captures du moteur</h2><p>Effets visuels seuls : aucune action ni aucun tour supplémentaire. Pas de clignotement global.</p>'
    for name,title in [('fan','Ventilation lente'),('steam','Filet de vapeur ponctuel'),('embers','Quelques braises au-dessus de la voiture')]:
        page+=f'<h3>{title}</h3><div class="scroll"><img src="{name}.gif" width="640" height="384" alt="{title}, taille réelle"></div>'
    page+='''<script>const map=document.getElementById('map');function start(){map.scrollTop=3400;map.scrollLeft=160;}document.getElementById('start').onclick=start;document.getElementById('north').onclick=()=>{map.scrollTop=0;map.scrollLeft=650;};start();</script></html>'''
    (OUT/'index.html').write_text(page,encoding='utf-8')
    print(json.dumps(dict(review='MAP-U01-EXP-01-'+REVISION,scenes=len(scenes),camp_round_trip=True,native=64)))


if __name__=='__main__':
    build()
