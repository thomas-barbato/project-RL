"""Author explicit per-model material palettes, masks and native review exports.

Authoring selectors propose material memberships separately for each source.
The durable JSON freezes the exact RGB membership and overrides after review;
the game never uses hue, saturation, rectangles or authoring selectors.
"""
import argparse
import colorsys
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[2]
BANK=ROOT/'assets/sprites/campaign-monsters'
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07'
CATALOGUE=ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
NATIVE=ROOT/'assets/source-art/refonte-histoire-2026-10-16/monster-material-palettes-07'
SECTIONS={'Démons et créations','Vers et larves démoniaques','Démons ailés','Minotaures ailés',
          'Démons de peste','Démons du froid','Démons humanoïdes','Élites démoniaques','Nécromanciens démoniaques'}

def read(path): return json.loads(path.read_text(encoding='utf-8'))
def write(path,value):
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def packed(rgb): return (rgb[0]<<16)|(rgb[1]<<8)|rgb[2]

# These are individual authoring selections, inspected against the native boards.
# All non-selected colors belong to protected materials and are copied verbatim.
GROUPS=[
 ('chair',[9,20,29,33,35,37,64,67,73,76,77,80,84,92,93,102,103,104,105,106,108,109,111,113,138,140,141,143,151,160,162],(16,70,.08,1,.12,.91)),
 ('chair rouge',[34,39,60,66,79,82,86,88,148,161],(0,38,.15,1,.12,.86)),
 ('carapace',[21,30,61,63,68,69,70,71,75,81,83,85,90,107,137,146],(0,95,.12,1,.10,.83)),
 ('peau olivâtre',[28,65,87,152,153],(40,100,.10,1,.11,.85)),
 ('peau bleue',[36,38,78,89,145,156,157,158,159],(175,265,.065,1,.105,.795)),
 ('carapace turquoise',[31],(150,225,.10,1,.105,.85)),
 ('robe',[100,136],(0,360,0,.25,.10,.38)),
 ('robe claire',[112],(20,70,.08,1,.14,.91)),
 ('chitine claire',[8,115],(16,75,.08,1,.14,.9)),
 ('membranes',[147],(265,330,.10,1,.10,.9)),
 ('robe turquoise',[139],(175,200,.18,1,.07,.7)),
 ('chair cendrée',[149],(15,45,.03,.3,.27,.8)),
 ('excroissances',[154,155],(45,105,.08,1,.12,.8)),
 ('chitine pourpre',[142],(315,380,.10,1,.12,.85)),
 ('chair sous capuche',[32],(0,65,.13,1,.15,.86)),
 ('muscles écorchés',[62],(345,375,.20,1,.11,.65)),
 ('chair et glandes',[72],(18,65,.08,1,.13,.86)),
 ('chair grise',[74],(0,65,.07,.5,.15,.75)),
 ('chair sous cage',[101],(20,45,.08,.4,.25,.83)),
]

def source_colors(number, image):
    matches=[(material,band) for material,numbers,band in GROUPS if number in numbers]
    assert len(matches)==1,(number,matches)
    material,(h0,h1,s0,s1,l0,l1)=matches[0]
    overrides={
        63:('carapace',(0,29,.15,1,.10,.78)),
        81:('carapace',(35,85,.10,1,.10,.72)),
        85:('kyste dorsal',(0,20,.18,1,.10,.72)),
        82:('chair rouge',(0,22,.15,1,.12,.75)),
        88:('chair et membranes',(0,22,.15,1,.12,.75)),
    }
    if number in overrides:
        material,(h0,h1,s0,s1,l0,l1)=overrides[number]
    colors=[]
    for rgb in sorted({p[:3] for p in image.get_flattened_data() if p[3]}):
        h,l,s=colorsys.rgb_to_hls(*(c/255 for c in rgb)); h*=360
        if h0>300 and h<30: h+=360
        if h0<=h<=h1 and s0<=s<=s1 and l0<=l<=l1:
            colors.append(list(rgb))
    assert colors,(number,'empty material selection')
    return material,colors

def target(rgb,kind,material,number):
    _,light,_=colorsys.rgb_to_hls(*(c/255 for c in rgb))
    cloth='robe' in material
    hue=(13+(number%4)*2) if kind=='orange' else (94+(number%5)*4)
    saturation=(.34 if cloth else .57) if kind=='orange' else (.25 if cloth else .39)
    proposed=colorsys.hls_to_rgb(hue/360,light,saturation)
    old_luma=sum(c*w for c,w in zip(rgb,(.2126,.7152,.0722)))
    new_luma=sum(c*255*w for c,w in zip(proposed,(.2126,.7152,.0722)))
    retain=min(1,old_luma/max(1,new_luma))
    return [round(c*255*retain) for c in proposed]

def author():
    if (BANK/'material-palettes.json').exists() and read(BANK/'material-palettes.json').get('extended_runtime_approved'):
        raise ValueError('Accepted palettes are frozen. Prepare a distinct review for any revision.')
    listing=read(CATALOGUE/'monster-candidate-list.json')
    manifest=read(BANK/'manifest.json')
    entries=[]
    for record in listing['records']:
        n=record['number']
        if record['section'] not in SECTIONS: continue
        base=next(e for e in manifest['entries'] if e['number']==n and e['variant']==0)
        image=Image.open(BANK/base['tile']).convert('RGBA')
        if n in {144,150}:
            # Freeze the four already accepted exact mappings, without restyling.
            variants=[]; selected=set()
            for v in (2,3):
                other=Image.open(BANK/f'tiles/{n}-palette-{v}.png').convert('RGBA')
                mapping={a[:3]:b[:3] for a,b in zip(image.get_flattened_data(),other.get_flattened_data()) if a[3] and a[:3]!=b[:3]}
                selected.update(mapping)
                variants.append(dict(variant=v,label='Rouge-orangé' if v==2 else 'Vert' if n==144 else 'Bleu',
                    colors=[dict(source=list(a),target=list(b)) for a,b in sorted(mapping.items())],visual_approved=True))
            material='chair et membranes'; colors=[list(c) for c in sorted(selected)]
        else:
            material,colors=source_colors(n,image)
            variants=[dict(variant=v,label=label,visual_approved=False,
                colors=[dict(source=c,target=target(c,kind,material,n)) for c in colors])
                for v,label,kind in [(2,'Rouge-orangé','orange'),(3,'Vert','green')]]
        entries.append(dict(number=n,code=record['code'],label=record['label'],section=record['section'],
            source=base['tile'],source_sha256=base['sha256'],side=base['side'],atlas_index=base['atlas_index'],
            foot_anchor=base['foot_anchor'],material=material,selected_rgb=colors,
            protected='All other RGB values, outlines, alpha, canvas, anchors and gameplay are unchanged.',variants=variants))
        protected_regions={
            101:[[[59,42],[68,42],[79,48],[88,60],[93,73],[87,81],[77,85],[55,83],[42,78],[39,68],[43,57],[50,49]]],
            138:[[[33,32],[39,32],[42,35],[41,38],[44,40],[41,45],[33,43],[29,41],[27,38],[29,35]]],
        }
        if n in protected_regions:
            entries[-1]['protected_regions']=protected_regions[n]
            entries[-1]['protected_region_notes']='Traced cage shell for 101; held skull for 138. Shared flesh RGBs do not recolor these materials.'
    assert len(entries)==90
    write(BANK/'material-palettes.json',dict(review_id='MONSTER-MATERIAL-PALETTES-07',
        date='2026-10-17',extended_runtime_approved=False,entries=entries))

def build(check=False):
    spec=read(BANK/'material-palettes.json'); manifest=read(BANK/'manifest.json')
    previous=read(REVIEW/'review.json') if (REVIEW/'review.json').exists() else {}
    rows=[]; codes=[]; masks={}
    for side in {e['side'] for e in manifest['entries']}:
        masks[side]=Image.new('RGBA',Image.open(BANK/f'living-{side}.png').size)
    for entry in spec['entries']:
        n,side,index=entry['number'],entry['side'],entry['atlas_index']
        frozen=next(e for e in manifest['entries'] if e['number']==n and e['variant']==0)
        assert (side,index,entry['source'],entry['foot_anchor'])==(frozen['side'],frozen['atlas_index'],frozen['tile'],frozen['foot_anchor'])
        path=BANK/entry['source']; assert sha(path)==entry['source_sha256']
        source=Image.open(path).convert('RGBA')
        selected={tuple(c) for c in entry['selected_rgb']}
        mask=Image.new('RGBA',source.size)
        mask.putdata([(255,0,0,255) if p[3] and p[:3] in selected else (0,0,0,0) for p in source.get_flattened_data()])
        # Explicit hand-traced material regions can disambiguate shared RGBs.
        if entry.get('primary_regions'):
            region=Image.new('L',source.size);draw=ImageDraw.Draw(region)
            for polygon in entry['primary_regions']: draw.polygon([tuple(p) for p in polygon],fill=255)
            mask.putdata([p if allowed else (0,0,0,0) for p,allowed in zip(mask.get_flattened_data(),region.get_flattened_data())])
        if entry.get('protected_regions'):
            draw=ImageDraw.Draw(mask)
            for polygon in entry['protected_regions']: draw.polygon([tuple(p) for p in polygon],fill=(0,0,0,0))
        masks[side].alpha_composite(mask,((index%8)*side,(index//8)*side))
        record=dict(entry,variants=[])
        raw=list(source.get_flattened_data()); selection=list(mask.get_flattened_data())
        for variant in entry['variants']:
            changes={tuple(c['source']):tuple(c['target']) for c in variant['colors']}
            result=source.copy()
            pixels=[(*changes.get(p[:3],p[:3]),p[3]) if m[0] else p for p,m in zip(raw,selection)]
            result.putdata(pixels)
            assert result.getchannel('A').tobytes()==source.getchannel('A').tobytes()
            assert result.getbbox()==source.getbbox()
            changed=sum(a!=b for a,b in zip(raw,pixels)); assert changed>0,(n,variant['variant'])
            assert all(a==b for a,b,m in zip(raw,pixels,selection) if not m[0])
            if n in {144,150}:
                assert result.tobytes()==Image.open(BANK/f'tiles/{n}-palette-{variant["variant"]}.png').convert('RGBA').tobytes()
            target_path=NATIVE/'native64'/f'{n:03}-palette-{variant["variant"]}.png'
            target_path.parent.mkdir(parents=True,exist_ok=True)
            if check:
                assert Image.open(target_path).convert('RGBA').tobytes()==result.tobytes()
            else: result.save(target_path,optimize=True)
            info=dict(variant=chr(65+variant['variant']),variant_index=variant['variant'],label=variant['label'],
                file=target_path.relative_to(ROOT).as_posix(),sha256=sha(target_path),cell=64,canvas=side,
                foot_anchor=entry['foot_anchor'],occupied=list(source.getbbox()),changed_pixels=changed,
                colors=len({p[:3] for p in pixels if p[3]}),runtime_palette=True,
                visual_approved=variant['visual_approved'],palette_review_id=spec['review_id'])
            record['variants'].append(info)
            pairs=', '.join(f'(0x{packed(c["source"]):06x}, 0x{packed(c["target"]):06x})' for c in variant['colors'])
            codes.append(f'    MaterialPalette {{ number: {n}, variant: {variant["variant"]}, colors: &[{pairs}] }},')
        record['mask_selected_pixels']=sum(bool(p[0]) for p in selection)
        rows.append(record)
    generated='// Generated from explicit material-palettes.json; no runtime hue classifier.\n'
    generated+=f'pub const EXTENDED_PALETTES_APPROVED: bool = {str(spec["extended_runtime_approved"]).lower()};\n'
    generated+='pub const MATERIAL_PALETTES: &[MaterialPalette] = &[\n'+'\n'.join(codes)+'\n];\n'
    code_path=BANK/'material-palettes.rs'
    if check: assert code_path.read_text(encoding='utf-8')==generated
    else: code_path.write_text(generated,encoding='utf-8')
    for side,mask in masks.items():
        path=BANK/f'materials-{side}.png'
        if check: assert Image.open(path).convert('RGBA').tobytes()==mask.tobytes()
        else: mask.save(path,optimize=True)
    runtime_entries=[dict(number=r['number'],variant=v['variant_index'],
        preview=v['file'],preview_relative_to='workspace',source_variant=0,
        implementation='src/game_art_campaign.rs',material_map='material-palettes.json',
        material_mask=f'materials-{r["side"]}.png',
        authorization='docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07/review.json',
        available_since_generation=150 if r['number'] in {144,150} else 151)
        for r in rows for v in r['variants']]
    if check: assert manifest['runtime_material_palettes']==runtime_entries
    else:
        manifest['runtime_material_palettes']=runtime_entries
        write(BANK/'manifest.json',manifest)
    if not check:
        write(REVIEW/'review.json',dict(review_id=spec['review_id'],date=spec['date'],
            request='Étendre les variantes rouge-orangé et vertes aux autres démons ; la demande ne concernait pas seulement un démon et deux cultistes.',
            numbered_models=90,new_models_with_additional_palettes=88,new_palettes=176,
            status='accepted' if spec['extended_runtime_approved'] else 'native_review_pending',
            runtime_connected=spec['extended_runtime_approved'],original_geometry_alpha_and_anchors_preserved=True,
            records=rows,acceptance=previous.get('acceptance')))
    print(f'{len(rows)} demon models, {len(codes)} material palettes (176 new), exact geometry and protected material pixels verified.')

def publish_catalogue():
    listing=read(CATALOGUE/'monster-candidate-list.json');review=read(REVIEW/'review.json')
    additions={r['number']:r for r in review['records']}
    for record in listing['records']:
        if record['number'] not in additions: continue
        row=additions[record['number']]
        record['variants']=sorted([v for v in record['variants'] if v['variant'] not in {'C','D'}]+row['variants'],key=lambda v:v['variant'])
        record['variant_files']=[v['file'] for v in record['variants']]
        record['palette_review_id']=review['review_id']
        record['palette_review_status']=review['status'] if record['number'] not in {144,150} else 'accepted'
    listing['current_palette_review']='../monster-material-palettes-07/review.json'
    write(CATALOGUE/'monster-candidate-list.json',listing)

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--author',action='store_true');parser.add_argument('--check',action='store_true');parser.add_argument('--publish-catalogue',action='store_true')
    args=parser.parse_args()
    if args.author: author()
    build(args.check)
    if args.publish_catalogue: publish_catalogue()
