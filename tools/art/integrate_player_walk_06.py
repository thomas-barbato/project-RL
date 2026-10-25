"""Install only the WALK06 class/rifle poses explicitly accepted by the player."""
import hashlib,json,sys
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-06'
ART=ROOT/'artifacts/player-walk-review-06'
DEST=ROOT/'assets/animations/walk-06'
SLUGS=['bastion','technomancien','traqueuse','arcaniste','metamorphe']

def main():
    if '--record-acceptance' in sys.argv:
        atlases=[]
        for ci,slug in enumerate(SLUGS):
            source=ART/f'{slug}-cycles-lossless.webp'
            atlases.append({'class_index':ci,'source':source.relative_to(ROOT).as_posix(),
                'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest()})
        decision={'review_id':'WALK06-ACCEPT01','date':'2026-10-23','timezone':'Europe/Paris',
            'player_quote':"je pense qu'on peut integrer :)",'status':'accepted',
            'scope':'25 class/armor silhouettes with assault rifle model2, original palettes, no extra equipped helmet; two lateral directions; six walk poses plus idle each',
            'runtime_integration_authorized':True,'other_grips_helmets_bestiary_accepted':False,
            'vertical_movement_modified':False,'atlases':atlases}
        path=DOC/'acceptance.json'
        if path.exists():assert json.loads(path.read_text('utf-8'))==decision
        else:path.write_text(json.dumps(decision,ensure_ascii=False,indent=2)+'\n','utf-8')
    decision=json.loads((DOC/'acceptance.json').read_text('utf-8'))
    assert decision['status']=='accepted' and decision['runtime_integration_authorized']
    DEST.mkdir(parents=True,exist_ok=True)
    files=[]
    for ci,slug in enumerate(SLUGS):
        source=ART/f'{slug}-cycles-lossless.webp'
        assert hashlib.sha256(source.read_bytes()).hexdigest()==decision['atlases'][ci]['source_sha256']
        image=Image.open(source).convert('RGBA');assert image.size==(448,640)
        target=DEST/f'{slug}-rifle.png';image.save(target,optimize=True)
        assert Image.open(target).convert('RGBA').tobytes()==image.tobytes()
        files.append({'class_index':ci,'file':target.relative_to(ROOT).as_posix(),
            'sha256':hashlib.sha256(target.read_bytes()).hexdigest(),
            'rgba_sha256':hashlib.sha256(image.tobytes()).hexdigest()})
    manifest={'review_id':'WALK06-CLASS-ARMOR-MATRIX','format_version':1,'native_tile':64,
        'columns':7,'rows':10,'weapon_model':2,'extra_helmets':False,'palettes':'Original only',
        'layout':'Each outfit uses two rows: left then right. Column0 is idle,1..6 are walk frames0..5.',
        'files':files}
    (DEST/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf-8')
    print(json.dumps({'installed_atlases':5,'outfits':25,'native_poses':350,
        'bytes':sum((ROOT/f['file']).stat().st_size for f in files),'decoded_rgba_matches_review':True},indent=2))

if __name__=='__main__':main()
