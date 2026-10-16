"""ANIM-S29: native first stealth attack and conditional discretion UI."""
import hashlib
import json
import inspect
from pathlib import Path

from PIL import Image, ImageChops
import prepare_agent_skill_animation_stealth as gallery
from add_browser_animation_comparison import add_comparison

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-59'
FRAMES = 160


def dump(name, obj):
    (OUT / name).write_text(json.dumps(obj, ensure_ascii=False, indent=2) + '\n', encoding='utf8')


def write_html(entries):
    gallery.OUT = OUT
    gallery.write_html(entries)
    page = (OUT / 'index.html').read_text(encoding='utf8')
    replacements = {
        '<h2>65 · Furtivité</h2>': '<h2>65 · Furtivité — attaque sournoise</h2>',
        '<h2>66 · Économie de mouvement furtif</h2>': '<h2>67 · Discrétion renforcée</h2>',
        'data-skill="SF01"': 'data-skill="SF02"',
        'La silhouette devient translucide ; le mode dépense 11 énergie par pas sans passif.': 'Le bonus renforce l’attaque lancée en furtivité, puis le mode prend fin.',
        'Le même camouflage, avec seulement 1 énergie par pas au rang 10 du passif.': 'La distance de repérage de l’exemple passe à 1 case lorsque Furtivité est active.',
        'Furtivité et économie · ANIM-S28': 'Attaque sournoise et discrétion · ANIM-S29',
        'Furtivité · lot 28': 'Furtivité · lot 29',
        'Deux candidats : le camouflage actif et le coût réduit des pas. Les tours avancent avec les boutons.': 'Une révision de Furtivité et un nouveau passif. Base de dégâts préparée : 20 ; au rang 10, l’attaque furtive affiche 30 (+10).',
        '>Activer Furtivité</button>': '>Préparer la scène</button>',
        "e.title+' : '+(t.active?'furtif':'à découvert')+' · énergie '+t.energy+' · dépensée '+t.spent+' · pas '+t.walks": "e.title+' : '+(t.active?'furtif':'à découvert')+' · dernier impact '+t.last_damage+' (bonus '+t.last_bonus+') · distance furtive '+t.distance+' '+(t.distance===1?'case':'cases')",
        "if(!tip.hidden)tip.textContent='Furtivité · '+t.cost+' énergie par pas · Distance de repérage : 2 cases';": "if(!tip.hidden)tip.textContent='Furtivité · attaque directe +'+t.bonus_percent+' % · Distance de repérage : '+t.distance+' '+(t.distance===1?'case':'cases')+' · 1 énergie par pas';",
        'return step<span?base+step:base+span+(step-span)%4;': 'return base+Math.min(step,23);',
        'es.every(e=>e.quiet)': "es.every(e=>e.quiet)||scenario.value==='CLOISON'",
        'Le camouflage reste actif entre les actions ; son icône n’a pas de timer. Survolez-la pour lire le coût du pas. « Action suivante » déclenche un pas ou l’événement du scénario choisi.': 'Après préparation : première attaque renforcée, attaque ordinaire à découvert, réactivation de Furtivité, puis nouvelle attaque renforcée. Utilisez « Action suivante ». Les impacts se terminent sans se répéter entre les tours.',
        'Cas préparés : attente, menu, soin, action offensive et sortie de furtivité à zéro énergie. Le même pas est alors effectué à découvert. Le passif n’ajoute pas une seconde icône de buff. La perception des ennemis et la régénération ne sont pas simulées ici.': 'Exemples : tir, mêlée, sort direct de feu, rangs 11/12, déplacement avant attaque et cloison. Les deux profils ont Économie au rang 10, soit 1 énergie par pas. Le passif de droite est au rang 11. La perception, le bruit et l’IA ne sont pas simulés ; les distances sont les exemples déjà validés.',
    }
    for old, new in replacements.items():
        assert old in page, old
        page = page.replace(old, new)
    (OUT / 'index.html').write_text(page, encoding='utf8')


def save_gif(path, images):
    source = inspect.getsource(gallery.encoder.gif).replace(
        '[14,20,24,28,32,64,68,72]',
        '[14,20,28,44,64,68,76,80,88,96,112,136,144,148,158]')
    namespace = dict(gallery.encoder.__dict__)
    namespace['FRAMES'] = FRAMES
    exec(compile(source, '<review59-palette-encoder>', 'exec'), namespace)
    return namespace['gif'](path, images)


def package():
    assert not (OUT / 'visual-acceptance-01.json').exists()
    proof = json.loads((OUT / 'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT / n).read_bytes()).hexdigest() == h for n, h in proof.items())
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    checks, animations, overview = [], [], []
    gallery.encoder.FRAMES = FRAMES
    for e in entries:
        directory = OUT / 'native' / e['folder']
        timeline = json.loads((directory / 'timeline.json').read_text(encoding='utf8'))
        originals = [Image.open(directory / f'{n:03}.png').convert('RGB') for n in range(FRAMES)]
        frames = [im.crop((32, 88, 608, 664)) for im in originals]
        dest = OUT / 'frames' / e['variant']
        dest.mkdir(parents=True, exist_ok=True)
        canonical, indices = {}, []
        for n, im in enumerate(frames):
            key = hashlib.sha256(im.tobytes()).hexdigest()
            if key not in canonical:
                canonical[key] = n
                im.save(dest / f'{n:03}.png', compress_level=1)
            indices.append(canonical[key])
        for n in e['control_frames']:
            control = Image.open(directory / f'control-{n:03}.png').convert('RGB')
            delta = ImageChops.difference(originals[n], control).crop((32, 88, 608, 664))
            for y, row in enumerate(e['map'].splitlines()):
                for x, cell in enumerate(row):
                    if cell == '#':
                        assert delta.crop((x * 64, y * 64, (x + 1) * 64, (y + 1) * 64)).getbbox() is None, (e['variant'], n, x, y)
        assert timeline[14]['preview_visible'] and not timeline[17]['preview_visible']
        assert all(t['distance'] == (1 if e['id'] == 'SF02' else 2) for t in timeline)
        rank = timeline[0]['rank']
        assert timeline[0]['bonus_percent'] == rank * 5
        last = timeline[-1]
        case = e['case']
        if case in ['ATTENTE', 'CLOISON']:
            assert last['active'] and last['shots'] == 0 and last['total_damage'] == 0 and last['energy'] == 100 and last['turn'] == 1
        elif case == 'PAS':
            assert timeline[64]['active'] and timeline[64]['energy'] == 99 and timeline[64]['player'] == [4, 5]
            assert timeline[88]['snapshot_bonus_percent'] == rank * 5 and not timeline[88]['active']
            assert last['shots'] == 2 and last['total_damage'] == 2 * (20 + rank) and last['energy'] == 99
        elif case == 'DECOUVERT':
            assert all(not t['active'] and t['snapshot_bonus_percent'] == 0 for t in timeline)
            assert last['shots'] == 3 and last['total_damage'] == 60 and last['last_damage'] == 20
        else:
            assert timeline[64]['shots'] == 1 and timeline[64]['snapshot_bonus_percent'] == rank * 5 and not timeline[64]['active']
            assert timeline[80]['last_damage'] == 20 + rank and timeline[80]['last_bonus'] == rank
            assert timeline[88]['shots'] == 2 and timeline[88]['snapshot_bonus_percent'] == 0
            assert timeline[108]['last_damage'] == 20 and timeline[108]['last_bonus'] == 0
            assert timeline[112]['active'] and timeline[112]['shots'] == 2
            assert timeline[136]['shots'] == 3 and timeline[136]['snapshot_bonus_percent'] == rank * 5
            assert last['total_damage'] == 60 + rank * 2 and last['last_damage'] == 20 + rank
        assert last['target_hp'] == 100 - last['total_damage']
        animations.append(save_gif(OUT / f"{e['variant']}-64.gif", frames))
        e.update(timeline=timeline, image_indices=indices)
        checks.append({'variant': e['variant'], 'walls_clear': True, 'first_bonus_then_normal_attack_checked': True, 'rank_and_detection_values_checked': True})
        if case == 'HAUT':
            overview.extend(originals)
            for n in [14, 44, 64, 68, 72, 80, 88, 96, 112, 136, 144, 158]:
                originals[n].save(OUT / f"{e['id']}-phase-{n:03}.png")
        print(e['variant'] + ' packaged', flush=True)
    animations.append(save_gif(OUT / 'sournoise-discretion-64.gif', overview))
    dump('verification.json', {'native_views': len(entries), 'native_tile_size': 64, 'resampled': False, 'protected_files_unchanged': len(proof), 'checks': checks, 'animations': animations})
    dump('review.json', {
        'batch': 'ANIM-S29', 'date': '2026-10-14', 'status': 'awaiting_visual_review', 'campaign_integration': False,
        'candidates': [{'number': 65, 'id': 'SF00', 'name': 'Furtivité — attaque sournoise', 'variant': 'SF00-A02', 'revision_of_accepted': 'SF00-A01'}, {'number': 67, 'id': 'SF02', 'name': 'Discrétion renforcée', 'variant': 'SF02-A01'}],
        'approved_cards': ['review-individual-description-SF00-13.json', 'review-individual-description-SF02-02.json'],
        'mechanics': 'SF00 direct attack bonus5% per effective rank, rank10=50%, rank11=55%, rank12=60%; weapons and spells. The initiating attack retains its bonus while stealth ends; later attack receives no bonus until stealth is active again. SF02 rank11 conditional detection example1cell instead of2; no timed buff. Both prepared profiles own SF01 rank10 with1energy step.',
        'visuals': 'Exact accepted cloak scan and opacity reused. Physical impacts use accepted steel streak, melee cut and blood; fire example uses accepted projectile/burst assets without blood. Shared target recoil, total damage with bonus and per-type icon; SF00 icon clears on offensive launch. SF02 updates conditional tooltip and permanent numeric panel without extra timed buff.',
        'prepared_numbers': 'Base20. Rank10 first impact30(+10), normal20, recloaked impact30(+10). Rank11 shows31(+11), rank12 shows32(+12). No critical, armor, variance or other damage modifier. TargetHP100 stays positive. Damage examples do not define real weapon or spell stats.',
        'scope': 'Isolated prepared actions. No campaign GameState, AI, real perception/hearing/memory, RNG, save or loot. Fire scene only illustrates a direct magical hit using accepted artwork; it does not implement the full FE05 area, burn, cost or cooldown. Wall attack is unavailable in the fixture, not a general failed-action rule. No hidden terrain or actors are revealed.',
        'pending': 'Full native sequences, bursts, areas, channels and delayed impacts; critical/modifier/defense order; rejected attack consumption; indirect status definitions; transformation/familiar/equipment snapshot interactions; SF02 perception metrics, attribute curves, sound and memory still need real integration.',
        'temporary_actors': 'Existing enemy32 art temporarily enlarged64; adaptation after animations.',
        'excluded_drafts': ['native-label-draft'],
        'draft_note': 'Only native/ and its packaged frames are the candidate; the old-label capture is unpresented and excluded.',
        'preserved_approvals': 'Review58 camouflage and SF01 remain frozen; SF00-SNEAK01-ACCEPT01 numeric approval is preserved. New bonus presentation and SF02 numeric UI await visual approval.',
    })
    write_html(entries)
    add_comparison(OUT)
    (OUT / 'render-source.rs').write_bytes((ROOT / 'src/agent_skill_lab_review/fire_painted/stealth_sneak.rs').read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    dump('progress-audit.json', {'date': '2026-10-14', 'previous_unshown': 45, 'newly_shown': ['SF02'], 'revised_skill': 'SF00', 'remaining_unshown': 44, 'visual_review_status': 'SF00-A02 and SF02-A01 awaiting approval; accepted SF00-A01 and SF01-A01 preserved.'})
    print(json.dumps({'views': len(entries), 'animations': len(animations), 'checks': 'passed'}))


if __name__ == '__main__':
    package()
