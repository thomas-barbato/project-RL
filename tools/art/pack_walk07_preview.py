"""Losslessly pack the exact authored-pose/real-weapon review layers."""
import base64,gzip,hashlib,io,json,math,sys
from pathlib import Path
from PIL import Image,ImageOps,ImageDraw,ImageFont
from build_player_walk_matrix_06 import grid_figures

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07';ART=ROOT/'artifacts/player-walk-review-07'
INPUT=ART/'recipe-final';OUT=ART/'preview-data';OUT.mkdir(parents=True,exist_ok=True)
NAMES=['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe']
SLUGS=['bastion','technomancien','traqueuse','arcaniste','metamorphe']
VIS=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c')
baseline=json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf-8'))
labels={c['weapon']:c['label'] for c in baseline['classConfigs'] if c['weapon'] is not None}

def label(config):
    if config['weapon'] is not None:
        value=labels[config['weapon']]
        for _ in range(2):
            if not any(x in value for x in ['Ã','Â','â']):break
            try:value=value.encode('cp1252').decode('utf-8')
            except (UnicodeError,ValueError):break
        return value
    return {'unarmed':'Sans arme','dual':'Deux pistolets','mixed':'Lame et pistolet','blades':'Deux lames'}[config['label']]

def pack(images):
    width=1024;placements={};x=y=rowh=0
    for i in sorted(range(len(images)),key=lambda i:(-images[i].height,-images[i].width)):
        im=images[i]
        if x+im.width+2>width:y+=rowh+2;x=0;rowh=0
        placements[i]=(x,y,im.width,im.height);x+=im.width+2;rowh=max(rowh,im.height)
    height=y+rowh
    assert height<16380,(width,height)
    atlas=Image.new('RGBA',(width,max(1,height)))
    for i,im in enumerate(images):atlas.paste(im,placements[i][:2])
    buffer=io.BytesIO();atlas.save(buffer,format='WEBP',lossless=True,method=6,exact=True)
    encoded=buffer.getvalue()
    assert Image.open(io.BytesIO(encoded)).convert('RGBA').tobytes()==atlas.tobytes()
    return encoded,[placements[i] for i in range(len(images))],atlas.size

def save(slug,title,images,options,armors,frames,recipe=False,class_indices=None,reference=None,revision=None,reference_label=None,default_armor=0,reference_rects=None,fragment_slug=None,extra_data=None):
    encoded,rects,size=pack(images)
    frame_json=json.dumps(frames,separators=(',',':')).encode()
    data={'atlas':'data:image/webp;base64,'+base64.b64encode(encoded).decode(),
        'rects':rects,'options':options,'armors':armors,'title':title,
        'framesGzip':base64.b64encode(gzip.compress(frame_json,mtime=0)).decode(),'recipe':recipe}
    if class_indices is not None:data['class_indices']=class_indices
    if default_armor:data['defaultArmor']=default_armor
    if revision:data['revision']=revision
    if reference:data.update({'referenceAtlas':reference,'defaultKey':'unarmed','revision':revision or 'r04-opposite-support-pilot',
        'referenceLabel':reference_label or 'Bastion validé · taille native'})
    if reference_rects is not None:data['referenceRects']=reference_rects
    if extra_data:data.update(extra_data)
    (OUT/f'{slug}.webp').write_bytes(encoded)
    (OUT/f'{slug}.json').write_text(json.dumps(data,ensure_ascii=False,separators=(',',':')),'utf-8')
    template=ROOT/'tools/art/walk07_preview.html'
    fragment_size=None
    if template.exists() and '--data-only' not in sys.argv:
        fragment=(template.read_text('utf-8').replace('__WALK07_ID__',slug).replace('__WALK07_TITLE__',title)
            .replace('__WALK07_DATA__',json.dumps(data,ensure_ascii=False,separators=(',',':'))))
        fragment_size=len(fragment.encode('utf-8'));assert fragment_size<1_000_000,(slug,fragment_size)
        (VIS/f'walk07-{fragment_slug or slug}.html').write_text(fragment,'utf-8')
    return {'slug':slug,'title':title,'sprites':len(images),'atlas_size':size,'atlas_bytes':len(encoded),'class_indices':class_indices,
        'frame_json_bytes':len(frame_json),'fragment_bytes':fragment_size,'options':len(options),'armors':len(armors)}

def equipment():
    reports=[]
    revision_path=DOC/'review-revision.json'
    revision=json.loads(revision_path.read_text('utf8')) if revision_path.exists() else {}
    accepted=ROOT/'assets/source-art/player-walk-review-01/accepted/native64'
    walk=Image.open(accepted/'bastion-walk-native64.png').convert('RGBA');idle=Image.open(accepted/'bastion-idle-native64.png').convert('RGBA')
    reference=Image.new('RGBA',(448,128))
    for side,row in enumerate([7,1]):
        reference.paste(idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64)),(0,side*64))
        for col in range(6):reference.paste(walk.crop((col*64,row*64,col*64+64,row*64+64)),((col+1)*64,side*64))
    buffer=io.BytesIO();reference.save(buffer,format='WEBP',lossless=True,method=6,exact=True)
    assert Image.open(io.BytesIO(buffer.getvalue())).convert('RGBA').tobytes()==reference.tobytes()
    reference='data:image/webp;base64,'+base64.b64encode(buffer.getvalue()).decode()
    for ci,name in enumerate(NAMES):
        only=next((arg.split('=',1)[1] for arg in sys.argv if arg.startswith('--only=')),None)
        if only is not None and SLUGS[ci]!=only:continue
        indices=list(range(ci*5,ci*5+5));pilot=False
        if revision.get('limit_reported_families_to_pilots'):
            if ci not in [2,4]:continue
            indices=[i for i in indices if i in revision['repair_pilot_indices']];pilot=True
        inputs=[INPUT/f'CLASS-{i:02}'/'layers.json' for i in indices]
        if not all(p.exists() for p in inputs):continue
        images=[];seen={};frames=[];options=None;armors=[]
        for rank,p in enumerate(inputs):
            d=json.loads(p.read_text('utf-8'));remap={}
            for i,s in enumerate(d['sprites']):
                im=ImageOps.flip(Image.open(p.parent/s['file']).convert('RGBA'))
                # Hidden RGB is not rendered; canonicalize only alpha-zero
                # pixels so identical visible layers deduplicate faithfully.
                im.paste((0,0,0,0),(0,0,*im.size),im.getchannel('A').point(lambda a:255 if a==0 else 0))
                key=(im.size,hashlib.sha256(im.tobytes()).digest())
                if key not in seen:seen[key]=len(images);images.append(im)
                remap[i]=seen[key]
            actual=[{'label':label(c),'key':c['label'],'weapon':c['weapon'],'hands':c['hands'],'foot':[64,96],'shadow':True} for c in d['configs']]
            if options is None:options=actual
            else:assert options==actual
            assert len(actual)==117 and len({c['weapon'] for c in actual if c['weapon'] is not None})==113
            actor_frames=[]
            for config in d['configs']:
                assert len(config['frames'])==14
                assert d['recipe'] is True
                # Four decimals change placement by <0.013 native pixel even
                # at the farthest corner. Keep every source raster lossless.
                compact=lambda n:int(round(n,4)) if round(n,4).is_integer() else round(n,4)
                actor_frames.append([[[[remap[cmd[0]],*[compact(n) for n in cmd[1:]]] for cmd in group] for group in frame] for frame in config['frames']])
            frames.append(actor_frames);armors.append(f'{indices[rank]+1:02} · Tenue {indices[rank]%5+1}'+(' · validée' if indices[rank] in revision.get('accepted_class_indices',[]) else ''))
        candidate=revision.get('candidate_revisions',{}).get(str(indices[0]),'r07')
        family_candidate=revision.get('family_candidate_revisions',{}).get(SLUGS[ci],candidate)
        title=name+(' · cinq tenues · essai '+family_candidate if pilot and len(indices)>1 else (' · tenue '+str(indices[0]%5+1)+' · essai '+candidate if pilot else ' · armes et tenues'))
        selected_reference=reference;reference_label=None
        reference_mode=revision.get('family_reference_modes',{}).get(SLUGS[ci],revision.get('reference_mode'))
        if pilot and reference_mode=='exact_original_bastion_canvas02':reference_label='Bastion original · référence 02'
        if pilot and reference_mode=='accepted_traqueuse_r07f':
            accepted_cycle=Image.open(ROOT/'assets/source-art/player-walk-review-07/accepted/traqueuse-r07f/whole-body-native64.png').convert('RGBA')
            encoded=io.BytesIO();accepted_cycle.save(encoded,format='WEBP',lossless=True,exact=True,method=6)
            assert Image.open(io.BytesIO(encoded.getvalue())).convert('RGBA').tobytes()==accepted_cycle.tobytes()
            selected_reference='data:image/webp;base64,'+base64.b64encode(encoded.getvalue()).decode()
            reference_label='Traqueuse · tenue 1 validée'
        if pilot and reference_mode=='same_character_accepted_walk06':
            bank=Image.open(ROOT/f'assets/animations/walk-06/{SLUGS[ci]}-rifle.png').convert('RGBA')
            start=indices[0]%5*128;accepted_cycle=bank.crop((0,start,448,start+128))
            encoded=io.BytesIO();accepted_cycle.save(encoded,format='WEBP',lossless=True,exact=True,method=6)
            assert Image.open(io.BytesIO(encoded.getvalue())).convert('RGBA').tobytes()==accepted_cycle.tobytes()
            selected_reference='data:image/webp;base64,'+base64.b64encode(encoded.getvalue()).decode()
            reference_label='Cycle validé · avec fusil'
        elif pilot and reference_mode=='same_character_previous_r05_unarmed':
            previous=Image.open(DOC/f'references/CLASS-{indices[0]:02}-r05-unarmed-cycle.png').convert('RGBA')
            encoded=io.BytesIO();previous.save(encoded,format='WEBP',lossless=True,exact=True,method=6)
            assert Image.open(io.BytesIO(encoded.getvalue())).convert('RGBA').tobytes()==previous.tobytes()
            selected_reference='data:image/webp;base64,'+base64.b64encode(encoded.getvalue()).decode()
            reference_label='Essai précédent · sans arme'
        reports.append(save(SLUGS[ci],title,images,options,armors,frames,recipe=True,class_indices=indices,
            reference=selected_reference if pilot else None,revision=revision.get('revision','')+'-'+revision.get('material_revision',''),reference_label=reference_label,
            default_armor=1 if pilot and len(indices)>1 and indices[0] in revision.get('accepted_class_indices',[]) else 0))
    return reports

def creatures(lot=1,placement_review=False):
    suffix='' if lot==1 else f'-lot{lot:02}'
    jobs=json.loads((DOC/f'creature-jobs{suffix}.json').read_text('utf-8'));images=[];options=[];frames=[];audit=[]
    reference_atlas=Image.new('RGBA',(128,128*len(jobs)));reference_rects=[]
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
    for j in jobs:
        path=Path(j['source_destination'])
        if not path.exists():continue
        source=Image.open(path).convert('RGBA')
        opaque_outer_edges=None
        if j.get('strict_source_outer_margin'):
            alpha=source.getchannel('A');w,h=source.size
            opaque_outer_edges=[sum(v>=128 for v in alpha.crop(b).tobytes()) for b in
                                [(0,0,1,h),(w-1,0,w,h),(0,0,w,1),(0,h-1,w,h)]]
            assert not any(opaque_outer_edges),(j['id'],'source figure reaches outer canvas edge',opaque_outer_edges)
        if j['id']=='MON-124-0':
            # The spirit source has5 columns instead of4: retain the first7
            # complete poses in each directional half, and discard extras.
            from build_player_walk_review_04 import components
            parts=sorted(components(source),key=lambda p:p['area'],reverse=True)[:20]
            parts.sort(key=lambda p:p['center'][1]);parts=[p for row in range(4) for p in sorted(parts[row*5:row*5+5],key=lambda p:p['center'][0])]
            figures=[source.crop(p['box']) for p in parts];indices=[*range(7),*range(10,17)];warnings=[]
        else:
            figures,warnings=grid_figures(source,j.get('source_columns',4),j.get('source_rows',4))
            indices=j.get('selected_source_indices',[0,*range(1,7),8,*range(9,15)])
        b=j['record']['bounds'];old_foot=j['record']['foot'];target_h=b[3]-b[1];target_w=b[2]-b[0]
        # Padding for tall native actors changes their atlas origin only;
        # their ground contact in the scene remains the same.
        foot=[64,max(112,old_foot[1]-b[1]+4)]
        assert foot[1]<=124,(j['id'],'native actor needs a larger review canvas')
        # Compare to the existing rendered figure at its exact native scale,
        # never to the enlarged identity-detail reference used for drawing.
        original_path=DOC/'references'/f'{j["id"]}-native-current.png'
        original=Image.open(original_path).convert('RGBA').crop(b)
        ref_x=foot[0]+b[0]-old_foot[0];ref_y=foot[1]+b[1]-old_foot[1]
        assert 0<=ref_x and ref_x+original.width<=128 and 0<=ref_y and ref_y+original.height<=128
        reference_atlas.paste(original,(ref_x,len(options)*128+ref_y))
        reference_rects.append([0,len(options)*128,128,128])
        selected=[figures[i] for i in indices]
        # A three-quarter pose can project a different width. Preserve native
        # standing height; do not shrink the whole spirit to its front-view
        # width. Limit only the review canvas, never deform a body axis.
        width_limit=j.get('review_width_limit',124)
        assert 0<width_limit<=126
        scale=min(target_h/max(im.height for im in selected),width_limit/max(im.width for im in selected))
        if j.get('preserve_native_height'):
            assert round(max(im.height for im in selected)*scale)==target_h,(j['id'],'review width would shrink native height')
        tiles=[]
        for figure in selected:
            im=figure.resize((max(1,round(figure.width*scale)),max(1,round(figure.height*scale))),Image.Resampling.LANCZOS)
            x=round(foot[0]-im.width/2)
            y=foot[1]+b[3]-old_foot[1]-im.height
            assert 0<=x and x+im.width<=128 and 0<=y and y+im.height<=128,(j['id'],x,y,im.size)
            tile=Image.new('RGBA',(128,128));tile.alpha_composite(im,(x,y));tiles.append(tile)
        # This source drew the quadruped's directional halves in reverse.
        reordered=j.get('directional_halves_reordered',j['id']=='MON-075-0')
        if reordered:tiles=tiles[7:]+tiles[:7]
        frozen_path=ROOT/f'assets/source-art/player-walk-review-07/accepted/creatures-lot{lot:02}/{j["id"]}-whole-body-native.png'
        if frozen_path.exists():
            frozen=Image.open(frozen_path).convert('RGBA')
            for pose,tile in enumerate(tiles):
                col,side=pose%7,pose//7
                assert frozen.crop((col*128,side*128,(col+1)*128,(side+1)*128)).tobytes()==tile.tobytes(),(j['id'],pose,'accepted creature changed')
        # Move the complete rendered figure, never its pixels or limbs. One
        # anchor for the whole cycle avoids compensating every raised foot.
        boxes=[tile.getbbox() for tile in tiles]
        cycle_bounds=[min(b[0] for b in boxes),min(b[1] for b in boxes),
                      max(b[2] for b in boxes),max(b[3] for b in boxes)]
        cycle_height=cycle_bounds[3]-cycle_bounds[1]
        vertical_offset=math.floor(foot[1]-32-(cycle_bounds[1]+cycle_bounds[3])/2+.5) if cycle_height<64 else 0
        f=[]
        for tile in tiles:
            bb=tile.getbbox();f.append([[len(images),bb[0],bb[1]]]);images.append(tile.crop(bb))
        frames.append(f)
        candidate=j.get('review_id',f'WALK07-M{len(options)+1:02}').split('-')[-1]
        support_label=j.get('support_label') or (f'{j["locomotor_legs"]} pattes' if j['locomotor_legs'] else '')
        options.append({'key':j['id'],'label':f'{candidate} · {j["label"]}'+(f' · {support_label}' if support_label else ''),
            'foot':foot,'shadow':j.get('shadow',j['id']!='MON-124-0'),'cycleBounds':cycle_bounds,
            'verticalOffset':vertical_offset,'placementMode':'center_short_creatures_native64'})
        board=Image.new('RGB',(896,288),'#293132');draw=ImageDraw.Draw(board)
        for side in range(2):
            draw.text((6,side*144+3),j['id']+(' / gauche' if side==0 else ' / droite'),font=font,fill='white')
            for col in range(7):
                native=tiles[side*7+col];board.paste(native,(col*128,side*144+16),native)
        board.save(ART/f'{j["id"]}-native-poses.png')
        audit.append({'id':j['id'],'source_poses':len(figures),'used_poses':14,'legs':j['locomotor_legs'],
            'direction_halves_reordered':reordered,'warnings':warnings,'scale':scale,
            'original_native_height':target_h,'packed_max_height':max(round(im.height*scale) for im in selected),
            'packed_height_range':[min(round(im.height*scale) for im in selected),max(round(im.height*scale) for im in selected)],
            'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
            'source_opaque_outer_edge_counts':opaque_outer_edges,
            'current_native_reference':original_path.relative_to(ROOT).as_posix(),
            'reference_native_sha256':hashlib.sha256(original_path.read_bytes()).hexdigest(),
            'uniform_scaling_only':True,'review_origin':foot,'review_width_limit':width_limit,'cycle_bounds':cycle_bounds,
            'cycle_height':cycle_height,'vertical_offset':vertical_offset})
    (DOC/f'creature-packing{suffix}.json').write_text(json.dumps(audit,indent=2)+'\n','utf-8')
    # Keep the full native reference RGBA, including hidden transparent RGB.
    # Lossless WebP leaves more room for large-creature lots in the inline file.
    buffer=io.BytesIO();reference_atlas.save(buffer,format='WEBP',lossless=True,method=6,exact=True)
    assert Image.open(io.BytesIO(buffer.getvalue())).convert('RGBA').tobytes()==reference_atlas.tobytes()
    lot_name={1:'Premier',2:'Deuxième',3:'Troisième',4:'Quatrième',5:'Cinquième',6:'Sixième',7:'Septième',8:'Huitième',9:'Neuvième',10:'Dixième',11:'Onzième'}.get(lot,f'Lot{lot:02}')
    title='Placement des monstres · selon leur taille' if placement_review else f'{lot_name} lot · {len(jobs)} créatures'
    if lot>=12:title=f'Créatures · groupe {lot:02} · {len(jobs)} silhouettes'
    focus=[j for j in jobs if j.get('preview_focus_revision')]
    assert len(focus)<=1,'Choose a single creature to focus in this review'
    extra={'placementComparison':True,'sceneLabel':'Après · centré si hauteur <64px'} if placement_review else {}
    revision=f'creatures-lot{lot:02}-placement01'
    if focus:
        extra['defaultKey']=focus[0]['id']
        revision+='-'+focus[0]['preview_focus_revision']
    return save('creatures'+suffix,title,images,options,['Morphologies'],[frames],
        reference='data:image/webp;base64,'+base64.b64encode(buffer.getvalue()).decode(),
        revision=revision,reference_label='Dessin actuel · taille native',reference_rects=reference_rects,fragment_slug='creatures',extra_data=extra or None)

if __name__=='__main__':
    reports=[]
    lot_args=[arg for arg in sys.argv if arg.startswith('--creatures-lot')]
    creature_lot=int(lot_args[0][len('--creatures-lot'):]) if lot_args else (1 if '--creatures' in sys.argv else 0)
    if creature_lot:reports=[creatures(creature_lot,'--placement-review' in sys.argv)]
    else:reports=equipment()
    report_path=DOC/(('creature-preview-pack.json' if creature_lot==1 else f'creature-preview-pack-lot{creature_lot:02}.json') if creature_lot else 'equipment-preview-pack.json')
    if any(arg.startswith('--only=') for arg in sys.argv) and report_path.exists():
        by_slug={r['slug']:r for r in json.loads(report_path.read_text('utf8'))}
        by_slug.update({r['slug']:r for r in reports})
        report_path.write_text(json.dumps(list(by_slug.values()),ensure_ascii=False,indent=2)+'\n','utf-8')
    else:report_path.write_text(json.dumps(reports,ensure_ascii=False,indent=2)+'\n','utf-8')
    print(json.dumps(reports,ensure_ascii=True,indent=2))
