"""Repair review-only mine playback without changing native animation frames."""
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REVIEWS = ROOT / 'artifacts/refonte-agent-2026-10-06'


def repair_html(page):
    if 'id="demo"' in page:
        return page
    page = page.replace('<button id="cast">', '<button id="demo">Rejouer la séquence</button><button id="cast">', 1)
    page = page.replace('</style>', '\n#demo{background:#344852;border-color:#9ab4c4}#status{min-height:48px}</style>', 1)
    page = page.replace('<p class="note">Après la pose,', '<p class="note">La démonstration joue automatiquement la pose, l’armement, l’approche et l’explosion, puis attend votre prochaine action. « Rejouer la séquence » permet de la revoir. Le scénario « Attente » conserve volontairement la mine inactive.</p><p class="note">En lecture manuelle, après la pose,', 1)
    match = re.search(r'<script>([\s\S]*?)</script>', page)
    assert match
    script = match.group(1)
    script = script.replace('const hovered=new Set();', "const hovered=new Set();let autoPlay=false;const demo=document.getElementById('demo');", 1)
    # Each action must actually paint its first frame; time spent loading, paused
    # or in a background tab must not fast-forward past a short detonation.
    script = script.replace('elapsed=0;', 'elapsed=0;last=0;')
    old_clock = 'if(last&&ready&&!paused)elapsed+=now-last;last=now;'
    assert old_clock in script
    script = script.replace(old_clock, 'if(last&&ready&&!paused&&!document.hidden)elapsed+=Math.min(1000/24,Math.max(0,now-last));last=now;', 1)
    script = script.replace('last=now;if(ready){ui();', 'last=now;if(ready){updateDemo();ui();', 1)
    script = script.replace('function ui(){const es=entries();', 'function ui(){const es=entries();demo.disabled=!ready;', 1)
    ui_end = ").join(' · ');}"
    assert ui_end in script
    script = script.replace(ui_end, ").join(' · ');if(autoPlay)status.textContent='Démonstration automatique · '+status.textContent;if(ready&&launched&&!next.disabled){const future=es[0].timeline[es[0].action_frames[turn]],current=es[0].timeline[frame(es[0])];next.textContent=future.explosions>current.explosions?'Déclencher la mine':current.status&&current.status.some(n=>n>0)?'Tour suivant':turn===0?'Armer la mine':'Faire approcher l’ennemi';}else next.textContent='Action suivante';}", 1)
    script = script.replace('if(cast.disabled)return;launched=true;', 'if(cast.disabled)return;autoPlay=false;launched=true;', 1)
    script = script.replace('if(next.disabled)return;turn++;', 'if(next.disabled)return;autoPlay=false;turn++;', 1)
    script = script.replace("document.getElementById('reset').onclick=()=>{launched=false;", "document.getElementById('reset').onclick=()=>{autoPlay=false;launched=false;", 1)
    script = script.replace('paused=!paused;', 'paused=!paused;last=0;', 1)
    script = script.replace('ready=true;ui();}', 'ready=true;beginDemo();ui();}', 1)
    extra = r'''
function beginDemo(){if(!ready)return;document.getElementById('reset').onclick();paused=false;document.getElementById('pause').textContent='Pause';autoPlay=true;elapsed=0;last=0;ui();}
function updateDemo(){if(!autoPlay||paused||document.hidden)return;if(!launched){if(elapsed>=600){launched=true;turn=0;elapsed=0;last=0;}return;}if(elapsed<1000)return;const es=entries();if(es.every(e=>e.timeline[frame(e)].explosions>0)||turn>=Math.max(...es.map(e=>e.action_frames.length))){autoPlay=false;return;}const previous=turn;next.onclick();autoPlay=turn>previous;}
demo.onclick=beginDemo;
document.addEventListener('visibilitychange',()=>{last=0;});
'''
    script += extra
    return page[:match.start(1)] + script + page[match.end(1):]


def repair_review(folder):
    folder = Path(folder).resolve()
    assert folder.is_relative_to(REVIEWS.resolve())
    assert not (folder / 'visual-acceptance-01.json').exists()
    protected = {p.relative_to(folder).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest()
                 for p in folder.rglob('*') if p.is_file() and p.suffix in ('.png', '.gif')}
    for name in ('index.html', 'cote-a-cote.html'):
        path = folder / name
        original = path.read_text(encoding='utf8')
        backup = folder / name.replace('.html', '-before-playback-repair.html')
        if not backup.exists():
            backup.write_text(original, encoding='utf8')
        path.write_text(repair_html(original), encoding='utf8')
    assert all(hashlib.sha256((folder / n).read_bytes()).hexdigest() == h for n, h in protected.items())
    (folder / 'browser-playback-repair.json').write_text(json.dumps({
        'date': '2026-10-14', 'scope': 'Review player only; native art, timeline and campaign unchanged.',
        'cause': 'Placement did not advance to arming/contact without three manual actions; RAF gaps could skip the short burst.',
        'automatic_demo': 'Preview, placement, arming, approach, one explosion; stop before advancing any status turn.',
        'quiet_scenario': 'No automatic arming or explosion in ATTENTE.',
        'replay_button': 'Rejouer la séquence', 'manual_actions_retained': True,
        'clock': 'Reset at actions, pause and visibility changes; bound a displayed step to one native 24fps frame.',
        'preserved_image_files': len(protected), 'image_hashes_unchanged': True,
    }, ensure_ascii=False, indent=2) + '\n', encoding='utf8')


if __name__ == '__main__':
    for number in (62, 63):
        repair_review(REVIEWS / f'skill-animation-review-{number}')
        print(f'Review {number}: playback repaired; image files unchanged.')
