"""Review actual candidate compositor exports against canonical fronts and WALK09."""
from pathlib import Path
import json,hashlib
from statistics import median
from PIL import Image,ImageDraw,ImageOps
import audit_walk10_all_proportions as audit

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-11'
ART=ROOT/'artifacts/player-walk-review-11/full-audit'

def main():
    audit.ART=ART;audit.DOC=DOC/'audit';audit.DOC.mkdir(exist_ok=True)
    audit.CREATURE_OVERRIDE_MANIFEST=ROOT/'assets/animations/walk-11/creatures/manifest.json'
    player=audit.players();campaign,cycles,poses,images=audit.campaigns();extra=audit.extras(cycles,poses,images)
    report=dict(review_id='WALK11-PROPORTIONS-r01',status='awaiting_visual_review',normal_game_modified=False,players=player,campaign=campaign,extras=extra)
    (audit.DOC/'audit.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    old=json.loads((ROOT/'docs/reviews/player-walk-10/full-audit/audit.json').read_text())
    before={c['id']:c for c in old['players']['base_cases']}
    comparisons=[]
    for c in player['base_cases']:
        comparisons.append(dict(id=c['id'],before=before[c['id']]['evaluation'],candidate=c['evaluation']))
    repaired=[c for c in campaign['cases'] if c['number'] in [14,15,22,24,115,124] and c['variant']==0]
    summary=dict(outfits=25,player_body_cases=len(player['body_cases']),player_pose_measurements=player['full_body_pose_measurements'],weapon_models=113,weapon_sample_cases=len(player['weapon_cases']),weapon_pose_measurements=player['weapon_pose_measurements'],campaign_pairs=len(campaign['cases']),candidate_animated_campaign_pairs=campaign['normal_animated_pairs'],creature_repairs=repaired,player_comparisons=comparisons,
      original_median_bulk=round(median(c['evaluation']['median_area_ratio_height_normalized'] for c in old['players']['base_cases']),3),candidate_median_bulk=round(median(c['evaluation']['median_area_ratio_height_normalized'] for c in player['base_cases']),3),
      players_height_outside_two_pixels=[c['id'] for c in player['base_cases'] if 'front_height_difference' in c['evaluation']['signals'] or 'within_cycle_height_variation' in c['evaluation']['signals']],
      remaining_numeric_signals=[dict(id=c['id'],signals=c['evaluation']['signals']) for c in player['base_cases'] if c['evaluation']['signals']],
      limits=['Projected silhouette widths depend on arm pose, overlapping legs and cloth; numeric signals are not artistic approval.','All helmets/all14 poses and all113 weapons on each first outfit at4 key poses; not a full Cartesian product.','Other accepted creature poses and proposed forms/drone drawings remain unchanged.'])
    (DOC/'audit-summary.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    # Native actual-compositor animation: five outfits per class, face/current/candidate.
    oldart=ROOT/'artifacts/player-walk-review-10/full-audit/players'
    for cls,name in enumerate(['Bastion','Technomancien','Traqueuse','Arcaniste','Metamorphe']):
        current=[];proposed=[]
        for rank in range(5):
            index=cls*5+rank;fn=f'CLASS-{index:02}-head0-weapon067.png'
            current.append(ImageOps.flip(Image.open(oldart/fn).convert('RGBA')))
            proposed.append(ImageOps.flip(Image.open(ART/'players'/fn).convert('RGBA')))
        frames=[];durations=[]
        for phase in range(28):
            board=Image.new('RGB',(960,690),'#253136');d=ImageDraw.Draw(board)
            d.text((10,6),f'{name} - FACE / ACTUEL / NOUVEL ESSAI - native64, 400ms/case',fill='white')
            step=0 if phase<14 else 1;local=phase%14;moving=local<12;p=min(local//4,2)+step*3+1 if moving else 0;progress=min(local/12,1)
            for rank in range(5):
                y=32+rank*130;d.text((5,y),f'Tenue {rank+1}/5 - hache67',fill='white')
                face=proposed[rank].crop((0,0,128,128));board.paste(face,(72,y-4),face)
                for direction in range(2):
                    col=1+direction*7+p;offset=round((1-progress if direction==0 else progress)*64)
                    for x,atlas in [(330,current[rank]),(650,proposed[rank])]:
                        yy=y+4+direction*58;d.rectangle((x,yy,x+127,yy+63),outline='#46545a')
                        im=atlas.crop((col*128,0,(col+1)*128,128));board.paste(im,(x+offset-32,yy-32),im)
            frames.append(board);durations.append([30,40,30][local%3] if moving else 260)
        frames[0].save(DOC/f'{name}-native64-axe.gif',save_all=True,append_images=frames[1:],duration=durations,loop=0,optimize=False)
    print(json.dumps({k:v for k,v in summary.items() if k not in ['creature_repairs','player_comparisons']},ensure_ascii=False))

if __name__=='__main__':main()
