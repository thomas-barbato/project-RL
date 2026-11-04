# LATERAL02-r01 — consigne de génération

Mode : outil intégré `image_gen`, nouvelle planche guidée par une référence,
fond transparent. Source conservée dans
`assets/source-art/animation-polish-01/lateral-identity-02/LATERAL02-r01-source.png`.

Référence : `reference-front-back.png` du même dossier, extraite des couches
VERT18 `traqueuse-armor3`. Le résultat est un candidat de revue, non accepté.

```text
Use case: stylized-concept / identity-preserving game sprite derivation.
Asset: LATERAL02-r01, one production-review sprite sheet, NOT runtime integration.
Reference image 1 is the SOLE approved identity and pixel-art treatment: the CURRENT front/back Traqueuse wearing heavy armor (native standing height about 60 pixels). Preserve that exact person and costume when turning her. The source reference sheet repeats the same 3 figures; do not draw those reference poses in the output.
Create complete NEW whole-body lateral walking figures, not limb grafts or band shifts.
Output an exactly organized transparent sprite sheet, ideally 1792x768, 7 equal columns and 2 equal rows (14 individual figures), with generous empty transparent gutters and no text, no grid, no floor, no shadows. Each cell is 256x384. Draw the character about 240 pixels tall (a clean 4x pixel-art presentation of a native60px figure), horizontally centered, feet resting near y336 in row1 and y720 in row2.
ROW 1: front-left three-quarter view, facing left about 35 degrees away from front. Head, chest, hips, knees and toes all face the same leftward direction. Columns 1-6: six compact whole-body walk phases spanning two alternating strides, three phases per step. Column7: a true stable idle with both feet grounded.
ROW 2: the same actual character viewed front-right three-quarter, facing right about 35 degrees from front, again six whole-body walk phases then a stable idle. Preserve the anatomical asymmetry of the armor; redraw the other view coherently rather than mirroring equipment.
Identity invariants from reference: slim athletic adult woman, small natural head, bare brown hair in the same high bun, exposed upper face, small muted blue face covering only as shown in reference; NO helmet, NO metallic cap, NO new goggles or visor covering her eyes, NO antenna. Keep exact pale blue-gray/white breastplate and compact shoulder/forearm/thigh plates over a black/dark navy fitted undersuit, narrow hips, segmented silver cybernetic lower legs and modest dark armored feet. Preserve the small angular armor seams and exact brown-hair/white-steel/black/blue palette. No cloak, new fabric skirt, oversized chest, different boot design or extra accessories.
Arms are bent compactly in the same empty-handed ranged-ready posture as reference, with both gloved hands in front at upper-waist level, ready to receive a separately composited shared rifle. Do NOT generate a weapon, muzzle flash, held prop, or weapon-like forearm extension. Hands must stay rigidly anchored relative to torso through all six phases.
Animation: low-amplitude compact walking, alternating left and right support, measured heel/contact, passing foot, transfer. No running, no high knee lift, no wide exaggerated stride, head/torso bob no more than one native pixel. Character height, head size, limb lengths, costume, palette and material details remain identical throughout all14 figures.
Style: match the supplied native pixel-art reference precisely: crisp clustered pixels, restrained readable detail, dark contours, matte metal with small quiet highlights; no photorealism, no smooth painterly gradients, no blur, no 3D render. Draw whole coherent figures with all body parts, exactly two legs and two arms. Avoid any resemblance to the obsolete blue-helmet lateral character. New side poses must unequivocally be the same unhelmeted brown-haired woman shown in the reference front/back.
Keep every figure complete and separated, no cropped feet or heads. True transparent background.
```

Préparation : une seule échelle commune, échantillonnage au plus proche,
translation de chaque figure entière sur une ligne d’appui et un repère
supérieur stables. Aucun membre, tête ou casque n’est découpé et déplacé
indépendamment. La source générée mesure 1916 × 821 px ; les cellules de
revue sont préparées à l’échelle native du personnage VERT18, hauteur opaque
de référence 58 px sur une grille de jeu de 64 px.
