"""Locate two-support creature candidates without treating metadata as visual proof."""
from pathlib import Path
import json

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
OUT=ROOT/'docs/reviews/animation-polish-01/BIPED-MOTION01-INVENTORY.json'


def main():
    current=json.loads((ROOT/'assets/animations/walk-07/manifest.json').read_text())['entries']
    metadata={}
    files=[DOC/'creature-jobs.json',*sorted(DOC.glob('creature-jobs-lot*.json'))]
    for path in files:
        rows=json.loads(path.read_text(encoding='utf-8-sig'))
        if not isinstance(rows,list):continue
        for row in rows:
            if not isinstance(row,dict) or 'record' not in row:continue
            record=row['record'];identity=row.get('id')
            metadata[identity]={'label':row.get('label',record.get('label')),'rig':record.get('rig'),'locomotor_legs':row.get('locomotor_legs'),'motion':row.get('motion'),'metadata_source':path.relative_to(ROOT).as_posix()}
    candidates=[];unknown=[]
    for entry in current:
        identity=entry['id'];info=metadata.get(identity,{})
        record={'id':identity,'number':entry['number'],'base_cycle':entry['file'],**info,'visually_reviewed_in_this_lot':False}
        # 'robe' and 'heavy' alone do not prove anatomy; prefer explicit supports.
        if info.get('locomotor_legs')==2:candidates.append(record)
        elif info.get('locomotor_legs') is None:unknown.append(record)
    report={'review':'BIPED-MOTION01-INVENTORY','date':'2026-11-02','status':'metadata_inventory_pending_visual_review','normal_game_modified':False,'assumption':'Two declared locomotor legs identify review candidates, not a confirmed visual defect. Flying and heavy bipeds need their own support/body check.','installed_base_cycles':len(current),'two_support_candidates':len(candidates),'candidates':candidates,'anatomy_not_confirmed':unknown,'npc_follow_up':{'scope':'Camp and armed humanoid NPCs','status':'need native identity and route coverage check','code':['src/cryogenic_camp_render.rs','src/game_art_walk.rs','src/game_art_vertical.rs']},'priority':'Redraw and review the four requested player classes first; then examine creature/NPC batches by morphology.'}
    OUT.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(json.dumps({'installed':len(current),'two_support_candidates':len(candidates),'anatomy_unconfirmed':len(unknown)},ensure_ascii=False))


if __name__=='__main__':main()
