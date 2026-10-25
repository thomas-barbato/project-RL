"""Compare native game framebuffer pixels to the accepted installed WALK06 atlases."""
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-06/engine'
DOC=ROOT/'docs/reviews/player-walk-06'
slugs=['bastion','technomancien','traqueuse','arcaniste','metamorphe']
comparisons=[]
for stage in range(5):
    for floor in ['sombre','clair']:
        screenshot=Image.open(ART/f'native-tenue-{stage+1}-{floor}.png').convert('RGB')
        assert screenshot.size==(1240,900)
        for ci,slug in enumerate(slugs):
            atlas=Image.open(ROOT/f'assets/animations/walk-06/{slug}-rifle.png').convert('RGBA')
            for side,column in [(0,0),(1,2)]:
                expected=atlas.crop((192,stage*128+side*64,256,stage*128+side*64+64))
                captured=screenshot.crop((70+ci*232+column*72,462,134+ci*232+column*72,526))
                total=matching=0
                for y in range(64):
                    for x in range(64):
                        p=expected.getpixel((x,y))
                        # Translucent edge pixels blend with the floor/shadow.
                        if p[3]!=255:continue
                        total+=1;q=captured.getpixel((x,y))
                        if max(abs(p[k]-q[k]) for k in range(3))<=3:matching+=1
                fraction=matching/max(1,total)
                comparisons.append({'id':f'CLASS-{ci*5+stage:02}','floor':floor,'side':'left' if side==0 else 'right',
                    'opaque_pixels':total,'matching_pixels':matching,'matching_fraction':fraction})
                assert total>80 and fraction>.99,comparisons[-1]
report={'review_id':'WALK06','renderer':'ordinary game_art::draw_world at native64',
    'native_framebuffer_comparisons':len(comparisons),'floors':['light','dark'],
    'outfits':25,'directions':2,'walk_frame':2,'rgba_tolerance':3,
    'minimum_opaque_pixel_match':min(c['matching_fraction'] for c in comparisons),
    'comparisons':comparisons}
(DOC/'engine-pixel-checks.json').write_text(json.dumps(report,indent=2)+'\n','utf-8')
print(json.dumps({k:v for k,v in report.items() if k!='comparisons'},indent=2))
