"""Compose distinct outfit candidates using the unchanged approved rig."""
from pathlib import Path
import json, hashlib, shutil, argparse
from PIL import Image
import build_armor_flat_review_01 as art

BASE=Path(__file__).resolve().parent
OUT=BASE/'armor-flat-review-02'
PREVIOUS=BASE/'armor-flat-review-01'
art.OUT=OUT
art.CAT=json.loads((OUT/'catalogue.json').read_text('utf-8'))
art.ENTRIES={e['id']:e for e in art.CAT['entries']}
art.RAMPS.update({
 'workpetrol':['#1b2d3b','#324d60','#557189'],
 'clothblue':['#203057','#385282','#5674b4'],
 'mutedolive':['#3c3b22','#67643f','#969068'],
 'indigo':['#181e3b','#2c365e','#4a5683'],
})
for e in art.CAT['entries']:
    art.MATERIALS[e['id']]=tuple(e['materials'])
(OUT/'palettes').mkdir(exist_ok=True)
shutil.copy2(PREVIOUS/'palettes/AR-N06.json',OUT/'palettes/AR-N06.json')

# Explicit sleeve source polygons: long cloth cuffs extend below an armored
# forearm. This only selects the generated cloth; joints and weapons stay fixed.
old_arm_polys=art.arm_polys
wide_sleeves={
 'independent':[
  [[100,390],[482,390],[483,473],[463,519],[438,572],[449,730],[310,795],[100,735]],
  [[770,390],[1160,390],[1160,735],[940,795],[805,730],[813,572],[792,519],[770,473]]],
 'rifle':[
  [[310,390],[472,390],[495,480],[660,463],[686,590],[640,730],[485,810],[310,660]],
  [[770,390],[889,390],[917,486],[1068,485],[1090,745],[920,795],[800,720],[770,552]]],
 'heavy':[
  [[280,390],[477,390],[485,500],[551,537],[555,725],[426,805],[225,710],[245,480]],
  [[778,390],[945,390],[990,526],[1000,710],[894,805],[744,725],[744,537],[779,500]]],
 'pole':[
  [[280,390],[474,390],[479,476],[565,502],[693,538],[705,710],[550,820],[350,720],[250,568]],
  [[710,385],[850,380],[890,414],[922,467],[990,493],[1000,750],[837,800],[770,650],[690,525]]],
}

def sleeve_polys(ident,pose):
    if ident!='AR-N10':return old_arm_polys(ident,pose)
    return [art.affine_points(wide_sleeves[pose][s],art.rig.JOINTS[pose][1][s],art.joints(ident,pose)[s]) for s in (0,1)]

art.arm_polys=sleeve_polys

def validate(data):
    checked=0;clipped=[];grips=0
    weapons={c['number']:[(i,s,art.rig.item_layer(i,c['pose'],s)[0]) for i,s in c['items']] for c in data['configurations']}
    for e in data['entries']:
        for key,view in e['views'].items():
            n=Image.open(OUT/view['native32']).convert('RGBA')
            large=Image.open(OUT/view['display64']).convert('RGBA')
            assert n.size==(64,64)
            assert large.tobytes()==n.resize((128,128),Image.Resampling.NEAREST).tobytes()
            bounds=n.getchannel('A').getbbox();assert bounds
            if min(bounds[:2])==0 or bounds[2]==64 or bounds[3]==64:clipped.append([e['id'],key])
            checked+=1
        for pose in art.POSES:
            b=art.body(e['id'],pose)
            for v in (1,2):
                assert art.material_variant(b,art.author_materials(e['id']),v).getchannel('A').tobytes()==b.getchannel('A').tobytes()
        for pose in art.POSES[1:]:
            b=art.body(e['id'],pose);ref=art.body(e['id'],'independent')
            for box in ((0,0,art.MS,24*art.F),(0,38*art.F,art.MS,art.MS)):
                assert b.crop(box).tobytes()==ref.crop(box).tobytes(),(e['id'],pose,box)
        # Recheck all grip contours, including retained outputs in a selective
        # rebuild. The palette alpha tests above establish the three variants.
        for c in data['configurations']:
            for side in (0,1):
                layers=[(i,l) for i,s,l in weapons[c['number']] if s==side or s is None]
                shield=any(s==side and art.rig.old.BY_ID[i]['kind']=='shield' for i,s in c['items'])
                if layers and not shield:
                    glove=art.fingers(e['id'],c['pose'],side)
                    assert any(art.ImageChops.multiply(glove.getchannel('A'),l.getchannel('A')).getbbox() for i,l in layers),(e['id'],c['number'],side)
                    grips+=3
    assert not clipped,clipped
    old=json.loads((PREVIOUS/'review.json').read_text('utf-8'))
    assert data['weapon_transforms']==old['weapon_transforms']
    assert data['common_rig']==old['common_rig']
    preserved=0
    for v in range(3):
        for c in data['configurations']:
            filename=f'AR-N06-V{v}-{c["number"]}.png'
            assert (OUT/'native32'/filename).read_bytes()==(PREVIOUS/'native32'/filename).read_bytes(),filename
            preserved+=1
    for path,expected in data['protected_active_files'].items():
        assert hashlib.sha256((art.ROOT/path).read_bytes()).hexdigest()==expected,path
    report=dict(review_id='ARMOR-FLAT02',new_proposals=8,light_starting_proposals=5,
        progression_proposals=3,unchanged_quality_reference='AR-N06',
        reference_compositions_preserved=preserved,variants_per_outfit=3,
        configurations_per_outfit=16,exact_nearest_doubling=checked,
        clipped_views=clipped,fixed_heads_and_lower_bodies_across_poses=True,
        common_weapon_transforms_unchanged=True,common_rig_unchanged=True,
        checked_grip_overlaps=grips,palette_alpha_preserved=True,
        active_files_unchanged=len(data['protected_active_files']),
        active_game_content_changed=False,visual_acceptance_pending=True,
        no_inventory_palette_variants=True)
    (OUT/'validation.json').write_text(json.dumps(report,indent=2),'utf-8')
    data['checked_grip_overlaps']=grips
    (OUT/'review.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
    (OUT/'review-data.js').write_text('window.armorReview='+json.dumps(data,ensure_ascii=False)+';','utf-8')
    print(json.dumps(report),flush=True)

art.validate=validate

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--build',action='store_true');parser.add_argument('--only')
    args=parser.parse_args()
    art.build(args.only.split(',') if args.only else None) if args.build else art.conditioning()
