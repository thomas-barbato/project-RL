"""Verify GPU silhouette masks and update the existing native camp gallery."""
from pathlib import Path
import hashlib
import json
import re
import numpy as np
from PIL import Image, ImageFilter

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/handcrafted-places-review-01/hover-outline-01'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def publish():
    checks=json.loads((OUT/'checks.json').read_text(encoding='utf-8'))
    assert checks['native_cell']==64 and not checks['outline_production_enabled']
    assert checks['npc_presentation_production_enabled']
    assert not (OUT/'stderr.log').read_text().strip()
    base=np.asarray(Image.open(OUT/'silhouettes-0.png').convert('RGB'))
    edged=np.asarray(Image.open(OUT/'silhouettes-1.png').convert('RGB'))
    expected=np.zeros(base.shape[:2],dtype=bool)
    expected_colors=np.zeros_like(base)
    samples=[]
    for sample in checks['masks']:
        i=sample['index']
        alpha=np.asarray(Image.open(OUT/f'mask-{i}.png').convert('RGBA'))[:,:,3]
        core=Image.fromarray(np.where(alpha>=26,255,0).astype('uint8'))
        stroke=(np.asarray(core.filter(ImageFilter.MaxFilter(5)))>0)&(np.asarray(core)==0)
        assert stroke.sum()>30, (i,'empty silhouette')
        stamp=Image.new('L',(base.shape[1],base.shape[0]))
        x,y,_=sample['placement']
        stamp.paste(Image.fromarray(stroke.astype('uint8')*255),(round(x),round(y)))
        mask=np.asarray(stamp)>0
        expected|=mask
        expected_colors[mask]=[70,238,114] if sample['tone']=='Friendly' else [250,65,53]
        samples.append(dict(index=i,stroke_pixels=int(mask.sum()),complete_mask=True))
    changed=np.any(base!=edged,axis=2)
    assert np.array_equal(changed,expected), dict(unexpected=int((changed&~expected).sum()),missing=int((expected&~changed).sum()))
    assert np.array_equal(edged[expected],expected_colors[expected]), 'Outline colors differ'
    assert np.array_equal(base[~expected],edged[~expected]), 'Body or ground modified outside the stroke'
    verification=dict(review='HOVER-OUTLINE-01',revision='r01',gpu_verified=True,exact_two_pixel_alpha_outline=True,body_pixels_unchanged=True,shadows_and_transparent_canvas_excluded=True,colors_verified=True,native_cell=64,samples=samples)
    (OUT/'verification.json').write_text(json.dumps(verification,indent=2)+'\n',encoding='utf-8')
    names={'officer':'01 · Officier — dialogue et mission','medical':'02 · Médecin — service','cultist':'03 · Ennemi — arme à deux mains','ally':'04 · Allié','large-demon':'05 · Grand démon'}
    cards=[]
    for scene in checks['scenes']:
        name=scene['name'];x,y,_,_=scene['cell']
        box=(round(x-160),round(y-224),round(x+224),round(y+96))
        frames=[Image.open(OUT/f'{name}-{p}.png').crop(box) for p in range(2)]
        frames[0].save(OUT/f'{name}-native.webp',save_all=True,append_images=frames[1:],duration=[1500,1500],loop=0,lossless=True)
        Image.open(OUT/f'{name}-2.png').crop(box).save(OUT/f'{name}-without.png')
        cards.append(f'<article style="width:384px"><h3>{names[name]}</h3><p>{"Vert" if scene["tone"]=="Friendly" else "Rouge"} · taille native</p><img width="384" height="320" src="hover-outline-01/{name}-native.webp" alt="Contour de la silhouette à taille native"><details><summary>Comparer sans contour</summary><img width="384" height="320" src="hover-outline-01/{name}-without.png" alt="Même scène sans contour"></details><p><a href="hover-outline-01/{name}-0.png">Capture complète du client</a></p></article>')
    page=OUT.parent/'index.html'
    text=page.read_text(encoding='utf-8')
    text=re.sub(r'<!-- target-outline:start -->.*?<!-- target-outline:end -->','',text,flags=re.S)
    text=re.sub(r'<!-- camp-npc-review:start -->.*?<!-- camp-npc-review:end -->','',text,flags=re.S)
    section='''<!-- target-outline:start --><section id="contours-r01" style="border:1px solid #89764f;padding:18px;margin:18px 0;background:#12171e"><h2>PNJ : ajouts conservés et intégrés</h2><p>Pictogrammes et respiration activés dans le jeu. L’officier porte aussi le « ! » de contact de mission à côté de la bulle. Les symboles reprennent le léger flottement existant ; les mouvements réduits les figent. Au-dessus des ennemis, seules les icônes de statuts sont autorisées : les anciens repères d’alerte et de perception sont retirés.</p><h2>HOVER-OUTLINE-01 r01 · contour au survol — à valider</h2><p>Trait de 2 px suivant le dessin complet, arme comprise. Vert pour alliés et PNJ ; rouge pour ennemis. Sans clic ni remplissage ; aucune cible cachée révélée. Les cinq scènes viennent du client réel en cases de 64 px, sans agrandissement. Les créatures sont des témoins isolés de revue, pas des ajouts au camp.</p><div style="display:flex;flex-wrap:wrap;gap:20px">'''+''.join(cards)+'''</div><p>Le nouveau contour reste à valider. Les services, dialogues et règles de jeu ne changent pas.</p></section><!-- target-outline:end -->'''
    page.write_text(text.replace('</header>','</header>'+section,1),encoding='utf-8')
    report=dict(review_id='HOVER-OUTLINE-01',revision='r01',date='2026-10-18',status='awaiting_visual_approval',production_enabled=False,gpu_checks=checks,gpu_verification=verification,capture_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('*.png'))})
    (DOC/'hover-outline-01.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(gallery=str(page),samples_verified=len(samples),scenes=5)))


if __name__=='__main__':publish()
