"""Package accepted frontal BD outfits. Never repaint the review sources."""
from pathlib import Path
import argparse, hashlib, json, re, sys
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = ROOT / 'assets/sprites/flat-agents'
sys.path.insert(0, str(BASE))
import build_armor_flat_review_02 as prepared
import build_armor_flat_review_01 as art
from armor_continuous_registration import Registration
art.RAMPS.update({'silver':['#505b65','#a3adb7','#e1e8eb'],
                  'industrialyellow':['#241b0a','#e2ae31','#ffe066']})

IDS = ['AR-N07','AR-N08','AR-N09','AR-N10','AR-N11','AR-N12',
       'AR-N13','AR-N14','AR-08','AR-10','AR-11','AR-12']
POSES = art.POSES
SIDE = 128

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def atlas(images, side, name):
    result = Image.new('RGBA', (16*side, ((len(images)+15)//16)*side))
    for i, im in enumerate(images): result.paste(im, ((i%16)*side, (i//16)*side))
    result.save(OUT/name)

def package_resolution64(check=False):
    review = BASE/'actor-resolution-review-01'
    receipt = json.loads((review/'visual-acceptance-01.json').read_text('utf-8'))
    manifest = json.loads((OUT/'manifest.json').read_text('utf-8'))
    cells = []
    frozen = {}
    for entry in manifest['outfits']:
        ident = entry['review_id']
        assert ident in receipt['accepted_active_outfits']
        for variant in range(3):
            for config in manifest['configurations']:
                relative = f'native64/{ident}-{variant}-{config["number"]}.png'
                path = review/relative
                expected = receipt['frozen_native64_files'][relative]
                assert sha(path) == expected, path
                image = Image.open(path).convert('RGBA')
                assert image.size == (128,128)
                cells.append(image)
                frozen[str(path.relative_to(ROOT))] = expected
    assert len(cells) == 576
    if check:
        sheet = Image.open(OUT/'reviewed-128.png').convert('RGBA')
        assert sheet.size == (2048,4608)
        for index, image in enumerate(cells):
            x,y = index%16*128,index//16*128
            assert sheet.crop((x,y,x+128,y+128)).tobytes() == image.tobytes(), index
        assert manifest['frozen_native64_sprites'] == frozen
        old_sheet = Image.open(OUT/'reviewed-64.png').convert('RGBA')
        for index,(relative, expected) in enumerate(manifest['frozen_review_sprites'].items()):
            path=ROOT/relative
            assert sha(path)==expected,path
            x,y=index%16*64,index//16*64
            assert old_sheet.crop((x,y,x+64,y+64)).tobytes()==Image.open(path).convert('RGBA').tobytes(),index
    else:
        atlas(cells,128,'reviewed-128.png')
        manifest.update({'resolution_review':'ACTOR-RES01-ACCEPT01',
                         'native_display_sizes':[32,64],
                         'frozen_native64_sprites':frozen})
        (OUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2),'utf-8')
    print(json.dumps({'native64_review_cells_verified':len(cells),'check':check}),flush=True)

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    reviews = {n: json.loads((BASE/f'armor-flat-review-{n:02}/review.json').read_text('utf-8')) for n in (2,4)}
    selected = []
    exact = []; bodies = []; fingers = []; frozen = {}
    for ident in IDS:
        revision = 4 if ident in ('AR-N13','AR-N14','AR-08','AR-10','AR-11','AR-12') else 2
        origin = BASE/f'armor-flat-review-{revision:02}'
        entry = next(e for e in reviews[revision]['entries'] if e['id']==ident)
        selected.append({'review_id':ident,'content_id':entry.get('content_id'),'origin':revision})
        for v in range(3):
            for c in reviews[revision]['configurations']:
                path = origin/entry['views'][f'{v}-{c["number"]}']['native32']
                exact.append(Image.open(path).convert('RGBA'))
                frozen[str(path.relative_to(ROOT))] = sha(path)
        art.OUT = origin
        art.ENTRIES = {e['id']:e for e in reviews[revision]['entries']}
        art.source.cache_clear(); art.joints.cache_clear(); art.body.cache_clear(); art.fingers.cache_clear()
        registration = Registration(art)
        previous_body, previous_registered = art.body, art.registered
        if revision == 4:
            art.body, art.registered = registration.body, registration.registered
        data = json.loads((origin/f'palettes/{ident}.json').read_text('utf-8'))
        def variant(im, v):
            if ident in ('AR-10','AR-11'):
                from PIL import ImageChops
                local = dict(data); local['variants'] = [data['source_materials'],data['variants'][v]]
                colored = art.material_variant(im, local, 1)
                r,g,b = im.convert('RGB').split()
                chroma = ImageChops.subtract(ImageChops.lighter(r,ImageChops.lighter(g,b)), ImageChops.darker(r,ImageChops.darker(g,b)))
                return Image.composite(colored,im,chroma.point(lambda x:255 if x>=6 else 0))
            result = art.material_variant(im,data,v)
            if ident == 'AR-N14' and v:
                from PIL import ImageChops
                r,g,b = im.convert('RGB').split()
                warm = ImageChops.multiply(ImageChops.subtract(r.point(lambda x:round(x*.5)),b).point(lambda x:255 if x>0 else 0), r.point(lambda x:255 if x>40 else 0))
                result = Image.composite(result,im,warm)
            if ident == 'AR-08':
                box=(0,0,art.MS,24*art.F); result.paste(im.crop(box),box)
            return result
        for v in range(3):
            for pose in POSES:
                bodies.append(variant(art.body(ident,pose),v).resize((SIDE,SIDE),Image.Resampling.LANCZOS))
                for hand in (0,1):
                    fingers.append(variant(art.fingers(ident,pose,hand),v).resize((SIDE,SIDE),Image.Resampling.LANCZOS))
        art.body, art.registered = previous_body, previous_registered
        print(ident,flush=True)
    atlas(exact,64,'reviewed-64.png'); atlas(bodies,SIDE,'bodies-128.png'); atlas(fingers,SIDE,'fingers-128.png')
    atlas([exact[(outfit*3+variant)*16].crop((16,16,48,48)) for outfit in range(len(IDS)) for variant in range(3)],32,'outfit-icons-32.png')
    # Keep every model's authored identity; transforms depend only on its grip.
    weapons = []; bindings = []; content_indices = dict((k,int(v)) for k,v in re.findall(r'"([^"]+)" => (\d+),', (ROOT/'src/game_art_catalog.rs').read_text('utf-8').split('fn held_weapon')[1].split('\n}')[0]))
    for e in art.rig.old.ENTRIES:
        if e['kind'] not in ('weapon','shield','focalizer','shaft'): continue
        ident=e['review_id']; hold=e.get('hold','Side')
        pose='pole' if hold=='MeleeTwoHanded' or e.get('family')=='shaft' else 'heavy' if e.get('family') in ('heavy_machinegun','plasma') or e.get('family') in (9,10,12,13) else 'rifle' if hold=='TwoHanded' else 'independent'
        if pose=='rifle' and int(ident[4:]) not in art.rig.old.GUN_GRIPS: pose='heavy'
        sides=(None,) if pose!='independent' else (0,1)
        for side in sides:
            im, meta = art.rig.item_layer(ident,pose,side)
            index=len(weapons); weapons.append(im.resize((SIDE,SIDE),Image.Resampling.LANCZOS))
            bindings.append({'id':e.get('content_id'),'review_id':ident,'model':content_indices.get(e.get('content_id')),'side':side,'pose':POSES.index(pose),'layer':index,'transform':meta})
    atlas(weapons,SIDE,'weapons-128.png')
    configs=reviews[2]['configurations']
    data={'review_id':'ARMOR-FLAT05-INTEGRATION01','outfits':selected,'configurations':configs,'weapons':bindings,'frozen_review_sprites':frozen,'exact_review_sprites':len(exact),'logical_tile':32,'display_tile':64,'layer_canvas':SIDE}
    (OUT/'manifest.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
    lines=['// Generated by tools/art/build_flat_agent_assets.py.']
    lines += ['const COMIC_WEAPONS: &[(u16, bool, u16, u16)] = &[']
    lines += [f'    ({w["model"]}, {str(w["side"]==0).lower()}, {w["pose"]}, {w["layer"]}),' for w in bindings if w['model'] is not None]
    lines += ['];','const COMIC_CONFIGS: &[&[(u16, bool)]] = &[']
    for c in configs:
        values=[]
        for ident,side in c['items']:
            entry=art.rig.old.BY_ID[ident]; model=content_indices.get(entry.get('content_id'))
            if model is None: values=['(u16::MAX, false)'];break
            values.append(f'({model}, {str(side==0).lower()})')
        lines.append('    &['+', '.join(values)+'],')
    lines += ['];']
    (ROOT/'src/game_art_flat_catalog.rs').write_text('\n'.join(lines)+'\n','utf-8')
    assert len(exact)==576 and len(bodies)==144 and len(fingers)==288
    for p,h in frozen.items(): assert sha(ROOT/p)==h
    print(json.dumps({'exact_review_sprites':len(exact),'outfits':len(IDS),'distinct_held_models':len({w['model'] for w in bindings if w['model'] is not None})}),flush=True)
    package_resolution64()

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--resolution64-only',action='store_true')
    parser.add_argument('--check',action='store_true')
    args=parser.parse_args()
    if args.check: package_resolution64(check=True)
    elif args.resolution64_only: package_resolution64()
    else: main()
