"""Review gallery and comparisons of actual output, including native tile size."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

BASE=Path(__file__).resolve().parent
OUT=BASE/'armor-flat-review-04'
data=json.loads((OUT/'review.json').read_text('utf-8'))
(OUT/'audit').mkdir(exist_ok=True)
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',21)
small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',16)

board=Image.new('RGB',(1300,560*len(data['entries'])),'#111c25')
d=ImageDraw.Draw(board)
for row,e in enumerate(data['entries']):
    ident=e['id'];v=2 if ident=='AR-N14' else 0
    origin=BASE/('armor-flat-review-02' if ident=='AR-N13' else 'armor-flat-review-03')
    y=row*560
    d.text((20,y+8),ident+' · '+e['name'],font=font,fill='white')
    for col,(label,folder) in enumerate([('Avant',origin),('Raccords corrigés',OUT)]):
        x=20+col*430
        d.text((x,y+42),label,font=small,fill='#b9cfda')
        im=Image.open(folder/f'portraits/{ident}-V{v}-00.png').convert('RGBA')
        bg=Image.new('RGBA',im.size,'#afbabe');bg.alpha_composite(im)
        board.paste(bg.resize((400,336),Image.Resampling.LANCZOS),(x,y+74))
    for col,key in enumerate(['00','02','09','10','11']):
        im=Image.open(OUT/f'display64/{ident}-V{v}-{key}.png').convert('RGBA')
        x=20+col*160
        bg=Image.new('RGBA',im.size,'#afbabe');bg.alpha_composite(im)
        board.paste(bg,(x,y+420))
        d.text((x+3,y+400),'64 px · '+key,font=small,fill='#b9cfda')
    for i,bgcolor in enumerate(['#afbabe','#071019']):
        x=920+i*174
        native=Image.open(OUT/f'native32/{ident}-V{v}-00.png').convert('RGBA')
        bg=Image.new('RGBA',native.size,bgcolor);bg.alpha_composite(native)
        board.paste(bg,(x,y+110))
        d.text((x,y+80),'32 px · natif',font=small,fill='#b9cfda')
        doubled=bg.resize((128,128),Image.Resampling.NEAREST)
        board.paste(doubled,(x,y+220))
        d.text((x,y+194),'64 px · ×2',font=small,fill='#b9cfda')
board.save(OUT/'audit/overview.png')

# A separate compact audit shows all equipment configurations, without
# smoothing or resizing the 64 px sprites that reviewers inspect.
for v in range(3):
    mosaic=Image.new('RGB',(16*140,6*164),'#111c25');draw=ImageDraw.Draw(mosaic)
    for row,e in enumerate(data['entries']):
        for col,c in enumerate(data['configurations']):
            im=Image.open(OUT/f'display64/{e["id"]}-V{v}-{c["number"]}.png').convert('RGBA')
            bg=Image.new('RGBA',im.size,'#afbabe');bg.alpha_composite(im)
            mosaic.paste(bg,(col*140,row*164+28))
            draw.text((col*140,row*164),e['id']+'/'+c['number'],font=small,fill='white')
    mosaic.save(OUT/f'audit/equipment-64-V{v}.png')

html=(BASE/'armor-flat-review-03/index.html').read_text('utf-8')
html=html.replace('ARMOR-FLAT03','ARMOR-FLAT04')
html=html.replace('Armures · corrections du catalogue','Armures · raccords corrigés')
html=html.replace('AR-08 : casque cuivré. AR-10 et AR-11 : nouvelles palettes. AR-12 : technicien redessiné. AR-N14 : artefacts jaunes corrigés.',
    'Les lignes et triangles gris étaient des fentes transparentes entre les couches du corps. Cette revue présente leur correction avec les équipements.')
html=html.replace('Les huit nouvelles tenues AR-N07 à AR-N14 sont validées. La correction des taches jaunes de l’armure très lourde est ajoutée à cette revue ; sa silhouette reste identique.',
    'AR-08 porte désormais le nom validé « Tablier de cuir ». Les corrections visuelles présentées ici attendent ton avis.')
html=html.replace('5 corrections','6 corrections').replace('Les cinq corrections','Les six corrections')
html=html.replace('Même gabarit frontal, mêmes positions de mains et mêmes placements d’armes. Les noms et les identifiants des quatre objets sont conservés.',
    'Même gabarit frontal et mêmes placements d’armes. Les dessins sources, identifiants et caractéristiques sont conservés.')
html=html.replace('Armure très lourde : variante rouge comparée ci-dessous. Les vues 32 et 64 px exactes et les autres palettes sont présentées sur chaque fiche.',
    'Chaque modèle est comparé sur fond clair. Les trois palettes et les seize configurations sont consultables en 32 px et en agrandissement exact 64 px.')
html=html.replace('Cinq armures, avant et après, numérotées','Six armures, avant et après, numérotées')
html=html.replace('Voir les variantes corrigées de l’armure très lourde</a></p>',
    'Voir l’armure très lourde</a> · <a href="#AR-N13">Armure lourde</a> · <a href="#AR-08">Tablier de cuir</a></p>')
html=html.replace('Revue précédente · même gabarit','Revue précédente · sans équipement')
html=html.replace('<a href="generation-specs.json">Prompts des dessins</a>',
    '<a href="audit/equipment-64-V0.png">Toutes les prises · référence</a> · <a href="audit/equipment-64-V1.png">Variante 2</a> · <a href="audit/equipment-64-V2.png">Variante 3</a>')
(OUT/'index.html').write_text(html,'utf-8')
print(OUT/'index.html')
