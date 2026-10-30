"""Check that every captured head choice preserves the calibrated body geometry."""
from pathlib import Path
import hashlib
import json
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/vertical-walk-review-18/current/size-C05'
ENGINE=OUT/'engine'


def main():
    assert (ENGINE/'size-checks.json').exists(), 'complete the engine capture first'
    frozen=0
    for name in ('acceptance-C01.json','acceptance-C02.json','acceptance-C03.json'):
        doc=json.loads((ROOT/'docs/reviews/vertical-walk-18'/name).read_text('utf8'))
        for filename,expected in doc['sha256'].items():
            assert hashlib.sha256((ROOT/filename).read_bytes()).hexdigest()==expected,filename
            frozen+=1
    manifest=json.loads((ROOT/'assets/source-art/vertical-walk-review-18/candidate-C05/manifest.json').read_text())
    compared=0
    for p in manifest['profiles']:
        if not p.get('head_variants'):continue
        for loadout,handkey in [('unarmed',0),('mixed',1)]:
            atlases=[]
            for head in range(7):
                path=ENGINE/'heads'/f'head-{head}-{loadout}'/f'prepared-{p["id"]}-{handkey}.png'
                # Macroquad's CPU export flips the complete atlas vertically.
                atlas=Image.open(path).convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM)
                assert atlas.size==(768,384)
                atlases.append(atlas)
            for head,atlas in enumerate(atlases[1:],1):
                for f in p['frames']+[{'direction':'idle','phase':0}]:
                    row=2 if f['direction']=='idle' else int(f['direction']=='back')
                    x=f['phase']*128
                    crop=(x,row*128+60,x+128,(row+1)*128)
                    assert atlas.crop(crop).tobytes()==atlases[0].crop(crop).tobytes(),(p['id'],loadout,head,f)
                    compared+=1
    base=json.loads((ENGINE/'base/checks.json').read_text())
    roster=json.loads((ENGINE/'roster/checks.json').read_text())
    equipment=json.loads((ENGINE/'equipment/checks.json').read_text())
    heads=json.loads((ENGINE/'heads/checks.json').read_text())
    assert base['front_returns']==12 and roster['front_returns']==12
    assert equipment['front_returns']==80 and equipment['one_handed_hand_contacts']==4056
    assert heads['front_returns']==224 and heads['sampled_weapon_contacts']==672
    camp_returns=sum(json.loads(p.read_text())['front_returns'] for p in ENGINE.rglob('camp-checks.json'))
    assert camp_returns==68
    result={'review':'VERT18-C05','status':'passed','normal_game_enabled':False,
        'approved_source_files_unchanged':frozen,'head_choices':7,'headgear_loadouts':2,'heavy_profiles':4,
        'body_region_comparisons_across_head_choices':compared,'body_region':'native y60..127 in all12 phases plus authored front idle',
        'flat_front_returns':12+12+80+224,'camp_front_returns':camp_returns,'total_front_returns':328+camp_returns,
        'one_handed_contacts_including_idle':4056,
        'headgear_sampled_weapon_contacts':672,'head_operations_preserving_lower_body':heads['head_operations_preserving_arms_and_lower_body'],
        'saved_game_loaded':False,'user_preferences_written':False,
        'limits':'Body geometry and contacts are technical checks; visual approval is still separate.'}
    (OUT/'engine-size-audit.json').write_text(json.dumps(result,indent=2)+'\n','utf8')
    print(json.dumps(result))


if __name__=='__main__':main()
