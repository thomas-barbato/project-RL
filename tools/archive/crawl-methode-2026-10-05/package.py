"""Extract generated sheet cells and package native-size preview tiles.

No repainting: crop, alpha cutoff for game sprites, nearest-neighbour fit.
Active assets are never an output of this script.
"""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import json, hashlib, math

ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
OUT.joinpath('tiles').mkdir(exist_ok=True)
MODELS=json.loads((ROOT/'artifacts/armures-corps-commun-2026-10-05/modeles.json').read_text(encoding='utf-8'))
MONSTERS=json.loads((OUT/'bestiary.json').read_text(encoding='utf-8'))
WEAPONS=['Couteau','Épée à une main','Épée à deux mains','Hache','Matraque','Arme d’hast',
         'Pistolet','Pistolet mitrailleur','Fusil d’assaut','Fusil à pompe','Mitrailleuse',
         'Fusil à énergie','Lance-flammes','Lance-grenades','Lance-roquettes','Lance-aiguilles']
font=ImageFont.truetype(r'C:\Windows\Fonts\consola.ttf',16)
small=ImageFont.truetype(r'C:\Windows\Fonts\consola.ttf',13)

def cells(name,cols,rows):
    sheet=Image.open(OUT/name).convert('RGBA')
    images=[]
    for i in range(cols*rows):
        x,y=i%cols,i//cols
        image=sheet.crop((round(x*sheet.width/cols),round(y*sheet.height/rows),
                          round((x+1)*sheet.width/cols),round((y+1)*sheet.height/rows)))
        # GPU cutout sprites: discard almost-transparent generation halos.
        image.putalpha(image.getchannel('A').point(lambda a:255 if a>=128 else 0))
        bbox=image.getbbox()
        if not bbox: raise ValueError((name,i,'empty'))
        images.append(image.crop(bbox))
    return images

def creature_cells(name,cols,rows):
    # A generated contact sheet is not a reliable atlas grid: some silhouettes
    # cross a nominal cell boundary. Extract complete connected subjects first.
    sheet=Image.open(OUT/name).convert('RGBA')
    alpha=sheet.getchannel('A'); w,h=sheet.size
    occupied=bytearray(a>=128 for a in alpha.get_flattened_data())
    groups=[[] for _ in range(cols*rows)]
    for start in range(w*h):
        if not occupied[start]: continue
        occupied[start]=0; stack=[start]; members=[]
        left=right=start%w; top=bottom=start//w
        while stack:
            n=stack.pop(); members.append(n); x,y=n%w,n//w
            left=min(left,x);right=max(right,x);top=min(top,y);bottom=max(bottom,y)
            for ny in range(max(0,y-1),min(h,y+2)):
                for nx in range(max(0,x-1),min(w,x+2)):
                    j=ny*w+nx
                    if occupied[j]: occupied[j]=0;stack.append(j)
        if len(members)<32: continue
        cx=sum(n%w for n in members)/len(members)
        cy=sum(n//w for n in members)/len(members)
        index=min(rows-1,int(cy*rows/h))*cols+min(cols-1,int(cx*cols/w))
        groups[index].append((members,(left,top,right+1,bottom+1)))
    result=[]
    for index,parts in enumerate(groups):
        if not parts: raise ValueError(('missing creature',index))
        biggest=max(len(part[0]) for part in parts)
        parts=[part for part in parts if len(part[0])>=max(32,biggest*.006)]
        bounds=(min(p[1][0] for p in parts),min(p[1][1] for p in parts),
                max(p[1][2] for p in parts),max(p[1][3] for p in parts))
        image=Image.new('RGBA',(bounds[2]-bounds[0],bounds[3]-bounds[1]))
        source=sheet.load(); target=image.load()
        for members,_ in parts:
            for n in members:
                x,y=n%w,n//w
                target[x-bounds[0],y-bounds[1]]=source[x,y][:3]+(255,)
        result.append(image)
    return result

def indexed(image,budget):
    # Final game-tile encoding: a finite palette without dithering. Source art
    # stays unchanged; alpha is preserved independently of indexed RGB values.
    alpha=image.getchannel('A')
    rgb=image.convert('RGB').quantize(colors=budget,method=Image.Quantize.MEDIANCUT,
                                    dither=Image.Dither.NONE).convert('RGBA')
    rgb.putalpha(alpha)
    return rgb

def tile(image,maximum,feet=True,budget=24):
    factor=min(maximum[0]/image.width,maximum[1]/image.height)
    size=(max(1,round(image.width*factor)),max(1,round(image.height*factor)))
    resized=image.resize(size,Image.Resampling.NEAREST)
    result=Image.new('RGBA',(32,32))
    result.alpha_composite(resized,((32-size[0])//2,32-size[1] if feet else (32-size[1])//2))
    return indexed(result,budget)

def save_set(group,images,names,cols):
    atlas=Image.new('RGBA',(cols*32,math.ceil(len(images)/cols)*32))
    entries=[]
    for i,(im,name) in enumerate(zip(images,names)):
        path=f'tiles/{group}-{i+1:02}.png'
        im.save(OUT/path)
        atlas.alpha_composite(im,((i%cols)*32,(i//cols)*32))
        entries.append({'number':i+1,'name':name,'path':path,'bounds':im.getbbox(),
                        'colors':len({p[:3] for p in im.get_flattened_data() if p[3]})})
    atlas.save(OUT/f'{group}-32.png')
    return entries

armor=[tile(im,(24,30)) for im in cells('armor-source.png',6,3)[:17]]
weapon_images=creature_cells('weapons-source.png',4,4)
sizes=[(10,18),(11,24),(14,29),(15,27),(10,24),(10,29),(13,18),(17,22),
       (21,28),(18,28),(22,28),(21,28),(23,28),(20,27),(22,28),(18,26)]
weapons=[tile(im,size,False,16) for im,size in zip(weapon_images,sizes)]
mon_sizes=[(20,13),(29,24),(30,29),(23,18),(27,18),(30,20),(30,25),(30,22),
           (28,27),(29,29),(24,28),(27,22),(28,26),(24,28),(30,23),(30,20),(28,28)]+[(24,30)]*7
monsters=[tile(im,size) for im,size in zip(creature_cells('monsters-source.png',6,4),mon_sizes)]
manifest={'tile_size':32,'preview_only':True,'armor':save_set('armor',armor,[m['name'] for m in MODELS],6),
          'weapons':save_set('weapons',weapons,WEAPONS,4),'monsters':save_set('monsters',monsters,MONSTERS,6)}
# Grips preserve full-cell coordinates rather than fitting opaque bounds.
source=Image.open(OUT/'grips-source.png').convert('RGBA')
grips=[]
offsets=[(-1,-3),(2,-5),(0,0),(0,0)]
for i,(dx,dy) in enumerate(offsets):
    x,y=i%2,i//2
    im=source.crop((round(x*source.width/2),round(y*source.height/2),
                    round((x+1)*source.width/2),round((y+1)*source.height/2)))
    im.putalpha(im.getchannel('A').point(lambda a:255 if a>=128 else 0))
    im=im.resize((32,32),Image.Resampling.NEAREST)
    native=Image.new('RGBA',(40,40))
    native.alpha_composite(im,(4+dx,4+dy))
    native=indexed(native,16)
    path=f'tiles/grip-{i+1:02}.png';native.save(OUT/path)
    grips.append({'number':i+1,'path':path,'origin':[4,4],'offset':[dx,dy]})
manifest['grips']=grips
manifest['encoding']={'alpha':'binary cutoff128','armor_palette':24,'weapon_palette':16,
                      'monster_palette':24,'dithering':False,'resampling':'nearest'}
(OUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2),encoding='utf-8')

def board(group,images,names,cols=4):
    width=cols*280; rows=math.ceil(len(images)/cols)
    canvas=Image.new('RGB',(width,66+rows*170),'#141c21');d=ImageDraw.Draw(canvas)
    d.text((14,12),f'{group.upper()} / TILES NATIVES 32 × 32',font=font,fill='#eff4ef')
    d.text((14,38),'Petit : taille native. Grand : pixels agrandis ×3 sans lissage.',font=small,fill='#a8bec8')
    for i,(im,name) in enumerate(zip(images,names)):
        x,y=(i%cols)*280,66+(i//cols)*170
        d.rectangle((x+5,y+4,x+274,y+164),fill='#202a30')
        d.text((x+12,y+12),f'{i+1:02} {name[:26]}',font=small,fill='#a9e4d9')
        canvas.paste(im,(x+30,y+68),im)
        canvas.paste(im.resize((96,96),Image.Resampling.NEAREST),(x+135,y+43),im.resize((96,96),Image.Resampling.NEAREST))
    canvas.save(OUT/f'{group}-numerote.png')
board('armures',armor,[m['name'] for m in MODELS])
board('armes',weapons,WEAPONS)
board('bestiaire',monsters,MONSTERS)

old=Image.open(ROOT/'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
old_player=Image.open(ROOT/'assets/sprites/cyberpunk/player-poses-unarmed-48.png').convert('RGBA').crop((8,16,40,48))
comparison=Image.new('RGB',(1120,640),'#141c21');d=ImageDraw.Draw(comparison)
d.text((14,12),'32 PIXELS / ACTUEL, CRAWL, NOUVEL ESSAI',font=font,fill='#eaf0f2')
refs=Path(r'C:\Users\User\Desktop\crawl-master\crawl-master\crawl-ref\source\rltiles')
reference_paths=['player/base/human_m.png','mon/animals/black_bear.png','mon/animals/bat.png','mon/animals/boulder_beetle.png']
for i,(old_index,new,ref) in enumerate(zip([0,5,8,7],[armor[0],monsters[2],monsters[5],monsters[4]],reference_paths)):
    x=i*280
    before=old_player if i==0 else old.crop(((old_index%16)*32,(old_index//16)*32,(old_index%16+1)*32,(old_index//16+1)*32))
    reference=(Image.open(OUT/'reference-style.png').convert('RGBA').crop((768,0,1024,256)).resize((32,32),Image.Resampling.NEAREST)
               if i==0 else Image.open(refs/ref).convert('RGBA'))
    for row,(label,img) in enumerate([('Actuel',before),('Référence Crawl',reference),('Nouvel essai',new)]):
        y=58+row*188
        d.text((x+18,y),label,font=small,fill='#a9e4d9')
        comparison.paste(img,(x+30,y+62),img)
        z=img.resize((128,128),Image.Resampling.NEAREST)
        comparison.paste(z,(x+118,y+29),z)
comparison.save(OUT/'comparaison-32.png')
print(json.dumps({'armor':len(armor),'weapons':len(weapons),'monsters':len(monsters),'native':32}))
