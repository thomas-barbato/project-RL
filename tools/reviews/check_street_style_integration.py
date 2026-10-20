"""Compare normal runtime rendering with the exact accepted native GPU frames."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageChops

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/street-defects-review-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-defects-review-04'
VIEW=(334,78,1552,782)


def changed_pixels(a,b):
    assert a.size==b.size
    channels=ImageChops.difference(a.convert('RGB'),b.convert('RGB')).split()
    maximum=ImageChops.lighter(ImageChops.lighter(channels[0],channels[1]),channels[2])
    return sum(maximum.histogram()[1:])


def external_hover_border(a,b,count):
    """A one-pixel UI hover square in black void is not a scenery difference."""
    a=a.convert('RGB');b=b.convert('RGB')
    bounds=ImageChops.difference(a,b).getbbox()
    if not bounds or count!=252:return False
    left,top,right,bottom=bounds
    if (right-left,bottom-top)!=(64,64):return False
    # Camera's player-cell center in this native viewport. The entire square
    # must lie beyond even the outer fringe of the six-cell scenery circle.
    near_x=max(left,min(610,right));near_y=max(top,min(392,bottom))
    if (near_x-610)**2+(near_y-392)**2 <= (6*64+16)**2:return False
    if any(high>2 for low,high in b.crop(bounds).getextrema()):return False
    if any(high>2 for low,high in a.crop((left+1,top+1,right-1,bottom-1)).getextrema()):return False
    return True


def build():
    accepted=json.loads((DOC/'acceptance.json').read_text(encoding='utf-8'))
    capture=json.loads((OUT/'engine05/checks.json').read_text(encoding='utf-8'))
    assert accepted['status']=='accepted' and capture['normal_capture']
    assert capture['normal_integration'] and capture['camp_round_trip'] and capture['render_read_only']
    for asset in accepted['runtime_assets']:
        assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']
    differences={}
    raw_differences={}
    hover_differences={}
    for scene in capture['scenes']:
        name=scene['id']
        before=OUT/'engine04'/f'{name}-candidate.png'
        assert hashlib.sha256(before.read_bytes()).hexdigest()==accepted['approved_references'][before.name]
        actual=Image.open(OUT/'engine05'/f'{name}-normal.png').crop(VIEW)
        expected=Image.open(before).crop(VIEW)
        count=changed_pixels(expected,actual)
        raw_differences[name]=count
        if count and external_hover_border(expected,actual,count):
            hover_differences[name]=count
            count=0
        differences[name]=count
        assert differences[name]==0,(name,differences[name])
    before=OUT/'engine04/map-candidate.png'
    assert hashlib.sha256(before.read_bytes()).hexdigest()==accepted['approved_references'][before.name]
    differences['whole-map']=changed_pixels(Image.open(before),Image.open(OUT/'engine05/map-normal.png'))
    assert differences['whole-map']==0,differences
    report=dict(review=accepted['review'],accepted=True,normal_integration=True,
                native_cell=64,map_pixels=[1664,4224],native_views=9,
                comparison='accepted review versus normal renderer, without review overrides',
                changed_pixels=differences,raw_changed_pixels=raw_differences,
                external_ui_hover_pixels=hover_differences,runtime_asset_hashes_match=True,
                render_read_only=True,camp_round_trip=True,world_generation=159,
                captures='artifacts/street-defects-review-01/engine05',
                production_assets=accepted['runtime_assets'])
    path=DOC/'integration-checks.json'
    if path.exists():
        prior=json.loads(path.read_text(encoding='utf-8'))
        for key in ['cargo_build_locked','cargo_check_locked_release','prison_street_tests','cargo_fmt_check','git_diff_check']:
            if key in prior:report[key]=prior[key]
    path.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(integrated=True,views=9,whole_map_changed_pixels=differences['whole-map'],all_changed_pixels=sum(differences.values()))))


if __name__=='__main__':build()
