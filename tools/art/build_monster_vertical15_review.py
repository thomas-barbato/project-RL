"""Publish the next native vertical walk review and keep earlier candidates visible."""
import argparse
import json
import build_monster_vertical14_review as previous
from build_monster_vertical03_review import bank
from prepare_monster_vertical15 import BATCH_MODELS

base = previous.base
REVIEW_COPY = {
    15: (
        'Le dos corrigé du n°42 est validé. Le lot présente une bête basse à quatre pattes, une aberration aux tentacules et un colosse sans tête.',
        'Vérifier les quatre appuis du n°43, les tentacules du n°44, le dos du n°45 et le raccord au repos.',
    ),
    16: (
        'Les n°43–45 sont validés. Le lot présente le colosse à carapace sombre, le démon au bras-lame et la brute voûtée.',
        'Vérifier les appuis du colosse, la lame du n°47 sur le même bras de face et de dos, et le raccord au repos.',
    ),
    17: (
        'Les n°46–48 sont validés. Le lot présente deux quadrupèdes et un scorpion.',
        'Observer les appuis des n°49 et 50, puis le mouvement des pattes, des pinces et de la queue du n°51.',
    ),
    18: (
        'Les n°49 à 51 sont validés. Le lot suivant présente trois bipèdes aux silhouettes distinctes.',
        'Observer les petits pas du n°52, la bosse du n°53 et les deux lames du n°54 dans les deux sens.',
    ),
    19: (
        'Les n°52 à 54 sont validés. Le lot suivant présente un quadrupède trapu, un oiseau au sol et une créature aux pattes anguleuses.',
        'Observer le corps bas du n°55, les ailes repliées du n°56 et les quatre appuis du n°57.',
    ),
    20: (
        'Les n°55 à 57 sont validés. Le lot suivant présente une créature à six pattes, un quadrupède pâle et un colosse enchaîné.',
        'Observer les appuis des n°58 et 59, puis la chaîne du n°60 devant le corps de face et masquée par les hanches de dos.',
    ),
}


def batch_labels(batch):
    return [f'{number} · {row[0]}' for number, row in enumerate(BATCH_MODELS[batch].values(), 43 + (batch - 15) * 3)]


def retained_bank(batch):
    retained = previous.retained_bank()
    prefix = 'monster-vertical-14/r02/current/'
    old = json.loads((base.PAGE.parent / prefix / 'checks.json').read_text(encoding='utf-8'))
    retained['items'] = [{**p, 'capture_base': prefix} for p in old['items']] + retained['items']
    retained['labels'] = [
        '40 · Démon trapu à épaule rouge',
        '41 · Démon pâle voûté',
        '42 · Démon aux griffes dorées · dos corrigé validé',
    ] + retained['labels']
    for earlier in range(15, batch):
        retained['items'] = bank(f'monster-vertical-{earlier}') + retained['items']
        retained['labels'] = [label + ' · validé' for label in batch_labels(earlier)] + retained['labels']
    assert len({p['id'] for p in retained['items']}) == len(retained['labels']) == 42 + (batch - 15) * 3
    return retained


def main(batch=15):
    base.PREFIX = f'monster-vertical-{batch}/r01/current/'
    base.OUT = base.PAGE.parent / base.PREFIX
    base.REVIEW = f'MONSTER-VERTICAL{batch}-r01'
    retained = retained_bank(batch)
    intro, review_note = REVIEW_COPY[batch]
    base.main(labels=batch_labels(batch), previous=retained, intro=intro)
    html = base.PAGE.read_text(encoding='utf-8')
    start = html.index('<section id="monstres-vertical">')
    end = html.index('</section>', start)
    section = html[start:end].replace('deux appuis alternés sur 800 ms', 'un cycle de 800 ms adapté à chaque morphologie')
    section = section.replace(
        'Vérifier les appuis, les silhouettes voûtée et massive, les deux mains sur la hache et le raccord au repos.',
        review_note,
    )
    base.PAGE.write_text(previous.previous.label_comparison_columns(html[:start]+section+html[end:]), encoding='utf-8')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--batch', type=int, choices=BATCH_MODELS, default=15)
    main(parser.parse_args().batch)
