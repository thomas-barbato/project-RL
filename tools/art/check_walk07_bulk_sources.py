"""Preflight completed bulk sources and build native-facing review contact sheets."""
import json,sys
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_matrix_06 import grid_figures

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
ART=ROOT/'artifacts/player-walk-review-07'
jobs=[j for p in sorted(DOC.glob('creature-jobs-lot*.json')) if int(p.stem.rsplit('lot',1)[1])>=12
      for j in json.loads(p.read_text('utf8'))]
rows=[];thumbnails=[]
for job in jobs:
    path=Path(job['source_destination'])
    if not path.exists(): continue
    im=Image.open(path).convert('RGBA');a=im.getchannel('A');w,h=im.size
    edges=[sum(v>=128 for v in a.crop(b).tobytes()) for b in [(0,0,1,h),(w-1,0,w,h),(0,0,w,1),(0,h-1,w,h)]]
    try:
        figures,warnings=grid_figures(im,7,2)
        count=len(figures)
        original=Image.open(DOC/'references'/f'{job["id"]}-native-current.png').convert('RGBA').crop(job['record']['bounds'])
        height=job['record']['bounds'][3]-job['record']['bounds'][1]
        scale=min(height/max(f.height for f in figures),124/max(f.width for f in figures))
        thumbnails.append((job,original,[figures[k].resize((round(figures[k].width*scale),round(figures[k].height*scale)),Image.Resampling.LANCZOS) for k in [0,7]]))
    except Exception as error:
        warnings=[str(error)];count=0
    rows.append({'id':job['id'],'review_id':job['review_id'],'figures':count,'warnings':warnings,'opaque_outer_edges':edges,
                 'technical_source_ready':count==14 and not warnings and not any(edges),
                 'orientation_and_anatomy_require_visual_review':True})

font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',15)
for first in range(0,len(thumbnails),18):
    group=thumbnails[first:first+18];board=Image.new('RGB',(1536,((len(group)+5)//6)*222),'#293132');draw=ImageDraw.Draw(board)
    for n,(job,original,figures) in enumerate(group):
        ox=(n%6)*256;oy=(n//6)*222
        draw.text((ox+3,oy+2),job['review_id'].split('-')[-1]+' / '+job['id'],font=font,fill='white')
        for col,figure in enumerate([original,*figures]):
            img=figure.resize((figure.width*2,figure.height*2),Image.Resampling.NEAREST)
            # Inspection sheet only: native pixels at x2, no modifications to sources.
            x=ox+col*85+(85-img.width)//2;y=oy+27+(190-img.height)
            board.paste(img,(x,y),img)
        draw.text((ox+2,oy+22),'actuel       rangée 1       rangée 2',font=font,fill='#ddd')
    board.save(ART/f'bulk-facing-{first//18+1:02}.png')
(DOC/'bulk-source-checks.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n','utf8')
print(json.dumps({'completed_sources':len(rows),'technically_ready':sum(r['technical_source_ready'] for r in rows),
                  'needs_repair':[r for r in rows if not r['technical_source_ready']],
                  'contact_sheets':(len(thumbnails)+17)//18}))
