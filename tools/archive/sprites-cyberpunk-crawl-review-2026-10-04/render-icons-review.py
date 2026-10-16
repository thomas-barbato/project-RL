"""Render the authored grids and labels as a standalone review board."""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

root = Path(__file__).resolve().parent
icons = json.loads((root / 'ground-icons/native-grid-source.json').read_text(encoding='utf-8'))['icons']
floors = json.loads((root / 'native/native-grid-source.json').read_text(encoding='utf-8'))['floors']
image = Image.new('RGB', (800, 866), '#101920')
draw = ImageDraw.Draw(image)
font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
title = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 23)
draw.text((16, 10), '19 icônes simples pour les objets au sol', font=title, fill='#e5e4d8')
draw.text((16, 43), 'Agrandies × 2 et taille réelle de 32 px. En bas : asphalte, béton, métal, ruines.', font=small, fill='#a8b9c2')

def grid(sprite, ox, oy, scale=1):
    for y, row in enumerate(sprite['pixels']):
        for x, index in enumerate(row):
            color = sprite['colors'][index]
            if color:
                draw.rectangle((ox+x*scale, oy+y*scale, ox+(x+1)*scale-1, oy+(y+1)*scale-1), fill=color)

for index, icon in enumerate(icons):
    x, y = index % 4 * 200 + 8, index // 4 * 156 + 78
    draw.rounded_rectangle((x, y, x+184, y+145), radius=5, fill='#21313a', outline='#40545f')
    draw.text((x+8, y+7), icon['name'], font=font, fill='#e5e4d8')
    grid(icon['sprite'], x+22, y+33, 2)
    grid(icon['sprite'], x+119, y+59)
    for f in range(4):
        sx, sy = x + 22 + f*36, y + 107
        grid(floors[f][0], sx, sy)
        grid(icon['sprite'], sx, sy)
image.save(root / 'icones-proposees-19.png')
print('19 icônes : icones-proposees-19.png')
