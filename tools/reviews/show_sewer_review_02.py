"""Native engine review for denser sewer scenery and proposed zombie support."""
from html import escape
from html.parser import HTMLParser
from urllib.parse import urlsplit,unquote
from PIL import Image,ImageDraw,ImageFont
import build_sewer_review_01 as base
from build_sewer_review_02 import ROOT,ART,DOC,OUT,SEEDS,read,write,rel,sha,accepted_unchanged

def publish():
    accepted_unchanged();p=read(DOC/'proposal.json');engine=read(OUT/'engine/checks.json')
    assert engine['review']=='EG-DEC-02' and not engine['resurrection_ai_active']
    labels=['entrance-before','entrance-proposal']
    labels += [f'seed-{i}-{v}' for i in range(1,4) for v in ['access','collector','encounter','reanimator','maintenance','corners']]
    labels += [f'street-{i}-{v}' for i in (1,2) for v in ['north','central']]
    latest=max(f.stat().st_mtime for f in ART.rglob('*.png'))
    native=OUT/'native';native.mkdir(exist_ok=True)
    for label in labels:
        path=OUT/'engine'/f'{label}.png'
        assert path.stat().st_mtime>=latest,('Stale capture',label)
        with Image.open(path) as im:
            assert im.width>=1552 and im.height>=894,(label,im.size)
            im.convert('RGB').crop((334,50,1552,894)).save(native/f'{label}.png')
    frames=[]
    for i in range(12):
        with Image.open(OUT/'engine'/f'water-{i:02}.png') as im:
            frames.append(im.convert('RGB').crop((334,50,1552,762)))
    frames[0].save(native/'ambience.gif',save_all=True,append_images=frames[1:],duration=180,loop=0,optimize=True)
    # Existing native actor sheets retain the identical source scale and baseline.
    board=Image.new('RGBA',(610,175),'#28302e');d=ImageDraw.Draw(board)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
    d.line((18,118,592,118),fill='#8e9b90')
    actors=[('Joueur','assets/sprites/art-finish-01/FIN-01.png'),
            ('Cultiste · 132','assets/sprites/art-finish-completion-06/monsters/132-0.png'),
            ('Zombie','assets/sprites/art-finish-05/FIN-31.png')]
    for x,(label,file) in zip((105,305,505),actors):
        im=Image.open(ROOT/file).convert('RGBA');assert im.size==(128,128),(file,im.size)
        board.alpha_composite(im,(x-64,22));d.text((x-d.textlength(label,font=font)/2,135),label,font=font,fill='#d7dbd3')
    board.convert('RGB').save(native/'support-scale.png')
    prefix='../sewer-review-02/'
    def scene(label,caption):
        return f'<figure><figcaption>{escape(caption)}</figcaption><div class="native"><img loading="lazy" src="{prefix}native/{label}.png" width="1218" height="844" alt="{escape(caption)}"></div></figure>'
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Égouts · décors et réanimateur</title>
<style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#101719;color:#d4d9d4;font:16px/1.6 system-ui}main{max-width:1280px;margin:auto}h1,h2,h3{color:#dbc7a0}h1{font-size:27px}h2{font-size:22px;margin-top:36px}h3{font-size:18px}a{color:#add2d8}nav,.tabs,.facts{display:flex;gap:18px;flex-wrap:wrap}.notice{padding:14px 18px;background:#23302c;border-left:3px solid #8fa385}.proposal{padding:14px 18px;background:#282a25;border-left:3px solid #bfa368}figure{margin:20px 0 30px}figcaption{color:#bbc4bf;margin-bottom:8px}.native{overflow:auto;max-width:100%;border:1px solid #4a5959;background:#090d10}.native img{display:block;max-width:none}.facts{color:#dfcfae;margin:14px 0}button{padding:9px 18px;background:#273638;border:1px solid #687976;color:#e5d8ba;font:inherit;cursor:pointer}button[aria-pressed=true]{background:#4b574e;border-color:#d3bf87}[hidden]{display:none!important}.tabs{margin:20px 0}.kit{display:flex;gap:18px;flex-wrap:wrap}.card{padding:16px;background:#26312e;border:1px solid #4e6058;min-width:210px}.card img{display:block;margin:12px auto;max-width:none}.card p{font-size:14px;margin:0}.overview{max-width:950px;border:1px solid #4a5959}.overview img{display:block;width:100%;height:auto}.small{font-size:14px;color:#b5c0b8}li{margin:9px 0}details{margin:18px 0}summary{cursor:pointer;color:#dac9a8}</style><main>
<h1>Égouts · décors enrichis et cultiste réanimateur</h1>
<p class="notice"><strong>Le socle EG01 est validé.</strong> L’entrée sur le trottoir gauche, les dix salles, les couloirs, les matériaux, les passerelles et les petits rats sont conservés. Les sources acceptées sont figées.</p>
<p class="proposal"><strong>Nouvelle revue : EG-DEC-02 et EG-NEC-02.</strong> Les ajouts de décor et le placement du cultiste sont montrés ci-dessous. La réanimation n’est pas encore exécutée dans le jeu : ses règles proposées sont présentées séparément. La campagne normale n’est pas modifiée par cet aperçu.</p>
<nav><a href="#ambience">Ambiance enrichie</a><a href="#walls">Murs, conduites et coins</a><a href="#plans">Trois agencements</a><a href="#support">Cultiste et réanimation</a><a href="#kit">Objets complets</a><a href="#entry">Entrée validée</a></nav>
<h2 id="ambience">EG-DEC-02 · Des salles qui ont une fonction</h2>
<p>Pompes adossées aux murs des collecteurs, établis et réserves dans les pièces d’entretien. Les déchets s’accumulent près des parois et des équipements. Les gros objets restent hors des passerelles et des galeries ; les petits déchets plats sont franchissables.</p>'''
    html+=scene('seed-1-collector','Collecteur enrichi · image réelle du moteur à taille native')
    html+='''<details><summary>Comparer avec le collecteur validé avant ces ajouts</summary><div class="native"><img loading="lazy" src="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-01/references/seed-1-collector.png" width="1218" height="844" alt="Collecteur EG01 validé avant les nouveaux décors"></div></details>'''
    html+=f'<figure><figcaption>Eau et petits rats animés · extrait du moteur</figcaption><div class="native"><img id="motion-image" src="{prefix}native/ambience.gif" width="1218" height="712" alt="Ambiance des égouts enrichis"></div></figure>'
    html+=f'<button id="motion" aria-pressed="true" data-gif="{prefix}native/ambience.gif" data-still="{prefix}native/seed-1-collector.png">Mettre l’aperçu en pause</button>'
    html+='<h2 id="walls">EG-MUR-02 · Conduites intégrées et vrais raccords d’angle</h2><p>Les conduites principales suivent les murs et font partie de leur décor. Les vannes et les gros raccords sont assemblés à cette surface ; les pompes sont placées contre une paroi. Les modules droits, les coins intérieurs et extérieurs partagent les mêmes points de jonction. Les faces avant gardent leur orientation dans chaque angle.</p>'
    html+=scene('seed-1-corners','Raccords des murs et des conduites · résultat réel dans le moteur')
    html+='<h2 id="plans">Même architecture validée · trois résultats</h2><p>Les salles, les couloirs et les canaux sont inchangés. Les acteurs déjà présents gardent leurs cases. Le lot ajoute des équipements et des coins encombrés, puis propose deux groupes accompagnés d’un cultiste dans chacun de ces exemples.</p><div class="tabs">'
    for i,seed in enumerate(SEEDS,1):html+=f'<button data-tab="{i}" aria-pressed="{str(i==1).lower()}">Génération {i} · {seed}</button>'
    html+='</div>'
    for i,c in enumerate(p['seeds'],1):
        html+=f'<section data-panel="{i}"'+(' hidden' if i!=1 else '')+f'><div class="facts"><span>10 salles</span><span>11 liaisons</span><span>{c["added_scenery"]} décors ajoutés</span><span>{c["support_cultists"]} réanimateurs proposés</span></div>'
        html+=scene(f'seed-{i}-maintenance','Pièce d’entretien · établi, équipements et accès dégagés')
        if i!=1:html+=scene(f'seed-{i}-collector','Collecteur · pompage, canal et passerelle')
        html+=scene(f'seed-{i}-reanimator','Une meute accompagnée d’un cultiste · placement proposé, capacité inactive')
        if i!=1:html+='<details><summary>Examiner les coins et les raccords</summary>'+scene(f'seed-{i}-corners','Murs en angle · conduites raccordées dans le décor des parois')+'</details>'
        html+=f'<details><summary>Voir le plan d’ensemble</summary><p class="small">Vue technique réduite, sans masque de vision. Les vues locales ci-dessus restent à un pixel pour un pixel.</p><div class="overview"><img loading="lazy" src="{prefix}engine/seed-{i}-map.png" width="3008" height="2816" alt="Plan complet enrichi {i}"></div><a href="{prefix}engine/seed-{i}-map.png">Ouvrir la carte entière à taille native</a></details></section>'
    html+='''<h2 id="support">EG-NEC-02 · Le cultiste qui relève les zombies</h2>
<p>Le dessin scientifique 132 existe déjà et garde le nom <strong>Cultiste</strong>. Il accompagne certaines meutes en retrait. Les autres groupes de zombies restent simples, ce qui permet de varier les rencontres.</p>'''
    html+=f'<div class="native"><img src="{prefix}native/support-scale.png" width="610" height="175" alt="Joueur, cultiste scientifique 132 et zombie à la même échelle native"></div>'
    html+='<p class="proposal"><strong>Fonctionnement proposé, chiffres à valider.</strong> Le rôle est accepté en principe ; les conditions ci-dessous ne sont pas encore actives.</p><ul>'
    for rule in p['resurrection_rules']['proposed_rules']:html+='<li>'+escape(rule)+'</li>'
    html+='</ul><p class="small">Les statistiques ordinaires et la progression d’XP existantes ne sont pas rééquilibrées dans cette revue. Les captures vérifient le dessin et le placement ; elles ne constituent pas un essai de difficulté avec la résurrection.</p>'
    html+='<h2 id="kit">Nouveaux dessins · objets entiers à taille de jeu</h2><p>Chaque pièce est dessinée isolément avec son contour complet. Aucune découpe d’illustration de salle ; les emprises de collision sont définies séparément.</p><div class="kit">'
    for key,code,label in [('pump','EG-D01','Pompe de relevage'),('manifold','EG-D02','Vannes du module mural'),('workbench','EG-D03','Établi d’entretien'),('trash','EG-D04','Déchets de berge'),('litter','EG-D05','Petits débris dispersés')]:
        w,h=Image.open(ART/'kit'/f'{key}-native.png').size
        html+=f'<div class="card"><strong>{code} · {label}</strong><p>{w} × {h} px · taille native</p><img src="../../assets/source-art/sewer-review-02/kit/{key}-native.png" width="{w}" height="{h}" alt="{label}"></div>'
    html+='</div><h2 id="entry">EG-ENT-01 r02 · Entrée validée sur le trottoir gauche</h2>'
    html+=scene('entrance-proposal','Trappe en X 2, Y 14 · passage libre sur le côté')
    html+='''<p class="small">Contrôles : 300 générations déterministes, toutes les cases praticables reliées, acteurs hors des emprises des machines, couloirs et passerelles préservés. Les ressources du socle accepté et la carte normale sont contrôlées par leurs empreintes.</p>
<p class="small">Dessins créés avec l’outil imagegen intégré · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-02/prompts.json">Prompts et références</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-02/resurrection-proposal.json">Proposition de réanimation</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-01/acceptance.json">Validation du socle EG01</a></p></main>
<script>document.querySelectorAll('[data-tab]').forEach(b=>b.onclick=()=>{document.querySelectorAll('[data-panel]').forEach(p=>p.hidden=p.dataset.panel!==b.dataset.tab);document.querySelectorAll('[data-tab]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});const b=document.getElementById('motion'),im=document.getElementById('motion-image');b.onclick=()=>{const on=b.getAttribute('aria-pressed')!=='true';b.setAttribute('aria-pressed',String(on));b.textContent=on?'Mettre l’aperçu en pause':'Animer l’aperçu';im.src=on?b.dataset.gif:b.dataset.still;im.height=on?712:844;};if(matchMedia('(prefers-reduced-motion: reduce)').matches)b.click();</script></html>'''
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):self.refs.extend(v for k,v in attrs if k in ('href','src','data-gif','data-still'))
    refs=Refs();refs.feed(html)
    for ref in refs.refs:
        url=urlsplit(ref)
        if not url.scheme and url.path:assert (OUT/unquote(url.path)).resolve().is_file(),ref
    (OUT/'index.html').write_text(html,encoding='utf-8')
    (ROOT/'artifacts/street-defects-review-01/index.html').write_text(html,encoding='utf-8')
    proofs=DOC/'references';proofs.mkdir(exist_ok=True)
    for label in ('seed-1-collector','seed-1-maintenance','seed-1-reanimator','seed-1-corners','support-scale'):
        import shutil
        shutil.copy2(native/f'{label}.png',proofs/f'{label}.png')
    write(DOC/'checks.json',dict(review='EG-DEC-02',status='awaiting_additions_review',engine=engine,
        approved_base_unchanged=True,normal_map_unchanged=True,html_local_references_checked=len(refs.refs),
        new_art={f.name:sha(f) for f in (ART/'kit').glob('*.png')},
        proof_sha256={f.name:sha(f) for f in proofs.glob('*.png')}))
    print('EG-DEC-02 published: denser native scenery, proposed support cultist, resurrection rules separated; approved base unchanged.')

if __name__=='__main__':publish()
