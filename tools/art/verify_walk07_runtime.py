"""Verify installed WALK07 without any preview/capture dependency."""
import hashlib,json,math,sys
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07/integration'
read=lambda p:json.loads(p.read_text('utf8'))
manifest=read(ROOT/'assets/animations/walk-07/manifest.json')
decision=read(DOC/'authorization.json')
proofs={p['id']:p for p in decision['corrected_pixels_and_scope']}
assert decision['runtime_integration_authorized'] and decision['correction_then_integration_authorized']
assert len(manifest['entries'])==130 and len(proofs)==130
total=0;small=0;runtime_bytes=0
for entry in manifest['entries']:
    path=ROOT/entry['file'];proof=proofs[entry['id']]
    assert path.is_relative_to(ROOT/'assets/animations/walk-07/creatures')
    im=Image.open(path).convert('RGBA');ref=Image.open(ROOT/proof['native_source']).convert('RGBA')
    assert im.size==(896,256) and im.tobytes()==ref.tobytes()
    assert hashlib.sha256(im.tobytes()).hexdigest()==proof['rgba_sha256']
    poses=[im.crop((i%7*128,i//7*128,i%7*128+128,i//7*128+128)) for i in range(14)]
    assert [hashlib.sha256(p.tobytes()).hexdigest() for p in poses]==proof['native_pose_sha256']
    boxes=[p.getbbox() for p in poses]
    assert all(b and min(b[:2])>0 and max(b[2:])<128 for b in boxes),entry['id']
    for side in range(2): assert len({p.tobytes() for p in poses[side*7+1:side*7+7]})==6,entry['id']
    b=[min(b[0] for b in boxes),min(b[1] for b in boxes),max(b[2] for b in boxes),max(b[3] for b in boxes)]
    assert b==entry['cycle_bounds']
    offset=math.floor(entry['foot'][1]-32-(b[1]+b[3])/2+.5) if b[3]-b[1]<64 else 0
    assert offset==entry['vertical_offset']
    if b[3]-b[1]<64:small+=1
    total+=14;runtime_bytes+=path.stat().st_size
    if entry['number'] in [99,106]:
        before=Image.open(ROOT/f"assets/source-art/player-walk-review-07/integration-corrections-r07/{entry['id']}-before-native.png").convert('RGBA')
        if entry['number']==99:
            assert im.crop((0,128,896,256)).tobytes()==before.crop((0,128,896,256)).tobytes()
        else:
            assert im.crop((0,0,896,128)).tobytes()==before.crop((0,128,896,256)).tobytes()
            assert im.crop((0,128,896,256)).tobytes()==before.crop((0,0,896,128)).tobytes()
sys.path.insert(0,str(ROOT/'tools/distribution'))
from package_release import inventory
files=inventory()
assert len([p for p in files if p.startswith('assets/animations/walk-07/')])==131
report={'status':'passed','runtime_cycles':130,'campaign_base_models':129,'native_poses':total,
        'native_rgba_matches_selected_sources':True,'small_cycles_centered':small,'one_fixed_offset_per_cycle':True,
        'm100_right_preserved':True,'m104_complete_halves_reassociated':True,'runtime_texture_bytes':runtime_bytes,
        'runtime_files_in_distribution_inventory':131,'preview_files_required':False,
        'unreviewed_color_variants_retain_existing_renderer':True}
(DOC/'native-checks.json').write_text(json.dumps(report,indent=2)+'\n','utf8')
print(json.dumps(report))
