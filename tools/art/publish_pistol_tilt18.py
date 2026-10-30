"""C09: slight outward pistol tilt with the accepted C08-B knife carry."""
from pathlib import Path
import hashlib,json,shutil

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
OUT=ROOT/'artifacts/vertical-walk-review-18/current/pistol-C09'

def read(p):return json.loads(p.read_text('utf8'))
def replace(source,old,new):
    assert old in source,old
    return source.replace(old,new)

def main():
    runtime='runtime-assets-C09' if (DOC/'runtime-assets-C09.json').exists() else 'runtime-assets-C08'
    banks=['candidate-resources-C06','candidate-resources-C08',runtime]
    if (DOC/'candidate-resources-C09.json').exists():banks.append('candidate-resources-C09')
    for name in banks:
        for path,digest in read(DOC/(name+'.json'))['sha256'].items():
            assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==digest,path
    accepted=read(DOC/'acceptance-C08.json');assert accepted['selected']=='B' and accepted['normal_game_enabled']
    definition=ROOT/accepted['approved_geometry'];assert hashlib.sha256(definition.read_bytes()).hexdigest()==accepted['approved_geometry_sha256']
    decision_path=DOC/'acceptance-C09.json';decision=read(decision_path) if decision_path.exists() else {}
    accepted_c09=decision.get('selected')=='B';enabled=bool(decision.get('normal_game_enabled',False));degrees=10
    if accepted_c09:
        pistol_definition=ROOT/decision['approved_geometry']
        assert hashlib.sha256(pistol_definition.read_bytes()).hexdigest()==decision['approved_geometry_sha256']
        degrees=read(pistol_definition)['outward_degrees']
    data=read(OUT.parent/'knife-C08/catalogue.json')
    data.update(review='VERT18-C09',normal_game_enabled=enabled,pistol_tilt_degrees=degrees,knife_axes=read(definition)['knife_axes'])
    OUT.mkdir(parents=True,exist_ok=True)
    (OUT/'catalogue.json').write_text(json.dumps(data,ensure_ascii=False)+'\n','utf8')
    js=(ROOT/'tools/art/knife_grip18.js').read_text('utf8')
    old=next(line for line in js.splitlines() if line.startswith('function knifeAngle('))
    new="function weaponAngle(model,r,kind){if(model>=68&&model<=79){const tilt=Math.PI/9,axis=data.knife_axes[`${model}-${+r.left_source}`];return (r.left_source?Math.PI-tilt:tilt)-axis;}if(kind==='b'&&model>=101&&model<=112)return r.angle+(r.left_source?1:-1)*data.pistol_tilt_degrees*Math.PI/180;return r.angle;}"
    js=replace(js,old,new)
    js=replace(js,'knifeAngle(layer.model,r,kind)','weaponAngle(layer.model,r,kind)')
    js=replace(js,"['a','b','c']","['a','b']")
    js=replace(js,'Préparation des trois prises…','Préparation des deux prises…')
    js=replace(js,'pistolet A conservé.','couteau B conservé.')
    js=replace(js,'ctx.fillRect(0,0,112,140)','ctx.fillRect(0,0,168,160)')
    js=replace(js,'56-current.foot[0],114-current.foot[1]','84-current.foot[0],134-current.foot[1]')
    head=(ROOT/'tools/art/vertical_extension18.js').read_text('utf8')
    head=head[head.index('async function helmet('):head.index('async function prepareWeapons(')]
    js=replace(js,'// HEAD_IMPLEMENTATION',head)
    (OUT/'app.js').write_text(js,'utf8')
    html=(ROOT/'tools/art/knife_grip18.html').read_text('utf8')
    html=replace(html,'18 — C08 · Prise du couteau','18 — C09 · Pistolet légèrement incliné')
    html=replace(html,'repeat(3,112px);gap:6px','repeat(2,168px);gap:8px')
    html=replace(html,'figure{width:112px}canvas{width:112px;height:140px}','figure{width:168px}canvas{width:168px;height:160px}')
    html=replace(html,'18 · C08 · PRISE DU COUTEAU','18 · C09 · PROPOSITION')
    html=replace(html,'Inclinaison de la lame','Pistolet vers l’extérieur')
    html=replace(html,'Le pistolet garde la prise A choisie. Comparons le couteau actuel avec une lame légèrement inclinée vers le bas ou vers le haut.','A est la prise actuelle. B incline le pistolet de 10° vers l’extérieur en gardant le canon vers le bas. Le couteau reste en B validée.')
    html=replace(html,'A · couteau actuel','A · prise actuelle')
    html=replace(html,'B · vers le bas','B · 10° vers l’extérieur')
    html=replace(html,'<figure><figcaption>C · vers le haut</figcaption><canvas id="c" width="112" height="140"></canvas></figure>','')
    html=replace(html,'width="112" height="140"','width="168" height="160"')
    html=replace(html,'B et C changent uniquement l’orientation des couteaux et dagues W068–W079. Ces deux candidats attendent leur validation.','B change uniquement l’orientation des pistolets W101–W112. Le couteau B, les corps et les prises restent identiques. Cette inclinaison attend sa validation avant activation.')
    if accepted_c09:
        html=replace(html,'A · prise actuelle','A · ancienne prise')
        html=replace(html,'B · 10° vers l’extérieur','B · validée · 10°')
        html=replace(html,'18 · C09 · PROPOSITION','18 · C09 · VALIDÉE')
        html=replace(html,'Cette inclinaison attend sa validation avant activation.','Cette inclinaison est validée'+(' et intégrée au jeu.' if enabled else ', intégration en cours.'))
    (OUT/'apercu.html').write_text(html,'utf8');shutil.copy2(ROOT/'tools/art/vertical_extension18.css',OUT/'app.css')
    proposal={'review':'VERT18-C09','date':'2026-10-30','status':'native_proposal_awaiting_visual_approval','request':'tu peux incliner tres legerement le pistolet vers l\'exterieur :) en faite comme le couteau :)','candidate_A':'current C06/C07-A pistol with accepted C08-B knife','candidate_B':'same pistol, rigid 10-degree outward tilt from current downward carry; accepted C08-B knife unchanged','scope':{'player_profiles':8,'pistol_models':list(range(101,113)),'hands':2,'poses':13,'weapon_palette_choices':7,'headgear_choices':7,'torso_palette':'Original'},'knife_acceptance':'acceptance-C08.json','pistol_C07_decision_revised_only_if_B_approved':True,'pivot_scale_targets_hands_and_body_unchanged':True,'normal_game_activated':False,'source_art_redrawn':False,'preview':'pistol-C09/apercu.html'}
    path=DOC/'proposal-C09.json'
    if path.exists():proposal={**read(path),**proposal}
    if accepted_c09:proposal.update(status='visual_accepted_integrated' if enabled else 'visual_accepted_pending_integration',selected='B',acceptance='acceptance-C09.json',normal_game_activated=enabled,pistol_C07_revision_accepted=True)
    path.write_text(json.dumps(proposal,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'review':'VERT18-C09','accepted':accepted_c09,'normal_game_activated':enabled,'knife_B_preserved':True}))

if __name__=='__main__':main()
