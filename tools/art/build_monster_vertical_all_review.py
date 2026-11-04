"""Publish one animated selector for the entire base bestiary."""
import argparse
import html
import json

import build_monster_vertical15_review as prior
from prepare_monster_vertical_all import OUT, REVIEW
from prepare_monster_vertical_all_corrections import ACCEPTANCE, CORRECTIONS, REVIEW as CORRECTION_REVIEW

base = prior.base
FOLDER = 'monster-vertical-all'
CORRECTION_NOTES = {
    62: 'Les deux tentacules latéraux restent visibles de face et de dos.',
    78: 'La crête, les épaules et le bassin se lisent de dos en remontant.',
    79: 'La croupe passe au premier plan. La tête et les mains sont au loin.',
    82: 'Le corps est vu par l’arrière, avec ses quatre appuis et l’appendice frontal au loin.',
    89: 'La robe retombe sur les deux hanches pendant la reptation.',
    95: 'Les bras croisés restent devant le corps. Le dos montre le manteau.',
}


def label(profile):
    return f'{profile["review_number"]:02} · {html.escape(profile["name"])} · {profile["id"]}'


def main(page_only=False, corrections=False):
    base.PREFIX = f'{FOLDER}/{"r02" if corrections else "r01"}/current/'
    base.OUT = base.PAGE.parent / base.PREFIX
    base.REVIEW = CORRECTION_REVIEW if corrections else REVIEW
    retained = prior.retained_bank(21)
    manifest = json.loads(((CORRECTIONS if corrections else OUT) / 'manifest.json').read_text(encoding='utf-8'))
    data = json.loads((base.OUT / 'checks.json').read_text(encoding='utf-8'))
    profiles = manifest['profiles']
    assert [p['id'] for p in profiles] == [item['id'] for item in data['items']]
    for profile, item in zip(profiles, data['items']):
        if corrections:
            number = profile['review_number']
            item['scope_note'] = CORRECTION_NOTES.get(number, 'Les deux ailes prennent appui sur les omoplates, devant le torse vu de dos.')
        legs = profile['locomotor_legs']
        if legs == 0:
            item['phase_names'] = [f'Déplacement {phase}' for phase in range(1, 9)] + ['Repos frontal']
        elif legs > 2:
            item['phase_names'] = [
                'Appuis A', 'Transfert A', 'Passage B', 'Reprise B',
                'Appuis B', 'Transfert B', 'Passage A', 'Reprise A', 'Repos frontal',
            ]
    (base.OUT / 'checks.json').write_text(
        json.dumps(data, indent=2, ensure_ascii=False) + '\n', encoding='utf-8',
    )
    labels = [label(p) for p in profiles]
    count = len(labels)
    assert count == (11 if corrections else 69) and not manifest['missing']
    intro = f'Les n°58 à 60 sont validés. Les {count} nouveaux modèles (n°61 à 129) sont à examiner.'
    first_group, second_group = 'Nouveaux modèles à examiner', 'Lots précédents'
    if corrections:
        accepted = json.loads(ACCEPTANCE.read_text(encoding='utf-8'))
        approved = {p['id']: p['review_number'] for p in accepted['accepted']}
        previous_profiles = json.loads((OUT / 'manifest.json').read_text(encoding='utf-8'))['profiles']
        previous_labels = {p['id']: label(p) for p in previous_profiles}
        previous_prefix = f'{FOLDER}/r01/current/'
        previous_checks = json.loads((base.PAGE.parent / previous_prefix / 'checks.json').read_text(encoding='utf-8'))
        for item in previous_checks['items']:
            if item['id'] in approved:
                retained['items'].append({**item, 'capture_base': previous_prefix})
                retained['labels'].append(previous_labels[item['id']])
        ordered = sorted(zip(retained['items'], retained['labels']), key=lambda pair: approved[pair[0]['id']])
        retained = {'items': [item for item, _ in ordered], 'labels': [title for _, title in ordered]}
        assert {p['id'] for p in retained['items']} == set(approved) and len(approved) == 118
        first_group, second_group = 'Corrections à examiner', 'Modèles validés'
        intro = 'Les 118 modèles validés sont disponibles dans le menu. Les 11 corrections passent en tête pour la comparaison.'
    assert len({p['id'] for p in retained['items']} | {p['id'] for p in profiles}) == 129
    base.main(
        page_only=page_only, labels=labels, previous=retained,
        intro=intro,
        webp_method=0,
        workers=4,
    )
    page = base.PAGE.read_text(encoding='utf-8')
    start = page.index('<section id="monstres-vertical">')
    end = page.index('</section>', start)
    section = page[start:end]
    first = section.index('<option value=')
    previous = section.index(f'<option value="{retained["items"][0]["id"]}">')
    last = section.index('</select>', previous)
    section = (section[:first] + f'<optgroup label="{first_group}">' +
               section[first:previous] + f'</optgroup><optgroup label="{second_group}">' +
               section[previous:last] + '</optgroup>' + section[last:])
    section = section.replace(
        '</select></label><label>Direction',
        '</select></label><button id="mvertical-prev-model">Monstre précédent</button>'
        '<button id="mvertical-next-model">Monstre suivant</button><label>Direction',
        1,
    )
    section = section.replace('deux appuis alternés sur 800 ms', 'un cycle de 800 ms adapté à chaque morphologie')
    section = section.replace(
        'Le chantier couvre les <strong>129 modèles du bestiaire et leurs 471 variantes</strong>.',
        'Cet aperçu rassemble les <strong>129 modèles de base</strong> du bestiaire.',
    )
    section = section.replace(
        'Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos. '
        'Chaque nouvelle figure est dessinée entière. Les autres morphologies suivront leur locomotion propre.',
        'Sélectionne un monstre pour comparer son déplacement dans les deux sens et son retour au repos.',
    )
    if corrections:
        section = section.replace('<option value="up">', '<option value="up" selected>')
    page = prior.previous.previous.label_comparison_columns(page[:start] + section + page[end:])
    page = page.replace('Nouveaux modèles et premier lot de référence, comparés dans le décor du jeu à taille native.',
                        'Les 129 modèles de base, comparés dans le décor du jeu à taille native.')
    script_end = '</script><!-- /MONSTER-VERTICAL01 SCRIPT -->'
    navigation = """
function stepMverticalModel(delta) {
    const select = document.getElementById('mvertical-item');
    if (select.disabled) return;
    const index = select.selectedIndex + delta;
    if (index < 0 || index >= select.options.length) return;
    select.selectedIndex = index;
    select.dispatchEvent(new Event('change'));
}
document.getElementById('mvertical-prev-model').onclick = () => stepMverticalModel(-1);
document.getElementById('mvertical-next-model').onclick = () => stepMverticalModel(1);
"""
    page = page.replace(script_end, navigation + script_end, 1)
    base.PAGE.write_text(page, encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--page-only', action='store_true')
    parser.add_argument('--corrections', action='store_true')
    args = parser.parse_args()
    main(args.page_only, args.corrections)
