"""Crop, normalize, resize and lay out generated comparison assets only.

No painting, palette quantization or active game-asset replacement.
Uses the existing body/feet normalization of the approved outfit packer.
"""
from pathlib import Path
import hashlib, importlib.util, json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('outfit_packer', ROOT/'tools/art/build_player_outfits.py')
packer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(packer)
POSES = ('unarmed', 'side', 'raised', 'two-handed')
font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 17)
small = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 14)
stats = {'outfits':[], 'weapons':[]}

def metric(tile):
    visible = [p for p in tile.get_flattened_data() if p[3]]
    return {'visible_pixels':len(visible), 'unique_rgb':len(set(p[:3] for p in visible)),
            'bbox':tile.getbbox()}

board = Image.new('RGBA',(1100,610),(15,23,27,255)); d=ImageDraw.Draw(board)
d.text((16,10),'TENUES : avant / proposition - taille native et vues agrandies',font=font,fill='white')
for row,(name,index,label) in enumerate([('technicien',14,'Technicien'),('bastion',9,'Bastion')]):
    frames = packer.frames(f'{name}-source.png',4,1,4,HERE)
    y=48+row*276
    d.text((16,y+12),label,font=font,fill=(162,221,218,255))
    for col,(frame,pose) in enumerate(zip(frames,POSES)):
        x=133+col*240
        d.text((x,y),['Sans arme','Epee','Pistolet','Fusil'][col],font=font,fill='white')
        natives=[]
        for size in (48,128):
            tile = frame.resize((size,size),Image.Resampling.NEAREST)
            tile.save(HERE/f'{name}-{col}-{size}.png')
        new=Image.open(HERE/f'{name}-{col}-48.png').convert('RGBA')
        atlas=Image.open(ROOT/f'assets/sprites/cyberpunk/player-poses-{pose}-48.png').convert('RGBA')
        old=atlas.crop((index*48,0,(index+1)*48,48))
        # Native body band excludes a held weapon rising above the helmet.
        alpha = new.getchannel('A'); band=alpha.crop((20,0,28,48)).getbbox()
        assert band[3]-band[1] == 30, (name,pose,band)
        assert new.getbbox()[3] == 47, (name,pose,new.getbbox())
        stats['outfits'].append({'name':name,'pose':pose,'body_height':30,'feet_baseline':46,
            'old':metric(old),'new':metric(new)})
        for j,(tile,title) in enumerate([(old,'Avant'),(new,'Proposition')]):
            bx=x+j*116
            d.text((bx,y+28),title,font=small,fill=(190,201,209,255))
            board.alpha_composite(tile,(bx+22,y+49))
            # Native zoom x2 deliberately uses native pixels, not a new portrait.
            board.alpha_composite(tile.resize((96,96),Image.Resampling.NEAREST),(bx,y+98))
        # Inspection previews derive from the authored sources at the game's 128px size.
        old128=Image.open(ROOT/f'assets/sprites/cyberpunk/player-poses-{pose}-128.png').convert('RGBA').crop((index*128,0,(index+1)*128,128))
        new128=Image.open(HERE/f'{name}-{col}-128.png').convert('RGBA')
        # A separate sheet keeps the large views out of the native comparison.
        natives.extend([old128,new128])
    d.text((16,y+218),'Corps : 30px',font=small,fill='white')
    d.text((16,y+239),'Zoom : x2',font=small,fill=(170,180,190,255))
board.save(HERE/'comparatif-tenues.png')

board = Image.new('RGBA',(1120,650),(15,23,27,255)); d=ImageDraw.Draw(board)
d.text((16,10),'VUES D\'INSPECTION 128px : avant / proposition (nouveau dessin)',font=font,fill='white')
for row,(name,index) in enumerate([('technicien',14),('bastion',9)]):
    y=40+row*296
    for col,pose in enumerate(POSES):
        x=16+col*276
        d.text((x,y),f'{name} / {pose}',font=small,fill='white')
        for j in range(2):
            if j:
                tile=Image.open(HERE/f'{name}-{col}-128.png').convert('RGBA')
            else:
                a=Image.open(ROOT/f'assets/sprites/cyberpunk/player-poses-{pose}-128.png').convert('RGBA')
                tile=a.crop((index*128,0,(index+1)*128,128))
            d.text((x+j*136,y+26),'Proposition' if j else 'Avant',font=small,fill=(190,201,209,255))
            # Source canvas carries a large blank top margin. Crop only blank
            # area when presenting the authored figure, not any weapon pixels.
            board.alpha_composite(tile,(x+j*136,y+48))
board.save(HERE/'comparatif-inspection.png')

weapon_board=Image.new('RGBA',(1020,500),(15,23,27,255));d=ImageDraw.Draw(weapon_board)
d.text((16,10),'ARMES : natif 32px / zoom x4 / illustration 128px',font=font,fill='white')
for row,(name,label) in enumerate([('fusil','Fusil'),('pistolet','Pistolet'),('epee','Epee')]):
    source=HERE/f'{name}-source.png'
    if not source.exists(): continue
    im=Image.open(source).convert('RGBA')
    im.putalpha(im.getchannel('A').point(lambda a:255 if a>=128 else 0))
    im=im.crop(im.getchannel('A').getbbox())
    for size,margin in [(32,28),(128,112)]:
        thumb=im.copy();thumb.thumbnail((margin,margin),Image.Resampling.NEAREST)
        tile=Image.new('RGBA',(size,size));tile.alpha_composite(thumb,((size-thumb.width)//2,(size-thumb.height)//2))
        tile.save(HERE/f'{name}-{size}.png')
    if name=='epee':
        a=Image.open(ROOT/'assets/sprites/cyberpunk/ground-32.png').convert('RGBA');old=a.crop((32,0,64,32))
        # Ground sword has one shared icon; its existing inspected item stays unchanged.
        old128=old.resize((128,128),Image.Resampling.NEAREST)
    else:
        index=0 if name=='fusil' else 8
        a=Image.open(ROOT/'assets/sprites/cyberpunk/ranged-families-32.png').convert('RGBA');old=a.crop((index*32,0,(index+1)*32,32))
        a=Image.open(ROOT/'assets/sprites/cyberpunk/ranged-families-128.png').convert('RGBA');old128=a.crop((index*128,0,(index+1)*128,128))
    new=Image.open(HERE/f'{name}-32.png').convert('RGBA');new128=Image.open(HERE/f'{name}-128.png').convert('RGBA')
    stats['weapons'].append({'name':name,'old':metric(old),'new':metric(new)})
    y=43+row*150;d.text((16,y+42),label,font=font,fill=(162,221,218,255))
    for col,(tile,large,label2) in enumerate([(old,old128,'Avant'),(new,new128,'Proposition')]):
        x=145+col*405;d.text((x,y),label2,font=font,fill='white')
        weapon_board.alpha_composite(tile,(x,y+59))
        weapon_board.alpha_composite(tile.resize((128,128),Image.Resampling.NEAREST),(x+50,y+18))
        weapon_board.alpha_composite(large,(x+210,y+18))
weapon_board.save(HERE/'comparatif-armes.png')
stats['source_hashes']={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in HERE.glob('*-source.png')}
(HERE/'mesures-propositions.json').write_text(json.dumps(stats,indent=2)+'\n',encoding='utf-8')
print(json.dumps(stats['outfits']+stats['weapons']))
