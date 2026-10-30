"""C07: compare current and level pistol carry using the frozen C06 artwork."""
from pathlib import Path
import json,shutil
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/vertical-walk-review-18/current/pistol-C07'
DOC=ROOT/'docs/reviews/vertical-walk-18'

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    data=json.loads((OUT.parent/'extension-C06/catalogue.json').read_text('utf8'))
    def rebase(value):
        if isinstance(value,dict):return {k:rebase(v) for k,v in value.items()}
        if isinstance(value,list):return [rebase(v) for v in value]
        if isinstance(value,str) and value.startswith('resources/'):return '../extension-C06/'+value
        return value
    published=rebase(data);published.update(review='VERT18-C07',normal_game_enabled=False)
    (OUT/'catalogue.json').write_text(json.dumps(published,ensure_ascii=False)+'\n','utf8')
    # Share the exact accepted head replacement implementation with C06.
    old=(ROOT/'tools/art/vertical_extension18.js').read_text('utf8')
    head=old[old.index('async function helmet('):old.index('async function prepareWeapons(')]
    js=(ROOT/'tools/art/pistol_orientation18.js').read_text('utf8').replace('// HEAD_IMPLEMENTATION',head)
    (OUT/'app.js').write_text(js,'utf8')
    for source,target in [('pistol_orientation18.html','apercu.html'),('vertical_extension18.css','app.css')]:shutil.copy2(ROOT/'tools/art'/source,OUT/target)
    proposal_path=DOC/'proposal-C07.json'
    proposal=json.loads(proposal_path.read_text('utf8')) if proposal_path.exists() else {'review':'VERT18-C07','status':'native_proposal_awaiting_visual_approval','reason':'User dislikes the pistol pointing down','candidate_A':'C06 current accepted carry','candidate_B':'same whole body, palms, weapon art, scale, colors and fingers; pistol angle zero (level) for models101..112 in both hands and all13 poses','body_profiles':8,'pistol_models':12,'normal_game_activated':False,'source_art_redrawn':False,'approved_source_bank':'candidate-resources-C06.json','preview':'pistol-C07/apercu.html'}
    if (DOC/'decision-C07.json').exists():
        proposal.update(status='closed_A_retained',decision='decision-C07.json',candidate_B_accepted=False,presentation_notes='A retained explicitly; horizontal B not selected; knife feedback continues in C08')
        page=OUT/'apercu.html';html=page.read_text('utf8').replace('CORRECTION PROPOSÉE','A RETENUE').replace('A conserve la prise actuelle. B garde le pistolet à l’horizontale, avec les mêmes bras et les mêmes points de prise.','Le joueur a choisi A. B reste le comparatif horizontal non retenu. La reprise du couteau se poursuit dans C08.')
        page.write_text(html,'utf8')
    proposal_path.write_text(json.dumps(proposal,ensure_ascii=False,indent=2)+'\n','utf8')
    print('C07 comparison published; existing decision preserved; normal game unchanged')

if __name__=='__main__':main()
