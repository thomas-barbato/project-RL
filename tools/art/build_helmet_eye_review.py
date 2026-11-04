"""Side-by-side source and native views for the HELM05-W02 optic correction."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont
from prepare_helmet_worn_review import source_cells

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/helmet-worn-r01'
OUT=ROOT/'artifacts/animation-polish-01/helmet-worn-r01/current/eye01'
VIEWS=[('front','Face',0,0),('right','Trois-quarts, optique proche',2,3),('left','Trois-quarts, optique éloignée',3,2)]


def build():
    if not (SOURCE/'H05-W02-views-r03.png').exists():return '', ''
    OUT.mkdir(parents=True,exist_ok=True)
    old=source_cells(Image.open(SOURCE/'H05-W02-views-r02.png').convert('RGBA'))
    new=source_cells(Image.open(SOURCE/'H05-W02-views-r03.png').convert('RGBA'))
    atlases=[Image.open(SOURCE/'H05-W02-before-eye01-atlas.png').convert('RGBA'),Image.open(SOURCE/'H05-W02-atlas.png').convert('RGBA')]
    factor=150/max(old[0][0].height,new[0][0].height)
    boards=[]
    for key,name,source_index,atlas_index in VIEWS:
        board=Image.new('RGB',(336,464),(30,40,49))
        draw=ImageDraw.Draw(board)
        font=lambda size:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)
        draw.text((10,9),name,font=font(15),fill='white')
        for col,(cells,atlas,label) in enumerate(zip([old,new],atlases,['Avant · r02','Reprise · r03'])):
            draw.text((16+col*168,42),label,font=font(13),fill=(227,235,238))
            crop=cells[source_index][0]
            size=tuple(round(v*factor) for v in crop.size)
            tile=crop.convert('RGBa').resize(size,Image.Resampling.LANCZOS).convert('RGBA')
            board.paste(tile,(col*168+84-size[0]//2,230-size[1]),tile)
            native=atlas.crop((atlas_index*32,0,(atlas_index+1)*32,32)).resize((128,128),Image.Resampling.NEAREST)
            board.paste(native,(20+col*168,286),native)
        draw.text((10,243),'Dessin source agrandi',font=font(12),fill=(174,191,200))
        draw.text((10,433),'Couche portée native · zoom entier ×4',font=font(12),fill=(174,191,200))
        board.save(OUT/f'eye-{key}-compare.png')
        boards.append(board)
    all_views=Image.new('RGB',(1008,464),(30,40,49))
    for n,board in enumerate(boards):all_views.paste(board,(n*336,0))
    all_views.save(OUT/'eye-three-angles.png')
    section='''<section id="casque02-optique"><h2>Casque 02 · reprise de l’optique</h2><p>La version précédente ne conservait pas correctement la lecture de l’œil entre les angles. Cette reprise montre la même optique sous l’arcade, avec une partie visible dans le trois-quarts opposé. Les tests techniques précédents ne validaient pas cette cohérence visuelle.</p><label>Angle<select id="eye-angle"><option value="left">Trois-quarts · optique éloignée</option><option value="front">Face</option><option value="right">Trois-quarts · optique proche</option></select></label><div class="native" style="max-width:336px;margin-top:16px"><img id="eye-compare" src="helmet-worn-r01/current/eye01/eye-left-compare.png" width="336" height="464" style="display:block;width:336px;height:464px;image-rendering:pixelated" alt="Avant et reprise du casque 02, optique du côté éloigné"></div><details><summary>Comparer les trois angles ensemble</summary><div class="native"><img src="helmet-worn-r01/current/eye01/eye-three-angles.png" width="1008" height="464" style="display:block;max-width:none;image-rendering:pixelated" alt="Avant et reprise dans les trois angles frontaux"></div></details><p class="note">Le vrai dos, les icônes validées et les quatre autres casques sont conservés. Le comparatif porté ci-dessous utilise la reprise. Le placement de l’optique reste à juger sur ces dessins et à taille de jeu.</p></section>'''
    script="$('eye-angle').onchange=()=>{$('eye-compare').src=`helmet-worn-r01/current/eye01/eye-${$('eye-angle').value}-compare.png`;$('eye-compare').alt=`Avant et reprise du casque 02 : ${$('eye-angle').selectedOptions[0].textContent}`;};"
    return section,script


if __name__=='__main__':
    build()
    print(json.dumps({'comparison':str(OUT),'front_angles':3}))
