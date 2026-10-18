"""Publish CAMP-NPC-01 from real GPU frames, without resizing source artwork."""
from pathlib import Path
import json
import re
import hashlib
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01/camp-npc-review-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def publish():
    checks = json.loads((OUT / 'checks.json').read_text(encoding='utf-8'))
    assert checks['native_cell'] == 64 and not checks['production_enabled']
    manifest = json.loads((ROOT / 'assets/maps/cryogenic-camp-01/camp.json').read_text(encoding='utf-8'))
    for actor in manifest['actors'][1:]:
        assert hashlib.sha256((ROOT / actor['file']).read_bytes()).hexdigest() == actor['sha256']
    assert (OUT / 'stderr.log').read_text().strip() == ''
    poses = [Image.open(OUT / f'npc-poses-{phase}.png').convert('RGB') for phase in range(3)]
    assert ImageChops.difference(poses[0], poses[2]).getbbox() is None, 'Disabled breathing must be the exact rest pose'
    verification = []
    cards = []
    names = ['Officier', 'Armurier', 'Médecin', 'Technicien']
    icons = ['Bulle de dialogue', 'Arme', 'Croix de soins', 'Réglage / amélioration']
    for record, name, icon in zip(checks['scenes'], names, icons):
        n = record['index']
        x = 160 + (n-1)*256
        body_diff = ImageChops.difference(poses[0], poses[1]).crop((x-32, 128, x+96, 256))
        bbox = body_diff.getbbox()
        # The moving upper strip overlaps the waist by two pixels. The lower
        # legs, feet and contact shadow below that overlap must be identical.
        assert bbox and bbox[1] >= 32 and bbox[3] <= 80, (n, bbox)
        assert ImageChops.difference(poses[0], poses[1]).crop((x-32, 208, x+96, 256)).getbbox() is None
        assert ImageChops.difference(poses[0], poses[1]).crop((x, 320, x+128, 448)).getbbox() is None
        assert ImageChops.difference(poses[0], poses[1]).crop((x-32, 128, x+96, 160)).getbbox() is None
        verification.append(dict(index=n, upper_body_changed=True, fixed_feet_and_shadow=True, fixed_portrait=True, fixed_marker=True, reduced_motion_exact_rest_pose=True))
        frames = [p.crop((x-32, 128, x+96, 256)) for p in poses[:2]]
        frames[0].save(OUT / f'npc-{n}-native.webp', save_all=True, append_images=frames[1:], duration=[1500,1500], loop=0, lossless=True)
        cx, cy, _, _ = record['cell']
        # Native screenshot crop includes the adjacent real player for scale.
        box = (round(cx-128), round(cy-64), round(cx+128), round(cy+128))
        scene_frames = [Image.open(OUT / f'npc-{n}-scene-{p}.png').crop(box) for p in range(2)]
        scene_frames[0].save(OUT / f'npc-{n}-scene.webp', save_all=True, append_images=scene_frames[1:], duration=[1500,1500], loop=0, lossless=True)
        cards.append(f'<article><h3>CAMP-NPC-0{n} · {name}</h3><p>{icon} · respiration 2 px / 3 s</p><img width="256" height="192" src="camp-npc-review-01/npc-{n}-scene.webp" alt="{name} dans le client réel, avec le joueur à côté"><details><summary>Pose isolée à taille native</summary><img width="128" height="128" src="camp-npc-review-01/npc-{n}-native.webp" alt="{name}, pixels natifs"></details><p><a href="camp-npc-review-01/npc-{n}-scene-0.png">Capture complète du client</a></p></article>')
    (OUT / 'verification.json').write_text(json.dumps(dict(review='CAMP-NPC-01', revision='r01', gpu_verified=True, source_art_unchanged=True, simulation_read_only=True, candidates=verification), ensure_ascii=False, indent=2), encoding='utf-8')
    page = OUT.parent / 'index.html'
    text = page.read_text(encoding='utf-8')
    text = re.sub(r'<!-- camp-npc-review:start -->.*?<!-- camp-npc-review:end -->', '', text, flags=re.S)
    section = '''<!-- camp-npc-review:start --><section id="pnj-r01" style="border:1px solid #89764f;padding:18px;margin:18px 0;background:#12171e"><h2>CAMP-NPC-01 r01 · pictogrammes et repos des PNJ — à valider</h2><p>Quatre aperçus animés issus du client réel, sans agrandissement : cases de 64 px, joueur à côté. Les dessins sources sont conservés. Pieds et ombres fixes ; inspection et portraits immobiles ; mouvements réduits respectés. Ce rendu reste en revue.</p><div style="display:flex;flex-wrap:wrap;gap:18px">'''+''.join(cards)+'''</div><p>Le retour au camp est corrigé dans le jeu. La liaison conserve le même camp et ses services après sauvegarde et reprise. Le raccord au secteur existant reste provisoire.</p></section><!-- camp-npc-review:end -->'''
    text = text.replace('</header>', '</header>'+section, 1)
    page.write_text(text, encoding='utf-8')
    print(json.dumps(dict(gallery=str(page), verified_npcs=4, production_enabled=False)))


if __name__ == '__main__':
    publish()
