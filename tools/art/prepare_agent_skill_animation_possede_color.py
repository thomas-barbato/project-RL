"""Static recolour review: lossless native capture, mechanical layout only."""
import hashlib
import json
import re
from pathlib import Path
from PIL import Image, ImageDraw
import prepare_agent_skill_animation_review as base

ROOT=base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-46'
BEFORE=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-41'
raw=OUT/'native/ground-native.rgba'
assert raw.resolve().is_relative_to(OUT.resolve())
data=raw.read_bytes()
assert len(data)==1280*720*4
png=raw.with_suffix('.png')
Image.frombytes('RGBA',(1280,720),data).save(png,compress_level=1)
with Image.open(png) as im:
    assert im.tobytes()==data
    game_floor=im.crop((96,152,160,216)).convert('RGB')
raw.unlink()
board=Image.new('RGB',(700,490),'#0b1218')
d=ImageDraw.Draw(board)
for j,(path,title,tag) in enumerate([
    (BEFORE/'possede-64.png','Avant · violet validé','TR64-BODY01'),
    (OUT/'possede-64.png','36 · essai chair','TR64-BODY04')]):
    x=24+j*350
    d.text((x,14),title,font=base.font(19,True),fill='#e0e7e7')
    with Image.open(path) as im:
        zoom=im.resize((192,192),Image.Resampling.NEAREST)
        board.paste(zoom,(x+46,49),zoom)
        d.text((x,249),'Zoom ×3 du dessin 64 px',font=base.font(14),fill='#a5b9c5')
        for i,(background,label) in enumerate([('#111820','Sombre'),('#bfc3c1','Clair'),(None,'Sol du jeu')]):
            tx=x+7+i*100;ty=295
            tile=game_floor if background is None else Image.new('RGB',(64,64),background)
            board.paste(tile,(tx,ty));board.paste(im,(tx,ty),im)
            d.text((tx,371),label,font=base.font(13),fill='#a5b9c5')
    d.text((x,427),tag+' · natif 64 px',font=base.font(14),fill='#e0e7e7')
d.text((24,463),'Recoloration proposée · corps chair, os ivoire, yeux rouges',font=base.font(14),fill='#a5b9c5')
board.save(OUT/'comparison-native.png')
html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Possédé couleur chair</title><style>*{box-sizing:border-box}body{max-width:1340px;margin:auto;padding:24px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}.muted{color:#a5b9c5}p{line-height:1.5}a{color:#93c1e0}.scroll{overflow:auto}.native{display:block;max-width:none;width:1280px;height:720px}.board{display:block;width:700px;height:490px}.sources{display:flex;gap:24px;flex-wrap:wrap}.sources img{width:360px;height:360px;object-fit:contain;background:#101b23}</style><h1>36 · Possédé · essai couleur chair</h1><p class="muted">TR64-BODY04 · torse, bras et jambes couleur chair · os ivoire et yeux rouges conservés</p>
<p>Les essais gris et chair limitée aux jambes ont été écartés. Cette variante propose la même chair beige et brune sur le torse, les bras et les jambes, avec la pose et la silhouette du Possédé. Le nouveau corps attend votre accord visuel.</p>
<div class="scroll"><img class="board" src="comparison-native.png" alt="Avant et essai chair, en64px natifs et zoom×3"></div>
<h2>Sur le sol du jeu · capture native</h2><div class="scroll"><img class="native" src="native/ground-native.png" alt="Avant à gauche, essai chair à droite, avec les autres acteurs du jeu"></div>
<h2>Dessins sources</h2><div class="sources"><figure><figcaption>Avant · violet validé</figcaption><img src="../skill-animation-review-41/possede-source.png"></figure><figure><figcaption>Essai chair · proposé</figcaption><a href="possede-source.png"><img src="possede-source.png"></a></figure></div>
<p class="muted">Recoloration seule : aucune nouvelle règle, attaque ou animation. Les anciens fichiers sont conservés ; la nouvelle variante n’est pas intégrée à la campagne.</p>
<p>Dessin retouché avec imagegen intégré : <a href="imagegen-possede-whole-body-chair-prompt.txt">prompt exact</a> · <a href="drawing-pack.json">conditionnement</a> · <a href="review.json">fiche de revue</a>.</p></html>'''
(OUT/'index.html').write_text(html,encoding='utf-8')
base.dump(OUT/'review.json',{'batch':'ANIM-TR64-COLOR01','status':'awaiting_visual_review',
    'candidate':{'number':36,'id':'TR64','body_variant':'TR64-BODY04','name':'Possédé','palette':'chair beige/brune sur tout le corps, os ivoire, yeux rouges'},
    'previous_approved':'TR64-BODY01 / review41',
    'excluded_trials':[
        {'variant':'TR64-BODY02','palette':'charcoal','reason':'User requested flesh colour instead.'},
        {'variant':'TR64-BODY03','palette':'Flesh on formerly violet areas only; red upper body retained.','reason':'User clarified that both upper and lower body should use flesh colour.'}],
    'user_request':'non mais sur les 2 parties de son corps :)',
    'scope':'Whole upper and lower body flesh recoloured. Ivory bones and red eyes retained. Native direct64px body, no transition, own-action, mechanical or campaign changes.'})
links=sorted(set(re.findall(r'(?:href|src)="([^"]+)"',html)))
assert all((OUT/link).is_file() for link in links)
pack=json.loads((OUT/'drawing-pack.json').read_text(encoding='utf-8'))
receipt_root=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-45'
receipt=json.loads((receipt_root/'visual-acceptance-01.json').read_text(encoding='utf-8'))
assert all(hashlib.sha256((receipt_root/name).read_bytes()).hexdigest()==digest for name,digest in receipt['artifact_hashes'].items())
base.dump(OUT/'validation.json',{'native_gpu_capture':'passed','capture_size':[1280,720],
    'lossless_native_encoding':True,'native_body_size':[64,64],'no_32px_intermediate':True,
    'native_silhouette_iou':pack['native_silhouette_iou'],'violet_pixels_in_body':pack['violet_pixels_in_body'],
    'Abysse_approved_artifacts_unchanged':len(receipt['artifact_hashes']),
    'local_links':{'checked':len(links),'missing':0},'build_locked_debug':'passed',
    'visual_status':'awaiting player approval','campaign_integration':False,
    'not_run':['Full engine suite','Release build','Browser rendering; native GPU readback inspected.']})
(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
(OUT/'render-source.rs').write_bytes((ROOT/'src/agent_skill_lab_review/fire_painted/possede_color.rs').read_bytes())
print(json.dumps({'candidate':'TR64-BODY04','native_comparison':True,'links':len(links),'previous_Abysse_artifacts_unchanged':len(receipt['artifact_hashes'])}))
