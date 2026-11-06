"""Compose isolated interior/roof candidates over the frozen approved lot01."""
from pathlib import Path
import json
from PIL import Image, ImageChops, ImageDraw
from build_city_cyberpunk12 import normalized, polygon_mask, sha, read

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/city-cyberpunk12/lot02'
DOC=ROOT/'docs/reviews/city-cyberpunk12'


def save_part(layer, ident, native, records):
    path=ART/(ident+'-native-r01.png')
    layer.save(path)
    native.alpha_composite(layer)
    records.append({'id':ident,'file':path.relative_to(ROOT).as_posix(),'sha256':sha(path),'bbox':layer.getchannel('A').getbbox()})


def masked(study, mask):
    layer=study.copy()
    layer.putalpha(ImageChops.multiply(study.getchannel('A'),mask))
    return layer


def main():
    m=read(ART/'manifest.json')
    assert not m.get('presented_for_user_review'), 'Preserve inspected revision; edit only a numbered part in a sibling revision.'
    for item in [*m['items'],*m.get('roofs',[])]:
        for part in item.get('parts',[]):
            assert sha(ROOT/part['file'])==part['sha256'], ('Preserve manual layer edit',part['id'])
    recipe=read(ART/'recipe.json')
    assert sha(ROOT/m['base'])==m['base_sha256']
    approved=read(DOC/'LOT01-ACCEPTANCE-r01.json')
    for p,digest in approved['source_and_layers'].items():
        assert sha(ROOT/p)==digest,p
    base=Image.open(ROOT/m['base']).convert('RGBA')
    security=read(ROOT/m['security_revision']) if m.get('security_revision') else None
    candidate=Image.open(ROOT/security['candidate']).convert('RGBA') if security else base.copy()
    authorized=Image.open(ROOT/security['authorized_mask']).convert('L') if security else Image.new('L',base.size)
    for number,item in enumerate(m['items']):
        assert sha(ROOT/item['original'])==item['original_sha256']
        size=tuple(item['native_size'])
        study,scale=normalized(ROOT/item['source'],size)
        combined=Image.new('RGBA',size)
        records=[]
        if number<2:
            spec=recipe['interiors'][number]
            boundary=Image.new('L',size)
            ImageDraw.Draw(boundary).polygon([tuple(v) for v in spec['boundary']],fill=255)
            remaining=boundary.copy()
            for code,box in spec['fixtures']:
                mask=Image.new('L',size)
                ImageDraw.Draw(mask).rectangle(box,fill=255)
                mask=ImageChops.multiply(mask,remaining)
                save_part(masked(study,mask),item['id']+'-'+code,combined,records)
                remaining=ImageChops.subtract(remaining,mask)
            save_part(masked(study,remaining),item['id']+'-D01-sol',combined,records)
            if number==0:
                # The generated front coping sits a few pixels above the real one.
                # Extend generated floor MATERIAL over the old floor only; preserve
                # the original masonry and jamb pixels at their exact coordinates.
                old=Image.open(ROOT/item['original']).convert('RGBA')
                extension=Image.new('L',size)
                zones=[(51,258,228,267),(315,258,477,267),(234,267,311,293)]
                pixels=old.load()
                for left,top,right,bottom in zones:
                    for y in range(top,bottom+1):
                        for x in range(left,right+1):
                            r,g,b,a=pixels[x,y]
                            if 70<r<180 and 5<=r-g<=26 and 5<=g-b<=30:
                                extension.putpixel((x,y),255)
                tile=study.crop((277,205,350,250))
                material=Image.new('RGBA',size)
                for y in range(0,size[1],tile.height):
                    for x in range(0,size[0],tile.width):
                        patch=tile if (x//tile.width)%2==0 else tile.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
                        material.paste(patch,(x,y))
                save_part(masked(material,extension),item['id']+'-D08-raccord-sol',combined,records)
                item['floor_extension_material']={'source_native_rect':[277,205,350,250],'target_zones':zones,'condition':'Copy generated floor material on original floor pigment only; original walls/jambs retained.'}
            item['boundary']=spec['boundary']
        else:
            spec=recipe['warehouse']
            original=Image.open(ROOT/item['original']).convert('RGBA')
            for code,points in spec['parts']:
                replacement=spec.get('replacements',{}).get(code)
                if replacement:
                    source=Image.open(ROOT/replacement['file']).convert('RGBA')
                    source=source.crop(source.getchannel('A').getbbox())
                    factor=replacement['width']/source.width
                    source=source.resize((replacement['width'],round(source.height*factor)),Image.Resampling.LANCZOS)
                    layer=Image.new('RGBA',size)
                    layer.alpha_composite(source,tuple(replacement['at']))
                else:
                    layer=masked(study,polygon_mask(size,[points]))
                if not replacement and code in spec['shifts']:
                    shifted=Image.new('RGBA',size)
                    shifted.alpha_composite(layer,tuple(spec['shifts'][code]))
                    layer=shifted
                layer.putalpha(ImageChops.multiply(layer.getchannel('A'),original.getchannel('A')))
                save_part(layer,item['id']+'-'+code,combined,records)
                if replacement:
                    records[-1]['label']=replacement['label']
                    records[-1]['source']=replacement['file']
        path=ART/(item['id']+'-native-r01.png')
        combined.save(path)
        item.update(parts=records,native=path.relative_to(ROOT).as_posix(),native_sha256=sha(path),source_sha256=sha(ROOT/item['source']),source_uniform_scale=scale,status='artwork-awaiting-user-prevalidation')
        at=tuple(item['at_px'])
        candidate.alpha_composite(combined,at)
        authorized.paste(combined.getchannel('A'),at)
    roof_records=[]
    for roof in recipe['roofs']:
        original=Image.open(ROOT/roof['original']).convert('RGBA')
        changed=original.copy()
        records=[]
        for spec in roof['parts']:
            source=Image.open(ROOT/spec['file']).convert('RGBA')
            source=source.crop(source.getchannel('A').getbbox())
            factor=spec['width']/source.width
            source=source.resize((spec['width'],round(source.height*factor)),Image.Resampling.LANCZOS)
            layer=Image.new('RGBA',original.size)
            layer.alpha_composite(source,tuple(spec['at']))
            layer.putalpha(ImageChops.multiply(layer.getchannel('A'),original.getchannel('A')))
            save_part(layer,roof['id']+'-'+spec['id'],changed,records)
            records[-1]['source']=spec['file']
        path=ART/(roof['id']+'-native-r01.png')
        # Retain the original roof's exact coverage, including damaged edges.
        changed.putalpha(original.getchannel('A'))
        changed.save(path)
        roof_records.append({**roof,'parts':records,'candidate':path.relative_to(ROOT).as_posix(),'original_sha256':sha(ROOT/roof['original']),'candidate_sha256':sha(path)})
    assert candidate.getchannel('A').tobytes()==base.getchannel('A').tobytes()
    difference=ImageChops.difference(candidate,base).convert('RGB')
    difference.paste((0,0,0),mask=authorized.point(lambda v:255 if v else 0))
    assert difference.getbbox() is None
    candidate_path=ART/'street-candidate-r01.png'
    candidate.save(candidate_path)
    authorized.save(ART/'authorized-mask-r01.png')
    m.pop('accepted_lot01_preserved',None)
    m.update(candidate=candidate_path.relative_to(ROOT).as_posix(),candidate_sha256=sha(candidate_path),roofs=roof_records,campaign_integrated=False,accepted_lot01_source_layers_preserved=True,accepted_lot01_other_parts_unchanged=True,outside_authorized_pixels_unchanged=True)
    (ART/'manifest.json').write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    views=[
        {'id':'lot02-secteur','name':'Quartier intermédiaire','rect':[0,1024,1664,1088],'actor':[18,31],'hide_roofs':[]},
        {'id':'lot02-logement-exterieur','name':'Logement · toit visible','rect':[384,1088,704,512],'actor':[11,23],'hide_roofs':[]},
        {'id':'lot02-logement-interieur','name':'Logement · intérieur','rect':[384,1088,704,512],'actor':[12,21],'hide_roofs':[2]},
        {'id':'lot02-boutique-exterieur','name':'Boutique · toit visible','rect':[384,1584,704,512],'actor':[11,31],'hide_roofs':[]},
        {'id':'lot02-boutique-interieur','name':'Boutique · intérieur','rect':[384,1584,704,512],'actor':[11,29],'hide_roofs':[3]},
        {'id':'lot02-entrepot','name':'Entrepôt en L','rect':[96,192,832,640],'actor':[10,8],'hide_roofs':[]},
        {'id':'lot02-garage-corrige','name':'Garage · correction sécurité','rect':[64,2304,640,640],'actor':[7,43],'hide_roofs':[],'item':'V1-B01-west-garage'},
        {'id':'lot02-bleu-corrige','name':'Bâtiment bleu · correction sécurité','rect':[1056,2112,608,704],'actor':[19,42],'hide_roofs':[],'item':'V1-B02-east-blue'},
        {'id':'lot02-annexe-corrige','name':'Annexe · correction sécurité','rect':[1056,2624,608,704],'actor':[19,48],'hide_roofs':[],'item':'V1-B03-east-south'},
    ]
    for view in views:
        view['item']=view.get('item') or ('V1-I01-logement' if 'logement-interieur' in view['id'] else 'V1-R01-logement' if 'logement-exterieur' in view['id'] else 'V1-I02-boutique' if 'boutique-interieur' in view['id'] else 'V1-R02-boutique' if 'boutique-exterieur' in view['id'] else 'V1-B04-entrepot' if 'entrepot' in view['id'] else '')
    current={'review':m['review'],'status':m['status'],'manifest':(ART/'manifest.json').relative_to(ROOT).as_posix(),'title':'Première ville · lot 02','summary':'Logement, boutique et entrepôt en L : intérieurs, équipements et toitures.','default_view':'lot02-garage-corrige','before':m['base'],'after':m['candidate'],'roof_overrides':{str(r['index']):r['candidate'] for r in roof_records},'views':views}
    (ART.parent/'review-current.json').write_text(json.dumps(current,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'new_parts':sum(len(i['parts']) for i in m['items'])+sum(len(r['parts']) for r in roof_records),'views':len(views),'approved_lot01_sources_preserved':True,'security_correction_only_on_requested_parts':bool(security)}))


if __name__=='__main__':
    main()
