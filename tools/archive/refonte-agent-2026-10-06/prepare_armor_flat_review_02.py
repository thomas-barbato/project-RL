"""Versioned review setup: distinct outfit designs, approved anatomical rig."""
from pathlib import Path
import json, hashlib

BASE = Path(__file__).resolve().parent
OUT = BASE / 'armor-flat-review-02'
PREVIOUS = BASE / 'armor-flat-review-01'
ROOT = BASE.parents[1]
OUT.mkdir(exist_ok=True)
(OUT / 'sources').mkdir(exist_ok=True)
old = json.loads((PREVIOUS / 'catalogue.json').read_text('utf-8'))
for path, expected in old['protected_active_files'].items():
    assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, path

designs = [
 ('AR-N07', 'Bastion · départ léger', 'new_starter', 'light',
  'A simple close-fitting charcoal ballistic jacket with broad ivory fabric chest and sleeves, compact dull copper shoulder caps, olive work trousers and low practical boots. The head is a short, squared copper protective skullcap with a clearly exposed separate dark-grey dual-filter respirator and two round glass eye lenses, NOT a closed T-visor helmet. Soft waist and unplated thighs, modest beginning equipment. Square chest silhouette, short jacket hem. No knight armor or elaborate full-body plating.',
  ['ivory','olive','petrol','bone','red','steelblue']),
 ('AR-N08', 'Technomancien · départ léger', 'new_starter', 'light',
  'A dark petrol cropped technical jacket with a broad light-grey shoulder yoke, graphite trousers, plain fabric sleeves and a compact integral harness with two large visible equipment sockets on the chest. A distinctive ROUND high-domed smoked-glass protective face bubble with a low steel rim, small breathing unit below the chin and a short black fabric neck shroud: NOT an angular combat helmet and NOT a T visor. Practical futuristic technician, no building-site overalls, no hardhat, no full plate, no military pauldrons. A few wide orange fabric accents at cuffs and waist. Light and inexpensive starting equipment.',
  ['workpetrol','orange','olive','bone','violet','steelblue']),
 ('AR-N09', 'Traqueur · départ léger', 'new_starter', 'light',
  'A narrow cobalt-blue short triangular shoulder poncho over a close-fitting charcoal tactical fabric outfit, a broad sand-grey chest panel and long simple cloth trousers. Low quiet boots with no large shin plates. The head has a snug angular blue cloth hood with a single wide COPPER goggle band across two oval lenses and a folded charcoal face scarf underneath, visibly fabric rather than an armored bucket helmet. Small shoulder width, straight lean lower silhouette, readable short pointed poncho corners and a simple high fabric collar. Modest beginning equipment, no full-body armor, no long cape or backpack.',
  ['clothblue','bone','olive','ivory','red','steelblue']),
 ('AR-N10', 'Arcaniste · robe légère de départ', 'new_starter', 'light_mage',
  'A vivid RED soft-fabric occult robe with a broad black vertical front sash and an ivory throat bib, long straight folded panels ending above simple dark boots. No armored chest, no plated legs, no metallic pauldrons. A tall angular cloth hood with a flat pale ritual mask inside: two narrow VERTICAL black eye openings and an uninterrupted pale lower face, absolutely no horizontal visor or armored T faceplate. Wide loose upper sleeves taper to the common wrists. Broad robe silhouette hides the thighs. A single large angular black ritual motif in the red fabric, without small runes or glow. Humble beginning mage clothing, genuinely a robe rather than repainted tactical armor.',
  ['red','bone','blue','ivory','olive','bone']),
 ('AR-N11', 'Métamorphe · départ organique léger', 'new_starter', 'light',
  'A simple ochre-olive wrap tunic with a wide folded hem and charcoal fabric trousers, short dark boots and soft sleeves. Sparse ivory curved organic collar and two broad leaf-like shoulder membranes, not metal combat plates. Head completely covered by a smooth elongated IVORY organic face mask, two deep oval vertical eye hollows, no visor, no angular knight jaw, with an irregular olive cloth hood open around its edges. Subtle bone clasp at the waist, spare organic seams on the tunic. A light occult initiate outfit that anticipates the mutation mantle without copying its complete advanced armor, no horned robotic helmet, no cyber superhero plating, no claws or transformation anatomy.',
  ['mutedolive','bone','petrol','ivory','violet','bone']),
 ('AR-N12', 'Mage · armure moyenne rituelle', 'new_progression', 'medium',
  'A dark INDIGO long split ceremonial coat with a broad IVORY rigid front panel, restrained steel rib guards over its sides and a wide stiff circular collar. No normal military shoulder plates. The head is a round ivory ceremonial protective mask with THREE vertical dark red apertures, enclosed in a shallow round indigo cowl: round dome and round face, no T visor, no pointed hood. Coat lower panels open into two broad geometric tails; charcoal trousers and steel toe caps below. Broad sculptural dark ritual bands, no tiny decoration. A clearly intermediate occult protective outfit, more structured than the light red robe, but not a full combat armor.',
  ['indigo','ivory','red','bone','olive','ivory']),
 ('AR-N13', 'Combat · armure lourde industrielle', 'new_progression', 'heavy',
  'A brutal RED and dark charcoal industrial protective suit: broad one-piece rectangular chest slab, two clearly separated steel shoulder blocks, thick but human-sized cloth elbows, simple squared thigh guards and broad steel boot fronts. The head is a low WIDE rounded industrial helmet with a recessed oval dark viewing window, two thick round side filters and a high mechanical steel collar. No medieval visor, no T face slit and no peaked forehead. Three wide diagonal red-black protective strips on the abdomen, not fine hazard lines. A blocky high-mass chest and wide shoulder silhouette, unmistakably heavy industrial armor rather than a recolored fitted superhero bodysuit. No robot anatomy.',
  ['red','steelblue','olive','bone','petrol','orange']),
 ('AR-N14', 'Combat · armure très lourde', 'new_progression', 'very_heavy',
  'A massive industrial protective outfit in MUSTARD YELLOW and steel grey, remaining a human wearing armor on the identical anatomical rig. A bulky barrel-shaped chest casing with a very high enclosing collar, broad rounded shoulder shells and thick rectangular shin housings. The small head sits visibly recessed inside the collar under a squat flat dark protective cap, only two small circular ivory viewing lenses visible; no exposed standard helmet shape, no T visor and no angular knight mask. A broad light-steel abdominal panel and graphite flexible joints. Boot housings large and square. Strong barrel silhouette, substantially more protective than the red heavy suit, but hands and joint centers remain unchanged, no stretched limbs or mechanized third limbs.',
  ['yellow','steelblue','petrol','bone','red','steelblue']),
]

shared = ('Use case: stylized-concept. Asset: one transparent outfit source sheet, exactly four full-body copies in an equal 2 by 2 grid. '
 'Image 1 is ONLY an anatomical pose and scale guide. Copy its joint positions and its cell layout exactly, NEVER its helmet, armor plates, chest shape, boots or ivory clothing. '
 'Change the actual clothing silhouette, head construction, hem, materials and protection volumes according to the design below. This must be a genuinely distinct outfit, not the guide repainted. '
 'Preserve human head height, leg length, shoulder centers, elbow centers, wrist centers, hand centers and floor line of the guide. '
 'Upper-left: independent empty clenched hands extended outward at waist height, forearms horizontal. Upper-right: rifle grip pose. Lower-left: horizontal heavy weapon grip at waist. '
 'Lower-right: two-handed diagonal melee grip, character left hand/viewer right upper grip turned upward/underhand, character right hand/viewer left lower grip turned downward/overhand. '
 'Draw NO actual weapons, rods, handles, shields or held objects. Dark charcoal rounded gloves with broad visible fingers, never black square hand blocks. '
 'STRICT frontal, no three-quarter rotation. Identical head, torso, lower clothing, legs and feet in all four cells; only arms change. '
 'Bande dessinee ink rendering of high quality: clear black contours, bold flat masses, two or three broad cel shadows per material, clean readable graphic forms, no photoreal texture or micro details. '
 'Match guide scale and margins in each cell, fixed human stature regardless of bulky clothes. Clothing remains continuous behind the moving arms. '
 'Plain true transparent background, no floor, shadow, halo, particles, text, labels, frames, pink or magenta. '
 'Outfit design: ')
entries=[];specs=[]
for ident,name,category,tier,brief,materials in designs:
    entries.append(dict(id=ident, old_review_id=None, content_id=None, name=name,
        kind='outfit', category=category, proposed_tier=tier, source_reference=None,
        source=f'sources/{ident}-poses.png', canonical=None, status='new_design_pending_review',
        brief=brief, materials=materials, variants=['Référence','Variante 2','Variante 3']))
    specs.append(dict(id=ident,prompt=shared+brief,
        references=[str(PREVIOUS/'references/common-four-poses.png')],
        output=str(OUT/f'sources/{ident}-poses.png')))
ref=next(e for e in old['entries'] if e['id']=='AR-N06').copy()
ref.update(category='quality_reference', source=str(PREVIOUS/ref['source']),
           status='quality_appreciated_design_not_yet_accepted_for_integration',
           replaces_candidate=None, proposed_tier='advanced_reference',
           materials=['olive','bone','petrol','ivory','violet','bone'])
entries.append(ref)
catalogue={**old,'review_id':'ARMOR-FLAT02','entries':entries,
    'scope':'Eight distinct review proposals, five light starts and three progression outfits. Appreciated mutation mantle retained unchanged.',
    'withdrawn_outfit_ids':['AR-02','AR-04','AR-13'],
    'pending_specific_revisions':{'AR-08':'Metallic/copper helmet without blue contamination',
        'AR-10':'Reference palette to revise','AR-11':'Replace cyan/red pairing',
        'AR-12':'Remove or completely redesign as futuristic technical clothing'},
    'active_game_content_changed':False,'inventory_piece_variants':False,
    'visual_acceptance_pending':True}
(OUT/'catalogue.json').write_text(json.dumps(catalogue,ensure_ascii=False,indent=2),'utf-8')
(OUT/'generation-specs.json').write_text(json.dumps({'mode':'built-in imagegen','specs':specs},ensure_ascii=False,indent=2),'utf-8')
print(json.dumps({'folder':str(OUT),'new_designs':len(specs),'ids':[s['id'] for s in specs]}))
