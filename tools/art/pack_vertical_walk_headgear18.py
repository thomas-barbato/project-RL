"""C04 head/collar layers on byte-preserved approved C03 body cycles."""
from pathlib import Path
from copy import deepcopy
import json
import hashlib
from PIL import Image, ImageDraw, ImageFont
from pack_vertical_walk_review18 import opaque_box

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT/'assets/source-art/vertical-walk-review-18'
OUT = ART/'candidate-C04'
PREVIEW = ROOT/'artifacts/vertical-walk-review-18/current/headgear-C04'


def normalize(figure, scale, foot):
    image = figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.LANCZOS)
    box = opaque_box(image)
    out = Image.new('RGBA',(128,128))
    out.alpha_composite(image,(round(64-(box[0]+box[2])/2),foot-box[3]))
    return out


def neutral_figures(image):
    # The front/back neutral cells are both in column one. Other columns may
    # have taller passing-foot poses, so find the row valleys in this column.
    column=image.crop((0,0,round(image.width/4),image.height))
    alpha=column.getchannel('A')
    score=[sum(v>=32 for v in alpha.crop((0,y,column.width,y+1)).tobytes()) for y in range(column.height)]
    edges=[0]
    for q in [1,2,3]:
        target=round(q*image.height/4)
        y=min(range(target-70,target+71),key=lambda y:(score[y],abs(y-target)))
        assert score[y]==0,('neutral column row boundary',q)
        edges.append(y)
    edges.append(image.height)
    result=[]
    for row in [0,2]:
        cell=column.crop((0,edges[row],column.width,edges[row+1]));box=opaque_box(cell)
        assert box and box[0]>0 and box[2]<cell.width
        whole=cell.crop(box);padded=Image.new('RGBA',(whole.width+4,whole.height+4));padded.alpha_composite(whole,(2,2));result.append(padded)
    return result


def canonical_head(image, mask):
    top = opaque_box(image)[1]
    shifted = Image.new('RGBA',(128,128))
    shifted.alpha_composite(image,(0,round(40-(top+6))))
    head = shifted.crop((48,24,80,56))
    head.putalpha(Image.composite(head.getchannel('A'),Image.new('L',(32,32)),mask.getchannel('A')))
    return head


def replace_head(image, head, mask, anchor):
    image = image.copy()
    x,y=round(anchor[0])-16,round(anchor[1])-16
    clear=Image.new('RGBA',(32,32))
    image.paste(clear,(x,y),mask.getchannel('A'))
    image.alpha_composite(head,(x,y))
    return image


def main():
    OUT.mkdir(parents=True,exist_ok=True);PREVIEW.mkdir(parents=True,exist_ok=True)
    old=json.loads((ART/'candidate-C03/manifest.json').read_text('utf8'))
    requests=json.loads((ROOT/'docs/reviews/vertical-walk-18/prompts-C04.json').read_text('utf8'))
    request_by_id={c.get('profile'):c for c in requests['calls'] if c.get('profile')}
    rigs={r['index']:r for r in json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf8'))['outfits']}
    index_by_id={'bastion-lourd':4,'traqueuse-lourde':14,'arcaniste-lourd':19,'metamorphe-lourd':24}
    profiles=[];unchanged=0
    board=Image.new('RGB',(960,1220),'#1e2831');draw=ImageDraw.Draw(board)
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),14)
    for row,profile in enumerate(old['profiles']):
        identity=profile['id'];sheet=Image.open(ROOT/profile['file']).convert('RGBA')
        mask=Image.new('RGBA',(32,32));md=ImageDraw.Draw(mask)
        md.rectangle((0 if identity=='arcaniste-lourd' else 5,0,31 if identity=='arcaniste-lourd' else 27,29 if identity=='arcaniste-lourd' else 24),fill=(255,255,255,255))
        if identity in request_by_id:
            req=request_by_id[identity];raw=Image.open(ROOT/req['output']).convert('RGBA');cells=neutral_figures(raw)
            full=[normalize(cell,profile['source_scale_xy_uniform'],profile['original_foot_y']) for cell in cells]
            source=req['output']
        else:
            full=[sheet.crop((0,0,128,128)),sheet.crop((0,128,128,256))]
            source=profile['source']
        heads=[canonical_head(im,mask) for im in full]
        head_paths=[]
        for name,head in zip(['front','back'],heads):
            path=OUT/f'{identity}-head-{name}.png';head.save(path);head_paths.append(path.relative_to(ROOT).as_posix())
        mask_path=OUT/f'{identity}-replace-mask.png';mask.save(mask_path)
        result=Image.new('RGBA',(768,256));frames=[]
        for f in profile['frames']:
            back=f['direction']=='back';phase=f['phase']
            before=sheet.crop((phase*128,back*128,(phase+1)*128,(back+1)*128))
            anchor=[64, f['box'][1]+6]
            after=replace_head(before,heads[int(back)],mask,anchor)
            assert before.crop((0,60,128,128)).tobytes()==after.crop((0,60,128,128)).tobytes()
            unchanged+=1
            result.paste(after,(phase*128,back*128))
            frame=deepcopy(f);frame['head_anchor']=anchor;frame['box']=list(opaque_box(after));frames.append(frame)
            board.paste(after,(150+phase*128,row*300+back*144+18),after)
        for back in [False,True]: draw.text((8,row*300+back*144+5),identity+(' / dos' if back else ' / face'),font=font,fill='white')
        path=OUT/f'{identity}-unhelmeted.png';result.save(path)
        standing_anchor=rigs[index_by_id[identity]]['head_center']
        standing=replace_head(Image.open(ROOT/profile['standing']).convert('RGBA'),heads[0],mask,standing_anchor)
        standing_path=OUT/f'{identity}-unhelmeted-standing.png';standing.save(standing_path)
        new=deepcopy(profile);new.update({'id':identity+'-headgear','file':path.relative_to(ROOT).as_posix(),'standing':standing_path.relative_to(ROOT).as_posix(),
            'source':source,'source_sha256':hashlib.sha256((ROOT/source).read_bytes()).hexdigest(),'head_variants':True,'frames':frames,'head_parts':{'front':head_paths[0],'back':head_paths[1],
            'replace_mask':mask_path.relative_to(ROOT).as_posix(),'standing_anchor':standing_anchor},'accepted_body_reference':profile['file']})
        profiles.append(new)
    # Exact existing front art/openings; only the six back views are authored anew.
    front=Image.open(ROOT/'assets/sprites/class-armors-01/helmets.png').convert('RGBA')
    openings=Image.open(ROOT/'assets/sprites/class-armors-01/helmet-openings.png').convert('RGBA')
    front_bank=Image.new('RGBA',(192,32));opening_bank=front_bank.copy();back_bank=front_bank.copy()
    back_source=next(c for c in requests['calls'] if c['id']=='helmet-backs')['output']
    backs=Image.open(ROOT/back_source).convert('RGBA')
    for i in range(6):
        front_bank.paste(front.crop((i*128+48,24,i*128+80,56)),(i*32,0))
        opening_bank.paste(openings.crop((i*128+48,24,i*128+80,56)),(i*32,0))
        assert front_bank.crop((i*32,0,(i+1)*32,32)).tobytes()==front.crop((i*128+48,24,i*128+80,56)).tobytes()
        c=backs.crop((round(i%3*backs.width/3),round(i//3*backs.height/2),round((i%3+1)*backs.width/3),round((i//3+1)*backs.height/2)))
        b=opaque_box(c);assert b and b[0]>0 and b[1]>0 and b[2]<c.width and b[3]<c.height
        c=c.crop(b);scale=11/c.height;c=c.resize((round(c.width*scale),11),Image.Resampling.LANCZOS)
        assert c.width<=24
        back_bank.alpha_composite(c,(i*32+16-c.width//2,11))
    for name,im in [('helmets-front',front_bank),('helmets-back',back_bank),('helmet-openings-front',opening_bank)]: im.save(OUT/f'{name}.png')
    ownership=Image.new('RGBA',(32,32));ImageDraw.Draw(ownership).polygon([(5,0),(27,0),(27,18),(22,22),(10,22),(5,18)],fill=(255,255,255,255));ownership.save(OUT/'head-ownership.png')
    (OUT/'headgear.json').write_text(json.dumps({'front':(OUT/'helmets-front.png').relative_to(ROOT).as_posix(),
        'back':(OUT/'helmets-back.png').relative_to(ROOT).as_posix(),'opening':(OUT/'helmet-openings-front.png').relative_to(ROOT).as_posix(),
        'ownership':(OUT/'head-ownership.png').relative_to(ROOT).as_posix(),'back_exposed_hair_models':[115,119],
        'front_art_unchanged':True,'models':[115,116,117,118,119,120]},indent=2)+'\n','utf8')
    (OUT/'manifest.json').write_text(json.dumps({'review':'VERT18-C04','status':'review_only','normal_game_enabled':False,
        'native_tile_px':64,'profiles':profiles,'body_frames_byte_identical_below_y60':unchanged},ensure_ascii=False,indent=2)+'\n','utf8')
    board.save(PREVIEW/'contact-unhelmeted-native64.png')
    (PREVIEW/'conditioning-checks.json').write_text(json.dumps({'review':'VERT18-C04','body_frames_unchanged_below_y60':unchanged,
        'old_front_helmet_cells_byte_identical':6,'new_back_helmet_cells':6,'selected_source_sha256':hashlib.sha256((ROOT/back_source).read_bytes()).hexdigest()},indent=2)+'\n','utf8')
    print(json.dumps({'profiles':4,'head_choices':7,'preserved_body_frames':unchanged}))


if __name__=='__main__': main()
