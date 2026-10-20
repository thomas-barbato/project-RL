"""Resumable completion registry; every master is made with built-in imagegen."""
import json
from pathlib import Path
from PIL import Image
from art_finish_batch_02 import ROOT, sha, write

ART=ROOT/'assets/source-art/art-finish-completion-06'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/art-finish-completion-06'
OUT=ROOT/'artifacts/art-quality-audit-01'
EXISTING={5:'FIN-08',6:'FIN-30',14:'FIN-28',16:'FIN-29',120:'FIN-31',144:'FIN-09',145:'FIN-10',164:'FIN-06',165:'FIN-07'}
EXISTING_BATCH={5:1,6:5,14:5,16:5,120:5,144:1,145:1,164:1,165:1}


def prepare():
    assert not (DOC/'acceptance.json').exists(), 'Accepted completion is frozen; start a distinct revision'
    ART.mkdir(parents=True,exist_ok=True);DOC.mkdir(parents=True,exist_ok=True)
    (ART/'references').mkdir(exist_ok=True);(ART/'generation').mkdir(exist_ok=True)
    manifest=json.loads((ROOT/'assets/sprites/campaign-monsters/manifest.json').read_text('utf-8'))
    models=sorted([e for e in manifest['entries'] if e['variant']==0],key=lambda e:e['number'])
    jobs=[]
    common=('Use case: style-transfer. Production source master for Project RL, a grimdark futuristic/cyberpunk roguelike. '
        'Image 1 is the EDIT TARGET, an existing isolated native game sprite: keep its exact recognizable creature design, pose, head direction, anatomy, equipment, costume and material colors. '
        'Image 2 is ONLY the approved painterly finish reference; never copy its body, limbs or clothing onto the target. '
        'Make a freshly painted large source master of this exact target, with matte worn materials, broad readable shadow masses, natural folds and restrained irregular dark contours, matching the approved reference. '
        'Use localized wear rather than cracks or speckles on every surface. Organic skin must read as flesh, fabric as cloth and machinery as metal; do not give everything a stone texture. '
        'Preserve simple, legible masses when reduced to the ORIGINAL game size. Do not add tiny busy ornaments, new limbs, weapons, horns, bones, wings, anatomy or effects. '
        'Keep the exact sprite view and proportions of the target, not isometric and not an overhead redesign. '
        'Render ONE COMPLETE specimen with all fingertips, antennae, tails, wing tips, weapons, wisps and feet inside the canvas, with at least 10 percent empty margin on EVERY side. '
        'TRUE TRANSPARENT background. No floor, scenery, text, frame or ground contact shadow. Preserve only flames or wisps already belonging to the target design; no new glow or aura. ')
    code=32
    for e in models:
        n=e['number']
        if n in EXISTING:continue
        reference=ART/'references'/f'MON-{n:03}-A.png'
        if not reference.exists():Image.open(ROOT/'assets/sprites/campaign-monsters'/e['tile']).save(reference)
        finish='FIN-06' if e['role'].startswith('Cultist') or e['section']=='Cultistes humains' or 'scientifiques' in e['section'] else 'FIN-08'
        hint=f'Existing design identification: {e["review_label"]}, reference {n:03} A. '
        if e['role'].startswith('Cultist') or 'scientifiques' in e['section']:
            hint+='Preserve the opaque hood/mask or scientific protective gear shown in image 1; do not expose a new human face. '
        if e['role']=='Spirit':hint+='Preserve the spectral silhouette and its existing translucency and floating lower shape. '
        if e['bounds'][3]-e['bounds'][1]<35:hint+='This is an intentionally small or low creature; preserve its low silhouette rather than inventing a tall humanoid. '
        jobs.append(dict(id=f'FIN-{code}',number=n,label=e['review_label'],display_name=e['display_name'],role=e['role'],section=e['section'],
            side=e['side'],foot_anchor=e['foot_anchor'],bounds=e['bounds'],source_tile=e['tile'],
            prompt=common+hint,referenced_image_paths=[reference.as_posix(),(ROOT/f'assets/source-art/art-finish-review-01/{finish}-source.png').as_posix()],
            transparent_background=True,output=(ART/f'FIN-{code}-source.png').as_posix(),
            result_file=(ART/'generation'/f'FIN-{code}.json').as_posix()))
        code+=1
    write(ART/'prompts.json',dict(review='ART-FINISH-COMPLETE-06',mode='built-in imagegen',jobs=jobs))
    receipt=dict(review='ART-FINISH-COMPLETE-06',date='2026-10-19',
        user_instructions=['je te laisse tous les finires','et les valider'],
        authorization='User delegates completion and visual acceptance of remaining existing designs to Codex; no repeated human visual approval required.',
        constraints='Existing identities, combat rules, sector bosses deferred, 64px grid and approved style preserved. New gameplay or player-facing names are outside this delegation.')
    write(DOC/'delegation.json',receipt)
    base=DOC/'production-before.json'
    if not base.exists():
        paths=[p for p in (ROOT/'assets/sprites').rglob('*') if p.is_file()]
        paths += [ROOT/'assets/maps/prison-street-expanded-01'/n for n in ['street.json','street.png','layout.txt']]
        write(base,{p.relative_to(ROOT).as_posix():sha(p) for p in paths})
    write(DOC/'inventory.json',dict(campaign_models=len(models),existing_finished_models=len(EXISTING),remaining_master_drawings=len(jobs),
        existing_native_variants=len(manifest['entries']),runtime_material_variants=len(manifest['runtime_material_palettes'])+len(manifest['runtime_blue_material_palettes']),
        scope='Complete current campaign bestiary and color variants; then audit remaining worn weapon combinations and equipment icons. Preserve current coherent camp NPCs. Furniture scale audit follows actor finish.',
        status='in_progress'))
    print(json.dumps(dict(models=len(models),remaining=len(jobs),previous_resources=len(json.loads(base.read_text('utf-8'))))))


def condition_master(job):
    path=Path(job['output']);image=Image.open(path).convert('RGBA')
    alpha=image.getchannel('A')
    box=alpha.point(lambda a:255 if a>128 else 0).getbbox()
    assert box and min(box[:2])>0 and box[2]<image.width and box[3]<image.height,(job['id'],'clipped source',box,image.size)
    whole=image.crop(box);whole.putalpha(whole.getchannel('A').point(lambda a:0 if a<40 else a))
    b=job['bounds'];ratio=min((b[2]-b[0])/whole.width,(b[3]-b[1])/whole.height)
    whole=whole.resize((round(whole.width*ratio),round(whole.height*ratio)),Image.Resampling.LANCZOS)
    side=job['side'];foot=job['foot_anchor'];result=Image.new('RGBA',(side,side))
    result.alpha_composite(whole,(foot[0]-whole.width//2,foot[1]-whole.height))
    native=ART/'native';native.mkdir(exist_ok=True)
    dest=native/f'{job["number"]:03}-0.png';result.save(dest,optimize=True)
    return dict(id=job['id'],number=job['number'],file=dest.relative_to(ROOT).as_posix(),sha256=sha(dest),bounds=list(result.getbbox()),source_sha256=sha(path))


def condition():
    assert not (DOC/'acceptance.json').exists(), 'Accepted completion is frozen'
    jobs=json.loads((ART/'prompts.json').read_text('utf-8'))['jobs'];rows=[];missing=[];rejected=[]
    for j in jobs:
        if not Path(j['output']).exists():
            retained=ART/'native'/f'{j["number"]:03}-0.png'
            if j['number']==113 and (DOC/'retained-113.json').exists():
                rows.append(dict(id=j['id'],number=113,file=retained.relative_to(ROOT).as_posix(),sha256=sha(retained),bounds=list(Image.open(retained).getbbox()),status='existing_art_retained_after_generator_refusal'))
                continue
            missing.append(j['id']);continue
        try:rows.append(condition_master(j))
        except AssertionError as ex:rejected.append(dict(id=j['id'],reason=str(ex)))
    write(DOC/'master-checks.json',dict(ready=rows,missing=missing,rejected=rejected))
    print(json.dumps(dict(ready=len(rows),missing=len(missing),rejected=rejected)))


if __name__=='__main__':
    import sys
    condition() if '--condition' in sys.argv else prepare()
