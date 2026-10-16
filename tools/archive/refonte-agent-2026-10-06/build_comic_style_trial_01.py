"""Crop/scale generated comic studies into a numbered comparison board."""
from pathlib import Path
import hashlib,json,math
from PIL import Image,ImageDraw
from build_focalizer_review_01 import font,ref,jref,save,write,url
ROOT=Path(__file__).resolve().parents[2];BASE=Path(__file__).resolve().parent
OUT=BASE/'comic-style-review-01';BG=(14,21,29);FG=(235,240,244);SUB=(160,182,196)
NAMES=['Bastion','Technomancien','Traqueur','Arcaniste','Métamorphe']
GEAR=['Lame et bouclier','Pistolet','Fusil de précision','Hampe de canalisation','Couteau et focaliseur']
CENTERS=[640,650,590,630,650]
DETAIL_BOXES=[(820,70,1038,759),(210,590,420,855),(684,364,943,626),(985,41,1170,280),(840,600,1077,858)]

def rgba_crop(image):
    return image.getchannel('A').point(lambda a:255 if a>=16 else 0).getbbox()
def fitted(image,max_size):
    ratio=min(max_size[0]/image.width,max_size[1]/image.height)
    return image.resize((max(1,round(image.width*ratio)),max(1,round(image.height*ratio))),Image.Resampling.LANCZOS)
def normalize(image,box,body_top,body_bottom,body_center,tile):
    # Body landmarks set scale; weapons may extend beyond the 32px body tile.
    crop=image.crop(box);target_height=30*tile/32
    scale=(target_height-1)/(body_bottom-body_top)
    scaled=crop.resize((math.ceil(crop.width*scale),math.ceil(crop.height*scale)),Image.Resampling.LANCZOS)
    canvas_side=tile*2;anchor=(tile,round(47*tile/32))
    offset=(round(anchor[0]-(body_center-box[0])*scale),round(anchor[1]-(body_bottom-box[1])*scale))
    assert offset[0]>=0 and offset[1]>=0 and offset[0]+scaled.width<=canvas_side and offset[1]+scaled.height<=canvas_side
    result=Image.new('RGBA',(canvas_side,canvas_side));result.alpha_composite(scaled,offset)
    return result,{'tile_pixels':tile,'body_target_height':target_height,'source_body_top':body_top,
                   'source_body_bottom':body_bottom,'source_body_center_x':body_center,'source_crop':list(box),
                   'scale':scale,'offset':list(offset),'canvas':list(result.size),'clipped_equipment_pixels':0,
                   'sampling':'LANCZOS;antialiased_comic_contours_preserved;no_palette_reduction'}

def main():
    assert not (OUT/'review-BD-STYLE01-01.json').exists()
    rig=ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json'
    active=json.loads(rig.read_text(encoding='utf-8'))
    assert all(hashlib.sha256((ROOT/r['path']).read_bytes()).hexdigest()==r['binary_sha256'] for r in active['files'])
    entries=json.loads((OUT/'prompts.json').read_text(encoding='utf-8'))['assets']
    references=json.loads((OUT/'references.json').read_text(encoding='utf-8'))
    floor=Image.open(BASE/'weapon-silhouette-review-03/review-floor-native32.png').convert('RGBA')
    terrain=Image.new('RGBA',(64,64))
    for y in (0,32):
        for x in (0,32):terrain.alpha_composite(floor,(x,y))
    floors={'terrain':terrain,'dark':Image.new('RGBA',(64,64),(18,25,33,255)),
            'light':Image.new('RGBA',(64,64),(175,184,186,255))}
    records=[];images={};details={}
    for i,e in enumerate(entries):
        n=i+1;cid=e['review_id'];path=OUT/'sources'/f'{cid}.png'
        raw=Image.open(path).convert('RGBA');initial_box=rgba_crop(raw)
        body_bottom=initial_box[3]-1
        if n==1:
            body_top=rgba_crop(raw.crop((520,0,760,raw.height)))[1]
        elif n==4:
            body_top=rgba_crop(raw.crop((450,0,700,600)))[1]
        else:body_top=initial_box[1]
        center=CENTERS[i];detail=raw.crop(DETAIL_BOXES[i])
        mirrored=n==2
        if mirrored:
            raw=raw.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            center=raw.width-1-center;detail=detail.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        box=rgba_crop(raw);crop=raw.crop(box)
        crop_ref=save(OUT/f'{cid}-comic-master.png',crop)
        images[(n,'master')]=crop;details[n]=fitted(detail,(230,135))
        current=Image.open(OUT/f'reference-C{n}-native64.png').convert('RGBA')
        qa=[];tiles={}
        for tile in (30,32):
            comic,measurement=normalize(raw,box,body_top,body_bottom,center,tile)
            native_ref=save(OUT/f'{cid}-tile{tile}.png',comic)
            tiles[str(tile)]=native_ref;qa.append(measurement)
            images[(n,f'comic{tile}')]=comic
            images[(n,f'current{tile}')]=current if tile==32 else current.resize((60,60),Image.Resampling.NEAREST)
        records.append({'review_id':cid,'class_number':n,'class_name':NAMES[i],'equipment_description':GEAR[i],
                        'source':ref(path),'comic_master':crop_ref,'native_tiles':tiles,
                        'current_reference':references['characters'][i]['native_reference'],
                        'horizontal_mirror_of_complete_generated_study':mirrored,'normalization':qa,
                        'approval':'pending_user_style_and_pixels_review','layer_readiness':'complete_equipped_study;not_a_body_weapon_arm_layer_bank'})
    def scene(n,mode,tile,floor_name='terrain'):
        side=tile*2;ground=floors[floor_name] if tile==32 else floors[floor_name].resize((side,side),Image.Resampling.NEAREST)
        result=ground.copy();result.alpha_composite(images[(n,f'{mode}{tile}')]);return result
    board=Image.new('RGB',(1500,1070),BG);d=ImageDraw.Draw(board)
    d.text((24,18),'Project RL · Essai bande dessinée',font=font(32),fill=FG)
    d.text((24,62),'BD-STYLE01 · cinq personnages équipés · contours encrés, couleurs en aplats et ombres simples',font=font(16),fill=SUB)
    for n in range(1,6):
        x=20+(n-1)*292;center=x+146
        d.text((x+12,108),f'{n:02} · {NAMES[n-1]}',font=font(20),fill=FG)
        d.text((x+12,137),GEAR[n-1],font=font(13),fill=SUB)
        record=records[n-1];source=images[(n,'master')]
        q=record['normalization'][1];body_height=q['source_body_bottom']-q['source_body_top']
        scale=279/body_height
        master=source.resize((round(source.width*scale),round(source.height*scale)),Image.Resampling.LANCZOS)
        assert master.width<280
        board.paste(master,(center-master.width//2,485-master.height),master)
        if n>1:d.line((x-7,104,x-7,997),fill=(37,50,61))
    d.text((24,510),'Détails des prises et des équipements',font=font(21),fill=FG)
    for n in range(1,6):
        x=20+(n-1)*292;im=details[n]
        board.paste(im,(x+146-im.width//2,553+(135-im.height)//2),im)
    d.text((24,711),'À l’échelle du jeu · style actuel et essai BD',font=font(21),fill=FG)
    for n in range(1,6):
        x=20+(n-1)*292
        d.text((x+22,751),'Actuel · 32 px',font=font(12),fill=SUB)
        d.text((x+144,751),'BD · 32 px',font=font(12),fill=SUB)
        board.paste(scene(n,'current',32),(x+28,775));board.paste(scene(n,'comic',32),(x+149,775))
        d.text((x+23,861),'Actuel · 30 px',font=font(12),fill=SUB)
        d.text((x+145,861),'BD · 30 px',font=font(12),fill=SUB)
        board.paste(scene(n,'current',30,'light'),(x+30,886));board.paste(scene(n,'comic',30,'light'),(x+151,886))
    d.text((24,990),'Sources BD en haute définition · petites vues à taille réelle dans le fichier · les armes peuvent dépasser de la case.',font=font(14),fill=SUB)
    d.text((24,1019),'Étude visuelle à examiner · le passage global au style BD reste à décider.',font=font(14),fill=SUB)
    board_ref=save(OUT/'comic-style-board-01.png',board)
    zoomboard=Image.new('RGB',(1500,690),BG);zd=ImageDraw.Draw(zoomboard)
    zd.text((24,18),'BD · vérification des petites vues',font=font(27),fill=FG)
    zd.text((24,60),'32 px sur le sol du jeu · 30 px sur fond clair · zoom ×4 des images réellement réduites',font=font(15),fill=SUB)
    for n in range(1,6):
        x=20+(n-1)*292;zd.text((x+12,100),f'{n:02} · {NAMES[n-1]}',font=font(19),fill=FG)
        zoomboard.paste(scene(n,'comic',32).resize((256,256),Image.Resampling.NEAREST),(x+12,144))
        zoomboard.paste(scene(n,'comic',30,'light').resize((240,240),Image.Resampling.NEAREST),(x+20,426))
    zoom_ref=save(OUT/'comic-style-native-zoom-01.png',zoomboard)
    payload={'classes':[{'id':r['review_id'],'name':r['class_name'],'number':r['class_number'],'gear':r['equipment_description'],
                         'master':url(images[(r['class_number'],'master')]),'frames':{f'{mode}{tile}':url(images[(r['class_number'],f'{mode}{tile}')]) for mode in ('comic','current') for tile in (30,32)}} for r in records],
             'floors':{k:url(im) for k,im in floors.items()},'board':url(board)}
    template=BASE/'comic_style_gallery_template_01.html';gallery=OUT/'comic-style-gallery-01.html'
    gallery.write_text(template.read_text(encoding='utf-8').replace('__PAYLOAD__',json.dumps(payload,ensure_ascii=False)),encoding='utf-8',newline='\n')
    manifest={'date':'2026-10-10','review_id':'BD-STYLE01','revision':1,'request':'A trial board of characters and weapons in comic-book style; global migration conditional on user liking the trial.',
              'source_references':jref(OUT/'references.json'),'prompts':jref(OUT/'prompts.json'),'generator':ref(Path(__file__)),
              'template':ref(template),'source_active_rig':jref(rig),'accepted_shafts':jref(BASE/'shaft-visual-acceptance-01.json'),
              'candidates':records,'board':board_ref,'native_zoom':zoom_ref,'gallery':ref(gallery),
              'approval':'pending_style_and_new_pixels;not_a_global_migration_authorization','mechanical_changes':'none',
              'runtime_integration':'none;complete_equipped_style_studies_only','qa':{'characters':5,'native_variants':10,'clipped_equipment_pixels':0,'transparent_sources':True,
               'normalization':'Scale from body landmarks rather than whole equipment bounds; native32 uses approximately30px body height.',
               'active_art_asset_changes':0}}
    assert all(hashlib.sha256((ROOT/r['path']).read_bytes()).hexdigest()==r['binary_sha256'] for r in active['files'])
    write(OUT/'review-BD-STYLE01-01.json',manifest)
    print(json.dumps({'characters':5,'native_variants':10,'body_landmarks':[r['normalization'][1] for r in records],'art_asset_changes':0}))

if __name__=='__main__':main()
