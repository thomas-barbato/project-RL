"""Publish the current habitat proposal using native engine captures."""
from html import escape
from html.parser import HTMLParser
from urllib.parse import urlsplit,unquote
import shutil
from PIL import Image,ImageDraw,ImageFont
from build_sewer_review_03 import ROOT,ART,DOC,OUT,SEEDS,read,write,rel,sha,accepted_unchanged

def publish():
    accepted_unchanged();proposal=read(DOC/'proposal.json');engine=read(OUT/'engine/checks.json')
    assert engine['review']=='EG-HAB-03' and not engine['resurrection_ai_active']
    native=OUT/'native';native.mkdir(exist_ok=True)
    latest=max(f.stat().st_mtime for f in ART.rglob('*.png'))
    labels=[s['id'] for s in engine['scenes']]
    for label in labels:
        path=OUT/'engine'/f'{label}.png';assert path.stat().st_mtime>=latest,('Stale capture',label)
        with Image.open(path) as im:
            assert im.size[0]>=1552 and im.size[1]>=894
            im.convert('RGB').crop((334,50,1552,894)).save(native/f'{label}.png')
    board=Image.new('RGBA',(940,220),'#2a3431');d=ImageDraw.Draw(board)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
    rows=[('shelter','EG-H01 · Bâche rapiécée'),('canopy','EG-H02 · Abri de tôle'),
          ('bedroll','EG-H03 · Couchage'),('belongings','EG-H04 · Affaires'),('barrels','EG-H05 · Barils')]
    for x,(key,label) in zip((12,194,376,558,740),rows):
        im=Image.open(ART/'kit'/f'{key}-native.png').convert('RGBA')
        board.alpha_composite(im,(x+(170-im.width)//2,152-im.height))
        d.text((x+3,170),label,font=font,fill='#d8c8aa')
    actor=Image.open(ROOT/'assets/sprites/art-finish-01/FIN-01.png').convert('RGBA')
    # Dedicated scale board: native human sheet and unscaled lying mattress.
    scale=Image.new('RGBA',(550,175),'#303b36');ds=ImageDraw.Draw(scale)
    scale.alpha_composite(actor,(25,0))
    bed=Image.open(ART/'kit/bedroll-native.png').convert('RGBA');scale.alpha_composite(bed,(203,28))
    tent=Image.open(ART/'kit/shelter-native.png').convert('RGBA');scale.alpha_composite(tent,(340,10))
    for x,label in [(35,'Joueur'),(190,'Matelas : 66 px'),(337,'Abri : 2 × 2 cases')]:ds.text((x,142),label,font=font,fill='#d8c8aa')
    board.save(native/'kit-board.png');scale.save(native/'scale-board.png')
    prefix='../sewer-review-03/'
    def scene(label,caption):
        return f'<figure><figcaption>{escape(caption)}</figcaption><div class="native"><img loading="lazy" src="{prefix}native/{label}.png" width="1218" height="844" alt="{escape(caption)}"></div></figure>'
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Égouts · habitats de fortune</title>
<style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#101719;color:#d4d9d4;font:16px/1.6 system-ui}main{max-width:1280px;margin:auto}h1,h2,h3{color:#dbc7a0}h1{font-size:27px}h2{font-size:22px;margin-top:36px}h3{font-size:18px}a{color:#add2d8}nav,.tabs,.facts{display:flex;gap:18px;flex-wrap:wrap}.notice,.proposal{padding:14px 18px;background:#23302c;border-left:3px solid #8fa385}.proposal{background:#282a25;border-color:#bfa368}figure{margin:20px 0 30px}figcaption{color:#bbc4bf;margin-bottom:8px}.native{overflow:auto;max-width:100%;border:1px solid #4a5959;background:#101719}.native img{display:block;max-width:none}.facts{color:#dfcfae;margin:14px 0}button{padding:9px 18px;background:#273638;border:1px solid #687976;color:#e5d8ba;font:inherit;cursor:pointer}button[aria-pressed=true]{background:#4b574e;border-color:#d3bf87}[hidden]{display:none!important}.tabs{margin:20px 0}.overview{max-width:950px;border:1px solid #4a5959}.overview img{display:block;width:100%;height:auto}.small{font-size:14px;color:#b5c0b8}details{margin:18px 0}summary{cursor:pointer;color:#dac9a8}</style><main>
<h1>EG-HAB-03 · Des refuges dans les égouts</h1>
<p class="notice"><strong>EG01 et les décors EG-DEC-02 sont validés.</strong> Les conduites intégrées aux murs, leurs coins intérieurs et extérieurs, les équipements, les rats et les matériaux acceptés sont conservés.</p>
<p class="proposal"><strong>Nouveaux dessins à examiner sur la carte.</strong> Des abris bricolés et des couchages occupent quelques coins secs. Les objets restent regroupés, avec des ouvertures vers l’espace libre. Cet aperçu n’active pas encore le donjon dans la campagne.</p>
<nav><a href="#habitats">Voir les refuges</a><a href="#kit">Dessins et échelle</a><a href="#map">Placement d’ensemble</a><a href="#support">Réanimation</a></nav>
<h2 id="habitats">Trois coins habités, sans répétition systématique</h2>
<p>Une bâche rafistolée, un abri de tôle récupérée et un simple coin de couchage. Barils, couvertures et affaires personnelles accompagnent ces refuges. Les salles techniques et les canaux gardent leur fonction.</p><div class="tabs">'''
    for i,seed in enumerate(SEEDS,1):html+=f'<button data-tab="{i}" aria-pressed="{str(i==1).lower()}">Génération {i} · {seed}</button>'
    html+='</div>'
    captions=['Bâche rapiécée, affaires récupérées et barils','Abri de tôle et couchage voisin','Matelas et petite réserve personnelle']
    for i,row in enumerate(proposal['seeds'],1):
        html+=f'<section data-panel="{i}"'+(' hidden' if i!=1 else '')+f'><div class="facts"><span>{row["habitats"]} refuges</span><span>{row["new_props"]} objets supplémentaires</span><span>64 px par case</span></div>'
        for j,caption in enumerate(captions,1):html+=scene(f'seed-{i}-habitat-{j}',f'{caption} · capture réelle du moteur, taille native')
        html+='</section>'
    html+='''<h2 id="kit">EG-H01 à EG-H05 · Les objets complets</h2><p>Chaque dessin est une pièce isolée avec un vrai fond transparent. Les objets ci-dessous sont affichés à leur taille prévue dans le jeu.</p>'''
    html+=f'<div class="native"><img src="{prefix}native/kit-board.png" width="940" height="220" alt="Cinq dessins de refuges à taille native"></div>'
    html+='<p>Le matelas reste à l’échelle d’une personne : sa longueur visible ne dépasse pas 66 pixels. L’abri occupe deux cases sur deux ; les affaires et les barils restent de petits objets.</p>'
    html+=f'<div class="native"><img src="{prefix}native/scale-board.png" width="550" height="175" alt="Comparaison native du joueur, du matelas et de l’abri"></div>'
    html+='<h2 id="map">Répartition dans les trois agencements</h2><p>Les refuges se trouvent au bord des salles, hors des canaux, passerelles et couloirs. Les monstres existants conservent leurs positions. Les vues d’ensemble sont réduites pour la lecture ; les liens ouvrent les cartes entières à leur taille native.</p>'
    for i in range(1,4):
        html+=f'<details'+(' open' if i==1 else '')+f'><summary>Carte {i} · trois refuges</summary><div class="overview"><img loading="lazy" src="{prefix}engine/seed-{i}-map.png" width="3008" height="2816" alt="Répartition des refuges dans la carte {i}"></div><a href="{prefix}engine/seed-{i}-map.png">Voir la carte entière à taille native</a></details>'
    html+='<details><summary>Revoir les murs et le collecteur déjà validés</summary><div class="native"><img loading="lazy" src="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-02/references/seed-1-collector.png" width="1218" height="844" alt="Collecteur EG-DEC-02 validé"></div></details>'
    html+='''<h2 id="support">Cultiste réanimateur</h2><p>Le rôle du cultiste est conservé. Les chiffres et conditions de réanimation EG-NEC-02 attendent une validation séparée ; la capacité reste inactive dans ces captures.</p>
<p class="small">Contrôles : 100 générations déterministes, circulation reliée, aucun chevauchement entre les nouveaux objets et les décors existants, chemins principaux et trajets des rats préservés. Les sources EG01/02 sont contrôlées par leurs empreintes.</p>
<p class="small">Dessins réalisés avec l’outil imagegen intégré. <a href="../../assets/source-art/sewer-review-03/kit/">Sources et dessins natifs</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-03/prompts.json">Prompts</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-02/acceptance.json">Validation EG-DEC-02</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-02/resurrection-proposal.json">Règles de réanimation proposées</a></p></main>
<script>document.querySelectorAll('[data-tab]').forEach(b=>b.onclick=()=>{document.querySelectorAll('[data-panel]').forEach(p=>p.hidden=p.dataset.panel!==b.dataset.tab);document.querySelectorAll('[data-tab]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});</script></html>'''
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):self.refs.extend(v for k,v in attrs if k in ('href','src'))
    refs=Refs();refs.feed(html)
    for ref in refs.refs:
        url=urlsplit(ref)
        if not url.scheme and url.path:assert (OUT/unquote(url.path)).resolve().exists(),ref
    for path in [OUT/'index.html',ROOT/'artifacts/street-defects-review-01/index.html']:path.write_text(html,encoding='utf-8')
    proof=DOC/'references';proof.mkdir(exist_ok=True)
    for name in ['seed-1-habitat-1','seed-1-habitat-2','seed-1-habitat-3','kit-board','scale-board']:
        shutil.copy2(native/f'{name}.png',proof/f'{name}.png')
    write(DOC/'checks.json',dict(review='EG-HAB-03',status='awaiting_new_habitat_visual_review',
        engine=engine,accepted_EG01_and_EG_DEC02_sources_unchanged=True,
        generation_audit_seeds=proposal['audit_seeds'],native_captures=len(labels),
        html_local_references_checked=len(refs.refs),new_actor_or_mechanic=False,
        assets_sha256={rel(f):sha(f) for f in ART.rglob('*') if f.is_file()},
        proof_sha256={rel(f):sha(f) for f in proof.glob('*.png')}))
    print('EG-HAB-03 published with native engine previews and five complete assets.')

if __name__=='__main__':publish()
