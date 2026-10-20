"""One native visual review for every ART-QUALITY-01 recommendation."""
from pathlib import Path
import hashlib
import html
import json
import re
from PIL import Image, ImageDraw, ImageFont, ImageChops

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01'
ART = ROOT / 'assets/source-art/art-finish-review-01'
OUT = ROOT / 'artifacts/art-quality-audit-01'
GPU = OUT / 'finish01-engine'
VIEW = (334, 78, 1552, 782)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def font(size):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', size)


def board(file, rows, title, controls=False):
    raw = Image.open(GPU / f'{file}-gpu.png').convert('RGB')
    assert raw.size == (1240, 900) and len(raw.getcolors(raw.width * raw.height)) > 1000
    height = 900 if file == 'native' else 500
    result = Image.new('RGB', (1240, height), '#11191d')
    d = ImageDraw.Draw(result)
    d.text((24, 14), title, font=font(23), fill='#decca7')
    d.text((24, 48), '64 px par case · à gauche : actuel · à droite : candidat · aucun agrandissement des sprites', font=font(16), fill='#bdc9c4')
    same = 0
    for i, (label, identity) in enumerate(rows):
        x, y = 20 + i % 4 * 300, 80 + i // 4 * 200
        d.text((x, y + 2), label, font=font(16), fill='#decca7')
        for column, caption in enumerate(['Actuel', 'Candidat']):
            xx = x + column * 140
            src = (xx, y + 34, xx + 128, y + 162)
            dst = (xx, y + 44, xx + 128, y + 172)
            panel = raw.crop(src)
            result.paste(panel, dst[:2])
            assert ImageChops.difference(panel, result.crop(dst)).getbbox() is None
            d.text((xx + 4, y + 24), caption, font=font(12), fill='#a7b6b0')
            same += 1
        d.text((x, y + 177), identity, font=font(12), fill='#aebfb9')
    if controls:
        # Accepted corpse and current camp NPC are unchanged reference controls.
        old = Image.open(ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-quality-audit-01/native-renderer.png').convert('RGB')
        corpse = Image.open(ROOT / 'assets/maps/prison-street-expanded-01/decor/civilians-08/CIV-U01-04-native.png').convert('RGBA')
        for i, label in [(13, 'Référence · civil accepté'), (14, 'Référence · PNJ conservé')]:
            x, y = 20 + i % 4 * 300, 80 + i // 4 * 200
            d.text((x,y+2),label,font=font(16),fill='#b8c9bf')
            for column in range(2):
                xx=x+column*140
                panel=Image.new('RGBA',(128,128),(57,61,55,255))
                if i==13: panel.alpha_composite(corpse,(24,48))
                else: panel=old.crop((1036,316,1164,444)).convert('RGBA')
                result.paste(panel.convert('RGB'),(xx,y+44))
                d.text((xx+4,y+24),'Référence identique',font=font(12),fill='#a7b6b0')
            d.text((x,y+177),'Repère de matière et d’échelle',font=font(12),fill='#aebfb9')
    path = OUT / f'finish01-{file}-native.png'
    result.save(path)
    result.save(DOC / f'{file}-native.png')
    return same


def build():
    if (ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/review.json').exists() and (OUT / 'finish02-engine/checks.json').exists():
        from show_art_finish_review_02 import build as show_current
        show_current()
        return
    if (DOC / 'integration-checks.json').exists():
        from verify_art_finish_integration import show_accepted
        show_accepted()
        return
    review = json.loads((DOC / 'review.json').read_text(encoding='utf-8'))
    captures = json.loads((GPU / 'checks.json').read_text(encoding='utf-8'))
    assert captures['review'] == review['review'] == 'ART-FINISH-01'
    assert len(review['entries']) == captures['candidates'] == 13
    before = json.loads((DOC / 'production-before.json').read_text(encoding='utf-8'))
    for path, digest in before.items():
        assert sha(ROOT / path) == digest, path
    baseline = Image.open(ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-quality-audit-01/native-renderer.png').convert('RGB')
    default = Image.open(OUT / 'default-check/native-comparison.png').convert('RGB')
    assert ImageChops.difference(baseline,default).getbbox() is None
    rows=[(f'{e["code"]} · {e["label"]}',e['identity']) for e in review['entries']]
    preserved=board('native',rows,'ART-FINISH-01 · les treize candidats ensemble',True)
    hands=[('FIN-01 · sans arme','Bastion · corps de référence'),('FIN-01 · une main','Pistolet · corps + arme + doigts'),('FIN-01 · deux mains','Fusil · manches + arme + doigts'),
           ('FIN-02 · sans arme','Cerbère · corps de référence'),('FIN-02 · une main','Pistolet · corps + arme + doigts'),('FIN-02 · deux mains','Fusil · manches + arme + doigts')]
    preserved+=board('hands',hands,'Prises en main · mêmes corps, armes séparées')
    colors=[('FIN-09 · rouge','144 A'),('FIN-09 · rouge-orangé','144 C'),('FIN-09 · vert','144 D'),('FIN-09 · bleu','144 B'),
            ('FIN-10 · bleu sombre','145 A'),('FIN-10 · rouge-orangé','145 C'),('FIN-10 · vert','145 D')]
    preserved+=board('palettes',colors,'Palettes de matière · vêtements, cornes et griffes protégés')
    for name in ['house','street']:
        for suffix in ['before','candidate']:
            im=Image.open(GPU/f'{name}-{suffix}.png').crop(VIEW)
            im.save(OUT/f'finish01-{name}-{suffix}.png')
        a=Image.open(OUT/f'finish01-{name}-before.png').convert('RGB')
        b=Image.open(OUT/f'finish01-{name}-candidate.png').convert('RGB')
        assert ImageChops.difference(a,b).getbbox() is not None, name
    notes={
        'FIN-01':'Matières du joueur, volumes moins plats et contours moins uniformes.',
        'FIN-02':'Céramique ivoire mate, sous-combinaison sombre et usure localisée.',
        'FIN-03':'Reconstruction native : protection ocre et pantalon bleu ; casque fermé.',
        'FIN-04':'Reconstruction native : plaques composites bleues et articulations sombres.',
        'FIN-05':'Reconstruction native : vraie veste rembourrée, tissu usé et masque fermé.',
        'FIN-06':'Cultiste humain au fusil, capuche et masque conservés, canon droit.',
        'FIN-07':'Cultiste humain à la hache, manche cohérent et deux mains sur l’arme.',
        'FIN-08':'Chair du possédé plus naturelle ; pantalon distinct de la peau.',
        'FIN-09':'Chair et membranes moins lisses, silhouette ailée et masse conservées.',
        'FIN-10':'Peau et membranes bleu sombre mates ; palette limitée aux matières choisies.',
        'FIN-11':'Drone redessiné depuis une vraie source détaillée, sans agrandir le 32 px.',
        'FIN-12':'Finition du fusil, icône et couche portée physiquement plus petite.',
        'FIN-13':'Finition du pistolet, icône et couche portée physiquement plus petite.',
    }
    table=''.join(f'<tr><th>{e["code"]}<br><span>{html.escape(e["label"])}</span></th><td>{notes[e["code"]]}</td><td><select data-code="{e["code"]}"><option value="">À examiner</option><option>Garder</option><option>À corriger</option><option>Refuser</option></select></td></tr>' for e in review['entries'])
    text=r'''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Finition des acteurs · ART-FINISH-01</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav,.buttons{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}button,select{font:inherit;color:#ddd9cd;background:#263932;border:1px solid #718078;padding:8px 12px;cursor:pointer}button[aria-pressed=true]{background:#4b503a;border-color:#d5c39a}table{border-collapse:collapse;max-width:1240px;width:100%}td,th{padding:13px;border:1px solid #43514e;text-align:left;vertical-align:top}th{width:235px;color:#decca7}th span{font-weight:400;color:#bac9c2}section{margin:36px 0}.note{color:#aebfb8}textarea{display:block;box-sizing:border-box;width:min(100%,1120px);min-height:110px;background:#17231e;color:#e3e9df;border:1px solid #66776d;padding:12px;font:16px Segoe UI}</style>
<h1>Une finition commune pour les acteurs et les décors</h1>
<p><strong>ART-FINISH-01 · treize candidats · en attente de ton avis.</strong> Ce lot pilote couvre toutes les recommandations de l’audit : personnage, armure récente, trois anciennes armures, armes et prises, humains, démons et drone. Il sert à choisir la finition avant de l’étendre aux autres familles.</p>
<p>Les corps acceptés et le PNJ du camp restent les repères. Le jeu normal conserve ses dessins actuels. Les vues ci-dessous sont des captures du moteur ; les variantes et les prises restent des propositions visuelles, sans nouveau pouvoir ni changement de combat.</p>
<nav><a href="#all">Tous les candidats</a><a href="#hands">Prises en main</a><a href="#palettes">Couleurs</a><a href="#scene">Dans la carte</a><a href="#choices">Mes choix par code</a><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-quality-audit-01/AUDIT.md">Audit initial</a></nav>
<section id="all"><h2>Tout le lot à taille réelle</h2><p>Chaque paire montre l’actuel à gauche et le candidat à droite. Les sprites gardent leur échelle prévue ; une grande toile transparente n’agrandit pas le personnage. Navigateur à 100 % pour conserver l’échelle native.</p><div class="scroll"><img src="finish01-native-native.png" width="1240" height="900" alt="Treize candidats numérotés avec leurs versions actuelles, à la même échelle, et deux références conservées"></div></section>
<section id="hands"><h2>Le corps, les armes et les doigts restent séparés</h2><p>Deux tenues, chacune sans arme, avec un pistolet et avec un fusil à deux mains. Les manches de la pose à deux mains viennent d’un dessin de pose dédié ; la tête, le tronc et les jambes conservent le corps de référence. Les armes sont de vraies couches séparées, recouvertes par les doigts aux points de prise.</p><div class="scroll"><img src="finish01-hands-native.png" width="1240" height="500" alt="Six comparaisons avant et après des poses du Bastion et de Cerbère"></div></section>
<section id="palettes"><h2>Les variantes gardent leur identité</h2><p>Rouge-orangé, vert et bleu restent distinguables. Les masques sélectionnent la peau et les membranes ; les éléments protégés gardent leurs pixels. Les nouveaux masques et couleurs sont soumis ici à ton avis.</p><div class="scroll"><img src="finish01-palettes-native.png" width="1240" height="500" alt="Sept comparaisons des palettes des deux démons"></div></section>
<section id="scene"><h2>Dans les vraies lumières de la carte</h2><p>Même décor, mêmes lumières, mêmes positions et même horloge. Le personnage est montré avec la tenue et le pistolet de la revue dans les deux versions. Les acteurs supplémentaires de la rue sont des figurants d’aperçu ; ils ne sont pas ajoutés à la partie.</p>
<h3>Auprès des corps, dans le logement</h3><div class="buttons"><button data-scene="house" data-mode="before">Actuel</button><button data-scene="house" data-mode="candidate" aria-pressed="true">Candidat</button></div><div class="scroll"><img id="scene-house" src="finish01-house-candidate.png" width="1218" height="704" alt="Comparaison des matières du joueur et des corps dans le logement"></div>
<h3>Humain, possédé, démon et drone dans la rue</h3><div class="buttons"><button data-scene="street" data-mode="before">Actuel</button><button data-scene="street" data-mode="candidate" aria-pressed="true">Candidat</button></div><div class="scroll"><img id="scene-street" src="finish01-street-candidate.png" width="1218" height="704" alt="Acteurs candidats dans le rendu normal éclairé de la rue"></div></section>
<section id="choices"><h2>Tu peux choisir l’ensemble ou préciser les codes</h2><p>Ces choix sont un brouillon local pour préparer ton retour. Rien n’est intégré automatiquement.</p><table><thead><tr><th>Candidat</th><th>Modification proposée</th><th>Ton avis</th></tr></thead><tbody>TABLE</tbody></table><div class="buttons"><button id="summary">Préparer mon retour</button></div><textarea id="reply" readonly aria-label="Retour à copier dans la conversation"></textarea></section>
<p class="note">Les identités, les statistiques et les règles restent celles du jeu. Ce lot porte sur les dessins et leurs prises montrées ; il ne valide pas toutes les autres tenues ni toutes les armes par extension. Sources et prompts du mode intégré imagegen : <a href="../../assets/source-art/art-finish-review-01/prompts.json">treize dessins</a> et <a href="../../assets/source-art/art-finish-review-01/generation-auxiliary.json">poses et reprise du drone</a>.</p>
<script>
for(const b of document.querySelectorAll('[data-scene]')) b.addEventListener('click',()=>{const {scene,mode}=b.dataset;document.querySelector('#scene-'+scene).src='finish01-'+scene+'-'+mode+'.png';for(const o of document.querySelectorAll('[data-scene="'+scene+'"]'))o.setAttribute('aria-pressed',String(o===b));});
let saved={};try{saved=JSON.parse(localStorage.getItem('ART-FINISH-01-choices')||'{}');}catch{}
for(const s of document.querySelectorAll('[data-code]')){s.value=saved[s.dataset.code]||'';s.addEventListener('change',()=>{saved[s.dataset.code]=s.value;try{localStorage.setItem('ART-FINISH-01-choices',JSON.stringify(saved));}catch{}});}
document.querySelector('#summary').addEventListener('click',()=>{const groups={};for(const s of document.querySelectorAll('[data-code]'))if(s.value)(groups[s.value]??=[]).push(s.dataset.code);document.querySelector('#reply').value='ART-FINISH-01\n'+Object.entries(groups).map(([k,v])=>k+' : '+v.join(', ')).join('\n');});
</script></html>'''.replace('TABLE',table)
    (OUT / 'index.html').write_text(text,encoding='utf-8')
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',text):
        assert (OUT/ref).exists(),ref
    checks=dict(captures,production_files_unchanged=len(before),ordinary_renderer_changed_pixels=0,
                native_image_panels_preserved=preserved,
                candidate_source_hashes={p.name:sha(p) for p in ART.glob('*-source.png')},
                review_page='artifacts/art-quality-audit-01/index.html')
    (DOC/'checks.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(review=checks['review'],candidates=13,native_panels_preserved=preserved,normal_renderer_changed_pixels=0)))


if __name__=='__main__':
    build()
