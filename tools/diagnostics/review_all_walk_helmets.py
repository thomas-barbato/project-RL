"""Show every equippable helmet using the existing normal-renderer exports."""
from pathlib import Path
import hashlib
import json
import re

from PIL import Image, ImageDraw
from review_walk_helmets import ROOT, OUT, NAMES, atlas, sprite


def helmet_catalogue():
    items = {}
    for path in (ROOT / 'content/core/items').glob('*.json5'):
        text = path.read_text(encoding='utf-8')
        if re.search(r'"slot"\s*:\s*"core:head_armor"', text):
            items[re.search(r'"id"\s*:\s*"([^"]+)"', text)[1]] = path
    source = (ROOT / 'src/game_art_catalog.rs').read_text(encoding='utf-8')
    models = {item: int(model) for item, model in re.findall(
        r'"(core:[^"]+)"\s*=>\s*ItemArt\s*\{\s*icon:\s*2,\s*illustration:\s*(\d+)', source)
        if item in items}
    assert set(models) == set(items)
    assert set(models.values()) == set(range(115, 121))
    locale = json.loads((ROOT / 'content/core/locales/fr_accessory_models.json5').read_text(encoding='utf-8'))['entries']
    return [{'item': item, 'model': model, 'head': model - 114,
             'name': locale[f'equipment.{item.removeprefix("core:")}.name']}
            for item, model in sorted(models.items(), key=lambda pair: pair[1])]


def animation(catalogue, rows, destination, heading):
    folder = ROOT / 'artifacts/walk-helmet-02/after'
    images = {(index, head['head']): atlas(folder, index, head['head'])
              for _, index in rows for head in catalogue}
    frames, durations = [], []
    for step, (start, end) in enumerate([(2, 1), (1, 0), (0, 1), (1, 2)]):
        direction = 0 if end < start else 1
        for sample in range(11):
            moving = sample < 10
            phase = min(sample * 3 // 10, 2) + step % 2 * 3
            column = direction * 7 + phase + 2 if moving else 0
            x = round((start + (end - start) * sample / 10) * 64)
            board = Image.new('RGB', (684, 950), '#253136')
            draw = ImageDraw.Draw(board)
            draw.text((8, 6), heading, fill='white')
            draw.text((8, 23), 'Natif64 - 400 ms / case - arret de face entre chaque pas', fill='white')
            for number, head in enumerate(catalogue):
                left = 10 + number % 3 * 226
                top = 48 + number // 3 * 450
                draw.text((left, top), f'{head["head"]}. {head["name"]}', fill='white')
                for row, (label, index) in enumerate(rows):
                    y = top + 38 + row * 80
                    draw.text((left, y - 14), label, fill='#ccd7db')
                    for tile in range(3):
                        draw.rectangle((left + tile * 64, y, left + tile * 64 + 63, y + 63), outline='#46545a')
                    image = sprite(images[index, head['head']], column)
                    board.paste(image, (left + x - 32, y - 32), image)
            frames.append(board)
            durations.append(40 if moving else 300)
    frames[0].save(destination, save_all=True, append_images=frames[1:],
                   duration=durations, loop=0, optimize=False)
    return frames[4]


def main():
    catalogue = helmet_catalogue()
    proof = json.loads((OUT / 'validation.json').read_text())
    assert proof['status'] == 'passed' and proof['runtime_correction_integrated']
    for path, wanted in proof['source_sha256'].items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == wanted, path
    executable = ROOT / 'target/release/project-rl.exe'
    assert hashlib.sha256(executable.read_bytes()).hexdigest() == proof['builds']['release_sha256']
    file = OUT / 'tous-les-casques-cinq-classes-native64.gif'
    overview = animation(catalogue, [(name, cls * 5) for cls, name in enumerate(NAMES)],
                         file, 'Tous les casques - cinq classes - tenue 1')
    overview.save(OUT / 'tous-les-casques-controle-native64.png')
    classes = []
    for cls, name in enumerate(NAMES):
        file = OUT / f'{name}-tous-casques-cinq-tenues-native64.gif'
        animation(catalogue, [(f'Tenue {rank+1} / 5', cls * 5 + rank) for rank in range(5)],
                  file, f'{name} - tous les casques - toutes les tenues')
        classes.append({'class': name, 'outfits': 5, 'helmets': 6, 'file': file.name})
    report = {'revision': 'WALK12-ALL-HELMETS-r01', 'status': 'passed',
              'catalogue': catalogue, 'outfits': 25, 'helmet_outfit_combinations': 150,
              'directions': 2, 'walking_frames_per_direction': 6,
              'milliseconds_per_cell': 400, 'preview_rest_ms': 300,
              'source': 'existing normal Rust compositor exports; source and release hashes verified',
              'new_game_code_required': False, 'all_six_helmets_already_integrated': True,
              'overview': 'tous-les-casques-cinq-classes-native64.gif', 'class_previews': classes}
    (OUT / 'all-helmets.json').write_text(json.dumps(report, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
    print(json.dumps({'status': 'passed', 'equippable_helmets': len(catalogue), 'outfits': 25,
                      'combinations': 150, 'gifs': len(classes) + 1}))


if __name__ == '__main__':
    main()
