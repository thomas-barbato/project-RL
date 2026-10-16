"""Read-only measurements and contact sheets from the renderer's exported pixels.
Only crop/nearest zoom/layout: no sprite painting or proposed replacement art.
"""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
metadata = json.loads((ROOT / 'assets/sprites/cyberpunk/player-poses.json').read_text())
names = ['Breche', 'Vigie', 'Creuset'] + metadata['torso_models']
display = {'veste_de_veille':'Tenue du technicien','manteau_de_mue':'Robe de mue',
           'plastron_bastion':'Armure Bastion','patched_plating':'Tenue du ferrailleur'}
for index in range(7):
    Image.frombytes('RGBA', (384,816), (HERE/f'figures-palette-{index}.rgba').read_bytes()).save(HERE/f'figures-palette-{index}.png')
records = json.loads((HERE / 'mesures.json').read_text())
blue_unarmed = [r for r in records if r['side'] == 48 and r['pose'] == 'unarmed' and r['palette'] == 'Bleu']
for record in blue_unarmed:
    record['name'] = names[record['outfit']]
    record['orange_outside_face_window'] = [p for p in record['retained_orange'] if not (22 <= p[0] < 27 and 21.5 <= p[1] < 25.5)]
print('Orange retained in BLUE unarmed outfits (no weapon):')
for record in sorted(blue_unarmed, key=lambda r: len(r['orange_outside_face_window']), reverse=True)[:8]:
    print(record['name'], 'orange=', len(record['retained_orange']), 'outside_face=',len(record['orange_outside_face_window']), 'coords=',record['orange_outside_face_window'])
world = [r for r in records if r['side'] == 48 and r['palette'] == "D'origine"]
print('World unique RGB range:', min(r['unique_rgb'] for r in world), max(r['unique_rgb'] for r in world))
print('Median unique RGB:', sorted(r['unique_rgb'] for r in world)[len(world)//2])

font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 16)
small = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 13)
board = Image.new('RGBA', (1040, 565), (15,19,22,255))
draw = ImageDraw.Draw(board)
draw.text((18,12),'ARMURES : pixels actuels, taille native + zoom x3 sans lissage',font=font,fill='white')
original = Image.open(HERE/'figures-palette-0.png').convert('RGBA')
blue = Image.open(HERE/'figures-palette-2.png').convert('RGBA')
for row, outfit in enumerate([14,8,9,10]):
    y = 47 + row*127
    draw.text((18,y+12),display.get(names[outfit],names[outfit]),font=small,fill=(165,220,220,255))
    for col,(sheet,pose,label) in enumerate([(original,0,'Origine / sans arme'), (blue,0,'Bleu / sans arme'), (blue,2,'Bleu / fusil'), (blue,7,'Bleu / paire mixte')]):
        x=215+col*202
        tile=sheet.crop((pose*48,outfit*48,(pose+1)*48,(outfit+1)*48))
        board.alpha_composite(tile,(x,y+33))
        board.alpha_composite(tile.resize((144,144),Image.Resampling.NEAREST).crop((0,36,144,144)),(x+52,y+17))
        draw.text((x,y),label,font=small,fill=(210,218,224,255))
board.save(HERE/'armures-echantillons.png')

guns = Image.open(ROOT/'assets/sprites/cyberpunk/ranged-families-32.png').convert('RGBA')
gun_names=['Assaut','Precision','Fusil lourd','Mitrailleuse','Pompe','Lanceur','Energie','Flammes','Pistolet','PM']
board = Image.new('RGBA',(1040,415),(15,19,22,255))
draw=ImageDraw.Draw(board)
draw.text((18,12),'ARMES AU SOL : dessins natifs 32 px et zoom x4 (aucune retouche)',font=font,fill='white')
gun_stats=[]
for index,name in enumerate(gun_names):
    tile=guns.crop((index*32,0,(index+1)*32,32))
    x=22+(index%5)*205; y=48+(index//5)*180
    draw.text((x,y),name,font=font,fill=(165,220,220,255))
    board.alpha_composite(tile,(x,y+34))
    board.alpha_composite(tile.resize((128,128),Image.Resampling.NEAREST),(x+49,y+23))
    opaque=[p for p in tile.get_flattened_data() if p[3]>0]
    gun_stats.append({'family':name,'bbox':tile.getbbox(),'visible_pixels':len(opaque),'unique_rgb':len(set(p[:3] for p in opaque))})
    draw.text((x,y+151),f'{len(opaque)} pixels / {len(set(p[:3] for p in opaque))} teintes RGB',font=small,fill=(150,160,170,255))
board.save(HERE/'armes-sol-echantillons.png')
(HERE/'resume-mesures.json').write_text(json.dumps({'blue_unarmed':blue_unarmed,'world_unique_rgb':{'min':min(r['unique_rgb'] for r in world),'max':max(r['unique_rgb'] for r in world),'median':sorted(r['unique_rgb'] for r in world)[len(world)//2]},'ranged_ground':gun_stats},indent=2)+'\n')
print('Ground guns:',gun_stats)
