"""Freeze the explicitly accepted torso review and package runtime PNGs only."""
import json
import html
import shutil
from pathlib import Path
from PIL import Image
from torso_icons_review_01 import ROOT, ART, DOC, OUT, sha, write

def integrate():
    data=json.loads((DOC/'candidates.json').read_text('utf-8'))
    assert data['appearances']==69 and data['revision']=='r02-red'
    dest=ROOT/'assets/sprites/torso-icons-01';dest.mkdir(parents=True,exist_ok=True)
    assets=[];accepted={}
    supplement_doc=ROOT/'docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02'
    supplement= json.loads((supplement_doc/'candidates.json').read_text('utf-8')) if (supplement_doc/'acceptance.json').exists() else None
    entries=data['entries']+(supplement['entries'] if supplement else [])
    for entry in entries:
        for variant in entry['variants']:
            for field in ['icon','worn']:
                p=ROOT/variant[field];assert sha(p)==variant[field+'_sha256'];accepted[variant[field]]=sha(p)
            p=ROOT/variant['icon'];target=dest/p.name;shutil.copy2(p,target);assets.append((p.stem,target))
        p=ROOT/entry['material_mask'];target=dest/p.name;shutil.copy2(p,target);assets.append((p.stem,target));accepted[p.relative_to(ROOT).as_posix()]=sha(p)
        for part,file in entry.get('new_palette_mask_overrides',{}).items():
            p=ROOT/file;target=dest/p.name;shutil.copy2(p,target);assets.append((f'TOR-RIG-01-{part}-material',target));accepted[file]=sha(p)
    assert len(assets)==(114 if supplement else 83)
    lines=['// Accepted TORSO-ICONS-01 r02: isolated torso items, native64 only.',
           "fn bytes_torso_icons_01(name: &str) -> Option<&'static [u8]> { Some(match name {"]
    lines += [f'    "{key}" => include_bytes!("../{p.relative_to(ROOT).as_posix()}"),' for key,p in assets]
    lines += ['    _ => return None,','}) }']
    (ROOT/'src/game_art_torso_catalog_01.rs').write_text('\n'.join(lines)+'\n',encoding='utf-8')
    write(DOC/'accepted-candidates.json',accepted)
    write(DOC/'runtime-assets.json',{p.relative_to(ROOT).as_posix():sha(p) for _,p in assets})
    proof=DOC/'proofs';proof.mkdir(exist_ok=True)
    for name,value in json.loads((DOC/'review-proof-hashes.json').read_text('utf-8')).items():
        assert sha(OUT/name)==value;shutil.copy2(OUT/name,proof/name)
    acceptance=DOC/'acceptance.json'
    if not acceptance.exists():
        write(acceptance,dict(review='TORSO-ICONS-01',revision='r02-red',date='2026-10-20',status='accepted_pending_integration',user_statement='ok , on peut les integrer :)',models=14,appearances=69,new_cosmetic_palettes=['Bleu ardoise','Vert mousse','Rouille','Rouge sombre'],red_additions=13,red_original_retained='TOR-109-A',approved_candidate_hashes='accepted-candidates.json',runtime_manifest='runtime-assets.json',native_cell=64,full_equipped_outfits_retained=True,stats_and_content_ids_unchanged=True))
    print('Frozen',69+(24 if supplement else 0),'accepted icon/outfit pairs; packaged',len(assets),'runtime PNGs')

def publish():
    """Publish accepted pixels and real-engine captures without rebuilding art."""
    supplement=ROOT/'docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02'
    for directory in [DOC,supplement]:
        assert json.loads((directory/'acceptance.json').read_text('utf-8'))['status']=='accepted_and_integrated'
    entries=json.loads((DOC/'candidates.json').read_text('utf-8'))['entries']
    starting=json.loads((supplement/'candidates.json').read_text('utf-8'))['entries']
    checks=json.loads((OUT/'engine/checks.json').read_text('utf-8'))
    assert checks['native_models']==19 and checks['approved_pairs']==93
    style='''<style>body{background:#11191d;color:#d1dad6;margin:24px;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1100px}a{color:#d8bd85}.scroll{overflow:auto;max-width:100%;margin:16px 0;border:1px solid #43514b}img{display:block;max-width:none}.card{background:#1b272d;margin:20px 0;padding:16px;overflow:auto}.strip{display:flex;gap:12px}.pair{min-width:230px;background:#293332;padding:10px}.pair img{display:inline-block;vertical-align:middle}.pair strong{display:block;color:#decca7}nav{display:flex;gap:24px;flex-wrap:wrap}summary{cursor:pointer;color:#decca7}details{margin:16px 0}</style>'''
    def picture(path,w,h,alt):
        with Image.open(OUT/path) as im:w,h=im.size
        return f'<div class="scroll"><img src="{path}" width="{w}" height="{h}" loading="lazy" alt="{alt}"></div>'
    def header(title):
        return f'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title}</title>{style}<h1>{title}</h1>'
    parts=[header('Armures de torse · 19 modèles intégrés')]
    parts.append('''<p><strong>Les deux lots sont validés et intégrés.</strong> Les 19 objets de torse utilisent leurs nouvelles icônes dans le loot au sol, l’inventaire, l’équipement et les fiches d’objet. Les 93 associations présentées sont actives. Équiper le torse conserve le changement de tenue complète.</p><p>A : origine ; B : bleu ardoise ; C : vert mousse ; D : rouille ; E : rouge sombre. La Robe de la Seconde Ombre et la Tenue rituelle gardent leur rouge A, sans doublon E. Les anciennes couleurs, les identités et les caractéristiques des objets sont conservées.</p><nav><a href="#integration">Captures en jeu</a><a href="depart.html">Cinq tenues de départ</a><a href="#catalogue">19 modèles et couleurs</a><a href="#preuves">Planches de validation</a></nav>''')
    parts.append('<h2 id="integration">Rendu actif dans le jeu</h2><p>Captures du moteur, à leurs dimensions réelles : six tenues avec éclairage, objets déposés au sol, inventaire et fiche d’objet. Les parties utilisées sont isolées ; leur capacité d’inventaire est augmentée uniquement pour ces contrôles.</p>')
    for label,title in [('mue-red','Manteau de mue · rouge sombre'),('assaut-blue','Tenue d’assaut · bleu ardoise'),('technique-green','Tenue technique · vert mousse'),('robe-blue','Robe de la Seconde Ombre · bleu ardoise'),('chrysalide-green','Carapace Chrysalide · vert mousse'),('cerbere-rust','Armure Cerbère · rouille')]:
        parts.append(f'<h3>{title}</h3><details'+(' open' if label=='mue-red' else '')+'><summary>Objet au sol et tenue portée</summary>'+picture(f'engine/{label}-ground.png',1920,1080,'Objet au sol et couleur portée dans la rue éclairée')+'</details><details><summary>Inventaire et fiche d’objet</summary>'+picture(f'engine/{label}-inventory.png',1920,1080,'Icône de torse dans l’inventaire, l’équipement et la fiche')+'</details>')
    parts.append('<h2 id="catalogue">19 modèles · 93 associations acceptées</h2><p>Icônes à 64 px ; les tenues conservent leur gabarit réel. Pour chaque modèle, le contrôle moteur montre aussi les icônes à 40 et 28 px et six configurations de mains.</p>')
    for e in entries+starting:
        source=(ROOT/e.get('source',e.get('output'))).relative_to(ROOT).as_posix()
        parts.append(f'<article class="card" id="{e["id"]}"><h3>{e["id"]} · {html.escape(e["label"])}</h3><div class="strip">')
        for v in e['variants']:
            parts.append(f'<div class="pair"><strong>{v["code"]} · {v["label"]}</strong><img src="../../{v["icon"]}" width="64" height="64" loading="lazy" alt="Icône de torse"><img src="../../{v["worn"]}" width="128" height="128" loading="lazy" alt="Tenue correspondante"></div>')
        parts.append('</div><details><summary>Contrôle moteur · six configurations de mains</summary>'+picture(f'engine/native-{e["illustration"]}.png',1280,900,'Contrôle moteur de toutes les palettes et configurations de mains')+f'</details><p><a href="../../{source}">Dessin source</a></p></article>')
    parts.append('<h2 id="preuves">Planches conservées de la validation</h2><p>Ces planches historiques ont servi à l’accord visuel. Leurs mentions de candidature décrivent cette étape antérieure ; les captures actives figurent en haut de page.</p>')
    for name in json.loads((DOC/'review-proof-hashes.json').read_text('utf-8')):
        parts.append('<details><summary>'+html.escape(name)+'</summary>'+f'<a href="{name}">Ouvrir la planche originale</a></details>')
    parts.append('<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/torso-icons-review-01/README.md">Bilan et contrôles</a> · <a href="../../assets/source-art/torso-icons-review-01/prompts.json">Prompts imagegen intégrés · 14 modèles</a> · <a href="../../assets/source-art/torso-starting-review-02/prompts.json">Prompts imagegen intégrés · 5 tenues de départ</a></p></html>')
    (OUT/'index.html').write_text('\n'.join(parts),encoding='utf-8')
    depart=[header('Cinq tenues de départ · intégrées')]
    depart.append('<p>TOR-200 à TOR-204 et leurs 24 associations icône/couleur/tenue ont été validés par le joueur puis intégrés. Ces cinq objets équipables complètent les quatorze modèles du premier lot. Le tissu de la Tenue d’assaut suit les nouvelles palettes ; sa peau reste protégée. Les anciennes peintures et les portraits restent inchangés.</p><p>La planche ci-dessous est la preuve originale de validation, conservée sans modification.</p>')
    depart.append(picture('depart-native.png',1468,912,'Les cinq tenues de départ et leurs associations acceptées'))
    depart.append('<p><a href="index.html#integration">Captures actives en jeu</a> · <a href="index.html#TOR-200">Catalogue et contrôles moteur des cinq modèles</a> · <a href="../../assets/source-art/torso-starting-review-02/prompts.json">Sources et prompts imagegen intégrés</a></p></html>')
    (OUT/'depart.html').write_text('\n'.join(depart),encoding='utf-8')
    print('Published 19 integrated models, 93 accepted pairs and six real-game comparisons')

def verify():
    """Check frozen art, runtime copies and captured GPU pixels; retain proofs."""
    supplement=ROOT/'docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02'
    entries=[]
    for directory in [DOC,supplement]:
        entries+=json.loads((directory/'candidates.json').read_text('utf-8'))['entries']
    assert len(entries)==19 and sum(len(e['variants']) for e in entries)==93
    for manifest in ['accepted-candidates.json','runtime-assets.json']:
        hashes=json.loads((DOC/manifest).read_text('utf-8'))
        for file,value in hashes.items():assert sha(ROOT/file)==value,file
    before=json.loads((DOC/'production-before.json').read_text('utf-8'))
    old_pngs={p:v for p,v in before.items() if p.endswith('.png')}
    for file,value in old_pngs.items():assert sha(ROOT/file)==value,file
    for e in entries:assert sha(ROOT/e.get('source',e.get('output')))==e['source_sha256'],e['id']
    for file,value in json.loads((DOC/'review-proof-hashes.json').read_text('utf-8')).items():
        assert sha(OUT/file)==value,file
    starting_checks=json.loads((supplement/'checks.json').read_text('utf-8'))
    assert sha(OUT/'depart-native.png')==starting_checks['proof_sha256']
    opaque=blended=error=opaque_error=0
    for e in entries:
        frame=Image.open(OUT/'engine'/f'native-{e["illustration"]}.png').convert('RGB')
        assert frame.size==(1280,900)
        for row,v in enumerate(e['variants']):
            icon=Image.open(ROOT/v['icon']).convert('RGBA');assert icon.size==(64,64)
            for y in range(64):
                for x in range(64):
                    pixel=icon.getpixel((x,y));a=pixel[3]
                    if not a:continue
                    actual=frame.getpixel((30+x,88+row*166+y))
                    expected=tuple(int((pixel[c]*a+(67,74,70)[c]*(255-a))/255+.5) for c in range(3))
                    delta=max(abs(expected[c]-actual[c]) for c in range(3));error=max(error,delta);blended+=1
                    if a==255:opaque+=1;opaque_error=max(opaque_error,delta)
                    assert delta<=1,(v['code'],x,y,expected,actual)
    gpu=dict(approved_pairs=93,opaque_source_pixels=opaque,max_rgb_error=opaque_error,pixels_over_tolerance=0,alpha_blended_pixels=blended,alpha_blended_max_rgb_error=error,alpha_blended_pixels_over_tolerance=0)
    write(DOC/'gpu-pixel-check.json',gpu)
    captures=OUT/'engine';proof=DOC/'proofs/engine';proof.mkdir(parents=True,exist_ok=True)
    frames=list(captures.glob('*.png'));assert len(frames)==31
    for frame in frames:shutil.copy2(frame,proof/frame.name)
    shutil.copy2(captures/'checks.json',proof/'checks.json')
    starting_proof=supplement/'proofs';starting_proof.mkdir(exist_ok=True)
    shutil.copy2(OUT/'depart-native.png',starting_proof/'depart-native.png')
    write(DOC/'engine-proof-hashes.json',{p.relative_to(ROOT).as_posix():sha(p) for p in sorted(proof.iterdir())})
    write(DOC/'asset-integrity-checks.json',dict(models=19,approved_pairs=93,runtime_pngs=114,accepted_pixels_unchanged=True,existing_production_pngs_unchanged=len(old_pngs),source_masters_unchanged=19,first_review_boards_unchanged=8,starting_review_board_unchanged=True,engine_frames=31,gpu=gpu))
    print(json.dumps(dict(models=19,approved_pairs=93,old_pngs_unchanged=len(old_pngs),gpu=gpu,engine_frames=31)))

if __name__=='__main__':
    import sys
    if '--publish' in sys.argv:publish()
    elif '--verify' in sys.argv:verify()
    else:integrate()
