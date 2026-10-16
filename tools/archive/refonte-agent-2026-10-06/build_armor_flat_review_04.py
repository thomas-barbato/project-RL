"""Versioned seam correction; prior drawings, palettes and weapon rig preserved."""
from pathlib import Path
import json, hashlib, shutil
from PIL import Image, ImageChops
import build_armor_flat_review_03 as prior
from armor_continuous_registration import Registration, polys

art = prior.art
BASE = Path(__file__).resolve().parent
OUT = BASE/'armor-flat-review-04'
PREVIOUS = BASE/'armor-flat-review-03'
ACCEPTED = BASE/'armor-flat-review-02'
OUT.mkdir(exist_ok=True)
cat = json.loads((PREVIOUS/'catalogue.json').read_text('utf-8'))
accepted = json.loads((ACCEPTED/'catalogue.json').read_text('utf-8'))
heavy = next(e for e in accepted['entries'] if e['id']=='AR-N13')
cat['entries'].insert(1, heavy)
cat['review_id'] = 'ARMOR-FLAT04'
cat['scope'] = 'Layer seam correction for six outfits; approved AR-08 name change'
cat['text_approvals'] = {'AR-08': {'name':'Tablier de cuir','approved':True}}
cat['active_game_content_changed'] = 'Approved name only; artwork remains review-only'
for e in cat['entries']:
    origin = ACCEPTED if e['id']=='AR-N13' else PREVIOUS
    e['category'] = 'revision'
    e['status'] = 'layer_seam_correction_pending_visual_review'
    if e['id']=='AR-08':
        e['name'] = 'Tablier de cuir'
    # Frozen previous compositions remain available for comparison.
    v = 2 if e['id']=='AR-N14' else 0
    e['source_reference'] = str((origin/f'portraits/{e["id"]}-V{v}-00.png').relative_to(art.ROOT))
    for relative in (e['source'], f'palettes/{e["id"]}.json'):
        src = origin/relative; dest = OUT/relative
        dest.parent.mkdir(parents=True,exist_ok=True)
        shutil.copy2(src,dest)
    if e['id']=='AR-N13':
        art.MATERIALS[e['id']] = tuple(e['materials'])
(OUT/'catalogue.json').write_text(json.dumps(cat,ensure_ascii=False,indent=2),'utf-8')
art.OUT = prior.OUT = OUT
art.CAT = cat
art.ENTRIES = {e['id']:e for e in cat['entries']}
old_body = art.body
old_registered = art.registered
registration = Registration(art)
art.body = registration.body
art.registered = registration.registered

def validate(data):
    views = grips = boundaries = 0
    seam_probes = {}
    for e in data['entries']:
        ident = e['id']; fixed = art.rig.normalize(art.source(ident,'independent'))
        for key,view in e['views'].items():
            n = Image.open(OUT/view['native32']).convert('RGBA')
            doubled = Image.open(OUT/view['display64']).convert('RGBA')
            assert n.size == (64,64)
            assert doubled.tobytes() == n.resize((128,128),Image.Resampling.NEAREST).tobytes()
            box = n.getchannel('A').getbbox()
            assert box and min(box[:2])>0 and box[2]<64 and box[3]<64, (ident,key,box)
            views += 1
        for pose in art.POSES:
            b = art.body(ident,pose)
            for v in range(3):
                assert art.material_variant(b,art.author_materials(ident),v).getchannel('A').tobytes()==b.getchannel('A').tobytes()
            for box in ((0,0,art.MS,24*art.F),(0,38*art.F,art.MS,art.MS)):
                assert b.crop(box).tobytes()==art.body(ident,'independent').crop(box).tobytes(), (ident,pose,box)
            # Probe the actual old transparency loss, restricted to the fixed
            # body outside both canonical arm masks. This catches the seam
            # holes missed by palette-only alpha tests in the earlier review.
            protected = ImageChops.multiply(fixed.getchannel('A').point(lambda a:255 if a>240 else 0),
                ImageChops.invert(ImageChops.lighter(registration.mask(ident,'independent'),registration.mask(ident,pose))))
            art.registered = old_registered
            old = old_body(ident,pose)
            art.registered = registration.registered
            missing = old.getchannel('A').point(lambda a:255 if a<150 else 0)
            probe = ImageChops.multiply(protected,missing)
            seam_probes[f'{ident}/{pose}'] = sum(probe.histogram()[1:])
            assert not ImageChops.multiply(probe,b.getchannel('A').point(lambda a:255 if a<240 else 0)).getbbox(), (ident,pose,'fixed-core gap')
            for side,poly in enumerate(polys(ident,pose)):
                for a,c in zip(poly,poly[1:]+poly[:1]):
                    for t in (0,.25,.5,.75):
                        point = tuple(a[k]+(c[k]-a[k])*t for k in (0,1))
                        actual = registration.inverse(ident,pose,side,*point)
                        assert max(abs(actual[k]-point[k]) for k in (0,1))<1e-7
                        boundaries += 1
                hand = art.rig.JOINTS[pose][1][side][2]
                inverse = registration.inverse(ident,pose,side,*hand)
                measured = art.joints(ident,pose)[side][2]
                assert max(abs(inverse[k]-measured[k]) for k in (0,1))<1e-7, (ident,pose,side,'grip')
        for c in data['configurations']:
            for side in (0,1):
                ls = [art.rig.item_layer(i,c['pose'],s)[0] for i,s in c['items'] if s==side or s is None]
                shield = any(s==side and art.rig.old.BY_ID[i]['kind']=='shield' for i,s in c['items'])
                if ls and not shield:
                    glove = art.fingers(ident,c['pose'],side)
                    assert any(ImageChops.multiply(glove.getchannel('A'),l.getchannel('A')).getbbox() for l in ls), (ident,c['number'],side)
                    grips += 3
    previous = json.loads((PREVIOUS/'review.json').read_text('utf-8'))
    assert data['common_rig']==previous['common_rig']
    assert data['weapon_transforms']==previous['weapon_transforms']
    for p,h in data['protected_active_files'].items():
        assert hashlib.sha256((art.ROOT/p).read_bytes()).hexdigest()==h, p
    frozen = json.loads((ACCEPTED/'visual-acceptance-01.json').read_text('utf-8'))
    for p,h in frozen['frozen_files'].items():
        assert hashlib.sha256((ACCEPTED/p).read_bytes()).hexdigest()==h, p
    for e in cat['entries']:
        origin = ACCEPTED if e['id']=='AR-N13' else PREVIOUS
        assert (OUT/e['source']).read_bytes()==(origin/e['source']).read_bytes()
    report = dict(review_id='ARMOR-FLAT04',exact_nearest_doubling=views,
        checked_grip_overlaps=grips,identity_boundary_probes=boundaries,
        fixed_core_repaired_alpha_probes=seam_probes,
        fixed_heads_and_lower_bodies_across_poses=True,palette_alpha_preserved=True,
        common_rig_unchanged=True,common_weapon_transforms_unchanged=True,
        copied_source_paintings_unchanged=True,
        accepted_flat02_files_unchanged=len(frozen['frozen_files']),
        active_art_files_unchanged=len(data['protected_active_files']),
        approved_name='Tablier de cuir',visual_acceptance_pending=True)
    (OUT/'validation.json').write_text(json.dumps(report,indent=2),'utf-8')
    print(json.dumps(report),flush=True)
art.validate = validate

if __name__=='__main__':
    art.build()
