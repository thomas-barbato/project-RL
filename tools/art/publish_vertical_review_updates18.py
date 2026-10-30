"""Publish only already-audited C11 rear and C12 review pages."""
from pathlib import Path
import json,shutil

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
ART=ROOT/'assets/source-art/vertical-walk-review-18'
CURRENT=ROOT/'artifacts/vertical-walk-review-18/current'

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')

def main():
    integrated=(DOC/'integration-C12.json').exists() and read(DOC/'integration-C12.json')['status']=='integrated_normal_debug_and_release'
    assert read(DOC/'ANALYSE-C11-back-r02.json')['status'] in {'native_rear_candidates_awaiting_visual_approval','visual_accepted_not_installed','visual_accepted_integrated'}
    assert read(DOC/'ANALYSE-C12.json')['status'] in {'native_candidates_awaiting_visual_approval','visual_accepted_pending_integration','visual_accepted_integrated'}
    knife=CURRENT/'knife-contact-C11';two=CURRENT/'grips-C12'
    shutil.copy2(ART/'knife-contact-C11/back-r02/browser-catalogue.json',knife/'catalogue.json')
    for ext in ['html','js']:shutil.copy2(ROOT/f'tools/art/knife_contact18.{ext}',knife/('apercu.html' if ext=='html' else 'app.js'))
    if (DOC/'acceptance-C11-back-r02.json').exists():
        path=knife/'apercu.html';html=path.read_text('utf8')
        notice='<p class="note"><strong>Dos B r02 validé le 30 octobre 2026.</strong> <a href="../grips-C12/apercu.html">Suite : vérifier les armes à deux mains dans C12</a>.</p>'
        html=html.replace('<h1>Les bras et les armes vus de dos</h1>','<h1>Les bras et les armes vus de dos</h1>'+notice)
        html=html.replace('la nouvelle vue arrière attend sa validation.','la nouvelle vue arrière est validée pour ces configurations.')
        if integrated:html=html.replace('Suite : vérifier les armes à deux mains dans C12','C11 et C12 sont intégrés : voir les armes à deux mains')
        path.write_text(html,'utf8')
    data=read(two/'catalogue.json');labels={p['id']:p['label'] for p in read(ART/'armor-C10/review-catalogue.json')['profiles']}
    for p in data['profiles']:p['label']=labels[p['id']]
    write(two/'catalogue.json',data)
    shutil.copy2(ROOT/'tools/art/vertical_twohand18.html',two/'apercu.html')
    if (DOC/'acceptance-C12.json').exists():
        path=two/'apercu.html';html=path.read_text('utf8')
        html=html.replace('PROPOSITIONS À REVOIR','PRISES VALIDÉES')
        html=html.replace('Ces nouvelles poses attendent leur validation avant intégration au jeu.','Ces prises ont été validées le 30 octobre 2026 ; leur état d’intégration est consigné dans la documentation.')
        if integrated:html=html.replace('Ces prises ont été validées le 30 octobre 2026 ; leur état d’intégration est consigné dans la documentation.','Ces prises validées sont intégrées au jeu normal, en Debug et Release, avec les configurations C11 acceptées.')
        path.write_text(html,'utf8')
    shared=(ROOT/'tools/art/vertical_extension18.js').read_text('utf8');head=shared[shared.index('async function helmet('):shared.index('async function prepareWeapons(')].replace('data.weapons.headgear','data.headgear')
    js=(ROOT/'tools/art/vertical_twohand18.js').read_text('utf8').replace('// SHARED_HELMET',head);(two/'app.js').write_text(js,'utf8')
    # Keep the original C10 comparison clearly labelled as the earlier stage.
    path=CURRENT/'armor-C10/apercu.html';html=path.read_text('utf8')
    notice='<p class="note" id="latest-reviews"><strong>Reprises après C10 :</strong> <a href="../knife-contact-C11/apercu.html">C11 : face validée, dos repris</a> · <a href="../grips-C12/apercu.html">C12 : les armes à deux mains</a>. Cette page conserve la proposition initiale.</p>'
    if 'id="latest-reviews"' not in html:html=html.replace('<h2>Toutes les tenues</h2>',notice+'<h2>Toutes les tenues</h2>')
    path.write_text(html,'utf8')
    print(json.dumps({'C11':'back-r02 published; approved fronts unchanged','C12':'74 two-handed models on twelve outfits; native proofs linked','normal_game_enabled':integrated}))

if __name__=='__main__':main()
