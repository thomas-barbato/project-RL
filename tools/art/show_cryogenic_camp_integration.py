"""Keep the current camp review and add the actual campaign-client captures."""
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def publish():
    capture = OUT / 'campaign-integration-01'
    checks = json.loads((capture/'checks.json').read_text(encoding='utf-8'))
    assert checks['generation_version'] == 153 and checks['native_cell'] == 64
    assert len(checks['scenes']) == 6 and all(s['render_is_read_only'] for s in checks['scenes'])
    assert all(s['native_cell'] == 64 for s in checks['scenes'] if s['hud_clip_checked'])
    assert not (capture/'stderr.log').read_text(encoding='utf-8').strip()
    old = json.loads((ROOT/'assets/prototypes/handcrafted-camp-01/r09/camp.json').read_text(encoding='utf-8'))
    current = json.loads((ROOT/'assets/maps/cryogenic-camp-01/camp.json').read_text(encoding='utf-8'))
    digest = lambda p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest()
    pairs = [(old[k], current[k]) for k in ['floor', 'structure', 'layout']]
    pairs += [(old['atmosphere'][k], current['atmosphere'][k]) for k in ['correction', 'irradiance']]
    assert [(p['id'], p['at']) for p in old['props']] == [(p['id'], p['at']) for p in current['props']]
    assert old['actors'] == current['actors']
    pairs += [(a['file'], b['file']) for a, b in zip(old['props'], current['props'])]
    assert all(digest(a) == digest(b) for a, b in pairs)
    native_sources = {b: digest(b) for _, b in pairs}
    native_sources.update({a['file']:digest(a['file']) for a in current['actors'][1:]})
    assert 'artifacts/' not in json.dumps(current)
    shader = lambda p: (ROOT/p).read_text(encoding='utf-8').split('const VERTEX:')[1]
    assert shader('src/camp_atmosphere.rs') == shader('examples/handcrafted_camp/atmosphere.rs')
    evidence = dict(campaign_connected=True, generation_version=153, spawn=[2,8],
                    native_cell=64, visual_revision='r09b', source_art_unchanged=True,
                    shader_sources_match_accepted_review=True, native_sources_sha256=native_sources,
                    gpu_capture_checks=checks, preliminary_sector='core:industrial_sector',
                    picture_dimensions=[38,18], logical_dimensions=[39,18],
                    padding='One unpainted right wall column; reviewed positions unchanged.')
    (capture/'integration-evidence.json').write_text(json.dumps(evidence,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    page = OUT/'index.html'
    text = page.read_text(encoding='utf-8')
    text = re.sub(r'<!-- campaign-client:start -->.*?<!-- campaign-client:end -->', '', text, flags=re.S)
    section = '''<!-- campaign-client:start -->
<section id="camp-en-jeu"><h2>Le camp dans la campagne jouable</h2>
<p>Les nouveaux personnages commencent près de la cuve ouverte. L’officier donne le dialogue corrigé ; l’armurier, le médecin et le technicien proposent les services existants.</p>
<p><a class="button" href="../../Jouer.cmd">Jouer — client normal</a></p>
<nav><button data-campaign="start">Départ réel</button><button data-campaign="officer">Officier</button><button data-campaign="hall">Hall</button><button data-campaign="medical">Infirmerie</button><button data-campaign="armorer">Armurier</button><button data-campaign="technician">Technicien</button></nav>
<p>Captures du moteur et de son interface : 1 600 × 1 000 px, cases de 64 px. Faire défiler si nécessaire ; aucune réduction des images. La visibilité et les zones mémorisées sont celles de la partie réelle.</p>
<div class="viewport"><img id="campaign-image" src="campaign-integration-01/camp-start.png" width="1600" height="1000" alt="Le nouveau personnage près de la cuve ouverte, dans le client jouable."></div>
<p>Les sauvegardes des anciennes générations conservent leur départ. La sortie du camp rejoint provisoirement le secteur industriel existant, en attendant notre nouveau premier secteur.</p>
<script>document.querySelectorAll('[data-campaign]').forEach(b=>b.addEventListener('click',()=>{document.getElementById('campaign-image').src='campaign-integration-01/camp-'+b.dataset.campaign+'.png';document.querySelectorAll('[data-campaign]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));}));</script></section>
<!-- campaign-client:end -->'''
    text = text.replace('</header>', '</header>'+section, 1)
    page.write_text(text,encoding='utf-8')
    print(json.dumps(dict(gallery=str(page),native_sources_verified=len(native_sources),scenes=6)))


if __name__ == '__main__':
    publish()
