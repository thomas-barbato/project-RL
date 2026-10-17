"""Check durable campaign assets and every opaque native catalogue pixel."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
BANK=ROOT/'assets/sprites/campaign-monsters'
DURABLE=ROOT/'docs/reviews/refonte-histoire-2026-10-16/bestiary-integration-06'

def read(path): return json.loads(path.read_text(encoding='utf-8'))
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def main(after_cleanup=False):
    listing=read(REVIEW/'monster-candidate-list.json')
    manifest=read(BANK/'manifest.json')
    records={r['number']:r for r in listing['records']}
    refused={13,23,27,91,163}
    assert len(records)==131 and not refused.intersection(records)
    assert {e['number'] for e in manifest['entries']}==set(records)-{18,19}
    assert len(manifest['entries'])==240
    assert (ROOT/manifest['authorization']).is_file()
    for entry in manifest['entries']:
        path=BANK/entry['tile']
        assert sha(path)==entry['sha256']
        assert sha(ROOT/entry['source'])==entry['source_sha256']
        assert entry['cell']==64
        assert Image.open(path).size==(entry['side'],entry['side'])
    native_count=0
    for number,record in records.items():
        assert sha(ROOT/record['source'])==record['source_sha256']
        assert record['runtime_authorized']==(number not in {18,19})
        if record['section']=='Cultistes humains': assert record['label']=='Cultiste'
        if record['section'] in {'Possédés et hybrides','Cultistes possédés'}: assert record['label']=='Possédé'
        for variant in record['variants']:
            assert variant['file'].startswith('assets/')
            assert sha(ROOT/variant['file'])==variant['sha256']
            image=Image.open(ROOT/variant['file']).convert('RGBA')
            assert image.getbbox()==tuple(variant['occupied'])
            assert set(image.getchannel('A').get_flattened_data())<={0,255}
            native_count+=1
        if number in {144,150}:
            assert len(record['variants'])==4
            base=Image.open(ROOT/record['variants'][0]['file']).convert('RGBA')
            for variant in record['variants'][2:]:
                image=Image.open(ROOT/variant['file']).convert('RGBA')
                assert image.getchannel('A').tobytes()==base.getchannel('A').tobytes()
    layout=read(REVIEW/listing['catalogue_layout'])
    assert layout['count']==131 and layout['actor_resize_operations']==0 and layout['scale']==1
    sheet=Image.open(ROOT/layout['image']).convert('RGB')
    assert sha(ROOT/layout['image'])==layout['image_sha256']
    expected={(n,v['variant']) for n,r in records.items() for v in r['variants']}
    assert {(p['number'],p['variant']) for p in layout['placements']}==expected
    opaque=0
    for placement in layout['placements']:
        source=Image.open(ROOT/placement['file']).convert('RGBA').crop(placement['source_crop'])
        x,y=placement['position']
        actual=sheet.crop((x,y,x+source.width,y+source.height))
        for src,dst in zip(source.get_flattened_data(),actual.get_flattened_data()):
            if src[3]==255:
                assert src[:3]==dst, placement
                opaque+=1
    if after_cleanup:
        plan=read(DURABLE/'cleanup-plan.json')
        for moved in plan['native_export_relocations']:
            assert sha(ROOT/moved['durable'])==moved['sha256']
        for removed in plan['targets']:
            assert not (ROOT/removed['path']).exists(),removed['path']
        assert (ROOT/'artifacts/map-editor').is_dir()
    result=dict(catalogue_models=131,packaged_models=129,packaged_native_variants=240,
        additional_material_palettes=sum(bool(v.get('runtime_palette')) for r in records.values() for v in r['variants']),catalogue_variants=native_count,verified_opaque_catalogue_pixels=opaque,
        native_scale=1,removed_numbers=sorted(refused),deferred_bosses=[18,19],cleanup_checked=after_cleanup)
    DURABLE.mkdir(parents=True,exist_ok=True)
    (DURABLE/'asset-validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(result))

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--after-cleanup',action='store_true')
    main(parser.parse_args().after_cleanup)
