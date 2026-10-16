"""ANIM-S28: native64 stealth and movement economy, prepared review only."""
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageChops
import prepare_agent_skill_animation_heavy_finale as encoder
from add_browser_animation_comparison import add_comparison

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-58'
FRAMES = 160


def dump(name, obj):
    (OUT / name).write_text(json.dumps(obj, ensure_ascii=False, indent=2) + '\n', encoding='utf8')


def write_html(entries):
    # Reuse only the HTML factory, writing into this new review directory.
    encoder.OUT = OUT
    encoder.write_html(entries)
    page = (OUT / 'index.html').read_text(encoding='utf8')
    replacements = {
        'length:128': 'length:160', 'turn>=3': 'turn>=4',
        'const base=[17,64,88,112][turn],span=turn===3?12:20;': 'const base=[17,64,88,112,136][turn],span=20;',
        "e.title+(e.permitted?(e.id==='HL05'?' : rayon '+t.radius+' · dégâts inchangés':' : PV '+t.hp+'/100 · soin +'+t.healed+' · recharge '+t.cooldown+(t.pending?' · explosion en attente':'')):' : indisponible')": "e.title+' : '+(t.active?'furtif':'à découvert')+' · énergie '+t.energy+' · dépensée '+t.spent+' · pas '+t.walks",
        '<h2>63 · Déflagration amplifiée</h2>': '<h2>65 · Furtivité</h2>',
        '<h2>64 · Drain d’impact</h2>': '<h2>66 · Économie de mouvement furtif</h2>',
        'data-skill="HL05"': 'data-skill="SF00"', 'data-skill="HL06"': 'data-skill="SF01"',
        'L’explosion de l’arme couvre un rayon plus large ; les dégâts restent identiques.': 'La silhouette devient translucide ; le mode dépense 11 énergie par pas sans passif.',
        'Un impact habituel, puis une impulsion verte et le soin reçu sur le joueur.': 'Le même camouflage, avec seulement 1 énergie par pas au rang 10 du passif.',
        '</canvas></article>': '</canvas><div class="tip" hidden></div></article>',
        'Déflagration et drain · ANIM-S27': 'Furtivité et économie · ANIM-S28',
        'Armes lourdes · lot 27': 'Furtivité · lot 28',
        'Deux candidats : une explosion agrandie et un tir qui rend de la vie. Les nombres servent d’exemple.': 'Deux candidats : le camouflage actif et le coût réduit des pas. Les tours avancent avec les boutons.',
        '>Lancer</button>': '>Activer Furtivité</button>',
        'h2{font-size:21px}': 'h2{font-size:21px;min-height:52px}',
        '.note{font-size:13px}': '.note{font-size:13px}.tip{position:absolute;right:26px;top:210px;background:#10151b;border:1px solid #737f91;color:#e0e7e7;max-width:282px;padding:12px;pointer-events:none}',
        'Les tours avancent uniquement avec « Action suivante ». L’impact instantané se termine, et le soin est reçu à l’impact. Dans l’exemple à retardement, cliquez sur « Action suivante » pour faire exploser le projectile.': 'Le camouflage reste actif entre les actions ; son icône n’a pas de timer. Survolez-la pour lire le coût du pas. « Action suivante » déclenche un pas ou l’événement du scénario choisi.',
        'Déflagration est un passif : elle agrandit l’explosion sans ajouter de dégâts ni créer d’explosion sur une arme qui n’en possède pas. Drain soigne à partir de la cible principale ; les voisins touchés ne donnent aucun soin supplémentaire.': 'Cas préparés : attente, menu, soin, action offensive et sortie de furtivité à zéro énergie. Le même pas est alors effectué à découvert. Le passif n’ajoute pas une seconde icône de buff. La perception des ennemis et la régénération ne sont pas simulées ici.',
        "canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);": "{canvas.getContext('2d').drawImage(sequences.get(e.id)[frame(e)],0,0);const tip=canvas.parentElement.querySelector('.tip');if(!e.timeline[frame(e)].active)tip.hidden=true;}",
    }
    for old, new in replacements.items():
        assert old in page, old
        page = page.replace(old, new)
    handler = """for(const canvas of document.querySelectorAll('canvas')){canvas.onmousemove=ev=>{const e=entries().find(e=>e.id===canvas.dataset.skill),box=canvas.getBoundingClientRect(),x=ev.clientX-box.left,y=ev.clientY-box.top,tip=canvas.parentElement.querySelector('.tip'),t=e?.timeline[frame(e)];tip.hidden=!(ready&&launched&&t?.active&&x>=488&&x<=528&&y>=16&&y<=56);if(!tip.hidden)tip.textContent='Furtivité · '+t.cost+' énergie par pas · Distance de repérage : 2 cases';};canvas.onmouseleave=()=>canvas.parentElement.querySelector('.tip').hidden=true;}
"""
    page = page.replace('scenario.onchange=load;', handler + 'scenario.onchange=load;', 1)
    (OUT / 'index.html').write_text(page, encoding='utf8')


def package():
    assert not (OUT / 'visual-acceptance-01.json').exists()
    proof = json.loads((OUT / 'review-protection.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT / n).read_bytes()).hexdigest() == h for n, h in proof.items())
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    checks, animations, overview = [], [], []
    encoder.FRAMES = FRAMES
    for e in entries:
        native = OUT / 'native' / e['folder']
        timeline = json.loads((native / 'timeline.json').read_text(encoding='utf8'))
        originals = [Image.open(native / f'{n:03}.png').convert('RGB') for n in range(FRAMES)]
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
            control = Image.open(native / f'control-{n:03}.png').convert('RGB')
            delta = ImageChops.difference(originals[n], control).crop((32, 88, 608, 664))
            for y, row in enumerate(e['map'].splitlines()):
                for x, cell in enumerate(row):
                    if cell == '#':
                        assert delta.crop((x * 64, y * 64, (x + 1) * 64, (y + 1) * 64)).getbbox() is None, (e['variant'], n, x, y)
        assert timeline[14]['preview_visible'] and not timeline[17]['preview_visible']
        assert timeline[17]['active'] and timeline[17]['energy'] == timeline[0]['energy']
        assert abs(timeline[44]['opacity'] - .56) < .001
        case, last, cost = e['case'], timeline[-1], timeline[0]['cost']
        if case == 'ATTENTE':
            assert last['turn'] == 1 and last['spent'] == 0 and last['active'] and last['walks'] == 0
        elif case == 'OBSTACLE':
            assert last['spent'] == 0 and last['player'] == [4, 6] and last['active'] and last['turn'] == 1
        elif case == 'MENU':
            assert last['turn'] == 1 and last['spent'] == 0 and last['active'] and timeline[64]['menu']
        elif case == 'SOIN':
            assert last['hp'] == 55 and last['active'] and last['spent'] == 0 and last['walks'] == 0
        elif case == 'OFFENSIVE':
            assert not timeline[64]['active'] and last['opacity'] == 1 and last['spent'] == 0 and last['walks'] == 0
        elif case == 'ZERO-ENERGIE':
            assert not timeline[64]['active'] and timeline[64]['player'] == [4, 5] and last['walks'] == 4 and last['energy'] == 0 and last['spent'] == 0
        elif case == 'EPUISEMENT':
            assert timeline[112]['active'] and timeline[112]['energy'] == 0 and timeline[112]['walks'] == 3
            assert not timeline[136]['active'] and last['walks'] == 4 and last['spent'] == cost * 3 and last['opacity'] == 1
        else:
            assert last['active'] and last['walks'] == 4 and last['spent'] == cost * 4 and last['energy'] == 100 - cost * 4
        assert all(t['cost'] >= 1 for t in timeline)
        animations.append(encoder.gif(OUT / f"{e['variant']}-64.gif", frames))
        e.update(timeline=timeline, image_indices=indices)
        checks.append({'variant': e['variant'], 'walls_clear': True, 'manual_steps_and_costs_checked': True, 'preview_removed': True})
        if case == 'HAUT':
            overview.extend(originals)
            for n in [14, 20, 26, 44, 64, 68, 88, 112, 136, 148]:
                originals[n].save(OUT / f"{e['id']}-phase-{n:03}.png")
        print(e['variant'] + ' packaged', flush=True)
    animations.append(encoder.gif(OUT / 'furtivite-economie-64.gif', overview))
    dump('verification.json', {'native_views': len(entries), 'native_tile_size': 64, 'resampled': False, 'protected_files_unchanged': len(proof), 'checks': checks, 'animations': animations})
    dump('review.json', {
        'batch': 'ANIM-S28', 'date': '2026-10-14', 'status': 'awaiting_visual_review', 'campaign_integration': False,
        'candidates': [{'number': 65, 'id': 'SF00', 'name': 'Furtivité', 'variant': 'SF00-A01'}, {'number': 66, 'id': 'SF01', 'name': 'Économie de mouvement furtif', 'variant': 'SF01-A01'}],
        'approved_cards': ['review-individual-description-SF00-11.json', 'review-individual-description-SF01-04.json'],
        'mechanics': 'SF00 purchased once; step cost11 without economy, activation0, stationary0 and cooldown0. SF01 rank10 cost1, rank9 cost2, effective12 remains1. Mode persists through idle, menu and healing; offense ends it. At zero energy, attempted movement ends the mode and the same step proceeds normally.',
        'visuals': 'Original player and weapon layers unchanged; smooth0.24s transition to56% opacity, short outlined steel-blue scan, fast0.12s movement. Persistent32px SF00 status icon without countdown; energy feedback above player and status tooltip. SF01 changes cost and permanent numeric panel without an extra timed buff.',
        'prepared_profiles': 'SF00 without economy versus SF01 learned10; cases rank9 and effective12. Energy100, zero or exactly3 payable steps; no regeneration. Fixed monsters are visual context. Healing+5 is a supplied review event, not a new healing skill.',
        'scope': 'Independent fixtures; no GameState, perception AI, sound propagation, save, loot or RNG mutation. Scene directions change positions while bodies stay frontal. Tooltip uses the accepted two-cell detection example without creating vision cones or erasing NPC memory. Both profiles own SF00.',
        'pending': 'Positive insufficient energy, zero energy combined with blocked movement, real turn/resource/regeneration ordering, mode exit action ordering, silence/transformation/gear interactions and detection curves/sounds/memory remain separate. Offensive scene removes camouflage without executing an attack.',
        'temporary_actors': 'Existing enemy32 art temporarily enlarged64; adaptation after animations.',
    })
    write_html(entries)
    add_comparison(OUT)
    (OUT / 'render-source.rs').write_bytes((ROOT / 'src/agent_skill_lab_review/fire_painted/stealth.rs').read_bytes())
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    dump('progress-audit.json', {'date': '2026-10-14', 'previous_unshown': 47, 'newly_shown': ['SF00', 'SF01'], 'remaining_unshown': 45, 'next_stealth_candidate': 'SF02', 'visual_review_status': 'SF00-A01 and SF01-A01 awaiting approval; other outstanding reviews preserved.'})
    print(json.dumps({'views': len(entries), 'animations': len(animations), 'checks': 'passed'}))


if __name__ == '__main__':
    package()
