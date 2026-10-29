"""Publish unscaled engine comparisons; no creative bitmap editing here."""
from pathlib import Path
import hashlib
import json
import sys
from PIL import Image,ImageDraw

ROOT=Path(__file__).resolve().parents[2]
WEAR='--corridor-wear' in sys.argv
GRATE='--corridor-grate' in sys.argv or WEAR
FUTURE='--future' in sys.argv or GRATE
INTERIOR='--interior-r03' in sys.argv or FUTURE
OUT=ROOT/'artifacts/floor-continuous-review-01'
if INTERIOR:OUT=OUT/('corridor-wear-07' if WEAR else ('corridor-grate-06' if GRATE else ('future-r04' if FUTURE else 'interior-r03')))
ACCEPTED=WEAR and (ROOT/'docs/reviews/floor-future-01/acceptance-07.json').exists()


def publish():
    checks=json.loads((OUT/'checks.json').read_text())
    outputs=[]
    labels={'camp-arrival':'Camp : arrivée','camp-corridor':'Camp : galerie','street-arrival':'Extérieur : arrivée en ville','street-plaza':'Extérieur : place','street-north':'Extérieur : galerie urbaine'}
    labels.update({'camp-storage':'Réserve','camp-security':'Salle de sécurité','camp-medical':'Infirmerie','camp-workshop':'Atelier','camp-doorway':'Porte du sas','camp-threshold':'Passage vers une salle'})
    for record in checks['records']:
        name=record['scene'];x,y,w,h=map(round,record['crop'])
        if FUTURE and name=='camp-arrival':x+=128;y-=64;w=512;h=528
        board=Image.new('RGB',(2*w+80,h+112),(9,13,18));draw=ImageDraw.Draw(board)
        draw.text((24,15),f'{checks["review"]} - {labels[name]} - taille native (64 px)',fill=(218,228,234))
        for i,phase in enumerate(['before','after']):
            image=Image.open(OUT/f'{name}-{phase}.png').convert('RGB')
            crop=image.crop((x,y,x+w,y+h))
            board.paste(crop,(24+i*(w+32),64))
            caption=('02 - beton propose precedemment' if i==0 else '04 - beton continu, pieds de decor et bordures corriges') if INTERIOR else ('01 - sol actuel' if i==0 else '02 - sol continu, sans quadrillage decoratif')
            if FUTURE:caption='04 - beton precedent' if i==0 else '05 - pont technique, plaques metalliques larges'
            if GRATE:caption='05 - plaques metalliques validees' if i==0 else '06 - salles 05 conservees, caillebotis dans les couloirs'
            if WEAR:caption='06 - caillebotis valide' if i==0 else '07 - usure irreguliere sur les barres et pres des murs'
            draw.text((24+i*(w+32),42),caption,fill=(218,228,234))
            if FUTURE and not GRATE and name=='camp-arrival' and phase=='after':
                single=Image.new('RGB',(w,h+36),(9,13,18));single.paste(crop,(0,36))
                ImageDraw.Draw(single).text((12,12),'05 - Pont technique - taille native 64 px',fill=(218,228,234))
                single.save(OUT/'camp-arrival-05-native64.png')
            if GRATE and name=='camp-corridor' and phase=='after':
                single=Image.new('RGB',(w,h+36),(9,13,18));single.paste(crop,(0,36))
                ImageDraw.Draw(single).text((12,12),'07 - Caillebotis plus use - taille native 64 px' if WEAR else '06 - Caillebotis dans le couloir - taille native 64 px',fill=(218,228,234))
                single.save(OUT/('camp-corridor-07-native64.png' if WEAR else 'camp-corridor-06-native64.png'))
        footer='Memes personnages, eclairage et camera. Les contours du sol suivent les pieds des murs. Proposition avant integration.' if FUTURE else ('Memes personnages, eclairage et camera. Pieds de decor restaures dans 04. Proposition avant integration.' if INTERIOR else 'Memes personnages, murs, eclairage et camera. Proposition avant integration.')
        if GRATE:footer='Les cinq salles sont identiques a 05. Memes personnages, eclairage et camera. Le couloir 06 reste en apercu.'
        if WEAR:footer='Salles 05 et ouvertures de la maille conservees. Memes eclairages et camera. La finition 07 reste en apercu.'
        if ACCEPTED:footer='Finition 07 validee et integree. Salles 05, ouvertures de la maille, eclairages et camera conserves.'
        draw.text((24,h+82),footer,fill=(153,174,186))
        path=OUT/f'{name}-comparison-native64.png';board.save(path)
        outputs.append({'file':path.name,'bytes':path.stat().st_size,'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'comparison_size':list(board.size),'source_crop':[x,y,w,h]})
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Sols continus · revue</title>
<style>body{margin:24px;background:#090d12;color:#dae4ea;font:16px system-ui}p{max-width:90ch;line-height:1.6}.viewport{overflow:auto}img{max-width:none;display:block}button{background:#172631;color:#dae4ea;border:1px solid #405565;padding:10px 14px;margin:0 6px 12px 0;cursor:pointer}</style>
<h1>Sols continus · camp et extérieurs</h1><p>01 : sol actuel. 02 : matière continue sans damier. Le dessin du sol ne suggère plus de fausses cases. Les cases logiques de 64 px, les murs, les personnages, les passages et l'éclairage restent conservés.</p>'''
    if INTERIOR:
        html=html[:html.index('<h1>')]+'''<h1>04 · Béton continu et pieds de décor conservés</h1><p>À gauche : le béton 02 montré précédemment. À droite : correction 04, avec la même matière continue dans les salles et les couloirs. Le caillebotis 03 est abandonné. Les deux couches du décor sont corrigées ensemble pour retirer les restes de damier sans effacer les pieds de murs et les boîtiers. Même caméra, mêmes personnages, même éclairage. L'extérieur validé est déjà intégré ; cet intérieur reste en aperçu.</p>'''
    if FUTURE:
        html=html[:html.index('<h1>')]+'''<h1>05 · Pont technique</h1><p>À gauche : le béton 04 écarté. À droite : une proposition de sol futuriste sobre, en grandes plaques métalliques mates et joints fins. Le contour réel du sol suit les pieds des murs et les équipements suspendus, dans les deux couches de l'architecture. Les petites marques colorées sont peintes ; elles n'émettent aucune lumière. Les personnages, accessoires, éclairages, passages et collisions sont conservés. L'extérieur validé reste intégré ; l'intérieur 05 est un aperçu.</p>'''
    if GRATE:
        html=html[:html.index('<h1>')]+'''<h1>06 · Salles 05 et couloirs en caillebotis</h1><p>À gauche : la version 05 validée, maintenant intégrée au jeu normal. À droite : proposition 06, avec une maille métallique fine uniquement dans les zones de circulation. Les cinq salles restent identiques à 05. Les raccords suivent les seuils et les pieds du décor ; les passages et collisions sont conservés. Même caméra, mêmes personnages, même éclairage. Le caillebotis 06 reste en aperçu.</p>'''
    if WEAR:
        html=html[:html.index('<h1>')]+'''<h1>07 · Caillebotis usé</h1><p>À gauche : le caillebotis 06 validé, maintenant intégré au jeu normal. À droite : finition 07, avec une usure irrégulière sur les barres, plus présente près des murs. Les ouvertures de la maille, les cinq salles 05, les personnages et les éclairages sont conservés. Le dessin d'usure est une couche de matière, sans nouvel objet ni effet de jeu. Cette finition reste en aperçu.</p>'''
    if ACCEPTED:
        html=html.replace('le caillebotis 06 validé, maintenant intégré au jeu normal', 'le caillebotis 06 précédemment validé').replace('Cette finition reste en aperçu.', 'Cette finition 07 est validée et intégrée au jeu normal.')
    for row in checks['records']:
        name=row['scene'];html+=f'<button onclick="show(\'{name}-comparison-native64.png\')">{labels[name]}</button>'
    first='camp-corridor' if GRATE else ('camp-arrival' if INTERIOR else 'street-arrival')
    html+=f'<div class="viewport"><img id="view" src="{first}-comparison-native64.png" alt="Comparaison native des sols"></div><p>Images à taille native, avec défilement sur écran étroit. Proposition à examiner avant intégration.</p><script>function show(file){{document.getElementById("view").src=file}}</script></html>'
    if ACCEPTED:html=html.replace('Proposition à examiner avant intégration.', 'Comparaison conservée de la finition validée et intégrée.')
    (OUT/'apercu.html').write_text(html,encoding='utf-8')
    if INTERIOR:
        folder='corridor-wear-07' if WEAR else ('corridor-grate-06' if GRATE else ('future-r04' if FUTURE else 'interior-r03'))
        (OUT.parent/'apercu.html').write_text(f'<!doctype html><meta charset="utf-8"><meta http-equiv="refresh" content="0;url={folder}/apercu.html"><a href="{folder}/apercu.html">Aperçu courant des sols</a>',encoding='utf-8')
    (OUT/'publication.json').write_text(json.dumps({'review':checks['review'],'native_cell':64,'resampled_engine_pixels':False,'whole_starting_room_in_future_review':FUTURE,'outputs':outputs},indent=2)+'\n')
    print(f'Published {len(outputs)} native comparisons without resampling.')


if __name__=='__main__':publish()
