# MAP07 — sources et prompts

Mode : outil intégré `image_gen.imagegen`, éditions sur fond transparent.

Sources conservées sans modification :

- `assets/source-art/north-city-01/style07/VOLUME07-01-r02-source.png`
- `assets/source-art/north-city-01/style07/ILOT07-B01-r02-source.png`

Empreintes, références et dimensions : `assets/source-art/north-city-01/style07/manifest.json`.

## CONTOURS07-r02-PROMPT.txt

```text
Use case: precise-object-edit / style-transfer.
Image 1 is the edit target: the complete ruined house. Image 2 is the actual approved native first-city garage, the LINE STYLE REFERENCE.
Change only the house outline treatment to MATCH THE GARAGE. Keep the house silhouette, roof perspective, front wall height, door/window proportions and positions, chimney, broken corner, colors, materials and transparent framing EXACTLY as image 1. No redesign or new objects.
The garage reference has CLEAR, SOLID BLACK hand-drawn ink around the exterior silhouette, coping stones, roof machinery and door, with thinner black structural seams. The house will be reduced uniformly from roughly 1056px useful width to 288px in the game, so paint its primary black contours BOLDER at source resolution: about 8–11px source for silhouettes / structural edges, equivalent to 2–3px at native game size. Internal material seams are finer (3–5px source). Match the garage's confident irregular inking, restrained warm grey bevel highlights, grouped matte shading and crisp shapes. Do not produce fine technical hairlines. Do not soften, blur, brighten or filter the image. Do not thin the black contours. Keep dark cast shadows distinct from ink. No pale outer halo.
The final size/proportions must stay as in the target, not enlarged to compensate for line weight. This is a surgical line-style change only.
Keep the entire building uncut in the same framing with genuinely transparent background. No floor patch, external shadow, text, annotations or watermark.
```

## ILOT07-PROMPT.txt

```text
Use case: precise-object-edit / style-transfer.
Edit target image 1: the current in-engine B01 apartment block. Images 2 and 3: the approved FIRST CITY native garage and roof, the exact drawing-style and scale references.
Create a complete isolated redrawn version of ONLY the central main B01 building in image 1, for a game asset on transparent background. Exclude the neighboring building at far left, pavement, player, lamp, title and all UI. Keep its rectangular ground footprint and square parallel world axes. No isometric rotation or converging perspective.
It is a long ruined European apartment block, native game footprint 1280 pixels wide by 704 pixels deep, with a short front facade about 96 native pixels high beneath the roof. The complete roof-and-facade asset proportions are 1280:800. Keep these proportions. Think four joined old city houses in a single continuous rectangular block, with short consistent facade height, ALL grounded at the same baseline. The street frontage is along the bottom. Keep the right side unobstructed for the existing east entrance. No visible new entrance or staircase on the bottom frontage, which currently has windows.
Replace the sterile repeated flat roof and identical windows with a convincing FULLY DRAWN ruined European city block: four adjoining irregular zinc/slate roof sections in muted blue/olive/charcoal shades, worn stone parapets and dark party seams, a few copper pipe runs, two modest chimneys, a single complete roof fan/duct, one localized broken roof corner with timber visible, sparse debris and weathering. Keep the roof continuous and complete, with no giant holes, no trees or oversized objects. Front facade: worn red brick and flaked grey plaster, subtle individual variation of shutters and window damage; no identical tiled window stamps. Window height at final native size about 55 pixels, comparable to the 58px visible player for scale. Roof fittings must be as compact and legible as in reference 2, not enlarged.
DRAWING STYLE MUST MATCH THE FIRST CITY REFERENCE: crisp solid BLACK irregular hand-drawn contours around silhouettes and structural forms, thinner internal joints, warm pale stone bevels, grouped matte colors and localized wear. Match edge sharpness and line-to-material balance at NATIVE game scale. Primary exterior ink 2–3 native pixels, secondary detail 1–2px. Do not create a photoreal render, smooth 3D render, pixel-art grid, line-art blueprint or a different illustration style.
Short vertical facade projects consistently below the roof toward the viewer. Roof is clearly above the ground; top coping stays above the facade, front facade grounded at the bottom. Keep all details within a whole complete rectangular building silhouette. Provide generous TRANSPARENT outer margin on every side. No ground patch, shadows outside the object, actor, text, captions, frame or watermark. Genuine transparency.
```

## ILOT07-r02-PROMPT.txt

```text
Precise geometry correction of image 1, the whole isolated long B01 block. Image 2 is the actual game-footprint reference.
Keep the SAME drawing style, black contours, colors, facade height, windows, chimneys, fan, broken corner, four adjacent roof sections, and whole complete object. Change ONLY THE DEPTH OF THE ROOF PLANE, so the whole building silhouette is MUCH TALLER on the page.
The current asset is too shallow (roughly 1480px wide x 630px high). The required whole-building silhouette is width:height = 1.6:1, approximately 1480px WIDE by 925px HIGH. Keep that same wide horizontal width; extend the roof about 295px vertically, from its back edge, while leaving the lower facade the same height. The full roof should occupy about 85 percent of the object height and the short front facade just 15 percent. A square roof plane viewed orthogonally from overhead, with a short front wall below. Do not enlarge the facade or windows to fill the height, do not stretch an image, and do not add more floors. Redraw a deeper roof as a single coherent complete drawing, maintaining the sizes of individual roof fittings. Avoid a narrow strip building. The second reference shows the intended roof depth and rectangular footprint.
Keep exactly parallel horizontal and vertical building axes with no isometric angle. Same ink and matte materials as the first city. Complete rectangular building on transparent background with outer margins, no neighbor, no ground or shadow outside, no player, no text or UI.
```
