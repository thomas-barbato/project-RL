# BIPED-LATERAL01-r01 — prompts conservés

Mode : outil intégré image_gen. Chaque modèle utilise son frontal actuel comme identité et le guide de huit poses comme repère de mouvement. Le guide ne remplace pas son anatomie.

## Prompt commun des quatre monstres

Create a coherent pixel-art sprite animation sheet for the existing Project RL creature. INPUT1 is the exact current identity reference; INPUT2 is a schematic timing/anatomy guide only, NOT an appearance to copy. Preserve INPUT1's silhouette, materials, equipment, scale and anatomical identity. Re-draw the COMPLETE figure in every pose: head, chest, arms, knees and feet agree in orientation. Never cut/shear/translate leg strips. Sharp old-school RPG raster drawing, readable opaque color clusters, dark contours, matte shading, matching INPUT1, no painterly blur or 3D render. OUTPUT EXACTLY 9 equal-width columns by 2 equal-height rows on a genuinely transparent RGBA background, absolutely no labels, captions, floor, shadows or decorations. Each full figure centered in its own equal cell with generous separation, no clipping/overlap. TOP ROW faces screen LEFT in near-frontal three-quarter view, BOTTOM ROW faces screen RIGHT in matching near-frontal three-quarter view, never back view, never merely mirrored asymmetrical equipment. Columns1..8 are one walking cycle: 1 foot A contacts ahead/foot B trails; 2 load A with knee flex, B begins lifting; 3 B passes beneath body clearly lifted while A supports; 4 B reaches forward, A preparing push-off; 5 B contacts ahead/A trails; 6 load B/A lifts; 7 A passes beneath while B supports; 8 A reaches forward/B pushes off. Column9 is a distinct two-feet-planted lateral IDLE, stable anatomy and same height. The two support legs must actually exchange front/back positions with bent swing knee and lifted passing foot, not merely open/close. A slow grounded 650ms footstep, 1300ms full cycle, minimal head movement, no splits, no jumping. Schematic leg guide is human; adapt leg angles, crouch and stride to this creature's real morphology. Keep constant head/torso scale across all18 drawings and identical baseline. No extra arms/legs, no missing feet, no duplicate hands, no invented weapon.

## MON-005

Lean hunched possessed humanoid, bare scarred beige-pink flesh, bowed head with sparse dark hair, long low hands with dark nails, dark charcoal trousers and heavy dark boots. Exactly TWO legs and TWO arms. Preserve hunched upper body and long hands, no added clothes, weapons or appendages. Alternate compact boot steps with bent knees; the existing slouch remains, not an upright soldier.

## MON-014

Human cultist with charcoal pointed hood, small pale masked face, deep red long front tabard/waist cloth, dark gray fitted coat, dark sleeves, belt, trousers and boots. Exactly TWO legs TWO arms. Preserve the small curved dark object/knife carried by the hand in the canonical reference, on the SAME anatomical hand in both views. Maintain its rigid grip and no duplicated/vanishing weapon. Restrained cloth secondary movement; boots visibly perform compact contact, support, passing, reach. Do not turn the cloth into a floor-length skirt; preserve the two separately visible lower legs.

## MON-156

Very broad hulking pale icy-blue horned demon, two large curved ivory horns, dark brown hairy shoulder collar and loincloth, massive muscular arms with dark hands, two thick legs and dark broad clawed feet. Exactly TWO weight-bearing legs TWO arms, no tail or wings. Heavy deliberate steps: stable chest, slightly flexed planted knee, clear short forward passing foot, little vertical bob. Preserve huge width and mass, do NOT give a human soldier's posture or slender legs.

## MON-144

Red muscular winged demon, small dark horns, broad two red bat wings with dark membranes, dark waist cloth and dark clawed feet/hands. Exactly TWO weight-bearing legs, TWO separate arms, TWO wings; preserve only appendages actually present in the canonical reference. Grounded compact alternating clawed steps with stable wings carried spread as reference, tiny joint motion only, no flapping/flight and no human boots or costume.

## heavy-alpha

Perform ONLY a precise background extraction on this exact 9-column 2-row demon sprite sheet. Make all white/light-gray backdrop and background shadow genuinely fully transparent alpha 0 including gaps between limbs and horns. Preserve EVERY foreground pixel of all eighteen pale blue demons, preserve the full exact canvas dimensions, figure placement, identity, colors, shape, pose, no repainting, no change to the grid or figures, no loss of pale flesh or ivory horns. Output a clean transparent RGBA sheet. No white/gray matte, no halo, no new shadow.

## bastion-ready

Edit this exact existing 9-column 2-row Bastion whole-figure walking sprite sheet to support carrying a modular rifle in TWO HANDS. Preserve the SAME eighteen complete figures, exact orange/charcoal armor, face, shoulder protection, every leg/boot pose and body proportions, first8walking+9idle each row; TOPfacesLEFT near-frontal3quarter, BOTTOMfacesRIGHT. Redraw COMPLETE anatomically coherent arms on these poses, now bent at elbows and held steadily across lower chest/upper abdomen instead of swinging. Primary near hand a closed dark armored glove by center of belly, SUPPORT hand a second closed dark glove about 10 native pixels AHEAD toward walking direction and about2nativepixels higher, a compact rifle cradling pose. Hands retain same spacing and height relative chest in every pose; upper body only small natural support motion, head tied to neck. The closed fingers should form open C-like grips ready to wrap an INDEPENDENT weapon, with knuckles visible and wrists anatomically connected; two arms and two hands total, no extra wrist or hanging original arm left behind. Do NOT draw any rifle, staff, rod, bar or visible weapon placeholder: output body and gloves only, the exact shared game rifle will be composited separately between these fixed grips. Do not change the leg gait or invent a new costume. Exact9equalcolumns x2equalrows, every figure fully inside its own cell, matching source canvases, generous transparent gaps, baseline consistent. Transparent RGBA background trulyalpha0, sharp pixelart opaque clusters, no matte/checkerboard, floor, shadows, text, blur or 3D.

Les sources sélectionnées sont copiées dans assets/source-art/animation-polish-01/biped-lateral-01/r01/ ; le Bastion avec ses bras prêts à tenir un fusil est sous weapon-fit-01/r01/. La correction heavy-alpha remplace la première sortie opaque pour MON-156. Les armes communes sont composées par le moteur, sans redessin de l'arme par image_gen.

