"""Prepare the remaining campaign identities for review, without runtime changes."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'

# Anatomy is read from the native reference, not inferred from legacy rig names.
DETAILS = {
5: 'Lean pale hunched human hybrid, bare long arms, small head, dark trousers and heavy dark boots; TWO legs and TWO arms. Retain the original hunched shoulder line.',
6: 'Pale skeletal hybrid, TWO legs in dark trousers, TWO long clawed arms, original red/orange upper back and skeletal rib structures. Keep all existing bony protrusions; no new face or limbs.',
7: 'Heavy asymmetrical pale hybrid, TWO legs and TWO arms. Its huge dark stone/plated arm is anatomically on the side shown on imageLEFT in the original front view; the other arm is pale flesh with long claws. Preserve this ownership while turning; never swap materials across frames.',
8: 'Lean dark red creature, TWO short walking legs and TWO long clawed forearms/hands, tall curved pale/red head crest, pale bony back ribs. Its crest is not the nose; keep the existing front and curved crest distinct.',
10: 'Human hybrid with TWO legs, mustard/yellow trousers, dark sleeveless upper clothing, pale flesh arm, and ONE rigid cylindrical gun replacing the arm shown imageLEFT in the original. Keep the gun on the same anatomical arm, muzzle straight and complete; other hand remains flesh.',
11: 'Pale skeletal plated creature, TWO weight-bearing legs, TWO clawed arms, and original paired tall tubular/bony dorsal structures. These dorsal structures are rigid appendages, never extra arms or walking legs.',
12: 'Pale skeletal biped, TWO legs and TWO long arms with large rust/orange claws, dark recessed head/hood, ribbed upper torso. Preserve the heavy claws and original dark gaps.',
14: 'Human red-hooded cultist, TWO booted legs and TWO arms, dark gray sleeves and trousers, long red front tabard, short straight knife in the originally shown hand. Preserve the face and actual weapon ownership.',
15: 'Human red-hooded cultist, TWO legs and TWO arms, gray dark clothes and red tabard; original long straight staff with a small purple top held in the same anatomical hand. Staff stays rigid, never curved.',
16: 'Red-hooded human, TWO legs and TWO arms, original long gray/red rectangular SHIELD and short straight blade in opposite hands. Preserve shield/blade ownership across both orientations, no mirroring asymmetric equipment.',
17: 'Human red-hooded cultist with long dark staff and small original top ornament, dark clothes and long red tabard. TWO legs and TWO arms; shaft stays rigid and hand grip connected.',
22: 'Red hooded human gunner, TWO legs and TWO arms, dark trousers and sleeves, red hanging cloth, actual long dark rifle held horizontally in TWO hands. Straight rigid barrel, fixed grips, no added third arm.',
24: 'Red hooded human gunner, TWO legs and TWO arms, original dark long firearm held across the torso in TWO hands, gray/dark clothing and red cloth. Keep this exact weapon design, rigid shaft and connected grips.',
25: 'Lean pale bald human hybrid, TWO legs and TWO long hanging arms, dark torn sleeveless garment/trousers, original visible bare pale foot versus dark covered foot. Preserve original clothing and limb asymmetry, no new metal armor.',
26: 'Short pale bald human hybrid, TWO bent legs in gray trousers, TWO long hanging dark forearms, ochre/yellow short upper garment, heavy dark footwear. Preserve the short crouched proportions.',
28: 'Broad olive/brown fleshy biped, TWO weight-bearing legs and TWO long arms, humped head and shoulders, pale folded belly and dark fingers. No horns or new armor; compact low heavy gait.',
32: 'Small pale-bellied dark red/brown biped, TWO small legs and TWO short clawed arms, long curved head/back protrusion, original near-round belly. Preserve the short proportions; head crest never defines reverse facing.',
34: 'Small orange/red biped, TWO thin clawed legs and TWO long narrow arms, angular horned head and small hunched back. No new wings, tail or extra fingers/limbs.',
36: 'Slender dark blue/gray biped, TWO long thin legs and TWO long clawed arms, original elongated angled head/back crest and small pale upper torso patch. No new wings or cape.',
60: 'Slender red sinewy biped, TWO walking legs and TWO long clawed arms, small original upper head and narrow torso. Keep its long forearm proportions and claws without splitting widely.',
61: 'Lean brown/pale biped, TWO thin legs and TWO long arms, dark angular head/back crest and pale exposed torso panels. Keep original crest and hand shapes, no added wings.',
62: 'Pale red/bone lean biped, TWO legs and TWO long clawed arms, pale pointed upper head and red inset torso. Preserve original narrow limbs and pale/red material regions.',
68: 'Heavy orange/brown plated biped, TWO weight-bearing legs and TWO large long clawed arms, tall curved dark head/back shell and orange folded torso. Keep the dark shell rigid, original large shoulders and claws.',
72: 'Stocky pale biped with red exposed upper back/shoulder mass, TWO legs and TWO long arms, pale ribbed belly and small recessed face. No additional eye, face or armor.',
73: 'Pale/red biped, TWO bent legs and TWO long hanging arms, red shoulder mantle/flesh, original large pale rounded head and pale belly. Retain its hunched neck and all original asymmetry.',
74: 'Tall dark blue/gray lean biped, TWO legs and TWO long clawed arms, orange-brown chest inset, elongated upright curved head/back crest. Preserve claw tips and original narrow head.',
77: 'Heavy pale biped, TWO large feet and TWO long heavy forearms/hands, small recessed dark head, red sternum/waist insets. Preserve broad pale shoulders and original large claws.',
78: 'Heavy dark blue-gray and pale biped, TWO legs and TWO large clawed arms, tall solid dorsal/head plate reaching upward. Keep the central plate rigid with the exact original silhouette, not a helmet or extra head.',
79: 'Red sinewy biped, TWO thin legs and TWO long clawed arms, pale inset chest and tall pointed dark red head crest. No new wings, tail or added armor.',
80: 'Heavy pale/red biped, TWO broad feet and TWO huge gray forearms/claws, pale skull-like head, red shoulder and groin regions. Keep each original material and large limb proportions.',
82: 'Exactly FOUR walking legs: long red/dark quadruped body, original black hooves/paws, pale curved beak/skull head, dark ridged upper back and long red tail. No humanoid arms or extra feet; compact four-foot gait with stable back.',
84: 'Pale hunched biped with TWO short weight-bearing legs and TWO long forearms/hands, folded pale upper back, dark red recessed front/belly. Keep original draped/folded body shapes, not a newly invented robe or quadruped.',
85: 'Tall pale biped, TWO long legs and TWO long thin arms, original large dark red ridged upper back/head mound. Keep its unusual hump attached and stable, without changing body height.',
86: 'Tall red/pale muscular biped, TWO legs and TWO long curved clawed arms, dark/pale recessed face and pale chest inset. Preserve the original large curved hand/claw tips.',
93: 'Broad pale muscular biped, TWO thick legs and TWO large clawed arms, red/brown head and shoulder covering, pale belly. Keep original body bulk, small head and broad clawed feet.',
96: 'Pale lean human hybrid, TWO legs in dark trousers and boots, TWO long bare hanging arms, original small head and exposed torso. Keep original clothing boundaries and hunched neck.',
98: 'Low very broad flesh/orange biped, TWO small weight-bearing hind legs/feet and TWO large ground-near forearms/hands; pale ribbed round belly, orange ridged shoulders. Preserve its original low silhouette; do not add four animal legs.',
99: 'Heavy pale/orange biped, TWO legs and TWO massive charcoal plated forearms, orange upper shoulders and small recessed head. Preserve dark arm plates on BOTH arms and original pale bare torso.',
100: 'Very slender dark/pale biped, TWO long narrow legs and TWO arms, original tall pointed pale/black head, angular shoulder plates and dark lower torso. No new robe, wings or staff.',
102: 'Heavy pale biped, TWO legs with dark lower supports and TWO massive dark forearms/claws, recessed dark small head and pale segmented abdomen. Stable broad shoulders and fixed muscular mass.',
105: 'Tall pale/red biped, TWO legs and TWO long hooked arms, red upper torso/shoulder mass and long pale angular head crest. Preserve long head shape and small dark claw ends.',
106: 'Low crouched pale/brown creature with TWO rear legs and TWO long forelimb/hand supports (FOUR contacts total). Original dark brown ridged upper body and lowered pale head; creeping gait, not upright human walking or six-legged insect redesign.',
108: 'Thin pale biped, TWO legs and TWO arms with the original large angular brown wing-like upper appendages. Preserve all original appendages and their attachments; no extra paired arms or new wings beyond INPUT1.',
111: 'Asymmetrical broad pale biped, TWO legs and TWO clawed arms, red/orange chest region and original huge dark/red armored shoulder/arm structure on the side shown imageLEFT in the source. Preserve anatomical material ownership in all poses; never swap the large plated side.',
112: 'Tall narrow pale/dark biped, TWO thin legs and TWO long tapered arm structures with the original hanging blade/cloth-like shapes. Preserve actual attachments and count from INPUT1, narrow pointed head and pale front; no invented bat wings or extra arms.',
115: 'Tall pale/dark biped, TWO thin legs and TWO long arms, original single long dark angled head crest and dark side plates. Keep the crest intact and the original lean proportions.',
116: 'Possessed human in red hood, gray sleeves/trousers and red front tabard, TWO legs and TWO arms with long pale clawed hands. Preserve original face and sleeve lengths.',
117: 'Possessed human in long red hooded coat/tabard with dark gray sleeves and trousers, TWO legs and TWO pale clawed arms. Keep the original long coat hem and narrow hooded head.',
119: 'Possessed human in red hood/waist cloth and dark gray sleeves/trousers, TWO booted legs and TWO long pale clawed arms. Preserve the original uncovered lower forearm extent.',
120: 'Bald pale human zombie, TWO booted legs and TWO arms, original torn charcoal long-sleeved work outfit with ochre chest band. Preserve hunched neck and actual clothing; no weapons or new wounds.',
121: 'Pale human zombie, TWO legs and TWO arms, black/dark trousers and short dark shirt with original orange torn front vest/strip. Preserve dangling hands, original face and clothing boundaries.',
123: 'Very stocky bald pale human zombie, TWO legs and TWO arms, charcoal work suit and original ochre/yellow panel on one side of the torso/leg. Preserve that panel on the same anatomical side; no material swapping or inflation.',
128: 'Human skeleton, TWO bony legs and TWO bony arms, original charcoal torn outfit with small orange shoulder patch, boots and exposed skull. Preserve bones, empty eye sockets and patch ownership, no extra armor.',
129: 'Mostly bare beige skeleton, TWO bony legs and TWO bony arms, original ribcage, pelvis, dark small waist scraps and skull. Keep each joint connected and bone count stable; no new clothes, armor or weapon.',
130: 'Dark rocky biped with TWO weight-bearing legs and TWO heavy arms; original orange flames attached to head and shoulders and orange torso fissures. Preserve existing fire only, no new aura, projectile or floor effects. Rigid body mass and grounded compact gait.',
132: 'Human scientist cultist, TWO legs and TWO arms, red hood, pale off-white lab coat over red/dark clothing, boots; one raised small dark device/instrument and the other holding the original dark book/case. Preserve exact object ownership and all straight rigid edges.',
133: 'Human scientist cultist, TWO legs and TWO arms, pale lab coat, red hood and dark trousers; original small handheld objects as in INPUT1. Do not copy props from other cultists; keep coat hem and existing grips.',
134: 'Human scientist cultist in pale lab coat and red hood, TWO legs and TWO arms, holding the original small green/teal object before the torso in both hands. Preserve its rigid shape and connected hands, no extra arm or staff.',
135: 'Human scientist cultist, TWO legs and TWO arms, red hood and long red front garment with original gray/pale coat panels and small dark handheld device. Preserve exact coat asymmetry and real object, no copied book.',
136: 'Tall thin pale skeletal necromancer, TWO bony legs and TWO arms, original ragged purple/dark robe and skull-topped rigid staff in its canonical hand. Skull on staff is an object, not a second head; retain actual small hooded head and stable grip.',
137: 'Small hunched horned necromancer, TWO thin lower legs and TWO bent arms, original purple hood/cloth and pale exposed bones/flesh, dark handheld object held at abdomen. Preserve its hunched short torso, head horns and object.',
138: 'Broad short necromancer, TWO small legs and TWO arms, pale round belly, red draped cloak, skull-topped staff on its original side and dark rectangular book in the other hand. Keep staff rigid, both props distinct, cloth not extra limbs.',
152: 'Heavy pale yellow/green biped, TWO legs and TWO long clawed arms, original lumpy shoulders, small horned head and swollen pinkish belly. Constant mass and fixed original lumps, no new particles, aura or wounds.',
159: 'Tall stocky ivory/pale blue biped, TWO heavy legs and TWO long arms with large blue crystalline forearm claws, original broad ivory horns and dark recessed face. Keep the icy plates rigid, no new frost particles or wings.',
160: 'Lean pale humanoid with TWO legs, TWO arms, TWO separate small dark red/brown bat wings and ONE curved tail; original dark robe/cloth with ochre crossed trim. Preserve actual horns and tail attachment, grounded two-foot walk with wings carried.',
161: 'Lean red humanoid, TWO legs and TWO arms, original charcoal clothing/armor panels, small dark horns and curved dark tail. Preserve actual red face and leg anatomy, no bat wings or new weapons.',
164: 'Red hooded human with long dark coat/tabard, TWO legs and TWO arms, original long rigid horizontal rifle held in both hands. Preserve exact rifle geometry and stock, two connected grips and actual gray/dark clothing.',
165: 'Red hooded human, TWO legs and TWO arms, original large two-handed axe with long straight shaft and gray metal blade held diagonally. Shaft and axe head remain rigid and symmetrical within their original design, both grips connected; no new weapon or shield.',
}

def main():
    records = json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf8'))['records']
    base = {r['number']: r for r in records if r['id'].startswith('MON-') and r['variant'] == 0}
    template = json.loads((DOC/'creature-jobs-lot11.json').read_text('utf8'))[1]
    grouped = []
    for serial, (number, detail) in enumerate(DETAILS.items(), 63):
        record=base[number];key=record['id'];lot=12+(serial-63)//4
        guide = 'creatures-lot11/MON-156-0'
        if number in [14,15,16,17,22,24,116,117,119,120,121,123,128,129,132,133,134,135,164,165]: guide='creatures-lot07/MON-155-0'
        if number==82: guide='creatures-lot05/MON-101-0'
        if number==106: guide='creatures-lot08/MON-118-0'
        if number in [108,160]: guide='creatures-lot11/MON-162-0'
        refs=[DOC/'references'/f'{key}-identity-detail.png',SRC/'accepted'/f'{guide}-whole-body-native.png']
        assert all(p.exists() for p in refs),key
        motion=detail+' Preserve anatomical ownership of asymmetric equipment while turning the complete figure. Restrained compact support changes, minimal bob, same apparent height throughout. True resting idle with stable support. INPUT2 guides movement ONLY: never copy its head, species, horns, armor, materials, clothes, weapons or appendages.'
        prompt=template['prompt'].replace(template['id'],key).replace(template['motion'],motion)
        prompt_path=DOC/'prompts'/f'{key}-bulk-r01.txt'
        assert not prompt_path.exists(), 'Preserve existing prompt'
        prompt_path.write_text(prompt,'utf8')
        legs=4 if number==82 else 2
        job={'id':key,'review_id':f'WALK07-M{serial:02}','kind':'creature','label':record['family']+' '+f'{number:03}',
             'record':record,'locomotor_legs':legs,'motion':motion,'reference':str(refs[0]),'references':[str(p) for p in refs],
             'source_destination':str(SRC/f'{key}-cycle-bulk-r01.png'),'source_columns':7,'source_rows':2,
             'selected_source_indices':list(range(14)),'shadow':True,'strict_source_outer_margin':True,
             'prompt_file':prompt_path.relative_to(ROOT).as_posix(),'prompt':prompt,'bulk_preparation':True}
        if number==106: job['support_label']='4 appuis : 2 mains et 2 pieds'
        grouped.append((lot,job))
    for lot in sorted({lot for lot,_ in grouped}):
        path=DOC/f'creature-jobs-lot{lot:02}.json';assert not path.exists()
        path.write_text(json.dumps([j for n,j in grouped if n==lot],ensure_ascii=False,indent=2)+'\n','utf8')
    manifest={'review_id':'WALK07-BULK-PREPARATION','player_quote':"en faite tu pourrais surement toutes les faires , puis je vérifierais comme je le fais actuellement , et je te dirais si y'en a à modifier ou non :)",
              'runtime_integration':False,'visual_acceptance':False,'scope':'Remaining campaign base identities, with material variants tracked separately; existing deferred cases are not silently accepted.',
              'jobs':[{'id':j['id'],'review_id':j['review_id'],'lot':lot,'status':'pending_generation'} for lot,j in grouped],
              'deferred_ids':['MON-029-0','MON-127-0','MON-151-0']}
    (DOC/'bulk-preparation.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'jobs':len(grouped),'lots':sorted({n for n,_ in grouped}),'runtime_integration':False}))

if __name__=='__main__':main()
