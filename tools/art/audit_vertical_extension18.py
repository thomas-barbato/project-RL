"""Native contact boards and geometric coverage, without claiming art approval."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18/extension-C06'
OUT=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06'

def main():
    d=json.loads((ART/'conditioned-monsters/manifest.json').read_text('utf8'))
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    checks=[]
    for start in range(0,len(d['profiles']),16):
        group=d['profiles'][start:start+16]
        board=Image.new('RGB',(768,140*((len(group)+1)//2)),'#1e2831');dr=ImageDraw.Draw(board)
        for k,e in enumerate(group):
            x=k%2*384;y=k//2*140;dr.text((x+2,y+2),f'MON-{e["number"]:03} / '+e['name'],font=font,fill='white')
            s=e['side'];sheet=Image.open(ROOT/e['file']).convert('RGBA');foot=e['foot']
            images=[Image.open(ART/f'references/MON-{e["number"]:03}-front.png').convert('RGBA'),sheet.crop((s,0,s*2,s)),sheet.crop((s*4,s,s*5,s*2))]
            for col,im in enumerate(images):
                if col:im=im.crop((foot[0]-64,foot[1]-112,foot[0]+64,foot[1]+8))
                else:
                    a=im.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox();cx=(a[0]+a[2])//2
                    im=im.crop((cx-64,a[3]-112,cx+64,a[3]+8))
                board.paste(im,(x+col*128,y+18),im)
            heights=[f['bounds'][3]-f['bounds'][1] for f in e['frames']]
            checks.append({'number':e['number'],'height_range': [min(heights),max(heights)],'reference_height':e['reference_height'],'size_policy':e['size_policy'],'width_check':e['size_check'],'height_ratio_range':[round(min(heights)/e['reference_height'],3),round(max(heights)/e['reference_height'],3)]})
        board.save(OUT/f'monsters-audit-{start//16:02}.png')
    result={'review':'VERT18-C06','conditioned_models':len(d['profiles']),'conditioning_errors':d['conditioning_errors'],'size_flags':[e for e in checks if e['width_check']=='width_drift' or e['height_ratio_range'][0]<.85 or e['height_ratio_range'][1]>1.15],'models':checks,'visual_approval':False}
    (OUT/'monster-geometry-audit.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({k:v for k,v in result.items() if k!='models'}))

if __name__=='__main__':main()
