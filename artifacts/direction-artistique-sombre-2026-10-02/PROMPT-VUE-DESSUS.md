# Reprise de la vue du dessus

2 octobre 2026. Le choix utilisateur privilégie la vue de gauche, mais rejette sa perspective encore oblique. Production avec **imagegen intégré** : une édition du comparatif, suivie d'une génération neuve pour réduire l'influence de l'image précédente. Aucun changement du moteur, du catalogue ou des cartes.

## Résultat et limites

Le résultat retenu pour discussion est `vue-dessus-v1.png`. Le personnage et la chaise montrent davantage leurs surfaces supérieures et le lampadaire possède une empreinte compacte. Il reste toutefois des conventions obliques : épaisseur frontale de certains éléments, notamment les murs et le fût. Cette image ne doit pas être validée comme référence géométrique exacte d'une projection verticale, ni utilisée telle quelle comme kit de textures.

L'édition précédente a conservé trop de la projection du comparatif et n'est pas retenue. Son original reste dans le dossier d'images générées : `exec-20bea473-adf3-43f9-98bd-865a8300946c.png`.

## Première tentative : édition du comparatif

```text
Use case: style-transfer.
Edit target: the LEFT scene in the supplied comparison board. The right scene is rejected and must not appear in the output.
Primary request: REDRAW the left scene as one single, rigorously TRUE OVERHEAD ORTHOGRAPHIC game view. The user rejected the existing left picture because it STILL shows vertical sides and front-facing people. Correct the projection, not merely the color. The final output is a single scene with NO labels, NO second panel and NO border.
Camera: a ceiling camera looking EXACTLY straight down at the floor, its optical axis perpendicular to the ground, 90-degree elevation, zero camera tilt, zero perspective convergence. Treat this as a richly textured architectural PLAN VIEW or a satellite image of a roofless interior. Every object obeys this same camera. All pavement rows and room edges are horizontal and vertical. Do NOT depict a dollhouse, cutaway front elevation, isometric scene, three-quarter view, or tilted birds-eye view.
Preserve the dark industrial science-fiction atmosphere, charcoal concrete, dirty steel, muted rust, cracked pavement, worn tabletop, subtle amber street lighting, restrained cyan controls, and approximate scene arrangement from the LEFT panel: small square roofless room in the upper half, thick walls with one south doorway, workbench and a chair near the north wall, paving outside in lower half, one streetlamp left of entrance, one armored human on the paving right of the lamp. The scene is original concept art, not a copied game asset.
ABSOLUTE PROJECTION RULES:
- Walls are ONLY horizontal top surfaces: thick concrete strips seen in plan, with their edges at the same height. NO visible inner or outer vertical wall faces, bricks on a facade, fascia, or tall front wall. All walls use the same top-view thickness.
- A doorway is simply a gap in the south wall plus a horizontal threshold. NO tall door portal, front-facing lintel, or luminous frontal doorway. Cyan control is a tiny rectangle on the top of a jamb.
- Workbench shows ONLY the horizontal tabletop and small tool tops. ZERO visible drawer fronts, drawer handles, legs, or cabinet sides.
- Chair shows ONLY the seat top and the narrow top edge of its backrest. ZERO chair legs or frontal upholstery. Any leg is completely hidden beneath the seat from this exact overhead camera.
- Cabinets show ONLY their rectangular top panels, never frontal doors, screens or handles.
- Streetlamp: depict the TOP of its small metal lamp head, with the bulb light falling on the ground and a long cast shadow indicating height. The vertical pole is hidden exactly below the head, NOT displayed as a long object lying on the ground. Do NOT show a big lamp plinth, pedestal or full-height pole.
- Human: standing upright directly under the camera; show ONLY circular top of helmet, shoulders on each side and a small visible top portion of backpack/weapon. NO face, no chest plate front, no belt, no stomach, no thighs, no knees, no boot fronts, no full body portrait. The helmet occludes the body beneath it. The footprint is compact and unmistakably an overhead human.
- Pipes attached vertically to walls are only small top cross-sections when vertical; horizontal conduits can appear in plan at their actual positions. NO front-facing ventilation grills mounted on walls.
- A barrel, if retained from the source, is ONLY a circular top disk, never an upright cylinder side.
Rendering: crisp deliberate pixel-art clusters with limited colors, readable silhouettes, purposeful wear, soft local light on the floor. Preserve the harsh, neglected mood without blacking out the image. NO cartoon, no shiny clean surfaces, no green grass, no arbitrary fine noise overlay. No text, no interface.
Priority: physically consistent exact overhead projection is more important than retaining any wrong-perspective details from the reference. When a vertical surface was visible in the reference, remove it completely.
```

## Dernière tentative : génération neuve

```text
Use case: stylized-concept.
Create one original 2D pixel-art game scene in STRICT VERTICAL TOP-DOWN PLAN VIEW, with a camera looking exactly straight down, orthographic nadir projection. It must resemble a detailed overhead tactical FLOOR PLAN, not an isometric room illustration. Important: no side of ANY upright object is visible. Every upright shape is collapsed to its TOP SURFACE. A single consistent view, without text or UI.
Scene in plan view: cracked charcoal pavement fills a square image. An industrial room occupies the upper half. Its roof is absent. The room boundary is a simple continuous thick pale-gray concrete strip, seen only as a flat upper surface, with a gap in the south edge. Inside, a scratched steel tabletop is a flat rectangle with little tools lying on it. A chair is a square brown seat top with a thin backrest top line on its north edge. Two small green storage boxes are flat rectangles showing ONLY top lids. A drum is a simple circular rusty top disk. Outside, a standing armored human is represented by one compact circular helmet top with shoulder pads to the left and right; the human body is entirely occluded beneath the head, ONLY HEAD AND SHOULDERS SEEN FROM DIRECTLY ABOVE. A streetlamp is a small flat rectangular metal top housing seen vertically from above, with a warm pool of light on the pavement and a long thin pole shadow. No visible pole because it is exactly underneath the lamp head. A drain is a circular flat grate.
Projection and shape language: every object aligned to a horizontal/vertical overhead map; rectangular objects have four square right-angle corners without any visible depth extrusion. Helmet is a small circle, shoulders short side lobes, no visible chest or legs. All concrete wall strips show a single planar material surface, no vertical facade texture. All objects have compact footprints. Heights are communicated only through cast shadows on the ground. Draw only upward-facing surfaces.
Art direction: oppressive neglected industrial science fiction, cold desaturated gray and soot black, muted rusty brown and olive, tiny amber light accents and one small cyan top-facing indicator. Substantial wear expressed through chips, stains and cracks. Handcrafted pixel clusters with sharp edges and limited colors, readable contrasting silhouettes, no painterly blur. Keep enough midtones to read the room and character. Square composition, close overhead map view, ample paving around room.
This is a strict projection test. Do not add three-quarter perspective, dollhouse walls, front-facing humanoids, table legs, drawer fronts, chair legs, cylindrical side faces or upright lamp sprites. No text, no border, no second panel.
```
