"""Contact sheets of existing generated art; no drawing or runtime writes."""
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import json, shutil
BASE=Path(__file__).resolve().parent
ROOT=BASE.parents[1]
OUT=BASE/'comic-catalogue-review-03'
OLD=BASE/'comic-catalogue-review-01'
FONT=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
def sheet(entries,name):
    cols=5;w=250;h=250
    board=Image.new('RGB',(cols*w,((len(entries)+cols-1)//cols)*h),(16,23,31))
    draw=ImageDraw.Draw(board)
    for i,e in enumerate(entries):
        im=Image.open(ROOT/e['source_path']).convert('RGBA')
        b=im.getchannel('A').point(lambda a:255 if a>96 else 0).getbbox()
        im=im.crop(b);im.thumbnail((230,198),Image.Resampling.LANCZOS)
        x=i%cols*w;y=i//cols*h
        board.paste(im,(x+(w-im.width)//2,y+35+(198-im.height)//2),im)
        draw.text((x+8,y+2),e['review_id']+' · '+str(e.get('family','')),font=FONT,fill='white')
        draw.text((x+8,y+20),e['name'][:32],font=FONT,fill=(180,201,215))
    (OUT/'audit').mkdir(parents=True,exist_ok=True);board.save(OUT/'audit'/name)
def main():
    cat=json.loads((OLD/'catalogue.json').read_text(encoding='utf8'))
    print('keys',list(cat));entries=cat['entries']
    for j in range(0,len(entries),25):sheet(entries[j:j+25],f'sources-{j//25+1:02}.png')
    print('entries',len(entries),'sheets',(len(entries)+24)//25)
    if (OUT/'pilot.json').exists():
        records=json.loads((OUT/'pilot.json').read_text('utf-8'))['entries']
        for start in range(0,len(records),4):
            group=records[start:start+4];board=Image.new('RGB',(1175,370*len(group)),(14,21,29));d=ImageDraw.Draw(board)
            for row,e in enumerate(group):
                for col,q in enumerate(e['compositions']):
                    x=col*235;y=row*370;d.text((x+5,y+3),e['review_id']+' · '+q['name'],font=FONT,fill='white')
                    im=Image.open(OUT/q['normal']).convert('RGBA');im=im.crop(im.getbbox());im.thumbnail((220,190),Image.Resampling.LANCZOS)
                    board.paste(im,(x+(235-im.width)//2,y+35),im)
                    small=Image.open(OUT/q['display64']).convert('RGBA');board.paste(small,(x+53,y+230),small)
            board.save(OUT/'audit'/f'pilot-{start//4+1}.png')
if __name__=='__main__':main()
