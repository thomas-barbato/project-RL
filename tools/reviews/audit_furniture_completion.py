"""Read-only native scenery audit. Comparison boards are not game assets."""
import json,math
from PIL import Image,ImageDraw,ImageFont
from art_finish_completion import ROOT,DOC,OUT
from art_finish_batch_02 import write

def build():
    folder=OUT/'furniture-audit';folder.mkdir(exist_ok=True)
    font=lambda s:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',s)
    human=Image.open(ROOT/'assets/sprites/art-finish-02/FIN-14.png').convert('RGBA')
    corpse=Image.open(ROOT/'assets/maps/prison-street-expanded-01/decor/civilians-08/CIV-U01-04-native.png').convert('RGBA')
    camp=json.loads((ROOT/'assets/maps/cryogenic-camp-01/camp.json').read_text('utf-8'))
    props=list({p['id']:p for p in camp['props']}.values());rows=[]
    page=Image.new('RGB',(1440,80+math.ceil(len(props)/4)*240),'#11191d');d=ImageDraw.Draw(page)
    d.text((16,12),'Mobilier du camp · comparaison native · aucune correction appliquée',font=font(24),fill='#decca7')
    d.text((16,47),'Personnage debout et corps au sol comme repères ; surfaces et faces verticales sont évaluées séparément.',font=font(16),fill='#becac3')
    for i,p in enumerate(props):
        x=i%4*360;y=80+i//4*240;im=Image.open(ROOT/p['file']).convert('RGBA');b=im.getbbox()
        d.rectangle((x+6,y+30,x+352,y+230),fill='#343d3b');page.paste(im,(x+12,y+60),im);page.paste(human,(x+136,y+60),human);page.paste(corpse,(x+232,y+60),corpse)
        d.text((x+10,y+3),p['id'],font=font(17),fill='#decca7');d.text((x+10,y+202),f'Dessin {b[2]-b[0]} × {b[3]-b[1]} px · emprise {p.get("width",1)} × {p.get("height",1)} case(s)',font=font(14),fill='#becac3')
        rows.append(dict(id=p['id'],file=p['file'],occupied=list(b),world_pixels=[b[2]-b[0],b[3]-b[1]],footprint=[p.get('width',1),p.get('height',1)]))
    page.save(folder/'camp-native.png')
    street=json.loads((ROOT/'assets/maps/prison-street-expanded-01/street.json').read_text('utf-8'));ground=Image.open(ROOT/street['background']).convert('RGBA')
    rooms=[]
    for roof in street['roofs']:
        if not roof['interior']:continue
        at=roof['at_px'];size=Image.open(ROOT/roof['file']).size;crop=ground.crop((at[0]-12,at[1]-12,at[0]+size[0]+12,at[1]+size[1]+72))
        board=Image.new('RGB',(max(800,crop.width+220),crop.height+70),'#11191d');dd=ImageDraw.Draw(board);dd.text((12,12),roof['id']+' · intérieur existant · taille native',font=font(22),fill='#decca7');board.paste(crop,(12,58),crop);board.paste(human,(crop.width+25,80),human);board.paste(corpse,(crop.width+90,80),corpse)
        path=folder/(roof['id']+'.png');board.save(path);rooms.append(dict(id=roof['id'],file=path.relative_to(ROOT).as_posix(),world_origin=[at[0]-12,at[1]-12],cell=64))
    write(DOC/'furniture-inventory.json',dict(status='audit_only_no_game_edit',cell=64,standing_reference_bounds=list(human.getbbox()),lying_reference_bounds=list(corpse.getbbox()),camp_props=rows,street_rooms=rooms,geometry_and_collision_unchanged=True))
    print('Furniture native audit:',len(props),'camp models;',len(rooms),'street interiors')

if __name__=='__main__':build()
