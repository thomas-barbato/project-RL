"""Package the explicitly accepted sewer test into durable runtime assets."""
import copy
import shutil
from PIL import Image,ImageDraw
import build_sewer_review_04 as review

ROOT=review.ROOT
DEST=ROOT/'assets/maps/prison-sewer-01'
read,write,rel,sha=review.read,review.write,review.rel,review.sha

def package():
    review.habitat.accepted_unchanged()
    agreement=review.DOC/'acceptance.json'
    if not agreement.exists():
        files=list(review.ART.rglob('*'))+list((review.DOC/'references').rglob('*'))
        dependencies=set()
        for i in (1,2,3):
            p=read(review.ART/f'seed-{i}/plan.json')
            dependencies.update(ROOT/x['file'] for x in p['props'])
        files+=list(dependencies)
        write(agreement,dict(review='EG-COMP-04 / EG-POP-04',date='2026-10-20',status='approved_for_in_game_test',
            visual_user_quote='je valide, pour ensuite aller vérifier moi meme en jue :)',
            population_user_quote='je valide pour tester en jeu :)',
            accepted=['three displayed sewer compositions','displayed isolated drawings and reused habitat pieces',
                      '85/104/78 ordinary monsters in 11/14/9 groups','six-cell clear arrival and existing combat profiles'],
            not_approved=['EG-NEC-02 resurrection numbers and activation','new combat balance or XP rules'],
            accepted_source_sha256={rel(f):sha(f) for f in sorted(set(files)) if f.is_file()},
            proposal_sha256=sha(review.DOC/'proposal.json'),
            normal_street_sha256_at_acceptance=review.base.hashes()))
    agreement_data=read(agreement)
    for file,digest in agreement_data['accepted_source_sha256'].items():assert sha(ROOT/file)==digest,file
    (DEST/'art').mkdir(parents=True,exist_ok=True)
    copied={}
    def asset(file):
        if file in copied:return copied[file]
        source=ROOT/file
        assert source.resolve().is_relative_to((ROOT/'assets').resolve()),file
        out=DEST/'art'/f'{source.stem}-{sha(source)[:12]}{source.suffix}'
        if not out.exists():shutil.copy2(source,out)
        assert sha(out)==sha(source)
        copied[file]=rel(out);return rel(out)
    def manifest(value):
        if isinstance(value,dict):return {k:manifest(v) for k,v in value.items()}
        if isinstance(value,list):return [manifest(v) for v in value]
        if isinstance(value,str) and value.startswith('assets/') and value.endswith('.png'):return asset(value)
        return value
    for i in (1,2,3):
        folder=DEST/f'variant-{i}';folder.mkdir(exist_ok=True)
        write(folder/'street.json',manifest(read(review.ART/f'seed-{i}/street.json')))
        plan=read(review.ART/f'seed-{i}/plan.json')
        # Gameplay consumes the exact accepted layout, actors and ambient routes;
        # review-only prop manifests and screenshots are not runtime inputs.
        keys=['seed','width','height','spawn','layout','groups','blocked','rats','views']
        write(folder/'plan.json',{k:plan[k] for k in keys})
    entry=read(review.ART/'entry/street.json')
    write(DEST/'entry.json',manifest(entry))
    defense=read(ROOT/'assets/source-art/first-sector-review-01/u01-defense-r10/street.json')
    original=read(ROOT/'assets/maps/prison-street-expanded-01/street.json')
    # Retain the existing opt-in gate defense when used by its test launcher.
    normal_props={(p['file'],tuple(p['at_px'])) for p in original['scenery_props']}
    entry_defense=copy.deepcopy(entry)
    entry_defense['scenery_props'] += [p for p in defense['scenery_props']
        if (p['file'],tuple(p['at_px'])) not in normal_props]
    write(DEST/'entry-defense.json',manifest(entry_defense))
    # Rebuild removal footprints from authored polygons. Cells still occupied by
    # a remaining solid are retained; opening the hatch never erases a building.
    placement=read(ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r05/decor-placements.json')
    removed_at={tuple(p['at_px']) for p in placement if p['code'] in ('DECO-U01-021','WAR-U01-05','WAR-U01-06')}
    shapes=read(ROOT/'assets/source-art/first-sector-review-01/u01-repairs-r09/collision-repairs-r09.json')['footprints']
    removed=Image.new('L',(1664,4224));remaining=Image.new('L',removed.size)
    for shape in shapes:
        x,y=shape['at_px'];points=[(x+a,y+b) for a,b in shape['polygon_px']]
        image=removed if shape['id']=='collapsed-wall' or (x,y) in removed_at else remaining
        ImageDraw.Draw(image).polygon(points,fill=255)
    cells=[]
    for y in range(66):
        for x in range(26):
            box=(x*64,y*64,(x+1)*64,(y+1)*64)
            if removed.crop(box).histogram()[255]>=12 and remaining.crop(box).histogram()[255]<12:
                cells.append([x,y])
    if [2,14] not in cells:cells.append([2,14])
    write(DEST/'entry-clearance.json',dict(cells=sorted(cells),entry=[2,14]))
    for filename in ('entry.json','entry-defense.json'):
        scene=read(DEST/filename)
        scene['obstructions']=[p for p in scene['obstructions'] if p not in cells]
        scene.pop('visual_review',None)
        write(DEST/filename,scene)
    for source,name in [(review.base.ART/'kit/water-native.png','water.png'),(review.base.ART/'kit/rat-native.png','rats.png')]:
        shutil.copy2(source,DEST/name)
    write(review.DOC/'runtime-assets.json',dict(root=rel(DEST),copies=copied,clearance_cells=cells,
        variants=[dict(index=i,seed=seed,monsters=sum(len(g['members']) for g in read(DEST/f'variant-{i}/plan.json')['groups'])) for i,seed in enumerate(review.SEEDS,1)]))
    review.habitat.accepted_unchanged()
    print(f'Packaged accepted sewer: {len(copied)} unique images, three layouts, {len(cells)} cleared debris cells.')

if __name__=='__main__':package()
