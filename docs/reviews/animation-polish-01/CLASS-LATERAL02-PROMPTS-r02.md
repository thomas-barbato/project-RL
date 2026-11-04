# CLASS-LATERAL02-r02 — vrais appuis et cadence proposée

Correction du fond de la Métamorphe :

```text
Use case: background-extraction. Preserve this EXACT eighteen-sprite game-art sheet: all 9 columns by 2 rows, eight different walking phases plus idle per direction, all character pixels, positions, costume, palette, anatomy and feet. Remove ONLY the noisy semi-transparent gray/red/yellow/black background and speckles around the sprites and between limbs. All background and gutters must become exactly transparent ALPHA ZERO. Remove shadows/floor/halo as background. Keep clean opaque character ink with a crisp neutral dark edge; do not add any white, gray fog or checkerboard texture and do not change the sprite drawing. Genuine transparent alpha output, no new figures or text.
```

Mode : image_gen intégré, édition guidée, alpha transparent.
Image 1 : frontal actuel de la classe (r02/*-reference.png).
Image 2 : r02/eight-pose-guide.png, guide anatomique schématique ; ses couleurs ne sont pas une palette de costume.
Huit poses de marche + repos par côté. Proposition à 650 ms par pas, 1300 ms par cycle ; rendu normal inchangé.

## Bastion

```text
Use case: identity-preserve. Redraw an anatomically readable WALK for a native64 tactical-game character.
IMAGE1 is the exact front costume/morphology reference and is authoritative for head, proportions, coverage, palette and materials. IMAGE2 is an ANATOMICAL KEY-POSE DIAGRAM, not art to copy. The warm leg is the anatomically NEAR leg; the blue leg is the FAR leg. The colors only identify leg ownership: NEVER copy the diagram colors or lines into the sprite.
Create exactly EIGHTEEN complete separated figures in exactly NINE COLUMNS by TWO ROWS. Top row all face LEFT in near-frontal three-quarter view; bottom row all face RIGHT in matching near-frontal three-quarter view. Columns0..7 are EIGHT different walking key poses in the exact order of IMAGE2; column8 is a distinct grounded IDLE. Do not repeat a three-pose shuffle.
Follow the diagram's ACTUAL FOOT AND KNEE POSITIONS, not just torso/arm variation:
0 CONTACT: near heel reaches ahead and touches down; far leg trails with toe pushing.
1 LOAD/DOWN: weight settles onto the near foot; near knee bends, far heel rises.
2 PASSING: near foot bears the body; far knee folds and its foot clearly lifts and passes UNDER the pelvis, with visible floor clearance.
3 UP/REACH: near leg extends behind as the support; far foot reaches forward above the floor with toe slightly up.
4 OPPOSITE CONTACT: FAR heel now leads ahead; near leg trails. Anatomical leg ownership and front/back occlusion must visibly swap compared with column0.
5 LOAD on the FAR foot; NEAR heel lifts behind.
6 PASSING: FAR leg supports, NEAR knee folds and its raised foot passes under pelvis.
7 UP/REACH: FAR support trails and the NEAR foot reaches forward above the floor.
8 IDLE: both feet planted, relaxed grounded stance.
This must read as two real measured steps, not shaking shins, feet sliding together, alternating splits, bouncing, skating or running. The support leg remains continuous from hip through knee to planted ankle. Keep the weight over the supporting foot. Moderate natural stride no wider than shoulder span. Far limb has restrained depth shading; it is not randomly exchanged with the near limb. Arms swing subtly opposite their corresponding legs. Head stays naturally attached to chest with only minimal coherent body rise/fall. No independent head movement, swelling or limb stretching.
Draw whole coherent figures independently; no pasted limb bands or grafted heads. Keep identical class identity and native standing height from IMAGE1. Calm matte pixel clusters, crisp dark contour, no smoothing, blur, glossy gradients, noisy highlights or added detail. Empty hands; no weapons or extra equipment.
Generous clear gutters, one constant scale and baseline for the sheet. GENUINE transparent alpha: all background and gaps alpha ZERO, opaque sprite ink. No floor, shadows, fog, labels, text, lines, checkerboard, bright fringe or extra figures.
Target: Stocky broad adult soldier, compact gray head protection with open face aperture, copper/orange chest, forearm and knee plates, dark blue under-suit, gray pauldrons and dark boots. Preserve the exact mass and helmet/face of the reference. Heavy measured footfalls, modest bent knees and no high-knee marching.
```

## Arcaniste

```text
Use case: identity-preserve. Redraw an anatomically readable WALK for a native64 tactical-game character.
IMAGE1 is the exact front costume/morphology reference and is authoritative for head, proportions, coverage, palette and materials. IMAGE2 is an ANATOMICAL KEY-POSE DIAGRAM, not art to copy. The warm leg is the anatomically NEAR leg; the blue leg is the FAR leg. The colors only identify leg ownership: NEVER copy the diagram colors or lines into the sprite.
Create exactly EIGHTEEN complete separated figures in exactly NINE COLUMNS by TWO ROWS. Top row all face LEFT in near-frontal three-quarter view; bottom row all face RIGHT in matching near-frontal three-quarter view. Columns0..7 are EIGHT different walking key poses in the exact order of IMAGE2; column8 is a distinct grounded IDLE. Do not repeat a three-pose shuffle.
Follow the diagram's ACTUAL FOOT AND KNEE POSITIONS, not just torso/arm variation:
0 CONTACT: near heel reaches ahead and touches down; far leg trails with toe pushing.
1 LOAD/DOWN: weight settles onto the near foot; near knee bends, far heel rises.
2 PASSING: near foot bears the body; far knee folds and its foot clearly lifts and passes UNDER the pelvis, with visible floor clearance.
3 UP/REACH: near leg extends behind as the support; far foot reaches forward above the floor with toe slightly up.
4 OPPOSITE CONTACT: FAR heel now leads ahead; near leg trails. Anatomical leg ownership and front/back occlusion must visibly swap compared with column0.
5 LOAD on the FAR foot; NEAR heel lifts behind.
6 PASSING: FAR leg supports, NEAR knee folds and its raised foot passes under pelvis.
7 UP/REACH: FAR support trails and the NEAR foot reaches forward above the floor.
8 IDLE: both feet planted, relaxed grounded stance.
This must read as two real measured steps, not shaking shins, feet sliding together, alternating splits, bouncing, skating or running. The support leg remains continuous from hip through knee to planted ankle. Keep the weight over the supporting foot. Moderate natural stride no wider than shoulder span. Far limb has restrained depth shading; it is not randomly exchanged with the near limb. Arms swing subtly opposite their corresponding legs. Head stays naturally attached to chest with only minimal coherent body rise/fall. No independent head movement, swelling or limb stretching.
Draw whole coherent figures independently; no pasted limb bands or grafted heads. Keep identical class identity and native standing height from IMAGE1. Calm matte pixel clusters, crisp dark contour, no smoothing, blur, glossy gradients, noisy highlights or added detail. Empty hands; no weapons or extra equipment.
Generous clear gutters, one constant scale and baseline for the sheet. GENUINE transparent alpha: all background and gaps alpha ZERO, opaque sprite ink. No floor, shadows, fog, labels, text, lines, checkerboard, bright fringe or extra figures.
Target: Slender adult caster, exact compact red hood and shadowed face, long split deep red robe with dark center and warm trim, dark gloves and small boots. The long robe must reveal the stepping boot through its split: let the hem respond slightly to each different supporting leg. Keep boots readable at contact and passing, not a single solid cone that slides. Do not shorten the robe or remove the hood.
```

## Métamorphe

```text
Use case: identity-preserve. Redraw an anatomically readable WALK for a native64 tactical-game character.
IMAGE1 is the exact front costume/morphology reference and is authoritative for head, proportions, coverage, palette and materials. IMAGE2 is an ANATOMICAL KEY-POSE DIAGRAM, not art to copy. The warm leg is the anatomically NEAR leg; the blue leg is the FAR leg. The colors only identify leg ownership: NEVER copy the diagram colors or lines into the sprite.
Create exactly EIGHTEEN complete separated figures in exactly NINE COLUMNS by TWO ROWS. Top row all face LEFT in near-frontal three-quarter view; bottom row all face RIGHT in matching near-frontal three-quarter view. Columns0..7 are EIGHT different walking key poses in the exact order of IMAGE2; column8 is a distinct grounded IDLE. Do not repeat a three-pose shuffle.
Follow the diagram's ACTUAL FOOT AND KNEE POSITIONS, not just torso/arm variation:
0 CONTACT: near heel reaches ahead and touches down; far leg trails with toe pushing.
1 LOAD/DOWN: weight settles onto the near foot; near knee bends, far heel rises.
2 PASSING: near foot bears the body; far knee folds and its foot clearly lifts and passes UNDER the pelvis, with visible floor clearance.
3 UP/REACH: near leg extends behind as the support; far foot reaches forward above the floor with toe slightly up.
4 OPPOSITE CONTACT: FAR heel now leads ahead; near leg trails. Anatomical leg ownership and front/back occlusion must visibly swap compared with column0.
5 LOAD on the FAR foot; NEAR heel lifts behind.
6 PASSING: FAR leg supports, NEAR knee folds and its raised foot passes under pelvis.
7 UP/REACH: FAR support trails and the NEAR foot reaches forward above the floor.
8 IDLE: both feet planted, relaxed grounded stance.
This must read as two real measured steps, not shaking shins, feet sliding together, alternating splits, bouncing, skating or running. The support leg remains continuous from hip through knee to planted ankle. Keep the weight over the supporting foot. Moderate natural stride no wider than shoulder span. Far limb has restrained depth shading; it is not randomly exchanged with the near limb. Arms swing subtly opposite their corresponding legs. Head stays naturally attached to chest with only minimal coherent body rise/fall. No independent head movement, swelling or limb stretching.
Draw whole coherent figures independently; no pasted limb bands or grafted heads. Keep identical class identity and native standing height from IMAGE1. Calm matte pixel clusters, crisp dark contour, no smoothing, blur, glossy gradients, noisy highlights or added detail. Empty hands; no weapons or extra equipment.
Generous clear gutters, one constant scale and baseline for the sheet. GENUINE transparent alpha: all background and gaps alpha ZERO, opaque sprite ink. No floor, shadows, fog, labels, text, lines, checkerboard, bright fringe or extra figures.
Target: Slim brown-haired adult human with a bare small head, olive/tan fitted outfit, ivory/tan organic segmented shoulder/chest/forearm/shin protection, dark joints, gloves and boots. Preserve the slender flexible silhouette and natural material shapes from the exact reference. Supple measured steps, not a yellow android, giant furry warrior, crouch run or exaggerated sway.
```

