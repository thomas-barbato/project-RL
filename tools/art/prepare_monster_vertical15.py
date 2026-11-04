"""Prepare native review candidates for the next three monsters."""
import argparse
import json

import prepare_monster_vertical01 as base

MODELS = {
    75: ('Bête brune aux épaules rouges', 'quadruped', 4),
    76: ('Aberration ivoire aux tentacules', 'biped', 2),
    77: ('Colosse ivoire sans tête', 'biped', 2),
}
NEXT_MODELS = {
    78: ('Colosse à carapace sombre', 'biped', 2),
    79: ('Démon rouge au bras-lame', 'biped', 2),
    80: ('Brute voûtée ivoire et rouge', 'biped', 2),
}
BATCH_MODELS = {
    15: MODELS,
    16: NEXT_MODELS,
    17: {
        81: ('Quadrupède à carapace brune', 'quadruped', 4),
        82: ('Bête rouge à cornes', 'quadruped', 4),
        83: ('Scorpion rouille', 'crawl', 8),
    },
    18: {
        84: ('Démon pâle aux longs bras', 'biped', 2),
        85: ('Créature à bosse rouge', 'biped', 2),
        86: ('Démon rouge aux deux lames', 'biped', 2),
    },
    19: {
        87: ('Quadrupède trapu au ventre ocre', 'quadruped', 4),
        88: ('Oiseau aux ailes rouges', 'winged', 2),
        89: ('Créature sombre aux pattes anguleuses', 'crawl', 4),
    },
    20: {
        90: ('Créature brune à six pattes claires', 'crawl', 6),
        92: ('Quadrupède pâle aux larges griffes', 'quadruped', 4),
        93: ('Colosse aux poignets enchaînés', 'biped', 2),
    },
}


def main(references=False, batch=15):
    models = BATCH_MODELS[batch]
    base.OUT = base.ROOT / f'assets/source-art/animation-polish-01/monster-vertical-{batch}/r01'
    base.NAMES = {n: row[0] for n, row in models.items()}
    base.REVIEW = f'MONSTER-VERTICAL{batch}-r01'
    base.ALIGN_SUPPORT_ROOT = True
    if references:
        base.references(write_inventory=False)
        return
    base.main()
    path = base.OUT / 'manifest.json'
    manifest = json.loads(path.read_text(encoding='utf-8'))
    for number, profile in enumerate(manifest['profiles'], 43 + (batch - 15) * 3):
        name, gait, legs = models[profile['number']]
        profile.update(name=name, review_number=number, locomotion=gait, locomotor_legs=legs)
    path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--references', action='store_true')
    parser.add_argument('--batch', type=int, choices=BATCH_MODELS, default=15)
    args = parser.parse_args()
    main(args.references, args.batch)
