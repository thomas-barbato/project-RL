"""Track anatomical leg ownership across the two successive three-frame steps.

The previous guard pinned silver to the foreground shin, which cannot establish
limb identity when the next half-cycle brings the other leg to the foreground.
These are reference annotations only; the output drawings are edited by image_gen.
"""
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from build_player_walk_review_04 import components

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-07'
SRC = ROOT / 'assets/source-art/player-walk-review-07'
font = ImageFont.truetype('C:/Windows/Fonts/arial.ttf', 19)
jobs = []
for side, old in [('left', 'r07b'), ('right', 'r07')]:
    source = SRC / f'BASTION-TRANSFER-10-{old}-{side}.png'
    im = Image.open(source).convert('RGBA')
    guide = im.copy()
    draw = ImageDraw.Draw(guide)
    parts = sorted(components(im), key=lambda p: p['area'], reverse=True)[:7]
    parts.sort(key=lambda p: p['center'][0])
    for n, part in enumerate(parts):
        x0, y0, x1, y1 = part['box']
        w, h = x1-x0, y1-y0
        label = 'IDLE KEEP' if n == 0 else ('STEP A KEEP' if n <= 3 else 'STEP B SAME RIGHT LEG')
        draw.text((x0, max(12, y0-38)), f'{n}: {label}', fill='#eeeeee', font=font)
        if n >= 4:
            # Outline the rear calf to receive silver, and the planted calf to
            # return to cloth. Color placement is an instruction, not a mask.
            rear = (.73, .82) if side == 'left' else (.22, .83)
            front = (.38, .81) if side == 'left' else (.62, .81)
            for (cx, cy), color in [(rear, '#37d6ff'), (front, '#ff5a9b')]:
                x, y = x0+cx*w, y0+cy*h
                draw.ellipse((x-w*.14, y-h*.08, x+w*.14, y+h*.08), outline=color, width=5)
    guide_path = DOC / f'references/CLASS-10-r07c-{side}-ownership-guide.png'
    guide.save(guide_path)
    direction = side.upper()
    prompt = f'''Use case: precise-object-edit.
EDIT TARGET: input1, SEVEN complete huntress figures in ONE ROW facing {direction}: column0 idle, columns1/2/3 walking step A, columns4/5/6 the next walking step B. Input2 is an annotation guide only. Input3 is the canonical character identity and materials.
Fix ONLY anatomical ownership of the asymmetric RIGHT LEG armor across the two successive steps. The movement geometry is already acceptable; do not redesign or reduce the gait.
The canonical character has ONE mechanical silver RIGHT SHIN, joined to the anatomical right knee and right thigh with CARGO POCKET. The other lower leg is ordinary DARK BLUE TROUSER CLOTH. A prosthesis cannot teleport between limbs as their front/back overlap changes. Screen-left and foreground do NOT define a limb.
Columns0,1,2,3: KEEP EXACTLY as input1, including the existing silver calf, pocket, hips, knees, boot locations, face, torso and arms.
Columns4,5,6 ONLY: this is the OPPOSITE SUPPORT HALF of the SAME gait. The right leg is now the bent BACK LEG, while the left leg is the planted FRONT LEG. Move the SAME silver mechanical calf shield and ankle onto the existing BENT REAR CALF (cyan outline in input2), faithfully foreshortened to that existing calf pose. Return the current planted/front shin (pink outline in input2) to ordinary dark blue cloth. There must be ONE silver mechanical shin per figure. The right-thigh cargo pocket must stay attached to that SAME right leg and therefore appear on the rear/right thigh in these three poses, not the opposite front thigh. Trace the right hip to its right knee, then right calf and boot, keeping this one anatomical chain coherent. Keep the two separate knees and boots; no fused legs or swapped joints.
Keep every existing outer silhouette and joint/foot position unchanged as closely as possible. Do not move, mirror, widen, shorten, scale or graft either leg. This edit changes materials and ownership of the existing armor/pocket only; it must NOT turn the figure or change the existing walking poses. Head, hair, face looking {direction}, body, arms, hands, outfit colors, proportions, camera and whole-figure pixel-art finish remain as input1. No new gear or weapon. No bare skin on legs. Do not copy the guide colors.
Output exactly the SAME SEVEN complete figures in ONE ROW in the same order, same equal scale, full hair and boots with margins. True transparent background. No annotations, labels, text, floor, shadow or guide lines.'''
    (DOC / f'prompts/CLASS-10-prosthesis-r07c-{side}.txt').write_text(prompt, 'utf8')
    jobs.append({'id': f'CLASS-10-prosthesis-r07c-{side}', 'references': [str(source), str(guide_path), str(ROOT/'docs/reviews/player-walk-06/references/CLASS-10-canonical.png')],
                 'prompt': prompt, 'destination': str(SRC/f'BASTION-TRANSFER-10-r07c-{side}.png'),
                 'unchanged_columns': [0,1,2,3], 'opposite_support_columns': [4,5,6],
                 'scope': 'armor and cargo-pocket ownership only; existing gait geometry retained'})
(DOC/'prosthesis-jobs-r07c.json').write_text(json.dumps(jobs, ensure_ascii=False, indent=2)+'\n', 'utf8')
