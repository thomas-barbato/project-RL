"""Prepare whole-figure proportional repairs against each actual front body."""
from pathlib import Path
import json,hashlib,shutil
from PIL import Image,ImageDraw

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-11'
SRC=ROOT/'assets/source-art/player-walk-review-11'

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    DOC.mkdir(exist_ok=True);SRC.mkdir(exist_ok=True)
    manifest=json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text())
    old=json.loads((ROOT/'docs/reviews/player-walk-09/direction-jobs.json').read_text(encoding='utf-8'))
    base=Image.open(ROOT/'assets/sprites/class-armors-01/base.png').convert('RGBA')
    names=['Bastion','Technomancien','Traqueuse','Arcaniste','Metamorphe'];jobs=[]
    for e in manifest['outfits']:
        index=e['index'];native=Path(e['native']['path']);high=ROOT/native.parent.parent/'sources'/f"{e['candidate_id']}-source.png"
        if not high.is_file():
            accepted=ROOT/'docs/reviews/class-readability-01'/native.parts[3]/'acceptance.json'
            records=json.loads(accepted.read_text(encoding='utf-8'))['entries']
            high=ROOT/next(r['source']['path'] for r in records if r['candidate_id']==e['candidate_id'])
        assert high.is_file(),high
        x=index%16*128;y=index//16*128
        front=base.crop((x,y,x+128,y+128));front.save(SRC/f'CLASS-{index:02}-front-native128.png')
        ref=SRC/f'CLASS-{index:02}-front-reference.png';front.crop((32,32,96,96)).resize((768,768),Image.Resampling.NEAREST).save(ref)
        for side in ['left','right']:
            original=next(j for j in old if j['index']==index and j['side']==side)
            target=Path(original['source']);destination=SRC/f'CLASS-{index:02}-{side}-proportions-r01.png'
            prompt=f'''Use case: identity-preserve. Project RL, {names[index//5]} outfit {index%5+1}, CLASS-{index:02}, facing SCREEN {side.upper()}.
Image1 is the existing eight-pose walking sheet to repair: preserve its compact six walking poses and sequence. Image2 is the authoritative FRONT character design and physique. Image3 is the actual native64 front sprite, authoritative for head-to-body, torso-to-leg, arm, glove and boot proportions.
Repair the current lateral sheet's inflated physique. Redraw EACH WHOLE figure to look like exactly the same person and outfit as the front, merely rotated into near-front three quarters {side.upper()}, both shoulders visible. Match the front's head size, slenderness or bulk, leg length, waist, biceps, hands, boots and armor thickness. The head must stay small at the same fraction of total height as the front; do not invent a large-headed stocky toy or thick padding. A three-quarter projection of this same torso should generally look a little narrower than the front. Do not copy the inflated build from image1; its role is the POSE sequence only. No horizontal bitmap squeeze. No new armor pieces, decorations, weapon, hair, colors or costume changes. Match image2's painting style: readable matte masses and restrained dark outlines, not thick simplistic bulging blocks.
Keep the EXACT original armor identity, coverage and default head/helmet for this outfit. Bare flesh stays bare and armor stays on its correct regions throughout all poses. For Traqueuse retain the established TWO silver armored shins and cybernetic forearm consistently, never swap which leg is protected or lose plates during a step. For Metamorphe preserve its specific olive/brown/ivory organic costume and front body build, not a muscular stone robot. Robes remain the exact same length, cut and thinness as the front, with the compact walk underneath; don't turn robes into trousers.
Preserve the compact alternating support cycle: one foot supports body weight while the other knee bends and the heel lifts, swing boot passes narrowly under the pelvis; then the other anatomical leg supports the body in the next three phases. No sideways leg spreading, splits, running, huge kicks or hopping. Every head, torso, pelvis, knee and boot points {side.upper()} together in all8 figures. Keep both complete arms relaxed, closed empty hands separated from thighs to attach the real weapons later; no holes, extra limbs or disconnected shoulders.
Output EXACTLY four columns by two rows,8 whole figures on transparent alpha. Row-major order: true both-feet-grounded idle, walk0, walk1, walk2, walk3, walk4, walk5, duplicate idle. Match the existing pose order. SAME character scale, body build and head size across all8 cells, equal support baseline and minimal natural body bob. Generous transparent gaps between cells and around all sides; every head and sole fully inside its cell. No text, labels, grids, shadows, floor, effects or background.'''
            job=dict(id=f'CLASS-{index:02}-{side}',kind='player',index=index,side=side,input=str(target),front_source=str(high),native_reference=str(ref),destination=str(destination),prompt=prompt)
            if index==0:
                previous=ROOT/f'assets/source-art/player-walk-review-10/CLASS-00-{side}-proportions-r01.png'
                shutil.copy2(previous,destination);job.update(reused=True,reused_from=str(previous),source_sha256=sha(previous))
            jobs.append(job)
    (DOC/'jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (DOC/'authorization.json').write_text(json.dumps(dict(user_quote='ok il faut donc corriger ca :)',scope='25 player outfits, confirmed monster body repairs014/015/115/124, small cultists022/024 and candidate palette-cycle coverage; native review before integration',normal_game_enabled=False),ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    protected=list((ROOT/'assets/animations/walk-07').rglob('*'))+list((ROOT/'assets/animations/walk-08').rglob('*'))+list((ROOT/'assets/animations/walk-09').rglob('*'))+list((ROOT/'artifacts/map-editor').rglob('*'))
    (DOC/'baseline.json').write_text(json.dumps({str(p.relative_to(ROOT)).replace('\\','/'):sha(p) for p in protected if p.is_file()},indent=2)+'\n')
    print(json.dumps(dict(jobs=len(jobs),reused=2,to_generate=48,front_sources_verified=25)))

if __name__=='__main__':main()
