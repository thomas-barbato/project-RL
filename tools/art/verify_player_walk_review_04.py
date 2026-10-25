"""Verify packaged poses and emit native contact sheets for human art review."""
import json,math,hashlib
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_review_04 import ROOT,ART,DOC,SITE,review_front,review_record

def main():
    data=json.loads((DOC/'baseline-data.json').read_text('utf-8'));records={r['id']:r for r in data['records']}
    package=json.loads((DOC/'packaging.json').read_text('utf-8'));entries={r['id']:r for r in package['entries']}
    jobs=json.loads((DOC/'jobs.json').read_text('utf-8'))+json.loads((DOC/'other-jobs.json').read_text('utf-8'))
    frontpaths={j['id']:j['fronts'] for j in jobs if 'fronts' in j};errors=[];checked=0;protected=0;boards=[]
    for key,e in entries.items():
        r=review_record(records[key]);side=e['size'];im=Image.open(ART/e['atlas']).convert('RGBA')
        if im.size!=(9*side,e['rows']*side):errors.append(key+' dimensions');continue
        for row in range(e['rows']):
            idle=im.crop((8*side,row*side,9*side,(row+1)*side));bbox=idle.getchannel('A').point(lambda a:255 if a>=64 else 0).getbbox()
            if not bbox:errors.append(key+f' empty idle {row}');continue
            equipped=row//3>0
            cut=round(r['foot'][1]-(r['foot'][1]-bbox[1])*(.17 if equipped else .33))
            for f in range(8):
                frame=im.crop((f*side,row*side,(f+1)*side,(row+1)*side));checked+=1
                if not frame.getbbox():errors.append(key+f' empty {row}/{f}')
                if r['rig'] not in ['float','serpent','roll']:
                    bob=round(-abs(math.sin(f/8*math.tau))*.8)
                    expected=Image.new('RGBA',(side,side));expected.alpha_composite(idle,(0,bob));box=(0,0,side,max(0,cut+bob-2))
                    if frame.crop(box).tobytes()!=expected.crop(box).tobytes():errors.append(key+f' altered upper body {row}/{f}')
                    protected+=1
            if row%3==1:
                cfg=row//3
                if key in frontpaths:source=Image.open(ROOT/frontpaths[key][cfg]).convert('RGBA')
                elif r.get('configs')=='legacy':
                    c=[0,1,2,4,9,11][cfg];source=Image.open(SITE/r['pack'][0]).convert('RGBA').crop((c*128,0,(c+1)*128,128))
                else:source=review_front(records[key])
                # RGB under alpha=0 is invisible and alpha_composite clears it.
                visible_equal=all(a==b or a[3]==b[3]==0 for a,b in zip(idle.getdata(),source.getdata()))
                if not visible_equal:errors.append(key+f' front source changed {cfg}')
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',12)
    # Native class comparisons: each class keeps all six loadouts and 3 views.
    for first in range(0,25,5):
        board=Image.new('RGB',(2160,900),'#263136');draw=ImageDraw.Draw(board)
        for i in range(5):
            key=f'CLASS-{first+i:02}';p=ART/f'{key}-native.png'
            if p.exists():board.paste(Image.open(p),(i*432,26))
            draw.text((i*432+8,8),records[key]['label'],font=font,fill='white')
        p=ART/f'QA-classes-{first//5+1:02}.png';board.save(p);boards.append(str(p.relative_to(ROOT)))
    # Other base models, L/front/R. No scaling of native pixels in these boards.
    actor_entries=[e for e in package['entries'] if not e['id'].startswith(('CLASS-','ARMOR-')) and 'derived_from' not in e]
    for first in range(0,len(actor_entries),24):
        subset=actor_entries[first:first+24];board=Image.new('RGB',(4*400,6*170),'#263136');draw=ImageDraw.Draw(board)
        for i,e in enumerate(subset):
            r=records[e['id']];side=e['size'];atlas=Image.open(ART/e['atlas']).convert('RGBA');x=i%4*400;y=i//4*170
            for d in range(3):
                pose=atlas.crop((8*side,d*side,9*side,(d+1)*side))
                # Large bodies get individual full-size native sheets.
                if side<=128:board.paste(pose,(x+d*128,y),pose)
                else:draw.text((x+d*128+4,y+55),'Grand corps : planche native',font=font,fill='#e9b779')
            draw.text((x+4,y+133),e['id']+' · '+r['label'][:38],font=font,fill='white')
            draw.text((x+4,y+150),r['rigLabel'],font=font,fill='#b0b9b6')
        p=ART/f'QA-actors-{first//24+1:02}.png';board.save(p);boards.append(str(p.relative_to(ROOT)))
    result={'review':'WALK04-r01','entries_checked':len(entries),'expected_entries':len(records),'missing':sorted(set(records)-set(entries)),'animation_frames_checked':checked,'upper_body_comparisons':protected,'source_fronts_exact':not any('front source' in e for e in errors),'errors':errors,'contact_sheets':boards,'runtime_integration':False,'visual_approval':False}
    (DOC/'verification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),'utf-8')
    print(json.dumps({k:v for k,v in result.items() if k not in ['contact_sheets','missing','errors']}|{'error_count':len(errors),'first_errors':errors[:12]},ensure_ascii=False))
    if errors:raise SystemExit(1)

if __name__=='__main__':main()
