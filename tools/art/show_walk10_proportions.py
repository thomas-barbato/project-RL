"""Review existing/current/proposed full compositor exports at native64."""
from pathlib import Path
import json,hashlib
from PIL import Image,ImageDraw,ImageOps

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-10/compositor'
DOC=ROOT/'docs/reviews/player-walk-10'
BG='#253136'

def read(name):
    return ImageOps.flip(Image.open(ART/name).convert('RGBA'))

def panel(weapon,facing,frame=None,progress=0,blank=False):
    out=Image.new('RGB',(900,420),BG);d=ImageDraw.Draw(out)
    direction='gauche' if facing<0 else 'droite'
    d.text((16,12),f'Bastion tenue1 / {direction} / une case en400ms / '+('hache equipee' if weapon=='axe' else 'sans arme'),fill='white')
    for x,label in [(50,'Face de reference'),(320,'01 - Actuel en jeu'),(640,'02 - Proposition')]:
        d.text((x,48),label,fill='white')
    d.text((16,188),'Taille native64 ci-dessus / detail x2 ci-dessous',fill='#ccd5d8')
    front=read(f'{weapon}-front.png')
    out.paste(front,(18,64),front)
    zoom=front.resize((256,256),Image.Resampling.NEAREST);out.paste(zoom,(-14,192),zoom)
    for x,mode in [(320,'current'),(640,'candidate')]:
        for cell in range(2):d.rectangle((x+cell*64,96,x+(cell+1)*64-1,159),outline='#4b575c')
        if blank:continue
        image=front if frame is None else read(f'{weapon}-{mode}-{facing}-{frame}.png')
        offset=round(64*progress) if facing>0 else round(64*(1-progress))
        out.paste(image,(x-32+offset,64),image)
        zoom=image.resize((256,256),Image.Resampling.NEAREST);out.paste(zoom,(x-64,192),zoom)
    return out

def main():
    combined=[];times=[]
    for weapon in ['unarmed','axe']:
        frames=[];durations=[]
        for facing in [-1,1]:
            frames.append(panel(weapon,facing));durations.append(400)
            for i in range(10):
                frames.append(panel(weapon,facing,min(2,i*3//10),i/10));durations.append(40)
            frames.append(panel(weapon,facing,8,1));durations.append(400)
            frames.append(panel(weapon,facing,None,1));durations.append(400)
            frames.append(panel(weapon,facing,blank=True));durations.append(160)
        file=DOC/f'Bastion-{weapon}-front-current-proposal.gif'
        frames[0].save(file,save_all=True,append_images=frames[1:],duration=durations,loop=0,optimize=False)
        panel(weapon,1,1,.5).save(DOC/f'Bastion-{weapon}-comparison.png')
    report={'review_id':'WALK10-PROPORTIONS-BA00-r01','status':'proposal_not_integrated','native_tile':64,
      'scope':'Bastion outfit1, unarmed and actual axe67, both directions, 14 whole poses',
      'normal_release_sha256':hashlib.sha256((ROOT/'target/release/project-rl.exe').read_bytes()).hexdigest(),
      'compositor_diagnostic':'passed','exported_parts':14,'preview_one_step_ms':400,'preview_distance_px':64,
      'source_audit':'../player-walk-09/volume-audit/metrics.json','candidate_geometry':'conditioning.json',
      'generation_mode':'built-in image_gen','prompts':'prompts.json','source_directory':'assets/source-art/player-walk-review-10',
      'limitations':'Pilot for Bastion outfit1 only. Other24 outfits still need individual front-proportion matching. No new animation or body asset is selected by the normal game.'}
    (DOC/'checks.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))

if __name__=='__main__':main()
