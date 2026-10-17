"""Check reviewed material pixels, six native boards and real renderer exports."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image
from prepare_monster_material_palettes import build

ROOT=Path(__file__).resolve().parents[2]
BANK=ROOT/'assets/sprites/campaign-monsters'
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07'

def read(path): return json.loads(path.read_text(encoding='utf-8'))
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def main(runtime,blue=False):
    build(check=True)
    review=read(REVIEW/'review.json')
    assert review['status']=='accepted'
    assert review['acceptance']['answer']=='Valider et intégrer le lot'
    records={r['number']:r for r in review['records']}
    assert len(records)==90
    if blue:
        from prepare_monster_blue_palettes import build as build_blue, validate_preview
        blue_review=build_blue(check=True);validate_preview()
        assert blue_review['status']=='accepted' and blue_review['acceptance']['answer']=='Valider et intégrer les 51 bleues'
        for row in blue_review['records']:
            assert sha(ROOT/row['source'])==row['source_sha256']
            if row['existing_blue']: continue
            records[row['number']]['variants'].append(dict(file=row['blue_file'],variant='E',variant_index=4,foot_anchor=row['foot_anchor']))
    original_hashes=[]; exports=[]; changed=protected=0
    for record in records.values():
        source=Image.open(BANK/record['source']).convert('RGBA')
        assert sha(BANK/record['source'])==record['source_sha256']
        original_hashes.append(dict(number=record['number'],file=record['source'],sha256=record['source_sha256']))
        side=record['side'];index=record['atlas_index']
        masks=Image.open(BANK/f'materials-{side}.png').convert('RGBA')
        x,y=(index%8)*side,(index//8)*side
        mask=masks.crop((x,y,x+side,y+side))
        for v in record['variants']:
            expected=Image.open(ROOT/v['file']).convert('RGBA')
            assert expected.size==source.size
            assert expected.getchannel('A').tobytes()==source.getchannel('A').tobytes()
            assert v['foot_anchor']==record['foot_anchor']
            for before,after,material in zip(source.get_flattened_data(),expected.get_flattened_data(),mask.get_flattened_data()):
                if not material[0]:
                    assert before==after
                    protected+=int(before[3]!=0)
                changed+=int(before!=after)
            exported=runtime/f'{record["number"]:03}-palette-{v["variant_index"]}.png'
            actual=Image.open(exported).convert('RGBA')
            assert actual.size==expected.size and actual.tobytes()==expected.tobytes(),exported
            exports.append(dict(number=record['number'],variant=v['variant_index'],
                expected=v['file'],sha256=sha(ROOT/v['file']),
                gpu_export=exported.relative_to(ROOT).as_posix(),gpu_sha256=sha(exported),rgba_identical=True))
    layout=read(REVIEW/'native-preview-layout.json')
    assert layout['scale']==1 and layout['actor_resize_operations']==0
    assert len(layout['pages'])==6
    assert {p['number'] for p in layout['placements']}==set(records)-{144,150}
    sheets=[Image.open(ROOT/p['file']).convert('RGB') for p in layout['pages']]
    for p in layout['placements']:
        record=records[p['number']]
        label=p['label'][0]
        path=BANK/record['source'] if label=='A' else ROOT/next(v['file'] for v in record['variants'] if v['variant']==label)
        source=Image.open(path).convert('RGBA')
        x,y=p['position']
        crop=sheets[p['page']-1].crop((x,y,x+source.width,y+source.height))
        for before,after in zip(source.get_flattened_data(),crop.get_flattened_data()):
            if before[3]==255: assert before[:3]==after,p
    checks=read(runtime/'checks.json')
    assert checks['simulation_unchanged'] and checks['real_game_renderer']
    assert checks['additional_material_palettes']==(231 if blue else 180) and checks['extended_palettes']
    assert checks.get('blue_palettes',False)==blue
    assert checks['actual_framebuffer_sizes']==[[w,h] for w,h in [(1920,1080),(2560,1440),(3840,2160)] for _ in range(2)]
    result=dict(review_id=review['review_id'],accepted=True,new_models=88,new_palettes=176,
        total_palette_models=90,total_material_palettes=len(exports),base_models=129,base_variants=240,
        changed_material_pixels=changed,protected_opaque_pixel_comparisons=protected,
        original_alpha_geometry_anchors_and_protected_pixels_identical=True,
        four_generation_150_palettes_identical=True,six_native_boards_verified=True,
        gpu_exports_rgba_identical=all(e['rgba_identical'] for e in exports),
        original_base_a_hashes=original_hashes,renderer_exports=exports,real_game_capture=checks)
    destination=REVIEW
    if blue:
        result.update(review_id=blue_review['review_id'],new_models=51,new_palettes=51,existing_blue_models=39,
            new_blues_preserve_masks_and_pixels=True,blue_native_boards_verified=True)
        destination=REVIEW.parent/'monster-blue-palettes-08'
    (destination/'asset-validation.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({k:v for k,v in result.items() if k not in {'original_base_a_hashes','renderer_exports','real_game_capture'}}))

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runtime',type=Path,default=ROOT/'artifacts/monster-material-palettes-07/runtime')
    parser.add_argument('--blue',action='store_true')
    args=parser.parse_args()
    main(args.runtime.resolve(),args.blue)
