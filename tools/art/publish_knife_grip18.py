"""C08: rigid knife orientation candidates; no runtime or accepted source edits."""
from pathlib import Path
import hashlib,json,math,shutil
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/vertical-walk-review-18/current/knife-C08'
DOC=ROOT/'docs/reviews/vertical-walk-18'

def main():
    frozen=json.loads((DOC/'candidate-resources-C06.json').read_text('utf8'))['sha256']
    for name,digest in frozen.items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,name
    OUT.mkdir(exist_ok=True,parents=True)
    data=json.loads((OUT.parent/'extension-C06/catalogue.json').read_text('utf8'))
    axes={}
    for b in data['weapons']['bindings']:
        if not 68<=b['model']<=79:continue
        source=Image.open(OUT.parent/'extension-C06'/b['palettes'][0]['file']).convert('RGBA');a=source.getchannel('A');box=a.point(lambda v:255 if v>=64 else 0).getbbox();assert box
        top=box[1];points=[(x,y) for y in range(top,top+2) for x in range(box[0],box[2]) if a.getpixel((x,y))>=64]
        tip=[sum(p[i] for p in points)/len(points)+.5 for i in range(2)];pivot=b['anchor'];assert pivot[1]-tip[1]>4
        axes[f'{b["model"]}-{int(b["left"])}']=math.atan2(tip[1]-pivot[1],tip[0]-pivot[0])
    decision_path=DOC/'acceptance-C08.json'
    decision=json.loads(decision_path.read_text('utf8')) if decision_path.exists() else {}
    accepted=decision.get('selected')=='B'
    enabled=bool(decision.get('normal_game_enabled',False))
    if accepted:
        definition=ROOT/decision['approved_geometry']
        assert hashlib.sha256(definition.read_bytes()).hexdigest()==decision['approved_geometry_sha256']
        assert axes==json.loads(definition.read_text('utf8'))['knife_axes']
    def rebase(v):
        if isinstance(v,dict):return {k:rebase(x) for k,x in v.items()}
        if isinstance(v,list):return [rebase(x) for x in v]
        if isinstance(v,str) and v.startswith('resources/'):return '../extension-C06/'+v
        return v
    data=rebase(data);data.update(review='VERT18-C08',normal_game_enabled=enabled,knife_axes=axes)
    (OUT/'catalogue.json').write_text(json.dumps(data,ensure_ascii=False)+'\n','utf8')
    old=(ROOT/'tools/art/vertical_extension18.js').read_text('utf8');head=old[old.index('async function helmet('):old.index('async function prepareWeapons(')]
    (OUT/'app.js').write_text((ROOT/'tools/art/knife_grip18.js').read_text('utf8').replace('// HEAD_IMPLEMENTATION',head),'utf8')
    for source,target in [('knife_grip18.html','apercu.html'),('vertical_extension18.css','app.css')]:shutil.copy2(ROOT/'tools/art'/source,OUT/target)
    proposal={'review':'VERT18-C08','date':'2026-10-30','status':'native_candidates_awaiting_visual_approval','pistol_decision':'A retained, decision-C07.json','candidate_A':'current knife C06 unchanged','candidate_B':'blade outward, 20 degrees below horizontal','candidate_C':'blade outward, 20 degrees above horizontal','scope':{'player_profiles':8,'knife_models':list(range(68,80)),'hands':2,'poses':13,'weapon_palette_choices':7,'headgear_choices':7},'method':'rigid rotation around the existing grip; blade axis measured from the top two opaque rows of original shared art; no body, hand, weapon pixel, scale or attachment change','pistol_angles_unchanged':True,'normal_game_activated':False,'source_bank':'candidate-resources-C06.json','source_hashes_verified':len(frozen),'preview':'knife-C08/apercu.html'}
    previous=DOC/'proposal-C08.json'
    if previous.exists():proposal={**json.loads(previous.read_text('utf8')),**proposal}
    if accepted:
        proposal.update(status='visual_accepted_integrated' if enabled else 'visual_accepted_pending_integration',selected='B',acceptance='acceptance-C08.json',normal_game_activated=enabled)
        page=OUT/'apercu.html';html=page.read_text('utf8').replace('A · couteau actuel','A · ancienne prise').replace('B · vers le bas','B · validée').replace('Ces deux candidats attendent leur validation.','B est validée'+(' et intégrée au jeu.' if enabled else ', intégration en cours.'))
        page.write_text(html,'utf8')
    previous.write_text(json.dumps(proposal,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'review':'VERT18-C08','knife_bindings':len(axes),'frozen_hashes_verified':len(frozen),'normal_game_activated':enabled}))

if __name__=='__main__':main()
