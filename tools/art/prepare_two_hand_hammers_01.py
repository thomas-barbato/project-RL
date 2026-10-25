"""Frame three new imagegen proposals; no live weapon or gameplay changes."""
import json
import shutil
from pathlib import Path
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import ROOT, GENERATED, sha, write_json, font

SOURCE=ROOT/'assets/source-art/two-hand-hammers-01'
DOC=ROOT/'docs/reviews/two-hand-hammers-01'
OUT=ROOT/'artifacts/class-armor-integration-01/hammers'
COMMON="Use case: stylized-concept. Create ONE isolated full futuristic two-handed war hammer for a grimdark cyberpunk roguelike sprite. Transparent background, no ground, no character or hands, no shadow cast onto a floor, no text, no labels. Whole weapon fully visible with generous empty margin, long shaft EXACTLY HORIZONTAL left to right, large striking head on RIGHT, clear side/front elevation appropriate for a front-facing standing character. Matte worn steel, restrained ivory edge highlights, simple quiet surfaces, dark charcoal grip. Two distinct hand grip bands at 32% and 62% along the shaft, unobstructed. It will be reduced to only 54 pixels wide inside a 64px actor tile: strong simple silhouette, thick 2-3px-equivalent shaft, broad material groups, no fine wires or tiny decorations, no glow. Game-ready painted raster illustration with crisp dark contour, readable robust volumes, muted grimdark comic finish, not photorealistic, no 3D product render. Very wide landscape composition. "
ENTRIES=[
 ('HAM-2H01','exec-fdc35a17-6b44-44cf-b273-6d73e7b68e5d.png','Industrial demolition maul, massive rectangular steel head, bevelled corners, broad worn cold-silver striking plates, short black shock absorber collar. Subtle copper reinforcement only at the head collar. Practical robust construction.'),
 ('HAM-2H02','exec-182160a5-94e7-4684-8223-603169ac7db7.png','Futuristic hydraulic siege hammer, wide compact cylindrical double striking head aligned vertically above and below shaft, two large dark steel lobes and thick pale steel striking caps, one visible central hydraulic block, restrained dark red panel. Different unmistakable silhouette from a rectangular sledge.'),
 ('HAM-2H03','exec-03cfe755-5dd1-45f4-9d59-418087e76fc0.png','Brutal salvaged reactor-core maul, chunky hexagonal striking head with one broad pale metal face, dark navy steel casing, squat triangular reinforcement behind head, one muted copper inset. Heavy angular silhouette, no spikes or exposed cables.')]


def main():
    for p in [SOURCE,DOC,OUT]:p.mkdir(parents=True,exist_ok=True)
    manifest=[]
    board=Image.new('RGBA',(800,520),(23,31,37,255)); d=ImageDraw.Draw(board)
    d.text((20,15),'Marteaux à deux mains · propositions 01–03',font=font(22),fill='white')
    for i,(identity,filename,description) in enumerate(ENTRIES):
        original=SOURCE/f'{identity}-source.png'
        if not original.exists():shutil.copy2(GENERATED/filename,original)
        im=Image.open(original).convert('RGBA')
        assert im.getchannel('A').getextrema()[0]==0
        box=im.getchannel('A').point(lambda a:255 if a>=32 else 0).getbbox()
        cropped=im.crop(box); reduced=cropped.resize((54,round(cropped.height*54/cropped.width)),Image.Resampling.LANCZOS)
        counts=[sum(reduced.getpixel((x,y))[3]>=96 for x in range(40)) for y in range(reduced.height)]
        rows=[y for y,n in enumerate(counts) if n>=max(counts)*.8]
        shaft=round(sum(rows)/len(rows))
        native=Image.new('RGBA',(64,64));native.paste(reduced,(8,32-shaft))
        np=SOURCE/f'{identity}-native64.png';native.save(np,optimize=True)
        shutil.copy2(np,OUT/np.name)
        y=55+i*150
        d.text((20,y+15),f'0{i+1}',font=font(24),fill='white')
        board.alpha_composite(native,(65,y))
        d.text((65,y+70),'taille native',font=font(13),fill='#b5c3cc')
        board.alpha_composite(native.resize((256,256),Image.Resampling.NEAREST).crop((0,70,256,190)),(180,y))
        d.text((460,y+20),identity,font=font(18),fill='white')
        d.text((460,y+50),'Manche horizontal · deux mains',font=font(15),fill='#b5c3cc')
        manifest.append({'id':identity,'number':i+1,'status':'proposed_not_integrated','source':original.relative_to(ROOT).as_posix(),'source_sha256':sha(original),'native':np.relative_to(ROOT).as_posix(),'native_sha256':sha(np),'prompt':COMMON+description,'tool':'built-in image_gen','shaft_y':32,'weapon_width':54,'native_canvas':[64,64]})
    board.save(OUT/'proposals-native.png');board.save(DOC/'proposals-native.png')
    write_json(DOC/'candidates.json',{'review_id':'HAM-2H01','user_direction':'Existing hammers remain one-handed and held like one-handed swords. Add a few separate two-handed hammer models, held horizontally.','integration_status':'visual_review_pending','existing_one_hand_models':[97,98],'new_gameplay_rules_or_stats':False,'candidates':manifest})
    (DOC/'README.md').write_text('''# Marteaux à deux mains — HAM-2H01

Trois nouveaux dessins proposés, numérotés 01 à 03, avec manche horizontal.
La réduction conserve le marteau entier, sur fond transparent, sans étirement.
Les sources et prompts exacts sont dans `candidates.json`.

Les deux marteaux actuels restent à une main. Aucun identifiant d’objet,
nom destiné au joueur, dégât, poids ou règle de prise n’a été ajouté.
Ces nouveaux candidats attendent leur revue visuelle avant ajout, selon
`AGENTS.md` : « Présenter les nouveaux candidats numérotés à taille native
avant leur intégration ». L’accord actuel autorise la préparation de ce lot.
''',encoding='utf-8')
    print(json.dumps({'new_proposals':3,'live_weapons_changed':0,'review':str(OUT/'proposals-native.png')}))


if __name__=='__main__':main()
