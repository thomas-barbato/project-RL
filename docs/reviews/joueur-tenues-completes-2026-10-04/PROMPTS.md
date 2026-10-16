# Tenues completes du joueur

Mode : imagegen integre. Sources PNG conservees dans ce dossier. Les prompts ci-dessous incluent la variante de bras leve ecartee et sa correction.

## side

Use case: style-transfer. Asset type: production transparent pixel-art full-character sprite sheet for an original French cyberpunk/post-apocalyptic roguelike.
Input image 1 is the edit target and visual style reference: 14 existing armor torso models in a 4-column by 4-row sheet, last two cells empty.
Primary request: turn EVERY model into a fully coherent complete outfit, replacing the naked face with a matching closed helmet or hood/mask, and replacing ALL red class-colored legs/boots with matching material from that torso. Draw integrated hands, short natural arms, trousers or long robe, knees and boots. Not pasted equipment icons. Each outfit is a single character.
Keep EXACT layout: equal 4x4 cells, 14 figures read row-major, last two empty. Each full figure centered, same height and baseline, front view, feet apart, arms hanging naturally with clenched gloved hands at hip level, hands not at knees. No weapons, no text, real transparent background.
Preserve torso design identity and palette per cell:
1 Cerbere segmented ivory plates with amber details, full ivory/charcoal armor, closed helmet.
2 Cognefer riveted pale industrial metal and brown leather, matching riveted helmet and boots.
3 Chrysalide ivory organic shell over olive bodysuit, cyan seams, matching shell helmet.
4 Composite ivory/graphite angular plates, matching closed helmet and lower armor.
5 Fantome graphite stealth bodysuit with sparse cyan piping, slim visor helmet.
6 Mue brown layered fabric: extrapolate into a long hooded robe reminiscent of a mage, split lower hem to expose two boots, integrated muted mask, no magic props.
7 Bastion heavy ivory armor with amber bands, strong closed helmet, thick matching leg armor.
8 Patched plating salvage metal with distinct rust-orange chest patch: extrapolate into a technician work suit, pockets/tool belt, matching work helmet and gloves.
9 Seconde ombre dark wrapped coat and diagonal bands: extrapolate into a long charcoal hooded robe, two boot tips visible.
10 Membranes olive/ivory layered jacket, matching hood-mask and trousers.
11 Peau seconde tight black/graphite suit with tiny cyan detail, sleek closed helmet.
12 Veille olive utility jacket with pockets and tiny amber patch, field-technician uniform, matching helmet and trousers.
13 Matelassee ivory padded vest, dark sleeves, matching padded helmet, pants and boots.
14 Fibres croisees olive crossed fabric panels and amber buckle, matching head wrapping/helmet and trousers.
Style: preserve the reference's crisp chunky pixel clusters, dark outlines and simple material shading; readable down to 32x32 game tiles. Head and body in exactly same pixel art style, head not enormous. Compact human limb proportions. No class red, blue or green underlayer anywhere in these armor outfits. All faces fully covered, no chin, hair, or exposed skin. Avoid gradients, photorealism, weapons, shadows on background, extra figures, labels.

## base

Use case: style-transfer. Asset type: transparent production pixel-art player full-body sprite sheet.
Input image 1 is the reference/target: three class heroes red (Breche), blue (Vigie), green (Creuset).
Primary request: redraw the three same heroes with integrated closed helmets instead of bare heads, compact human arms and naturally integrated glove hands. Exactly NINE figures in an equal 3-column by 3-row grid. Every row ordered red, blue, green. Helmet uses class color panels and charcoal metal with narrow cyan/amber visor, no visible face/chin/hair.
Row 1: front view neutral combat pose, arms down, clenched gloved hands beside hips at waist-bottom level, never knees.
Row 2: same three bodies/helmets/legs, elbows bent and both hands held in front of abdomen to hold a rifle across body. Their own right hand on image LEFT around lower-left abdomen, their own left hand on image RIGHT a little higher to support the rifle fore-end. Compact fingers wrapped around imaginary grips, not holes in sleeves. Hands roughly at normalized x=.38/.68 and y=.51/.48 from helmet-top to boot-bottom. No actual rifle: leave body continuous between hands.
Row 3: same three bodies/helmets/legs, only image-RIGHT arm bent at elbow with forearm vertical and hand raised beside the helmet, as though holding an upward-pointed pistol. Show glove from its SIDE with curled fingers and thumb, not broad back of the hand. Hand beside head, never overlapping face, normalized x=.76,y=.27. Image-LEFT arm rests naturally with hand at hip, both arms short proportional.
Full-body front silhouettes centered in each equal cell, boots at matching baseline. Match existing chunky dark outlined pixel-art style, simple deliberate pixel clusters readable in 32x32 tiles, coherent shading across head and body, fully transparent backdrop. Keep red/blue/green class torso/leg identities. No text, weapons, props, ground shadows, extra figures, or detached hands.

## two

Use case: identity-preserve. Asset type: production transparent pixel-art full-character sprite sheet, 14 coherent armor outfits.
Input image 1 is the EDIT TARGET: fourteen helmeted complete outfits on equal 4x4 sheet, last two cells empty.
Change ONLY the arm/hand pose in every figure to a TWO-HANDED RIFLE READY POSE. Keep every model's exact helmet, faceplate, torso materials/patterns, robe/legs/boots, class-free colors, pixel style, size and row-major identity. Keep 4x4 equal cells with last two empty.
Each front-view figure has elbows bent naturally at waist level, both compact gloved hands IN FRONT of the lower chest/upper abdomen, grasping an imaginary horizontal rifle across the body. The own RIGHT hand on IMAGE LEFT at normalized x=.38, y=.56 (relative to helmet-top/boot-bottom); own LEFT support hand on IMAGE RIGHT at x=.67,y=.49. Hand heights differ slightly, about one glove-height. Both hands show fingers curled tightly around a grip, hands visibly connected to their sleeve cuffs. Natural short upper arms, no long floppy arms. Hands should lie inside torso width; rifle will be added by the game.
The glove designs belong to each outfit, not independent equipment. KEEP COMPLETE continuous armor/fabric BEHIND the hands; do not remove chest or put circular hollow cuff holes there. Leave no weapon, no barrel, no rifle, no floating props. No text, no extra figures.
Maintain the reference chunky deliberate pixel-art clusters/dark outline and 32x32 readability, fully transparent background. No bare skin/chin, no red class-colored underlayer. Full figure centered at original baseline in each cell.

## raised

Use case: identity-preserve. Asset type: production transparent pixel-art full-character sprite sheet, 14 coherent armor outfits.
Input image 1 is the EDIT TARGET: fourteen helmeted complete outfits on equal 4x4 sheet, last two cells empty.
Change ONLY the arm/hand pose in every figure to an UPWARD ONE-HANDED PISTOL READY POSE. Keep each model's exact helmet, torso materials/patterns, robe/legs/boots, class-free palette, pixel style, size and row-major identity. Keep equal 4x4 cells with last two empty.
Front view. The arm on IMAGE RIGHT is bent at the elbow near ribs; its forearm is vertical up and its compact gloved hand is beside the right edge of the helmet at roughly normalized x=.78,y=.27 relative to full figure. Clearly show the narrow SIDE/EDGE of this hand: thumb and curled finger segments around an imaginary upright grip. The broad BACK of the hand must NOT face the camera. Fingers extend toward the middle of image, thumb near opposite edge. Sleeve ends immediately under the hand, integrated anatomical joint. No bare skin. No weapon or gun silhouette. The hand is fully beside the helmet, never in front of its visor.
The arm on IMAGE LEFT hangs naturally, slightly bent, short proportional arm, glove at hip level (normalized y=.68), never down by knee. Preserve compact human shoulders, elbows and limb lengths. Both glove designs belong to each coherent outfit.
Fully transparent background, same chunky dark outlines and crisp simple pixel clusters with 32x32 readability. Do not add a gun, bar, stick, exposed chin, class shirt, detached hand, labels or extra figures.

## raised_correction

Use case: precise-object-edit. Asset type: transparent 4x4 pixel-art sprite sheet, 14 figures, last two cells empty.
EDIT TARGET image1. IMPORTANT anatomical correction in ALL fourteen figures:
Currently the raised hand is on the LEFT SIDE OF THE PICTURE. It must be on the RIGHT SIDE OF THE PICTURE instead.
Swap only the ARM POSES: arm visible at picture-left must hang DOWN, glove beside hip. Arm visible at picture-right must bend UP with narrow edge-view glove beside right side of helmet. From viewer perspective: DOWN ARM | HELMET/TORSO | UP ARM.
Keep each original helmet, chest decoration orientation, belt, long robes, legs, boots, material palette, figure position/size and grid absolutely unchanged. Do NOT mirror whole figures. Just redraw both arms to exchange the lowered and raised poses, preserving outfit-matched sleeves and cuffs.
Raised glove seen from its narrow SIDE with curled fingertips wrapping imaginary upright grip, no broad back of hand. Short compact upper arms and forearms. Hand connected immediately to sleeve. No gun, no weapon, no stick protruding above fist. Hand beside helmet, not covering visor. Fully transparent background, same crisp chunky pixel art and dark outlines, no text, no extra figures.
