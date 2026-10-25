"""Select review drafts, preserving raw attempts and every accepted source.

Only complete authored figures are extracted/assembled here. No pixel editing,
mirroring, limb transformations, or phase-specific scale correction is applied.
Selection is not the player's visual acceptance or runtime integration.
"""
import hashlib,json
from pathlib import Path
from PIL import Image
from build_player_walk_matrix_06 import grid_figures

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
SRC=ROOT/'assets/source-art/player-walk-review-07'
read=lambda p:json.loads(p.read_text('utf8'))
write=lambda p,d:p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()

def assemble(identity,left,right,revision='r05'):
    destination=SRC/f'{identity}-cycle-bulk-{revision}-assembled.png'
    figures=[];inputs=[]
    for side,(name,columns,rows,indices) in zip(['left','right'],[left,right]):
        source=SRC/name
        poses,warnings=grid_figures(Image.open(source).convert('RGBA'),columns,rows)
        assert not warnings,(source,warnings)
        selected=[poses[i].crop(poses[i].getbbox()) for i in indices]
        # Independently generated input sheets have different resolutions.
        # Calibrate each WHOLE directional series once from its standing idle;
        # all its walking phases retain that same factor and relative bob.
        factor=400/selected[0].height
        selected=[im.resize((round(im.width*factor),round(im.height*factor)),Image.Resampling.LANCZOS) for im in selected]
        figures.extend(selected)
        inputs.append({'file':source.relative_to(ROOT).as_posix(),'sha256':digest(source),
                       'selected_whole_figure_indices':indices,
                       'direction':side,'uniform_source_resolution_factor':factor})
    cell=(max(im.width for im in figures)+64,max(im.height for im in figures)+64)
    atlas=Image.new('RGBA',(7*cell[0],2*cell[1]))
    for n,im in enumerate(figures):
        atlas.paste(im,(n%7*cell[0]+(cell[0]-im.width)//2,n//7*cell[1]+cell[1]-32-im.height))
    if destination.exists():
        assert Image.open(destination).convert('RGBA').tobytes()==atlas.tobytes()
    else:atlas.save(destination)
    write(DOC/f'whole-figure-assembly-{identity}-{revision}.json',{
        'identity':identity,'inputs':inputs,'file':destination.relative_to(ROOT).as_posix(),
        'sha256':digest(destination),'whole_figures_only':True,'raster_pixels_edited':False,
        'uniform_source_resolution_calibration':True,'per_phase_scaling':False,
        'visual_acceptance':False,'runtime_integration':False})
    return destination

def main():
    assembled={
        'MON-119-0':assemble('MON-119-0',('MON-119-0-cycle-bulk-left-r03.png',4,2,list(range(7))),
                           ('MON-119-0-cycle-bulk-right-r03.png',4,2,list(range(7)))),
        'MON-025-0':assemble('MON-025-0',('MON-025-0-left-cycle-bulk-r05.png',4,2,list(range(7))),
                           ('MON-025-0-cycle-bulk-r04.png',7,2,list(range(7,14)))),
        'MON-152-0':assemble('MON-152-0',('MON-152-0-left-cycle-bulk-r06.png',4,2,list(range(7))),
                           ('MON-152-0-cycle-bulk-r04.png',7,2,list(range(7,14))),revision='r06'),
        'MON-135-0':assemble('MON-135-0',('MON-135-0-left-cycle-bulk-r06.png',4,2,list(range(7))),
                           ('MON-135-0-cycle-bulk-r05.png',7,2,list(range(7,14))),revision='r06'),
    }
    # Additional prop owners have a correctly authored half in two separate
    # attempts. Keep each complete half; never repair pixels or graft hands.
    for ident,right_name,right_columns,right_rows,right_indices in [
        ('MON-014-0','MON-014-0-cycle-bulk-r01.png',7,2,list(range(7))),
        ('MON-017-0','MON-017-0-cycle-bulk-r02.png',7,2,list(range(7))),
        *[(f'MON-{n:03}-0',f'MON-{n:03}-0-right-cycle-bulk-r06.png',4,2,list(range(7))) for n in [132,136,138]],
    ]:
        assembled[ident]=assemble(ident,(f'{ident}-cycle-bulk-r06.png',7,2,list(range(7))),
                                  (right_name,right_columns,right_rows,right_indices),revision='r06')
    swaps=set(read(DOC/'bulk-facing-observations.json')['source_top_is_right'])|{'MON-077-0','MON-105-0'}
    r02={f'MON-{n:03}-0' for n in [17,61,80,106,133]}
    r04={f'MON-{n:03}-0' for n in [7,10,15,25,111,121,123,134,135,152,159]}
    r05={'MON-011-0','MON-121-0'}
    selected=[]
    for path in sorted(DOC.glob('creature-jobs-lot*.json')):
        lot=int(path.stem.rsplit('lot',1)[1])
        if not 12<=lot<=28:continue
        jobs=read(path)
        for job in jobs:
            ident=job['id']
            revision='r05' if ident in r05 else 'r04' if ident in r04 else 'r02' if ident in r02 else 'r01'
            source=assembled.get(ident) or SRC/f'{ident}-cycle-bulk-{revision}.png'
            assert source.exists(),source
            assert not (DOC/f'visual-acceptance-creatures-lot{lot:02}.json').exists()
            job['source_destination']=str(source)
            job['source_columns']=7;job['source_rows']=2;job['selected_source_indices']=list(range(14))
            job['directional_halves_reordered']=ident in swaps if revision=='r01' else ident=='MON-133-0'
            job['draft_revision']=source.stem.split('cycle-bulk-',1)[1] if ident in assembled else revision
            job['visual_status']='draft_selected_for_direction_inspection'
            job['visual_acceptance']=False
            selected.append({'id':ident,'review_id':job['review_id'],'lot':lot,
                             'file':source.relative_to(ROOT).as_posix(),'sha256':digest(source),
                             'revision':job['draft_revision'],'halves_reordered':job['directional_halves_reordered']})
        write(path,jobs)
    deferred=[]
    for lot,ident,revision in [(2,'MON-029-0','r04'),(4,'MON-127-0','r05'),(7,'MON-151-0','r04')]:
        entry=read(DOC/f'creature-deferred-lot{lot:02}.json')
        job=entry['original_job']
        assert job['id']==ident
        job.update(source_destination=str(SRC/f'{ident}-cycle-bulk-{revision}.png'),source_columns=7,source_rows=2,
                   selected_source_indices=list(range(14)),directional_halves_reordered=False,
                   strict_source_outer_margin=True,draft_revision=revision,
                   visual_status='draft_selected_for_direction_inspection',visual_acceptance=False)
        assert Path(job['source_destination']).exists()
        deferred.append(job)
        selected.append({'id':ident,'review_id':job['review_id'],'lot':29,
                         'file':Path(job['source_destination']).relative_to(ROOT).as_posix(),
                         'sha256':digest(Path(job['source_destination'])),'revision':revision,'halves_reordered':False})
    write(DOC/'creature-jobs-lot29.json',deferred)
    write(DOC/'bulk-draft-selection.json',{'status':'direction_inspection_pending','visual_acceptance':False,
        'runtime_integration':False,'date':'2026-10-24','entries':selected,
        'source_order_rule':'Actual whole-body facing is checked before associating a half with left or right; no phase reversal or mirroring.'})
    print(json.dumps({'selected':len(selected),'accepted_assets_changed':False,'runtime_integration':False}))

if __name__=='__main__':main()
