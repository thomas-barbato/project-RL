"""Native review of slightly larger NPC status/damage labels; art unchanged."""
import hashlib
import json
import time
from pathlib import Path
from PIL import Image, ImageChops
import prepare_agent_skill_animation_mines_variants_refined as player
from add_browser_animation_comparison import add_comparison
from repair_mine_preview_playback import repair_review

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-64'
OLD = OUT.parent / 'skill-animation-review-62'


def convert_native():
    native = (OUT / 'native').resolve()
    assert native.is_relative_to(ROOT.resolve())
    converted = 0
    while not (native / 'capture.json').exists():
        converted += convert_ready(native)
        time.sleep(0.3)
    converted += convert_ready(native)
    assert not list(native.rglob('*.rgba'))
    (OUT / 'native-encoding-verification.json').write_text(json.dumps({
        'converted_frames': converted, 'pixel_identical_to_native_readback': True,
        'resampling': False, 'raw_readbacks_removed_only_after_pixel_verification': True
    }, indent=2) + '\n', encoding='utf8')


def convert_ready(native):
    count = 0
    for raw in native.rglob('*.rgba'):
        if raw.stat().st_size != 640 * 720 * 4:
            continue
        data = raw.read_bytes()
        if len(data) != 640 * 720 * 4:
            continue
        image = Image.frombytes('RGBA', (640, 720), data)
        target = raw.with_suffix('.png')
        image.save(target, compress_level=1)
        assert Image.open(target).convert('RGBA').tobytes() == data
        # Only this generated readback is removed, after a lossless PNG check.
        assert raw.resolve().is_relative_to(native)
        raw.unlink()
        count += 1
    return count


def package():
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    proof = json.loads((OUT / 'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT / n).read_bytes()).hexdigest() == h for n, h in proof.items())
    assert len(entries) == 6
    for entry in entries:
        native = OUT / 'native' / entry['folder']
        timeline = json.loads((native / 'timeline.json').read_text(encoding='utf8'))
        previous = OLD / 'native' / entry['folder'].replace('-A03-', '-A02-')
        assert timeline == json.loads((previous / 'timeline.json').read_text(encoding='utf8'))
        dest = OUT / 'frames' / entry['variant']; dest.mkdir(parents=True, exist_ok=True)
        images = [Image.open(native / f'{n:03}.png').convert('RGB').crop((32, 88, 608, 664)) for n in range(256)]
        keys = {}; indices = []
        for n, image in enumerate(images):
            digest = hashlib.sha256(image.tobytes()).hexdigest()
            if digest not in keys:
                keys[digest] = n; image.save(dest / f'{n:03}.png', compress_level=1)
            indices.append(keys[digest])
        hovers = []
        for n in entry['hover_frames']:
            Image.open(native / f'hover-{n:03}.png').crop((32, 88, 608, 664)).save(dest / f'hover-{n:03}.png', compress_level=1)
            hovers.append({'frame': n, 'file': f'hover-{n:03}.png', 'step': timeline[n]['step'], 'led_on': timeline[n]['led_on']})
        for idle in entry['idle_images']:
            idle_keys = {}; idle_indices = []; step = idle['step']
            assert idle['hp'] == timeline[entry['action_frames'][step-1] + 23]['hp']
            for n in range(48):
                image = Image.open(native / f'idle-{step}-{n:03}.png').convert('RGB').crop((32, 88, 608, 664))
                digest = hashlib.sha256(image.tobytes()).hexdigest()
                if digest not in idle_keys:
                    idle_keys[digest] = n; image.save(dest / f'idle-{step}-{n:03}.png', compress_level=1)
                idle_indices.append(idle_keys[digest])
            idle['image_indices'] = idle_indices
        for n in entry['control_frames']:
            control = Image.open(native / f'control-{n:03}.png').convert('RGB').crop((32, 88, 608, 664))
            delta = ImageChops.difference(images[n], control)
            for y, row in enumerate(entry['map'].splitlines()):
                for x, tile in enumerate(row):
                    if tile == '#':
                        assert delta.crop((x*64, y*64, (x+1)*64, (y+1)*64)).getbbox() is None
        entry.update(timeline=timeline, image_indices=indices, hover_images=hovers)
        player.gif(OUT / f'{entry["variant"]}-64.gif', images)
        if entry['case'] == 'HAUT':
            for n in (14, 116, 132, 144, 232):
                images[n].save(OUT / f'{entry["id"]}-phase-{n:03}.png')
        print(f'{entry["variant"]}: labels packaged; timeline unchanged', flush=True)
    player.OUT = OUT; player.write_html(entries)
    page = (OUT / 'index.html').read_text(encoding='utf8')
    page = page.replace('Mines élémentaires · lot 31', 'Mines élémentaires · repères agrandis')
    page = page.replace('Les éclats froids et le Gel reprennent les dessins déjà acceptés.', 'Les dessins des explosions et des statuts sont conservés ; seuls leurs chiffres et pictogrammes près des PNJ sont agrandis.')
    page = page.replace('href="poison-burst-source.png"', 'href="../skill-animation-review-62/poison-burst-source.png"')
    page = page.replace('href="imagegen-poison-burst-prompt.txt"', 'href="../skill-animation-review-62/imagegen-poison-burst-prompt.txt"')
    page = page.replace('</main>', '<p class="note">Repères proposés : icône de statut 18 px, durée 15 px ; total de dégâts 24 px, détail 15 px et pictogramme 16 px. Les valeurs et les effets restent identiques. Résidu persistant n’est pas encore présent.</p></main>', 1)
    (OUT / 'index.html').write_text(page, encoding='utf8')
    add_comparison(OUT); repair_review(OUT)
    for name in ('index.html', 'cote-a-cote.html'):
        path = OUT / name
        path.write_text(path.read_text(encoding='utf8').replace('Le scénario « Attente » conserve volontairement la mine inactive.', ''), encoding='utf8')
    (OUT / 'review.json').write_text(json.dumps({
        'review_id': 'NPC-FEEDBACK-SIZE01', 'date': '2026-10-14', 'status': 'awaiting_visual_review',
        'request': 'Les valeurs affichées à côté des PNJ et l’icône sont trop petites ; un tout petit peu plus gros.',
        'candidates': [{'number': 70, 'id': 'SM02', 'variant': 'SM02-A03'}, {'number': 71, 'id': 'SM03', 'variant': 'SM03-A03'}],
        'sizes': {'status_icon': [14, 18], 'duration': [12, 15], 'damage_total': [21, 24], 'damage_detail': [12, 15], 'damage_icon': [12, 16]},
        'numeric_timelines_identical': True, 'source_art_preserved': True,
        'status_counter_moved_below_icon_to_avoid_overlap': True,
        'previous_reviews_preserved_image_files': len(proof), 'native_scenes': len(entries),
        'campaign_integration': False, 'SM05': 'Absent; terrain-residue preview remains next.'
    }, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
    (OUT / 'render-source.rs').write_bytes((ROOT / 'src/agent_skill_lab_review/fire_painted/mines_variants_refined.rs').read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())


if __name__ == '__main__':
    import sys
    if '--convert' in sys.argv:
        convert_native()
    else:
        package()
