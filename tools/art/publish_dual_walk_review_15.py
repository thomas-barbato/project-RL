"""Frame native engine exports for DUAL15 review; never alter source drawings."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/dual-walk-review-15/r01'
ENGINE=OUT/'engine'
NAMES={0:'Bastion',4:'Bastion · armure lourde',10:'Traqueuse',14:'Traqueuse · armure lourde',15:'Arcaniste',20:'Métamorphe'}
LABELS={'pistols':'Deux pistolets','blades':'Deux armes de mêlée','mixed':'Lame + pistolet','mixed-reversed':'Pistolet + lame'}
FONT=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),15)
BOLD=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Bold.ttf'),18)
BG=(30,40,49)
ACCEPTED=(ROOT/'docs/reviews/dual-walk-15/acceptance.json').exists()


def publish_gif(checks,label,indices,filename,zoom=1):
    banks={}
    for index in indices:
        case=next(r for r in checks['records'] if r['index']==index and r['loadout']==label and r['lighting'])
        banks[index]={r['proposed']:Image.open(ENGINE/r['file']).convert('RGB') for r in case['variants']}
        if checks['exported_pngs_need_vertical_flip']:
            banks[index]={key:im.transpose(Image.Transpose.FLIP_TOP_BOTTOM) for key,im in banks[index].items()}
    cell_width=192*zoom;row_height=134*zoom
    width=156+len(indices)*cell_width;height=104+4*row_height
    frames=[];durations=[]
    for step in range(2):
        for tick in range(13):
            moving=tick<12;progress=(tick+1)/12 if moving else 1
            phase=min(tick//4,2)+3*step
            board=Image.new('RGB',(width,height),BG);d=ImageDraw.Draw(board)
            d.text((16,10),f'01 · {LABELS[label]} · DUAL15-r01',font=BOLD,fill='#ecf1f5')
            d.text((16,38),f'{"Taille native 64 px" if zoom==1 else f"Détail ×{zoom}"} · 1 case en 400 ms · vraie pose de face à l’arrivée',font=FONT,fill='#c2cdd5')
            for col,index in enumerate(indices):
                d.text((156+col*cell_width+12,72),NAMES[index],font=FONT,fill='#ecf1f5')
            for row,(facing,proposed) in enumerate([(-1,False),(-1,True),(1,False),(1,True)]):
                top=104+row*row_height
                d.text((16,top+30),('Gauche' if facing<0 else 'Droite')+' · '+('01' if proposed else 'actuel'),font=FONT,fill='#8bdbff' if proposed else '#c2cdd5')
                for col,index in enumerate(indices):
                    bank=banks[index][proposed]
                    n=1+(6 if facing>0 else 0)+phase if moving else 0
                    sprite=bank.crop((n*128,0,(n+1)*128,128))
                    if zoom!=1:sprite=sprite.resize((128*zoom,128*zoom),Image.Resampling.NEAREST)
                    dx=round(facing*(progress-.5)*64*zoom) if moving else 0
                    x=156+col*cell_width+32*zoom+dx
                    board.paste(sprite,(x,top))
            if step==0 and tick==6:board.save(OUT/filename.replace('.gif','.png'))
            frames.append(board.quantize(colors=256))
            durations.append((40 if tick%3==0 else 30) if moving else 600)
    frames[0].save(OUT/filename,save_all=True,append_images=frames[1:],duration=durations,loop=0,disposal=2,optimize=False)
    return {'path':filename,'sha256':hashlib.sha256((OUT/filename).read_bytes()).hexdigest(),'native_cell_px':64,'presentation_zoom':zoom,'engine_pixels_resampled':zoom!=1}


def main():
    checks=json.loads((ENGINE/'checks.json').read_text(encoding='utf-8'))
    assert checks['restored_normal_pixels_after_every_comparison']
    results=[]
    for label in LABELS:
        results.append(publish_gif(checks,label,[0,10,15,20],f'{label}-natif64.gif'))
    results.append(publish_gif(checks,'mixed',[4,14],'armures-lourdes-natif64.gif'))
    results.append(publish_gif(checks,'pistols',[10],'pistols-detail-x3.gif',zoom=3))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>01 · Marche avec deux armes</title>
<style>body{margin:24px;background:#131c24;color:#edf1f5;font:16px system-ui}p{max-width:90ch;line-height:1.6}button{padding:10px 14px;margin:4px;background:#263744;color:#edf1f5;border:1px solid #526b7a;cursor:pointer}.view{overflow:auto}img{display:block;max-width:none}</style>
<h1>01 · Marche avec deux armes</h1><p>Comparaison capturée par le moteur, à taille native de 64 px. « Actuel » : WALK12 / ARM14 dans le jeu normal. « 01 » : essai avec port plus bas des pistolets, un léger balancement des deux bras et le bras éloigné derrière le torse. Chaque main reste liée à son arme. La démarche, les dessins d'armes, les armures et le retour de face sont conservés. Les pauses de lecture ne changent pas la vitesse du jeu.</p>
<p>Cette proposition reste en aperçu avant validation. Les quatre classes sont montrées dans leur tenue de départ ; le comparatif d'armures ajoute le Bastion et la Traqueuse en tenue lourde. Les deux éclairages du diagnostic sont contrôlés ; cette page montre le mode éclairé.</p>'''
    if ACCEPTED:
        html=html.replace('WALK12 / ARM14 dans le jeu normal', 'ancien rendu WALK12 / ARM14').replace('essai avec port plus bas', 'rendu validé avec port plus bas').replace('Cette proposition reste en aperçu avant validation.', 'La version 01 est validée et intégrée au jeu normal.')
    for result in results:
        name=result['path'];title=LABELS.get(name.removesuffix('-natif64.gif'),{'armures-lourdes-natif64.gif':'Armures lourdes','pistols-detail-x3.gif':'Détail pistolets ×3'}.get(name,name))
        html+=f'<button onclick="document.getElementById(\'view\').src=\'{name}\'">{title}</button>'
    html+='<div class="view"><img id="view" src="mixed-natif64.gif" alt="Animation actuelle et proposition 01"></div><p>Pixels natifs conservés. Seul le bouton de détail utilise un zoom entier ×3.</p></html>'
    (OUT/'apercu.html').write_text(html,encoding='utf-8')
    (OUT.parent/'apercu.html').write_text('<!doctype html><meta charset="utf-8"><meta http-equiv="refresh" content="0;url=r01/apercu.html"><a href="r01/apercu.html">Aperçu courant DUAL15</a>',encoding='utf-8')
    (OUT/'publication.json').write_text(json.dumps({'review':'DUAL15-r01','status':'accepted_and_integrated' if ACCEPTED else 'awaiting_visual_approval','engine_capture':'engine/checks.json','outputs':results},indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'comparisons':len(results),'preview':str(OUT/'apercu.html')}))


if __name__=='__main__':main()
