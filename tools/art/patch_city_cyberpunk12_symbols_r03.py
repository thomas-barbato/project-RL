"""Apply four tiny display edits; preserve every other reviewed pixel/layer."""
from pathlib import Path
import copy
import json
from PIL import Image, ImageChops
from build_city_cyberpunk12 import normalized, read, sha

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/city-cyberpunk12'
OUT=ART/'symboles-r03'


def main():
    path=OUT/'manifest.json'
    assert not path.exists(),'Keep the inspected revision; change only the requested part in a sibling version.'
    previous=read(ART/'lot02/manifest.json')
    assert sha(ROOT/previous['candidate'])==previous['candidate_sha256']
    source=OUT/'V1-I01-sans-barres-source-r03.png'
    study,scale=normalized(source,(527,339))
    edits=[
        {'name':'évier','part':'D03-cuisine','rect':[237,106,248,113]},
        {'name':'réfrigérateur','part':'D03-cuisine','rect':[328,83,340,91]},
        {'name':'lit','part':'D05-lit','rect':[450,72,462,81]},
        {'name':'appareil sur la table','part':'D06-table','rect':[222,205,233,212]},
    ]
    m=copy.deepcopy(previous)
    before=Image.open(ROOT/previous['candidate']).convert('RGBA')
    candidate=before.copy()
    background=Image.open(ROOT/previous['base']).convert('RGBA')
    authorized=Image.new('L',before.size)
    item=m['items'][0]
    world=item['at_px']
    changed=[]
    for part in item['parts']:
        relevant=[e for e in edits if part['id'].endswith(e['part'])]
        if not relevant:
            continue
        assert sha(ROOT/part['file'])==part['sha256']
        old=Image.open(ROOT/part['file']).convert('RGBA')
        updated=old.copy()
        for edit in relevant:
            box=tuple(edit['rect'])
            assert old.getchannel('A').crop(box).getextrema()[0]>0
            updated.paste(study.crop(box),box)
        updated.putalpha(old.getchannel('A'))
        pp=OUT/(part['id']+'-native-r03.png')
        updated.save(pp)
        changed.append({'id':part['id'],'before_file':part['file'],'before_sha256':part['sha256'],'after_file':pp.relative_to(ROOT).as_posix(),'after_sha256':sha(pp),'rects':[e['rect'] for e in relevant]})
        part.update(file=pp.relative_to(ROOT).as_posix(),sha256=sha(pp),symbol_removal_source=source.relative_to(ROOT).as_posix())
        for edit in relevant:
            l,t,r,b=edit['rect'];box=(world[0]+l,world[1]+t,world[0]+r,world[1]+b)
            patch=background.crop(box)
            patch.alpha_composite(updated.crop((l,t,r,b)))
            candidate.paste(patch,box)
            authorized.paste(255,box)
    assert candidate.getchannel('A').tobytes()==before.getchannel('A').tobytes()
    outside=ImageChops.difference(before,candidate).convert('RGB')
    outside.paste((0,0,0),mask=authorized)
    assert outside.getbbox() is None
    # Source layers outside the three requested fixtures remain byte-identical.
    changed_ids={c['id'] for c in changed}
    kept={p['file']:p['sha256'] for i in [*m['items'],*m['roofs']] for p in i['parts'] if p['id'] not in changed_ids}
    for filename,digest in kept.items():
        assert sha(ROOT/filename)==digest,filename
    combined=Image.new('RGBA',tuple(item['native_size']))
    for part in item['parts']:
        combined.alpha_composite(Image.open(ROOT/part['file']).convert('RGBA'))
    combined_path=OUT/'V1-I01-logement-native-r03.png';combined.save(combined_path)
    item.update(native=combined_path.relative_to(ROOT).as_posix(),native_sha256=sha(combined_path))
    candidate_path=OUT/'street-candidate-r03.png';candidate.save(candidate_path)
    mask_path=OUT/'authorized-mask-r03.png';authorized.save(mask_path)
    m.update(review='CITY-CYBERPUNK12-LOT02-SYMBOLES-r03',parent_review='assets/source-art/city-cyberpunk12/lot02/manifest.json',parent_candidate=previous['candidate'],parent_candidate_sha256=previous['candidate_sha256'],candidate=candidate_path.relative_to(ROOT).as_posix(),candidate_sha256=sha(candidate_path),source=source.relative_to(ROOT).as_posix(),source_sha256=sha(source),source_uniform_scale=scale,prompt='docs/reviews/city-cyberpunk12/SYMBOLES-r03-PROMPT.txt',display_edits=edits,changed_parts=changed,unchanged_parts=kept,authorized_mask=mask_path.relative_to(ROOT).as_posix(),outside_display_pixels_unchanged=True,presented_for_user_review=True)
    path.write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    current_path=ART/'review-current.json';current=read(current_path)
    for view in current['views']:
        if view['id']=='lot02-logement-interieur':
            view['id']='symboles-r03-logement-interieur'
    current.update(review=m['review'],manifest=path.relative_to(ROOT).as_posix(),before=previous['candidate'],after=m['candidate'],before_roof_overrides=current['roof_overrides'],before_label='avant retrait des pictogrammes',after_label='pictogrammes retires',default_view='symboles-r03-logement-interieur',revision_note='Retouche ciblée du logement : pictogrammes à trois barres retirés ; autres dessins conservés.')
    current_path.write_text(json.dumps(current,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'changed_parts':len(changed),'display_fields':len(edits),'other_layers_unchanged':len(kept),'pixels_outside_fields_unchanged':True}))


if __name__=='__main__':
    main()
