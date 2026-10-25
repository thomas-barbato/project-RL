"""Condition whole extra-creature proposals for the isolated WALK08 review."""
import json,hashlib,shutil,sys,math
from pathlib import Path
from PIL import Image,ImageDraw
from build_player_walk_matrix_06 import grid_figures
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-08'
DEST=ROOT/'assets/animations/walk-08/extras'
LEFT_ROWS={'FORM-PREDATOR':1}  # The authored Predator rows are right, then left.
def main():
    DEST.mkdir(exist_ok=True)
    jobs=json.loads((DOC/'extra-generation.json').read_text('utf-8-sig'));entries=[];proof=[]
    contact=Image.new('RGB',(768,768),'#303a40');draw=ImageDraw.Draw(contact)
    for i,j in enumerate(jobs):
        assert j.get('generated'),j
        source=Path(j['destination']);shutil.copyfile(j['generated'],source)
        figures,warnings=grid_figures(Image.open(source).convert('RGBA'),4,4)
        ref=Image.open(j['reference']).convert('RGBA');box=ref.getbbox()
        figures=[f.crop(f.getbbox()) for f in figures]
        scale=min((box[3]-box[1])/max(f.height for f in figures),110/max(f.width for f in figures))
        atlas=Image.new('RGBA',(896,256));poses=[]
        for n,k in enumerate([0,1,2,3,4,5,6,8,9,10,11,12,13,14]):
            f=figures[k];small=f.resize((round(f.width*scale),round(f.height*scale)),Image.Resampling.LANCZOS)
            tile=Image.new('RGBA',(128,128));tile.alpha_composite(small,(round(64-small.width/2),112-small.height));poses.append(tile)
            atlas.paste(tile,(n%7*128,n//7*128))
        bounds=[p.getbbox() for p in poses];union=[min(b[0] for b in bounds),min(b[1] for b in bounds),max(b[2] for b in bounds),max(b[3] for b in bounds)]
        offset=math.floor(112-32-(union[1]+union[3])/2+.5) if union[3]-union[1]<64 else 0
        path=DEST/(j['id']+'.png');atlas.save(path,optimize=True)
        entries.append({'id':j['id'],'file':path.relative_to(ROOT).as_posix(),'foot':[64,112],'vertical_offset':offset,'cycle_bounds':union,'left_row':LEFT_ROWS.get(j['id'],0)})
        proof.append({'id':j['id'],'source':source.relative_to(ROOT).as_posix(),'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'warnings':warnings,'uniform_scale':scale,'height_range':[min(b[3]-b[1] for b in bounds),max(b[3]-b[1] for b in bounds)]})
        x=(i%3)*256;y=(i//3)*256;draw.text((x+8,y+6),j['id'],fill='white')
        for side in range(2):
            tile=poses[side*7];crop=tile.crop((24,32,104,120)).resize((160,176),Image.Resampling.NEAREST)
            contact.paste(crop,(x+side*112-10,y+40),crop)
        board=Image.new('RGB',(1792,512),'#303a40')
        for n,p in enumerate(poses):
            q=p.resize((256,256),Image.Resampling.NEAREST);board.paste(q,(n%7*256,n//7*256),q)
        board.save(DOC/(j['id']+'-pair-x2.png'))
    (DEST/'manifest.json').write_text(json.dumps(entries,indent=2)+'\n')
    (DOC/'extra-conditioning.json').write_text(json.dumps(proof,indent=2)+'\n')
    contact.save(DOC/'extra-directions-overview.png')
    print(json.dumps({'proposed_cycles':len(entries),'warnings':[p for p in proof if p['warnings']]}))
if __name__=='__main__':main()
