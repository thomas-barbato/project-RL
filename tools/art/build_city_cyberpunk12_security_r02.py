"""Revise security equipment only; keep the approved lot01 source layers frozen."""
from pathlib import Path
import copy
import json
from PIL import Image, ImageChops
from build_city_cyberpunk12 import read, sha

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/city-cyberpunk12'
OUT=ART/'security-r02'


def fit(path, width, height=None):
    image=Image.open(path).convert('RGBA')
    box=image.getchannel('A').point(lambda v:255 if v>8 else 0).getbbox()
    image=image.crop(box)
    scale=width/image.width
    if height is not None:
        scale=min(scale,height/image.height)
    return image.resize((round(image.width*scale),round(image.height*scale)),Image.Resampling.LANCZOS)


def main():
    approved=read(ROOT/'docs/reviews/city-cyberpunk12/LOT01-ACCEPTANCE-r01.json')
    for p,digest in approved['source_and_layers'].items():
        assert sha(ROOT/p)==digest,p
    original=read(ART/'manifest.json')
    path=OUT/'manifest.json'
    if path.exists():
        assert not read(path).get('presented_for_user_review'),'Preserve inspected revision'
    hvac=fit(OUT/'HVAC-source-r02.png',100,64)
    hvac.save(OUT/'HVAC-native-r02.png')
    aerial=fit(OUT/'ANTENNE-source-r02.png',60,75)
    aerial.save(OUT/'ANTENNE-native-r02.png')
    candidate=Image.open(ROOT/'assets/maps/prison-street-expanded-01/street.png').convert('RGBA')
    authorized=Image.new('L',candidate.size)
    revised=copy.deepcopy(original)
    removed=[]
    for number,item in enumerate(revised['items']):
        parts=[]
        combined=Image.new('RGBA',tuple(item['native_size']))
        at=tuple(item['at_px'])
        for part in item['parts']:
            old=Image.open(ROOT/part['file']).convert('RGBA')
            remove='-acces' in part['id'] or (number==1 and '-D02-' in part['id'])
            replace=number in (0,1) and '-D01-' in part['id']
            if remove or replace:
                mask=Image.new('L',candidate.size);mask.paste(old.getchannel('A'),at)
                authorized=ImageChops.lighter(authorized,mask)
            if remove:
                removed.append(part['id'])
                continue
            if replace:
                layer=Image.new('RGBA',old.size)
                if number==0:
                    device=fit(OUT/'HVAC-native-r02.png',82,51)
                    layer.alpha_composite(device,(134,101))
                    part['label']='D01 · Refroidissement du garage'
                else:
                    layer.alpha_composite(aerial,(130,193))
                    part['label']='D01 · Antennes de communication passives'
                pp=OUT/(part['id']+'-native-r02.png')
                layer.save(pp)
                part.update(file=pp.relative_to(ROOT).as_posix(),sha256=sha(pp),native_bbox=layer.getchannel('A').getbbox(),revision='r02-security-scope')
                name='HVAC' if number==0 else 'ANTENNE'
                part.update(source=(OUT/(name+'-source-r02.png')).relative_to(ROOT).as_posix(),source_sha256=sha(OUT/(name+'-source-r02.png')),prompt='docs/reviews/city-cyberpunk12/'+name+'-SECURITE-r02-PROMPT.txt')
                mask=Image.new('L',candidate.size);mask.paste(layer.getchannel('A'),at)
                authorized=ImageChops.lighter(authorized,mask)
            else:
                layer=old
            combined.alpha_composite(layer)
            parts.append(part)
        combined_path=OUT/(item['id']+'-native-r02.png')
        combined.save(combined_path)
        item.update(parts=parts,native=combined_path.relative_to(ROOT).as_posix(),native_sha256=sha(combined_path),status='security-correction-awaiting-user-review')
        candidate.alpha_composite(combined,at)
    source=Image.open(ROOT/original['candidate']).convert('RGBA')
    difference=ImageChops.difference(candidate,source).convert('RGB')
    difference.paste((0,0,0),mask=authorized.point(lambda v:255 if v else 0))
    assert difference.getbbox() is None
    target=OUT/'street-candidate-r02.png';candidate.save(target)
    mask_path=OUT/'authorized-mask-r02.png';authorized.save(mask_path)
    revised.update(review='CITY-CYBERPUNK12-SECURITY-r02',status='artwork-awaiting-user-prevalidation',accepted_original='docs/reviews/city-cyberpunk12/LOT01-ACCEPTANCE-r01.json',candidate=target.relative_to(ROOT).as_posix(),candidate_sha256=sha(target),authorized_mask=mask_path.relative_to(ROOT).as_posix(),removed_parts=removed,visible_scanner_controllers=False,ordinary_building_scanners=False,other_approved_parts_unchanged=True,presented_for_user_review=False)
    path.write_text(json.dumps(revised,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    lotpath=ART/'lot02/manifest.json';m=read(lotpath)
    m['security_revision']=path.relative_to(ROOT).as_posix()
    lotpath.write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    rp=ART/'lot02/recipe.json';recipe=read(rp)
    for roof in recipe['roofs']:
        roof['parts'][0]['file']=(OUT/'HVAC-native-r02.png').relative_to(ROOT).as_posix()
    recipe['warehouse']['replacements']={'D01-controleur':{'file':(OUT/'HVAC-native-r02.png').relative_to(ROOT).as_posix(),'at':[45,175],'width':88,'label':'D01 · Refroidissement de l’entrepôt'}}
    rp.write_text(json.dumps(recipe,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'removed_scanner_layers':len(removed),'unchanged_approved_sources':len(approved['source_and_layers']),'scope':'Security only; no layout, openings, interiors or unrelated device edits.'}))


if __name__=='__main__':
    main()
