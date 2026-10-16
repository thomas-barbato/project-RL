"""Prepare proposal metadata and reference crops; never writes active assets."""
from pathlib import Path
from PIL import Image
import json, hashlib

ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
for sub in ('sources','tiles','references','review'):
    (OUT/sub).mkdir(exist_ok=True)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
baseline={str(p.relative_to(ROOT)).replace('\\','/'):digest(p) for parent in ('src','content','assets','docs','tools/art') for p in (ROOT/parent).rglob('*') if p.is_file() and '__pycache__' not in str(p)}
(OUT/'baseline.json').write_text(json.dumps(baseline,indent=2),encoding='utf-8')
specs=[
('Monstrueux','Écorché','Eyeless hunched humanoid; huge ivory shoulders, small sunken head, thick crimson forearms, dark purple belly, two short digitigrade legs; empty clawed hands.',(30,30)),
('Monstrueux','Gueulard','Squat broad orange-to-rust beast on two legs with a single enormous vertical ivory-toothed mouth replacing its head, dark blue heavy forearms, no eyes.',(30,28)),
('Monstrueux','Rampant','Long low six-limbed flesh crawler, pale mint ribbed carapace and substantial purple soft belly; hooked front claws and small eyeless snout. Side three-quarter view.',(30,19)),
('Monstrueux','Crâne-fendu','Lean asymmetric charcoal mutant, clearly split ivory skull, one oversized blue claw arm, one short crimson arm, two thin legs; clear negative spaces.',(25,30)),
('Lovecraftien','Œil des caves','Radial floating fleshy eye creature; one large ivory eyeball with purple iris, four thick turquoise dangling tentacles with pale inner surfaces. Transparent gaps between tentacles.',(29,27)),
('Lovecraftien','Noueur','Headless creature made of three thick interlaced ochre and deep violet fleshy ribbons around one large EMPTY transparent central cavity; thick lower loops contact the ground.',(28,27)),
('Lovecraftien','Porte-bouche','Upright cyan pear-shaped creature on two sprawling purple fleshy leg-fins; its stomach is one huge vertical ivory-toothed black maw, small stalk head without face.',(26,30)),
('Lovecraftien','Nageur du vide','Compact dark blue hovering fish-like monster, a split ivory beak, broad orange membranous fins and three trailing purple thick appendages; side three-quarter view, no halo.',(30,23)),
('Humains et possédés','Pillard','Human full height front view, mustard hood with visible face, dark teal vest, ochre sleeves, plum trousers, short metal club held correctly down in right hand. Boots and head separated.',(24,30)),
('Humains et possédés','Fusilier','Human front view, angular blue helmet with narrow dark opening, large ivory tactical vest, blue trousers and sleeves, charcoal rifle held horizontally across chest in BOTH hands.',(29,30)),
('Humains et possédés','Possédé','Possessed human with pale gray skin, bent neck, tiny white eyes, large crimson torn jacket and dark turquoise trousers, empty bent hands, slight stoop. Same human height.',(24,30)),
('Humains et possédés','Fanatique','Human in deep violet hooded long coat with large ivory front panel and covered face; heavy short rusty cleaver held by proper handle in one hand, other hand empty. Industrial cult style.',(25,30)),
('Humains robotisés','Greffé','Human face exposed on one side, one ivory plate over the other half, one cobalt mechanical shoulder and arm, broad brick red work vest, navy legs. Bare other arm, no weapons.',(25,30)),
('Humains robotisés','Mâchoire d’acier','Bald human with oversized dark steel lower jaw, purple armored torso, wide ochre belt and trousers, two mechanical shins; clawed metal hand clearly attached to wrist.',(26,30)),
('Humains robotisés','Porte-sonde','Human wearing ivory long technician coat with substantial teal sleeves and trousers; one compact blue cranial apparatus, one steel telescopic forearm ending in thick blunt medical probe.',(26,30)),
('Humains robotisés','Marcheur soudé','Human upper torso in mustard industrial chest shell and burgundy sleeves, clearly organic head; lower body replaced by TWO thick gunmetal articulated legs and broad cobalt feet.',(28,30)),
('Démons et mages','Cornu','Stocky organic demon with broad dark crimson torso and calves, two large curved ivory horns, substantial blue loincloth and forearms, blunt claw hands, NO armor or weapon.',(29,30)),
('Démons et mages','Mage des cendres','Thin horned mage in charcoal long robe with broad violet inner panel, pale skull-like face and small crimson horns; short dark-metal staff with ONE chunky orange ember at top held in hand.',(27,30)),
('Démons et mages','Sorcière du sang','Demonic feminine silhouette with sweeping teal horns, crimson heavy robe and broad ivory center panel, one pale claw hand raised and one lowered; no particle effects or magic halos.',(27,30)),
('Démons et mages','Dévoreur','Four-legged muscular purple demon, massive ivory bony head, broad copper back plates, large blunt teeth and thick short tail, low predatory three-quarter side posture.',(30,24)),
('Robots et androïdes','Broyeur','Compact ochre industrial robot supported by TWO black continuous caterpillar tracks; large rectangular blue body with a single ivory vertical central panel; two short attached crusher arms.',(30,25)),
('Robots et androïdes','Veilleur','Four-legged low gunmetal security robot with clearly separate supports, cobalt squat body, one large ivory round sensor on front and red lens. No floating guns.',(29,24)),
('Robots et androïdes','Androïde','Front-facing human-height angular robot, cobalt chest and limbs, ivory face and large ivory pelvis plates, dark charcoal joints, one short squared metal hand each side. No visor helmet.',(23,30)),
('Robots et androïdes','Araignée d’acier','Six-legged low mechanical spider, deep violet armored round body with TWO broad ivory plates, copper hinge joints, one small orange central eye; all six legs distinct.',(30,20)),
('Robots-démons','Cornu de fer','Organic crimson demon head with two unequal ivory horns, broad deep teal mechanical torso, one large exposed crimson fleshy arm and one dark steel clamp arm, armored boots.',(29,30)),
('Robots-démons','Prêcheur bionique','Thin horned ivory demonic face above burgundy long robe with LARGE turquoise chest harness; one mechanical black forearm and a thick bent steel spine behind neck, clear human scale.',(26,30)),
('Robots-démons','Gueule mécanique','Low four-legged organic purple monster with copper machine jaws, large ivory armored back shell and two organic plus two mechanical thick legs; unmistakable hybrid body.',(30,24)),
('Robots-démons','Ailé d’acier','Compact red demon torso, two blue steel wings folded upward as broad angular plates with ivory leading edges, two mechanical dark legs and pale claws. No fine feathers or bolts.',(30,30)),
('Boss · 64 px','Le Colosse','Giant monstrous hunched biped, enormous ivory armored shoulders and skull, thick deep purple body, huge crimson forearms reaching ground, blunt thick claws. NOT a scaled-up small enemy; native 64 pixel clusters.',(59,59)),
('Boss · 64 px','Le Démon enchaîné','Huge broad crimson demon with four heavy curled ivory horns, charcoal chest shell and two thick cobalt broken industrial chains attached to wrists, ivory claws, short heavy legs.',(59,60)),
('Boss · 64 px','Le Char possédé','Huge TWO-track ochre battle chassis, large central exposed purple demonic maw and ivory teeth, broad teal top shell, one short cannon organically attached at one side. Clear track supports and hybrid identity.',(60,49)),
('Boss · 64 px','La Matrice des yeux','Huge pale ivory three-lobed organic mass with THREE large violet eyes, broad turquoise flesh beneath, six thick purple tentacles touching the floor with transparent negative spaces. No dozens of tiny eyes.',(60,57)),
]
catalog=[]
for i,(family,name,description,bounds) in enumerate(specs,1):
    catalog.append(dict(number=i,review_id=f'proposal:{i:03}',content_id=None,status='proposal',family=family,name=name,name_status='provisoire',description=description,tile_size=64 if i>=29 else 32,maximum=list(bounds),source=f'sources/{i:03}.png',living=f'tiles/{i:03}-vivant.png',corpse=f'tiles/{i:03}-cadavre.png',corpse_includes_blood=True,can_equip='Humains' in family,anchors_status='review; future equipment compatibility must be authored before integration'))
names=json.loads((ROOT/'artifacts/crawl-methode-2026-10-05/bestiary.json').read_text(encoding='utf-8'))[:23]
old_ids=['core:rubble_nibbler','core:friche_biter','core:bone_breaker','core:vibration_burrower','core:moss_grazer','core:ruin_bat','core:armored_worm','core:cave_anemone','core:fault_howler','core:sentinel_projector','core:riveter','core:cave_scaled','core:watcher','core:spectre','core:marsh_spitter','core:laggard','core:void_maw']+[None]*6
old_sizes=[(20,10),(30,14),(30,16),(23,11),(27,11),(30,12),(30,15),(30,13),(28,16),(29,16),(28,16),(29,14),(28,14),(28,14),(30,12),(30,12),(28,14)]+[(30,14)]*6
atlas=Image.open(ROOT/'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
for index,name in enumerate(names):
    n=33+index; cell=index+3;x=cell%16*32;y=cell//16*32
    tile=atlas.crop((x,y,x+32,y+32))
    tile.save(OUT/f'tiles/{n:03}-vivant.png')
    tile.resize((512,512),Image.Resampling.NEAREST).save(OUT/f'references/{n:03}.png')
    catalog.append(dict(number=n,review_id=f'existing-art:{cell}',content_id=old_ids[index],status='existing-art-corpse-proposal',family='Bestiaire existant',name=name,name_status='existant',tile_size=32,actor_index=cell,maximum=list(old_sizes[index]),source=f'sources/{n:03}.png',living=f'tiles/{n:03}-vivant.png',corpse=f'tiles/{n:03}-cadavre.png',corpse_includes_blood=True,living_sha256=digest(OUT/f'tiles/{n:03}-vivant.png'),reference=f'references/{n:03}.png'))
(OUT/'catalog.json').write_text(json.dumps(catalog,ensure_ascii=False,indent=2),encoding='utf-8')
print(f'{len(catalog)} numbered entries, {len(baseline)} active-file fingerprints; living references copied unchanged.')
