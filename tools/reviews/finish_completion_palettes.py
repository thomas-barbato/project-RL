"""Compile existing material palettes onto the newly authored master layers.

No new palette identity, animation or game rule. Compiled RGB results are frozen
by export hashes; the game reads precompiled PNGs, never authoring selectors.
"""
import colorsys
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from art_finish_completion import ROOT, ART, DOC, OUT, EXISTING, EXISTING_BATCH
from art_finish_batch_02 import sha, write

BANK=ROOT/'assets/sprites/campaign-monsters'
DEST=ROOT/'assets/sprites/art-finish-completion-06/monsters'
sys.path.insert(0,str(ROOT/'tools/art'))
from prepare_monster_material_palettes import source_colors,target
from prepare_monster_blue_palettes import target as blue_target

def read(path): return json.loads(path.read_text('utf-8'))

def nearest_tree(points,axis=0):
    if not points:return None
    points.sort(key=lambda p:p[0][axis]);i=len(points)//2
    return points[i],axis,nearest_tree(points[:i],(axis+1)%3),nearest_tree(points[i+1:],(axis+1)%3)

def nearest(tree,point,best=None):
    if tree is None:return best
    node,axis,left,right=tree
    dist=sum((a-b)**2 for a,b in zip(node[0],point))
    if best is None or dist<best[0]:best=(dist,node[1])
    delta=point[axis]-node[0][axis]
    first,second=(left,right) if delta<0 else (right,left)
    best=nearest(first,point,best)
    if delta*delta<best[0]:best=nearest(second,point,best)
    return best

def coordinate(rgb):return (rgb[0]*.8,rgb[1],rgb[2]*.8)

def existing_base(number):
    code=EXISTING[number];batch=EXISTING_BATCH[number]
    return ROOT/f'assets/sprites/art-finish-{batch:02}/{code}.png'

def transform(rgb,old,new):
    if old==new:return rgb
    h,l,s=colorsys.rgb_to_hls(*(c/255 for c in rgb))
    oh,ol,os=colorsys.rgb_to_hls(*(c/255 for c in old))
    nh,nl,ns=colorsys.rgb_to_hls(*(c/255 for c in new))
    # Match the accepted material hue while preserving the painted relief.
    delta=(h-oh+.5)%1-.5
    h=(nh+delta*.2)%1
    s=min(.88,max(0,s*ns/max(.04,os)))
    l=min(.95,max(.025,l+(nl-ol)*.5))
    return tuple(round(c*255) for c in colorsys.hls_to_rgb(h,l,s))

def compile_palettes():
    assert not (DOC/'acceptance.json').exists(), 'Accepted palette exports are frozen'
    manifest=read(BANK/'manifest.json');entries={(e['number'],e['variant']):e for e in manifest['entries']}
    jobs={j['number']:j for j in read(ART/'prompts.json')['jobs']}
    material={e['number']:e for e in read(BANK/'material-palettes.json')['entries']}
    all_keys={(e['number'],e['variant']):e for k in ('entries','runtime_material_palettes','runtime_blue_material_palettes') for e in manifest[k]}
    DEST.mkdir(parents=True,exist_ok=True);(ART/'material-memberships').mkdir(exist_ok=True)
    rows=[];missing=[]
    for number in sorted({k[0] for k in all_keys}):
        base_file=existing_base(number) if number in EXISTING else ART/'native'/f'{number:03}-0.png'
        if not base_file.exists():missing.append(number);continue
        base=Image.open(base_file).convert('RGBA');old=Image.open(BANK/entries[number,0]['tile']).convert('RGBA')
        # Assign new painted RGB values to an explicit old material swatch.
        old_colors={p[:3] for p in old.get_flattened_data() if p[3]>128}
        tree=nearest_tree([(coordinate(c),c) for c in old_colors])
        membership={rgb:nearest(tree,coordinate(rgb))[1] for rgb in {p[:3] for p in base.get_flattened_data() if p[3]}}
        protected=Image.new('L',base.size)
        for poly in material.get(number,{}).get('protected_regions',[]):ImageDraw.Draw(protected).polygon([tuple(p) for p in poly],fill=255)
        for variant in sorted(k[1] for k in all_keys if k[0]==number):
            spec=all_keys[number,variant]
            old_target=Image.open(BANK/spec['tile'] if 'tile' in spec else ROOT/spec['preview']).convert('RGBA')
            assert old_target.getchannel('A').tobytes()==old.getchannel('A').tobytes(),(number,variant)
            pairs=defaultdict(Counter)
            for a,b in zip(old.get_flattened_data(),old_target.get_flattened_data()):
                if a[3]>128:pairs[a[:3]][b[:3]]+=1
            mapping={a:counts.most_common(1)[0][0] for a,counts in pairs.items()}
            colors={rgb:transform(rgb,swatch,mapping.get(swatch,swatch)) for rgb,swatch in membership.items()}
            if variant>=2 and number not in {144,145,150}:
                mat,selected=source_colors(number,base)
                bands={31:(135,245,.02,.055,.82),78:(175,275,.02,.05,.82),89:(175,275,.02,.05,.82),87:(22,105,.05,.09,.86),139:(135,225,.025,.045,.78)}
                if number in bands:
                    lo,hi,minimum,low,high=bands[number]
                    selected=[list(c) for c in membership if (lambda h,l,s:lo<=h*360<=hi and s>minimum and low<l<high)(*colorsys.rgb_to_hls(*(v/255 for v in c)))]
                selected={tuple(c) for c in selected}
                colors={c:tuple(blue_target(c,'robe' in mat,number) if variant==4 else target(c,'orange' if variant==2 else 'green',mat,number)) if c in selected else c for c in membership}
            pixels=[p if block or not p[3] else (*colors.get(p[:3],p[:3]),p[3]) for p,block in zip(base.get_flattened_data(),protected.get_flattened_data())]
            result=base.copy();result.putdata(pixels)
            # Keep previously accepted exact exports where they exist.
            alias={(144,1):'FIN-09-blue',(144,2):'FIN-09-orange',(144,3):'FIN-09-green',(145,2):'FIN-10-orange',(145,3):'FIN-10-green'}.get((number,variant))
            if alias:result=Image.open(ROOT/f'assets/sprites/art-finish-01/{alias}.png').convert('RGBA')
            assert result.getchannel('A').tobytes()==base.getchannel('A').tobytes(),(number,variant,'alpha')
            assert result.getbbox()==base.getbbox(),(number,variant,'bounds')
            dest=DEST/f'{number:03}-{variant}.png';result.save(dest,optimize=True)
            changed=sum(a!=b for a,b in zip(base.get_flattened_data(),result.get_flattened_data()))
            fallback=False
            if variant and not changed:
                # A worn source can move beyond its former swatches. Re-author
                # its material membership with the established per-model band.
                mat,selected=source_colors(number,base)
                selected={tuple(c) for c in selected}
                colors={c:tuple(blue_target(c,'robe' in mat,number) if variant==4 else target(c,'orange' if variant==2 else 'green',mat,number)) for c in selected}
                result=base.copy();result.putdata([p if block or p[:3] not in colors else (*colors[p[:3]],p[3]) for p,block in zip(base.get_flattened_data(),protected.get_flattened_data())])
                result.save(dest,optimize=True)
                changed=sum(a!=b for a,b in zip(base.get_flattened_data(),result.get_flattened_data()));fallback=True
            assert variant==0 or changed>0,(number,variant,'empty palette')
            rows.append(dict(number=number,variant=variant,code=EXISTING.get(number,jobs.get(number,{}).get('id')),side=base.width,foot_anchor=entries[number,0]['foot_anchor'],label=entries[number,0]['review_label'],role=entries[number,0]['role'],file=dest.relative_to(ROOT).as_posix(),sha256=sha(dest),bounds=list(result.getbbox()),changed_pixels=changed,alpha_identical_to_base=True,accepted_exact_alias=alias,material_band_fallback=fallback))
        write(ART/'material-memberships'/f'{number:03}.json',dict(number=number,algorithm='nearest approved source material swatch; HLS transfer retaining painted lightness',rgb=[dict(painted=list(a),approved_source=list(b)) for a,b in sorted(membership.items())]))
    write(DOC/'exports.json',dict(review='ART-FINISH-COMPLETE-06',native_cell=64,expected=len(all_keys),exports=rows,missing=missing,gameplay_files_modified=False))
    print(json.dumps(dict(exports=len(rows),expected=len(all_keys),missing=missing)))

def boards():
    rows=read(DOC/'exports.json')['exports'];models=sorted({r['number'] for r in rows})
    for start in range(0,len(models),12):
        subset=models[start:start+12];page=Image.new('RGB',(1920,80+math.ceil(len(subset)/2)*200),'#11191d');d=ImageDraw.Draw(page)
        font=lambda size:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)
        d.text((20,12),'Bestiaire terminé · palettes · taille native',font=font(24),fill='#decca7')
        d.text((20,49),'Même silhouette et mêmes points d’appui dans toutes les couleurs · grille de jeu 64 px',font=font(16),fill='#becac3')
        for i,number in enumerate(subset):
            rr=[r for r in rows if r['number']==number];x=(i%2)*960;y=80+(i//2)*200
            d.text((x+12,y+4),f'{rr[0]["code"]} · {number:03} · {rr[0]["label"]}',font=font(17),fill='#decca7')
            for col,r in enumerate(rr):
                xx=x+12+col*185;d.rectangle((xx,y+34,xx+178,y+189),fill='#343d3b')
                im=Image.open(ROOT/r['file']).convert('RGBA');foot=r['foot_anchor'];page.paste(im,(xx+89-foot[0],y+172-foot[1]),im)
                d.text((xx+7,y+37),chr(65+r['variant']),font=font(14),fill='#b9c8c0')
        page.save(OUT/f'completion-palettes-{start//12+1:02}.png')
    print('Palette boards:',math.ceil(len(models)/12))

def catalog():
    data=read(DOC/'exports.json');assert not data['missing']
    rows=data['exports'];assert len(rows)==data['expected']==471
    code='// Compiled visual-only exports; ART-FINISH-COMPLETE-06 delegated review.\n'
    code+='#[cfg(any(debug_assertions,test))]\npub(super) const COMPLETED_MONSTERS: &[(u16,u8,u16)] = &[\n'+''.join(f'    ({r["number"]}, {r["variant"]}, {r["side"]}),\n' for r in rows)+'];\n'
    code+='fn bytes_06(name: &str) -> Option<&\'static [u8]> {\n    Some(match name {\n'
    code+=''.join(f'        "MON-{r["number"]:03}-{r["variant"]}" => include_bytes!("../{r["file"]}"),\n' for r in rows)
    code+='        _ => return None,\n    })\n}\n'
    (ROOT/'src/game_art_finish_catalog_06.rs').write_text(code,encoding='utf-8')
    print('Embedded catalogue:',len(rows))

if __name__=='__main__':
    import sys
    if '--catalog' in sys.argv:catalog()
    else:compile_palettes();boards()
