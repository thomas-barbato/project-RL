"""Integrate the explicitly approved U01 lot, omitting three requested props."""
import copy
import shutil
from PIL import Image, ImageChops, ImageDraw
from build_street_repairs_09 import ROOT, ART, DOC, ACTIVE, read, write, sha
import build_street_paving_review as paving
import build_street_expansion_review as lighting

REMOVED = ['warehouse-crate', 'warehouse-cabinet', 'warehouse-barrel']


def integrate():
    baseline = ART / 'baseline'
    approval = DOC / 'visual-candidate.json'
    reviewed = read(approval)
    assert sha(ART/'street-candidate.png') == reviewed['candidate_sha256']
    if not baseline.exists():
        for file in ['street.png', 'street.json', 'correction.png', 'irradiance.png']:
            assert sha(ACTIVE/file) == reviewed['active_sha256'][file], file
        baseline.mkdir()
        for file in ['street.png', 'street.json', 'correction.png', 'irradiance.png']:
            shutil.copy2(ACTIVE/file, baseline/file)
    tiles = [Image.open(paving.ART/f'{n}-native.png').convert('RGBA')
             for n in ['PAV-U01-06', 'PAV-U01-06B', 'PAV-U01-06C']]
    grass = read(paving.ART/'composition-checks.json')['grass_at_px']
    # Explicit follow-up: exposed cells use the already accepted fragmented /
    # nearly erased slabs, with the same coordinate-based rotation as elsewhere.
    worn_cells = {(x,y):tiles[1+(x+y)%2] for y in (7,8) for x in range(9,13)}
    final = paving.compose([tiles[0]]+[tiles[1]]*6+[tiles[2]]*5, grass,
        warehouse=ART/'warehouse-native.png',
        asphalt=Image.open(ART/'ROAD-U01-09-conditioned.png').convert('RGBA'),
        repair_join=True, omit_modules=REMOVED, paving_overrides=worn_cells)
    # Prove the human amendments affect only these props and their exposed floor.
    candidate = Image.open(ART/'street-candidate.png').convert('RGBA')
    delta = ImageChops.difference(candidate, final).convert('RGB')
    allowed = Image.new('L', final.size)
    d = ImageDraw.Draw(allowed)
    removed = []
    for module in read(paving.PARTS/'module-placements.json'):
        if module['id'] not in REMOVED:
            continue
        x, y = module['at']; w, h = Image.open(ROOT/module['file']).size
        d.rectangle((x,y,x+w-1,y+h-1), fill=255)
        removed.append(dict(id=module['id'],at_px=[x,y],size=[w,h]))
    for x,y in worn_cells:
        d.rectangle((x*64,y*64,x*64+63,y*64+63),fill=255)
    assert delta.getbbox() is not None
    assert ImageChops.multiply(delta.convert('L'), ImageChops.invert(allowed)).getbbox() is None
    target = ART/'integrated'
    target.mkdir(exist_ok=True)
    final.save(target/'street.png')
    scene = copy.deepcopy(read(ART/'street.json'))
    cfg = scene['atmosphere']
    floor = {(x,y) for y,row in enumerate((ACTIVE/'layout.txt').read_text().splitlines())
             for x,c in enumerate(row) if c=='.'}
    for file in ['collision-repairs.json','collision-repairs-r09.json','warehouse-clearance-r09.json']:
        floor.update(map(tuple,read(ACTIVE/file)['cells']))
    lighting.ART = target
    lighting.build_lightmaps(final, floor, cfg, cfg['lights'])
    correction = Image.open(target/'correction.png').convert('RGBA')
    d = ImageDraw.Draw(correction)
    for light in cfg['lights']:
        if light['id'].startswith('post-') and light['box']:
            l,t,r,b = light['box']
            d.rounded_rectangle((l+1,t+1,r-1,b-1),radius=2,
                fill=tuple(round(v*255) for v in light['lens_color'])+(255,))
    correction.save(target/'correction.png')
    # Production paths only. The debug candidate remains intact for comparison.
    scene['background'] = (ACTIVE/'street.png').relative_to(ROOT).as_posix()
    cfg['correction'] = (ACTIVE/'correction.png').relative_to(ROOT).as_posix()
    cfg['irradiance'] = (ACTIVE/'irradiance.png').relative_to(ROOT).as_posix()
    write(target/'street.json', scene)
    for file in ['street.png','street.json','correction.png','irradiance.png']:
        shutil.copy2(target/file, ACTIVE/file)
    # Historical geometry is immutable; generation 161 clears only (11,7).
    for file in ['layout.txt','collision-repairs.json','collision-repairs-r09.json',
                 'room-vision.json','roof-0.png','roof-1.png','roof-2.png','roof-3.png']:
        assert sha(ACTIVE/file) == reviewed['active_sha256'][file], file
    assert scene['obstructions'] == read(baseline/'street.json')['obstructions']
    write(DOC/'acceptance.json',dict(review='U01-REPAIRS-09',date='2026-10-20',
        status='accepted_and_integrated',
        question='Valides-tu le lot visuel U01-REPAIRS-09 présenté dans la revue : entrepôt moins vif, trait noir retiré de la route et jonction sud recomposée avec une annexe entière, des haies complètes et le corps replacé au sol ?',
        answer='1 cependant il faudrait supprimer à coté du bâtiment en "L" la petite caisse en bois, le baril bleu et la poubelle verte que tu as mis .',
        accepted=['STYLE-U01-09','ROAD-U01-09','JONC-U01-09'],
        amendment=dict(removed_modules=removed,
            paving_request='il faut que tu applique la meme politique de semi-effacement des rebords des cases ou se trouvaient ces 3 objets :) comme partout ailleurs .',
            paving_cells=[list(at) for at in worn_cells],
            paving_sources=['PAV-U01-06B-native.png','PAV-U01-06C-native.png'],
            delta_confined_to_removed_props_and_their_floor=True,
            extra_clear_cells=[[11,7]],generation=161),
        native_cell=64,reviewed_candidate_sha256=reviewed['candidate_sha256'],
        runtime_sha256={file:sha(ACTIVE/file) for file in
            ['street.png','street.json','correction.png','irradiance.png','warehouse-clearance-r09.json']},
        historical_geometry_unchanged=True,normal_runtime_uses_assets_maps=True))
    print('Integrated U01-REPAIRS-09; removed three whole modules; historical geometry preserved.')


if __name__ == '__main__':
    integrate()
