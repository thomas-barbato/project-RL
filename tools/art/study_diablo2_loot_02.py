"""Exact category-level analysis of the user's extracted vanilla Diablo II TCs.

This is not an item/quality drop calculator. It fixes a named treasure class,
assumes successful item placement, and models ordered negative Picks, NoDrop,
nested TCs and the six-item cap. No game files or rules are modified.
"""
from collections import defaultdict
from fractions import Fraction
from functools import lru_cache
from pathlib import Path
import csv
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
RAW = ROOT / 'artifacts/itemisation-loot-01/diablo2-reference'
DOC = ROOT / 'docs/reviews/itemisation-loot-01'
CATEGORIES = ('weapon_armor', 'jewelry', 'gold', 'other')
ZERO = (0, 0, 0, 0)


def read_json(path):
    return json.loads(path.read_text('utf-8'))


def rows(name):
    return list(csv.DictReader((RAW / f'{name}.txt').read_text('cp1252').splitlines(), delimiter='\t'))


TABLES = {r['Treasure Class']: r for r in rows('TreasureClassEx') if r['Treasure Class']}


def number(row, key):
    return int(row.get(key) or 0)


def entries(row):
    return [(row[f'Item{i}'], number(row, f'Prob{i}')) for i in range(1, 11)
            if row[f'Item{i}'] and number(row, f'Prob{i}') > 0]


def category(name):
    name = name.split(',')[0]
    if re.fullmatch(r'(weap|armo|bow)\d+', name):
        return 0
    if name in ('rin', 'amu'):
        return 1
    if name == 'gld':
        return 2
    return 3


def effective_no_drop(row, exponent):
    no_drop = number(row, 'NoDrop')
    total = sum(w for _, w in entries(row))
    if exponent == 1 or no_drop == 0:
        return no_drop
    ratio = Fraction(no_drop, total + no_drop) ** exponent
    return int(total * ratio / (1 - ratio))


@lru_cache(maxsize=None)
def distribution(name, capacity=6, exponent=1):
    if capacity == 0:
        return {ZERO: Fraction(1)}
    if name not in TABLES:
        state = list(ZERO)
        state[category(name)] = 1
        return {tuple(state): Fraction(1)}
    row = TABLES[name]
    choices = entries(row)
    total = sum(w for _, w in choices)
    assert total > 0, name
    picks = number(row, 'Picks')
    result = {ZERO: Fraction(1)}
    for draw in range(max(abs(picks), 1)):
        if picks < 0:
            # The engine uses draw=0..abs(Picks)-1 as an ordered ticket index.
            # Prob columns set interval lengths, not independent random rolls.
            cursor = 0
            weighted = []
            for child, weight in choices:
                if cursor <= draw < cursor + weight:
                    weighted = [(child, Fraction(1))]
                    break
                cursor += weight
            if not weighted:
                weighted = [(None, Fraction(1))]
        else:
            no_drop = effective_no_drop(row, exponent)
            weighted = [(child, Fraction(weight, total + no_drop)) for child, weight in choices]
            if no_drop:
                weighted.append((None, Fraction(no_drop, total + no_drop)))
        following = defaultdict(Fraction)
        for current, current_prob in result.items():
            remaining = capacity - sum(current)
            if remaining == 0:
                following[current] += current_prob
                continue
            for child, child_prob in weighted:
                outcomes = {ZERO: Fraction(1)} if child is None else distribution(child, remaining, exponent)
                for addition, probability in outcomes.items():
                    state = tuple(a + b for a, b in zip(current, addition))
                    following[state] += current_prob * child_prob * probability
        result = dict(following)
    assert sum(result.values()) == 1, name
    assert all(sum(state) <= capacity for state in result), name
    return result


def summarize(name, exponent=1):
    result = distribution(name, 6, exponent)
    means = [sum(prob * state[i] for state, prob in result.items()) for i in range(4)]
    weapon_chance = sum(prob for state, prob in result.items() if state[0] > 0)
    return {
        'treasure_class': name,
        'effective_no_drop_exponent': exponent,
        'no_object_percent': float(result.get(ZERO, 0) * 100),
        'weapon_armor_at_least_one_percent': float(weapon_chance * 100),
        'weapon_armor_probability_fraction': str(weapon_chance),
        'mean_objects': float(sum(means)),
        'mean_by_category': dict(zip(CATEGORIES, map(float, means))),
        'mean_weapon_armor_fraction': str(means[0]),
    }


def main():
    inspection = read_json(RAW / 'inspection.json')
    for name, info in inspection['tables'].items():
        assert hashlib.sha256((RAW / f'{name}.txt').read_bytes()).hexdigest() == info['sha256'], name
    selected = [
        ('Monstre ordinaire, début acte I', 'Act 1 H2H A'),
        ('Lanceur de sorts, table acte I', 'Act 1 Cast A'),
        ('Tireur, table acte I', 'Act 1 Miss A'),
        ('Spectre, table acte I', 'Act 1 Wraith A'),
        ('Champion, début acte I', 'Act 1 Champ A'),
        ('Monstre nommé, début acte I', 'Act 1 Unique A'),
        ('Monstre ordinaire, fin acte V Enfer', 'Act 5 (H) H2H C'),
        ('Andariel, hors récompense de quête', 'Andariel'),
        ('Mephisto, hors récompense de quête', 'Mephisto'),
        ('Diablo, hors récompense de quête', 'Diablo'),
        ('Baal, hors récompense de quête', 'Baal'),
        ('Andariel, table de quête', 'Andarielq'),
        ('Mephisto, table de quête', 'Mephistoq'),
    ]
    examples = [dict(label=label, **summarize(tc)) for label, tc in selected]
    assert distribution('Act 1 H2H A')[ZERO] == Fraction(5, 8)
    assert sum(prob * state[0] for state, prob in distribution('Act 1 H2H A').items()) == Fraction(1, 10)
    assert sum(prob * state[0] for state, prob in distribution('Act 1 Champ A').items()) == Fraction(37, 100)
    assert sum(prob * state[0] for state, prob in distribution('Act 1 Unique A').items()) == Fraction(29, 31)
    assert summarize('Act 1 Champ A')['mean_objects'] == 3
    assert summarize('Act 1 Unique A')['mean_objects'] == 5
    boss_expected = Fraction(7) * Fraction(65, 80) - Fraction(65, 80) ** 7
    assert sum(prob * sum(state) for state, prob in distribution('Mephisto').items()) == boss_expected
    assert sum(prob * state[0] for state, prob in distribution('Mephisto').items()) == boss_expected * Fraction(52, 65)
    player_settings = [dict(virtual_players=count, **summarize('Act 1 H2H A', exponent))
                       for count, exponent in [(1, 1), (3, 2), (5, 3), (7, 4)]]
    report = {
        'review_id': 'ITEM-LOOT01-DROPSTUDY02', 'date': '2026-10-23',
        'source': inspection, 'categories': CATEGORIES,
        'assumptions': [
            'LoD vanilla archive tables from the local installation, not ProjectD2 data.',
            'Named treasure classes fixed as listed, with no further level-based class substitution.',
            'Single-player /players 1 except for the explicitly labeled virtual-player comparison.',
            'Successful item creation and placement; six-item cap.',
            'Category probabilities, not exact model or final item-quality probabilities.',
            'Quest treasure classes are reported separately; persistent quest-state/Andariel behavior is not modeled.',
        ],
        'ordinary_first_act_weights': {'nothing': 100, 'gold': 21, 'weapons_armor': 16, 'junk': 21, 'good': 2, 'total': 160},
        'examples': examples, 'virtual_player_comparison': player_settings,
        'quality': {
            'separate_from_base_drop': True,
            'itemratio_rows': rows('ItemRatio'),
            'champion_named_weapon_armor_magic_factor': 1024,
            'champion_named_unique_set_rare_factors': 800,
            'ordinary_item_factors': 0,
            'warning': 'TC factors are internal modifiers, not percentages. Final rarity depends on base level/type, monster level, Magic Find and available unique/set versions.',
            'sequence': ['unique', 'set', 'rare', 'magic', 'superior', 'normal', 'inferior'],
        },
        'verified_checks': ['All distributions sum exactly to one.', 'All counts respect the six-item cap.', 'Ordinary, champion and named-equipment means match hand calculations.', 'Boss capped-binomial mean matches an independent formula.', 'Input-table SHA-256 matches the read-only extraction report.'],
        'primary_sources': [
            'https://classic.battle.net/diablo2exp/items/basics.shtml',
            'https://classic.battle.net/diablo2exp/items/magic/magicfind.shtml',
            'https://github.com/ThePhrozenKeep/D2MOO/blob/master/source/D2Game/src/ITEMS/Items.cpp',
        ],
        'rl_implications_proposed': [
            'Use a chance of no equipment on ordinary eligible deaths instead of one uniform equipment reward per kill.',
            'Separate amount of loot, base-model eligibility and final quality.',
            'Distinguish ordinary, champion/named and boss reward profiles.',
            'Keep material/crafting changes outside the current scope.',
            'Study per-expedition encounter counts before assigning RL drop rates.',
            'Do not automatically turn excluded unique/set outcomes into extra rare rewards.',
        ],
        'current_rl_observation': {
            'source': 'src/game/equipment_bonuses.rs and src/loot/equipment.rs',
            'eligible_hostile_death': 'One equipment reward if the active monster catalogue is nonempty.',
            'model_selection': 'Uniform catalogue draw, excluding the previous model, without depth/monster-level filtering.',
            'enchanted_chance_percent': 60,
            'note': 'Observed existing code; not a proposed rate for the refactor.',
        },
        'rl_new_rates_approved': False, 'runtime_integration': False,
    }
    (DOC / 'diablo2-drop-study-02.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    lines = [
        '# ITEM-LOOT01-DROPSTUDY02 — butin de Diablo II classique', '',
        'Étude des tables locales, complétée par le guide officiel et la reconstruction du moteur D2MOO. Aucun nouveau taux Project RL appliqué.', '',
        '## Exemples calculés', '',
        'Solo `/players 1`, table nommée fixée, armes et armures hors anneaux/amulettes. Les valeurs de boss sont des moyennes par mort, pas des probabilités par tirage.', '',
        '| Source | Aucun objet | Au moins une arme/armure | Armes/armures moyennes | Tous objets moyens |',
        '| --- | ---: | ---: | ---: | ---: |',
    ]
    for r in examples:
        lines.append(f"| {r['label']} | {r['no_object_percent']:.4f} % | {r['weapon_armor_at_least_one_percent']:.2f} % | {r['mean_by_category']['weapon_armor']:.3f} | {r['mean_objects']:.3f} |")
    lines += ['', '## Lecture et limites', '',
              '- Les monstres ordinaires de la table H2H de départ ont 100 tickets sans objet sur 160 ; les armes/armures occupent 16 tickets, soit 10 %.',
              '- Le champion a un tirage principal avec 37 tickets d’armes/armures sur 100, puis des potions. Le monstre nommé utilise 58 tickets sur 62 pour son tirage principal, soit environ 93,55 %.',
              '- Les boss ont plusieurs tirages avec un plafond de six objets ; tous ces objets ne sont pas des équipements.',
              '- La qualité est un second calcul. Le facteur Magic=1024 des champions et monstres nommés garantit le test magique s’il est atteint après les qualités supérieures. Les facteurs ne sont pas des pourcentages de rareté.',
              '- Les tables H2H de départ et de fin d’Enfer gardent ici 10 % d’armes/armures ; leurs catalogues de modèles sont différents.',
              '- Le Magic Find intervient sur la qualité, le nombre virtuel de joueurs sur NoDrop. Le niveau et la source déterminent les modèles accessibles.',
              '- Les chiffres ne sont pas un calculateur de chances pour un objet nommé, ni une règle universelle pour toutes les familles de monstres.', '',
              '## Adaptation à Project RL', '',
              'Conserver les trois qualités standard, modifié et rare. Étudier séparément la fréquence des équipements, le palier des bases et la qualité. Le craft reste reporté. Les taux propres à Project RL nécessitent ensuite une calibration sur les rencontres d’une expédition ; aucune transposition brute ni conversion automatique des uniques/ensembles en rares n’est validée.', '',
              'Les distributions exactes, hypothèses, empreintes, contrôles et références sont dans `diablo2-drop-study-02.json`.', '']
    (DOC / 'DIABLO2_BUTIN_02.md').write_text('\n'.join(lines), 'utf-8')
    print(json.dumps({'cases': len(examples), 'checks': 'passed', 'examples': [
        {k: r[k] for k in ['label', 'no_object_percent', 'weapon_armor_at_least_one_percent', 'mean_by_category']} for r in examples
    ]}, ensure_ascii=False))


if __name__ == '__main__':
    main()
