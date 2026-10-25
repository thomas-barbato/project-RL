"""Lossless, self-contained review groups with simultaneous left/right lanes."""
import base64,gzip,hashlib,io,json,math
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from pack_walk07_preview import pack,save

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
ART=ROOT/'artifacts/player-walk-review-07'
read=lambda p:json.loads(p.read_text('utf8'))

def entries():
    rows=[]
    for lot in range(12,30):
        data=read(ART/f'preview-data/creatures-lot{lot:02}.json')
        atlas=Image.open(io.BytesIO(base64.b64decode(data['atlas'].split(',')[1]))).convert('RGBA')
        frames=json.loads(gzip.decompress(base64.b64decode(data['framesGzip'])))
        jobs=read(DOC/f'creature-jobs-lot{lot:02}.json')
        for n,(option,job) in enumerate(zip(data['options'],jobs)):
            poses=[]
            for commands in frames[0][n]:
                assert len(commands)==1
                sid,x,y=commands[0];ax,ay,w,h=data['rects'][sid]
                poses.append((atlas.crop((ax,ay,ax+w,ay+h)),x,y))
            rows.append({'option':option,'job':job,'poses':poses})
    priority=['MON-151-0','MON-159-0','MON-152-0','MON-135-0','MON-134-0','MON-121-0','MON-119-0','MON-123-0']
    rows.sort(key=lambda r:(priority.index(r['option']['key']) if r['option']['key'] in priority else len(priority),
                            int(r['job']['review_id'].split('M')[-1])))
    assert len(rows)==71 and len({r['option']['key'] for r in rows})==71
    return rows

def payload(rows):
    images=[];frames=[]
    for row in rows:
        poses=[]
        for im,x,y in row['poses']:
            poses.append([[len(images),x,y]]);images.append(im)
        frames.append(poses)
    return images,[frames]

def inspection(rows):
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
    for first in range(0,len(rows),10):
        batch=rows[first:first+10];board=Image.new('RGB',(1536,154*math.ceil(len(batch)/2)),'#293132');d=ImageDraw.Draw(board)
        for n,row in enumerate(batch):
            ox=n%2*768;oy=n//2*154;job=row['job']
            d.text((ox+3,oy+3),job['review_id']+' '+job['id']+' / G repos,2,5 | D repos,2,5',font=font,fill='white')
            for k,p in enumerate([0,2,5,7,9,12]):
                im,x,y=row['poses'][p];board.paste(im,(ox+k*128+x,oy+24+y),im)
        board.save(ART/f'direction-audit-{first//10+1:02}.png')

def main():
    rows=entries();inspection(rows)
    groups=[];current=[]
    # Keep every native RGBA value. Partition before the host's1MB ceiling;
    # the selection menu remains small enough to review each complete cycle.
    for row in rows:
        trial=[*current,row];images,_=payload(trial);encoded,_,_=pack(images)
        if current and len(encoded)*4/3+45_000>970_000:
            groups.append(current);current=[row]
        else:current=trial
    if current:groups.append(current)
    reports=[]
    for n,group in enumerate(groups,1):
        slug=f'creatures-review{n:02}';images,frames=payload(group)
        fingerprint=hashlib.sha256(';'.join(r['job']['source_destination'] for r in group).encode()).hexdigest()[:10]
        report=save(slug,f'Nouvel essai · groupe {n}/{len(groups)}',images,
                    [r['option'] for r in group],['En revue'],frames,
                    revision='bulk-opposite-whole-figures-r06-'+str(n)+'-'+fingerprint,
                    extra_data={'dualDirections':True})
        report['ids']=[r['option']['key'] for r in group]
        reports.append(report)
    (DOC/'bulk-review-groups.json').write_text(json.dumps({'status':'awaiting_player_review',
        'visual_acceptance':False,'runtime_integration':False,'identity_count':len(rows),
        'rgba_lossless':True,'simultaneous_opposite_directions':True,'groups':reports},ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'groups':len(groups),'identities':len(rows),'fragment_bytes':[r['fragment_bytes'] for r in reports]}))

if __name__=='__main__':main()
