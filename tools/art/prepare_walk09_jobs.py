"""One accepted whole-figure gait guide, exact current outfit references."""
from pathlib import Path
import json
from PIL import Image, ImageDraw
from build_player_walk_matrix_06 import grid_figures

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-09'
SRC = ROOT / 'assets/source-art/player-walk-review-09'


def whole(sheet, row):
    im = Image.new('RGBA', (128,128))
    for p in [0,1,3,2,4]:
        im.alpha_composite(sheet.crop((p*128,row*128,(p+1)*128,(row+1)*128)))
    return im


def main():
    DOC.mkdir(exist_ok=True);SRC.mkdir(exist_ok=True)
    manifest = json.loads((ROOT/'assets/animations/walk-08/player/manifest.json').read_text())
    guide = Image.new('RGBA',(1024,1024)); guide_tiles=[]
    for direction,facing in enumerate([-1,1]):
        ref = ROOT / f'docs/reviews/player-walk-05/references/approved-{"left" if facing<0 else "right"}-six.png'
        figures,_ = grid_figures(Image.open(ref).convert('RGBA'),3,2)
        e=next(e for e in manifest if e['id']=='CLASS-04' and e['facing']==facing and e['frame']==8)
        idle=whole(Image.open(ROOT/e['file']).convert('RGBA'),e['sheet_row'])
        figures=[idle.crop(idle.getbbox()),*figures,idle.crop(idle.getbbox())]
        for n,im in enumerate(figures):
            b=im.getbbox();im=im.crop(b);h=200
            im=im.resize((round(im.width*h/im.height),h),Image.Resampling.LANCZOS)
            cell=Image.new('RGBA',(256,256));cell.alpha_composite(im,(128-im.width//2,230-im.height))
            k=direction*8+n;guide.alpha_composite(cell,(k%4*256,k//4*256));guide_tiles.append(cell)
    guide.save(SRC/'common-gait-reference.png')
    classes=['Bastion','Technomancien','Traqueuse','Arcaniste','Metamorphe'];jobs=[]
    identities=Image.new('RGB',(1280,1280),'#303a40');draw=ImageDraw.Draw(identities)
    for index in range(25):
        ident=Image.new('RGBA',(512,512))
        for direction,facing in enumerate([-1,1]):
            e=next(e for e in manifest if e['id']==f'CLASS-{index:02}' and e['facing']==facing and e['frame']==8)
            native=whole(Image.open(ROOT/e['file']).convert('RGBA'),e['sheet_row']).crop((28,24,100,100))
            big=native.resize((288,304),Image.Resampling.NEAREST)
            ident.alpha_composite(big,(direction*230-8,72))
        ref=SRC/f'CLASS-{index:02}-identity.png';ident.save(ref)
        thumb=ident.resize((256,256),Image.Resampling.NEAREST)
        identities.paste(thumb,(index%5*256,index//5*256),thumb)
        draw.text((index%5*256+8,index//5*256+8),f'{classes[index//5]} {index%5+1}',fill='white')
        prompt=f'''Use case: identity-preserve. Project RL whole-figure game walk cycle, {classes[index//5]} outfit {index%5+1}, CLASS-{index:02}.
INPUT1 is the mandatory COMMON GAIT pose master. Copy its exact sequence of body/leg poses, foot overlaps, bent swing knees and raised trailing boots. INPUT2 is the exact outfit identity, colors, head and current left/right projection. Apply ONLY INPUT2's appearance to the poses of INPUT1; do not copy INPUT1's orange armor or bulky body onto this character.
The goal is one consistent walk used by all25 outfits. This is NOT an idle leg-opening animation. In passing poses, the swing boot is OFF THE GROUND and passes UNDER the pelvis; the leg silhouettes OVERLAP/CROSS narrowly instead of spreading sideways. The supporting leg stays under the body's weight, the other knee bends and its heel lifts behind. Both anatomical legs take turns as the planted support. Copy master poses1/2/3 then4/5/6 as six ordered phases; no repeated leading leg or six variations of a wide standing stance. No splits, sideways leg spreading, running, big kicks, high knees or hopping.
Output exactly4 columns by4 rows,16 whole separate figures on true transparent alpha. Match INPUT1's layout: first2 rows face SCREEN LEFT, last2 rows face SCREEN RIGHT. Row-major cells0..7: true idleLEFT, left walk0,1,2,3,4,5, duplicate idleLEFT. Cells8..15: true idleRIGHT,right walk0,1,2,3,4,5,duplicate idleRIGHT. ALL head, torso, pelvis, knees and boots face the stated direction together, in near-front three quarters with both shoulders visible. No backs or pure profiles.
Preserve INPUT2's character identity, body build, proportions, skin, face/hair/helmet, exact outfit coverage, garment length, colors and large matte material masses. Maintain the SAME character scale and head size across ALL16 cells, same scale for both directions. Small grounded body bob only. Keep two complete relaxed arms and closed empty hands separated from thighs to attach existing weapons later. No weapon, tool, new item or added decoration. Arms must remain attached at the shoulders; no holes or extra limbs.
For Traqueuse preserve BOTH silver armored shins and the accepted cybernetic forearm; never swap limb materials. For Metamorphe preserve the olive/brown/ivory organic outfit, not grey robot armor. Robes follow the same leg cycle underneath with visible alternating boots; do not replace them by trousers.
Each figure is a complete coherent painting, not a grafted lower body. Crisp painted native64 game finish, a few broad matte facets, continuous controlled dark outlines and restrained highlights, no noise or microdetail. A uniform regular grid with generous transparent margins between cells and at all outside edges; every head and sole fully inside its cell. Keep whole figures at70percent cell height so feet cannot be cropped. No labels, text, grids, floor, shadow, effects or background.'''
        jobs.append({'id':f'CLASS-{index:02}','index':index,'label':f'{classes[index//5]} {index%5+1}/5','reference':str(ref),'gait_reference':str(SRC/'common-gait-reference.png'),'source':str(SRC/f'CLASS-{index:02}-common-r01.png'),'prompt':prompt})
    identities.save(DOC/'identity-review.png')
    (DOC/'jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf-8')
    (DOC/'authorization.json').write_text(json.dumps({'user_request':'fait le pour les personnages :)','clarification':"en faite ce qui m'ennuie c'est surtout que ces animations ne sont pas égales entre tous les personnages, elles different toutes un peu",'scope':'Five player classes,25 outfits, shared gait guide; preview before integration','normal_game_enabled':False},ensure_ascii=False,indent=2)+'\n','utf-8')
    print('25 references and prompts prepared from one gait guide')


if __name__=='__main__':main()
