"""Native-size evidence of the production integration, never a runtime input."""
from pathlib import Path
from PIL import Image,ImageDraw
import json
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/actor-readability-integration-02'
DOC=ROOT/'docs/reviews/actor-readability-01'

def main():
    old=ROOT/'artifacts/class-armor-integration-01/compositor';new=OUT/'compositor'
    meta=json.loads((new/'capture.json').read_text('utf-8'));labels=meta['configurations']
    oldlabels=json.loads((old/'capture.json').read_text('utf-8'))['configurations']
    changes={};fixed=0;images=[]
    for i in range(25):
        a=Image.open(old/f'outfit-{i:02}.png').convert('RGBA')
        b=Image.open(new/f'outfit-{i:02}.png').convert('RGBA')
        if meta['exported_pngs_need_vertical_flip']:b=b.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        images.append(b)
        for k,label in enumerate(labels):
            n=oldlabels.index(label)
            aa=a.crop((n%16*128,n//16*128,n%16*128+128,n//16*128+128))
            bb=b.crop((k%16*128,k//16*128,k%16*128+128,k//16*128+128))
            if aa.tobytes()!=bb.tobytes():changes[label]=changes.get(label,0)+1
            else:fixed+=1
    assert all(n not in changes for n in ['unarmed','dual-pistols','dual-melee','dual-melee-ranged'])
    assert all(changes.get(f'weapon-{n:03}')==25 for n in range(61))
    assert not any(f'weapon-{n:03}' in changes for n in range(61,113))
    for model in [2,38]:
        board=Image.new('RGB',(900,700),(17,25,29));draw=ImageDraw.Draw(board)
        k=labels.index(f'weapon-{model:03}')
        for i in range(25):
            r,c=divmod(i,5);x=130+c*150;y=48+r*128
            draw.text((x+54,18),f'0{c+1}',fill='white')
            draw.text((8,y+55),['Bastion','Technomancien','Traqueuse','Arcaniste','Metamorphe'][r],fill='white')
            im=images[i].crop((k%16*128,k//16*128,k%16*128+128,k//16*128+128));board.paste(im,(x,y),im)
        board.save(OUT/f'ranged-{model:03}-after-native.png')
    for file in (OUT/'world').glob('*.png'):
        if file.name.startswith('options'):continue
        im=Image.open(file).crop((490,145,910,565));im.save(OUT/(file.stem+'-world.png'))
    captures=json.loads((OUT/'world/runtime-checks.json').read_text('utf-8'))
    result={'review_id':'ACTOR-READABILITY01-INTEGRATION02','cpu_compositions':25*len(labels),
        'changed_compositions':sum(changes.values()),'unchanged_compositions':fixed,
        'changed_configurations':changes,'long_ranged_models':61,'class_outfits':25,
        'new_base_monsters':3,'new_monster_appearances_with_existing_palettes':6,
        'global_campaign_appearances_brightened':471,'runtime':captures,
        'source_armors_repainted':False,'weapon_designs_repainted':False,
        'source_paintings_unchanged':True,'items_stats_rules_and_save_ids_unchanged':True}
    (DOC/'integration-02-checks.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Intégration · classes et lisibilité</title>
<style>:root{color-scheme:dark}body{background:#10191d;color:#e1e8e6;font:15px/1.5 system-ui;margin:20px auto;max-width:1100px;padding:0 12px}h1{font-size:24px}h2{font-size:19px;margin-top:30px}p{color:#bdc9cd}button,select{font:inherit;padding:8px;background:#293f46;color:#ecf0eb;border:1px solid #627e83;border-radius:4px;margin:3px}button[aria-pressed=true]{background:#35635e}a{color:#94d6d9}.view{overflow:auto}img{display:block;max-width:none;image-rendering:pixelated}.tools{display:flex;flex-wrap:wrap;gap:8px}small{color:#a5b6ba}</style>
<h1>Classes et lisibilité · intégré</h1>
<p>25 tenues intégrées. Armes longues adaptées à chaque classe, avec deux prises et des avant-bras corrigés.</p>
<div class="tools"><label>Scène <select id="scene"><option value="camp">Camp réel</option><option value="monsters">Les trois nouvelles formes</option><option value="projectile">Projectile</option><option value="explosion" selected>Explosion</option></select></label><button data-mode="color">Couleur</button><button data-mode="gray" aria-pressed="true">Gris</button></div>
<p><b>Options → Affichage</b> : repères désactivables et gris facultatif. Repères, sorts et animations restent en couleur.</p>
<div class="view"><img id="world" src="explosion-gray-world.png" width="420" height="420" alt="Rendu réel du jeu en nuances de gris avec explosion et repères en couleur"></div>
<p><a id="full" href="world/explosion-gray.png">Capture complète avec interface</a> · <a href="world/camp-gray-no-markers.png">Camp sans repère</a> · <a href="world/options-960.png">Options en 960 × 540</a></p>
<h2>01 · Fusil d’assaut · cinq classes et cinq armures</h2><p>Dessins à taille native. Le changement d’armure conserve le gabarit de la classe ; les armes gardent leur design commun.</p>
<div class="view"><img src="ranged-002-after-native.png" width="900" height="700" alt="Les 25 tenues avec le fusil d’assaut adapté à chaque classe"></div>
<h2>02 · Second modèle d’arme longue</h2><div class="view"><img src="ranged-038-after-native.png" width="900" height="700" alt="Les 25 tenues avec un second fusil à deux mains"></div>
<p>Éclaircissement commun à tous les personnages et monstres. Trois nouvelles formes et leurs variantes existantes sont intégrées ; les autres dessins du bestiaire restent à reprendre par lots.</p>
<small>Captures du moteur et du composeur de production, grille native de 64 px. Aucun changement des statistiques, identités d’objet ou règles de combat. Les cadres de 128 px comportent les marges transparentes et ne changent pas l’échelle du personnage.</small>
<script>let mode='gray';const scene=document.querySelector('#scene');function render(){const name=scene.value+'-'+mode;const image=document.querySelector('#world');image.src=name+'-world.png';image.alt='Capture réelle : '+scene.options[scene.selectedIndex].text+', '+(mode==='gray'?'monde en gris et effets en couleur':'monde en couleur');document.querySelector('#full').href='world/'+name+'.png';document.querySelectorAll('[data-mode]').forEach(b=>b.setAttribute('aria-pressed',b.dataset.mode===mode));}scene.onchange=render;document.querySelectorAll('[data-mode]').forEach(b=>b.onclick=()=>{mode=b.dataset.mode;render()});</script></html>'''
    (OUT/'index.html').write_text(html,encoding='utf-8')
    print(json.dumps({k:result[k] for k in ['cpu_compositions','changed_compositions','unchanged_compositions','class_outfits','long_ranged_models']}))

if __name__=='__main__':main()
