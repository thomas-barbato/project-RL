"""EG-COMP-04 / EG-POP-04, actual native engine maps and local scenes."""
from html import escape
from html.parser import HTMLParser
from urllib.parse import urlsplit,unquote
import shutil
from PIL import Image
from build_sewer_review_04 import ROOT,ART,DOC,OUT,SEEDS,read,write,rel,sha,habitat

def publish():
    habitat.accepted_unchanged();proposal=read(DOC/'proposal.json');engine=read(OUT/'engine/checks.json')
    assert engine['review']=='EG-COMP-04' and not engine['resurrection_ai_active']
    native=OUT/'native';native.mkdir(exist_ok=True)
    latest=max(f.stat().st_mtime for f in ART.rglob('*.png'))
    for row in engine['scenes']:
        label=row['id'];path=OUT/'engine'/f'{label}.png'
        assert path.stat().st_mtime>=latest,('Stale capture',label)
        with Image.open(path) as im:im.convert('RGB').crop((334,50,1552,894)).save(native/f'{label}.png')
    for i in (1,2,3):
        for kind in ('scenery','map'):
            with Image.open(OUT/'engine'/f'seed-{i}-{kind}.png') as im:
                assert im.size==(3008,2816)
    css=(ROOT/'artifacts/sewer-review-03/index.html').read_text(encoding='utf8').split('<style>')[1].split('</style>')[0]
    css+=' .cards{display:flex;flex-wrap:wrap;gap:18px}.card{min-width:260px;padding:16px;border:1px solid #50605a;background:#25312c}.card img{display:block;margin:12px auto;max-width:none}.compare{display:flex;gap:10px;flex-wrap:wrap;margin:12px 0}.overview{max-width:1150px}.metric{padding:10px 16px;background:#24332f}ol li{margin:9px 0}'
    prefix='../sewer-review-04/'
    def scene(label,caption):
        return f'<figure><figcaption>{escape(caption)}</figcaption><div class="native"><img loading="lazy" src="{prefix}native/{label}.png" width="1218" height="844" alt="{escape(caption)}"></div></figure>'
    html=f'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Égouts · composition et nombreuses meutes</title><style>{css}</style><main>'
    html+='''<h1>EG-COMP-04 / EG-POP-04 · Des égouts occupés</h1>
<p class="notice"><strong>Le socle EG01 et les décors EG-DEC-02 restent validés.</strong> La nouvelle proposition reprend leur architecture et leur réseau mural. Les habitats EG-HAB-03 sont retravaillés dans cette composition, avec trois nouveaux dessins.</p>
<p class="proposal"><strong>Deux aspects à examiner : le décor et le peuplement.</strong> Les cartes sont plus meublées selon la fonction des salles, puis remplies de meutes plus nombreuses. Les boutons permettent de voir les décors seuls, d’ajouter les monstres ou de revoir l’ancien habillage. Ces changements restent dans l’aperçu.</p>
<nav><a href="#maps">Cartes entières</a><a href="#native">Vues natives</a><a href="#population">Groupes de monstres</a><a href="#kit">Dessins</a></nav>
<h2 id="maps">Une fonction et une composition par salle</h2>
<p>Les refuges sont regroupés dans une alcôve. Les réserves ont des piles de matériel et une allée d’accès ; les ateliers réunissent établis et rayonnages. Les berges accueillent équipements hydrauliques, barils et dépôts près de l’eau. Les passages et les espaces de combat restent ouverts.</p><div class="tabs">'''
    for i,seed in enumerate(SEEDS,1):html+=f'<button data-tab="{i}" aria-pressed="{str(i==1).lower()}">Carte {i} · {seed}</button>'
    html+='</div>'
    for i,row in enumerate(proposal['seeds'],1):
        pop=row['population']
        html+=f'<section data-panel="{i}"'+(' hidden' if i!=1 else '')+f'><div class="facts"><span class="metric">{pop["original_count"]} → {pop["proposed_count"]} monstres</span><span class="metric">{pop["groups"]} groupes</span><span class="metric">{row["new_scenery"]} décors ajoutés au lot EG-DEC-02</span></div>'
        html+='<div class="compare">'
        for mode,label in [('scenery','Décors seuls'),('map','Décors et monstres'),('before','Avant cette reprise')]:
            path=f'{prefix}engine/seed-{i}-{mode}.png' if mode!='before' else f'../sewer-review-03/engine/seed-{i}-map.png'
            html+=f'<button data-map="{i}" data-src="{path}" data-caption="{escape(label)}" aria-pressed="{str(mode=="scenery").lower()}">{label}</button>'
        html+=f'</div><figure><figcaption id="map-caption-{i}">Décors seuls · vue d’ensemble réduite</figcaption><div class="overview"><img id="map-image-{i}" loading="lazy" src="{prefix}engine/seed-{i}-scenery.png" width="3008" height="2816" alt="Carte {i} · décors et espaces de combat"></div></figure>'
        html+=f'<p><a id="map-link-{i}" href="{prefix}engine/seed-{i}-scenery.png">Ouvrir cette vue entière à taille native</a></p>'
        html+='</section>'
    html+='''<p class="small">Les cartes entières sont des exports du moteur. La vue avec monstres montre volontairement tout le peuplement pour la revue ; la vision réelle du joueur garde ses règles habituelles.</p>
<h2 id="native">La composition avec le personnage, à taille réelle</h2><p>Captures locales du moteur, un pixel d’image pour un pixel de jeu. Les objets gardent leur échelle ; les vues d’ensemble ci-dessus sont seulement réduites pour la lecture.</p>'''
    for identifier,caption in [('refuge','Alcôve de refuge · couchages et affaires regroupés'),('workshop','Atelier · rayonnages, établis et réserves'),('collector','Collecteur · équipements de berge et dépôts'),('store','Réserve · stockage cohérent et allée de passage'),('encounter','Rencontre · une meute plus nombreuse')]:
        html+=scene(f'seed-1-{identifier}',caption)
    html+='<details><summary>Voir les mêmes fonctions dans les autres agencements</summary>'
    for i in (2,3):
        html+=f'<h3>Carte {i}</h3>'
        for identifier,caption in [('refuge','Refuge'),('workshop','Atelier'),('encounter','Rencontre')]:html+=scene(f'seed-{i}-{identifier}',caption)
    html+='</details><h2 id="population">EG-POP-04 · Davantage de meutes</h2><p>'
    html+=f'Les exemples proposent <strong>{min(r["population"]["proposed_count"] for r in proposal["seeds"])} à {max(r["population"]["proposed_count"] for r in proposal["seeds"])} monstres</strong>, contre environ trente auparavant. Ils sont répartis en groupes de zombies et de cultistes, avec deux cultistes scientifiques dans certains groupes de zombies.</p>'
    html+='''<ol><li>Les meutes de zombies visent généralement 8 à 10 membres ; les groupes de cultistes 5 à 7. Les groupes supplémentaires restent plus petits.</li><li>Les grandes salles peuvent accueillir deux foyers séparés. Les meubles et les canaux guident les positions, avec des intervalles entre les rencontres.</li><li>Le départ garde une zone libre de six cases. Les nouveaux ennemis sont placés à au moins huit pas de trajet de l’arrivée.</li><li>Les profils ordinaires, les dégâts, les règles d’XP et le butin sont conservés pour cette proposition. Les chiffres et conditions de réanimation EG-NEC-02 attendent toujours une validation séparée.</li></ol>
<p class="small">Ces captures servent à juger la densité et le placement. La simulation n’est pas avancée pour la prise de vue ; l’équilibrage des combats reste une étape d’essai.</p>
<h2 id="kit">Trois dessins supplémentaires, montrés à taille de jeu</h2><div class="cards">'''
    for key,code,label in [('shelf','EG-C01','Rayonnage de service'),('pallet','EG-C02','Palette de réserve'),('runoff','EG-C03','Dépôt d’écoulement')]:
        w,h=Image.open(ART/'kit'/f'{key}-native.png').size
        html+=f'<div class="card"><strong>{code} · {label}</strong><p>{w} × {h} px · image native</p><img src="../../assets/source-art/sewer-review-04/kit/{key}-native.png" width="{w}" height="{h}" alt="{label}"></div>'
    html+='''</div><p>Les abris, couchages, affaires et barils EG-H01 à EG-H05 du précédent aperçu sont réemployés dans cette nouvelle disposition. Les écoulements et les grilles de sol sont des décors ; les canalisations principales restent intégrées aux murs.</p>
<p class="small">Contrôles : 100 générations déterministes, sol praticable relié, anciennes silhouettes et placements de corps conservés, réserves hors des passages protégés. Sources EG01/02 et carte normale inchangées.</p>
<p class="small">Outil imagegen intégré · <a href="../../assets/source-art/sewer-review-04/kit/">Sources des nouveaux dessins</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-04/prompts.json">Prompts</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-04/proposal.json">Proposition et nombres</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/sewer-review-02/acceptance.json">Décors EG-DEC-02 validés</a></p></main>
<script>document.querySelectorAll('[data-tab]').forEach(b=>b.onclick=()=>{document.querySelectorAll('[data-panel]').forEach(p=>p.hidden=p.dataset.panel!==b.dataset.tab);document.querySelectorAll('[data-tab]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});document.querySelectorAll('[data-map]').forEach(b=>b.onclick=()=>{const i=b.dataset.map;document.getElementById('map-image-'+i).src=b.dataset.src;document.getElementById('map-link-'+i).href=b.dataset.src;document.getElementById('map-caption-'+i).textContent=b.dataset.caption+' · vue d’ensemble réduite';document.querySelectorAll('[data-map="'+i+'"]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));});</script></html>'''
    class Refs(HTMLParser):
        def __init__(self):super().__init__();self.refs=[]
        def handle_starttag(self,tag,attrs):self.refs.extend(v for k,v in attrs if k in ('href','src','data-src'))
    refs=Refs();refs.feed(html)
    for ref in refs.refs:
        url=urlsplit(ref)
        if not url.scheme and url.path:assert (OUT/unquote(url.path)).resolve().exists(),ref
    (OUT/'index.html').write_text(html,encoding='utf8')
    (ROOT/'artifacts/street-defects-review-01/index.html').write_text(html,encoding='utf8')
    proofs=DOC/'references';proofs.mkdir(exist_ok=True)
    for identifier in ('refuge','workshop','collector','store','encounter'):
        shutil.copy2(native/f'seed-1-{identifier}.png',proofs/f'seed-1-{identifier}.png')
    write(DOC/'checks.json',dict(review='EG-COMP-04',status='awaiting_composition_and_population_approval',engine=engine,
        native_engine_views=len(engine['scenes']),full_map_pairs=3,html_references_checked=len(refs.refs),
        audit_seeds=100,approved_sources_unchanged=True,normal_campaign_unchanged=True,
        assets_sha256={rel(f):sha(f) for f in ART.rglob('*') if f.is_file()},
        proofs_sha256={rel(f):sha(f) for f in proofs.glob('*.png')}))
    print('EG-COMP-04 / EG-POP-04 published: native scenery-only and populated maps, local engine views.')
    if (DOC/'integration.json').exists():
        from record_sewer_integration_04 import refresh_page
        refresh_page(OUT/'index.html')
        refresh_page(ROOT/'artifacts/street-defects-review-01/index.html')

if __name__=='__main__':publish()
