"""Record verified EG-COMP-04 integration and refresh the current review page."""
from html import escape
from html.parser import HTMLParser
from urllib.parse import urlsplit, unquote
from PIL import Image
import build_sewer_review_04 as review

ROOT, DOC = review.ROOT, review.DOC
OUT = ROOT / 'artifacts/sewer-playtest-04'


def refresh_page(path):
    if not path.exists() or not (DOC / 'integration.json').exists():
        return
    html = path.read_text(encoding='utf8')
    start, end = '<!-- sewer-integration-start -->', '<!-- sewer-integration-end -->'
    if start in html:
        a, rest = html.split(start, 1)
        html = a + rest.split(end, 1)[1]
    section = start + '''<section id="integration">
<h2>Intégré au jeu · essai des égouts</h2>
<p class="notice"><strong>Décors et peuplement validés le 20 octobre.</strong>
Les trois agencements acceptés sont jouables, avec 85, 104 et 78 monstres.
Ce premier essai choisit l’un de ces trois plans selon la graine de partie.
Le générateur de plans entièrement variables reste une étape suivante.</p>
<p>Pour jouer : lance <a href="../../Jouer.cmd">Jouer.cmd</a>, ou <code>cargo run --locked</code>,
puis commence une nouvelle partie au camp. Le portail sud rejoint la ville ; la trappe est en X 2, Y 14.
E ou le clic permet d’entrer ; l’escalier d’arrivée permet de revenir dans la rue.
Les nouvelles parties normales proposent aussi ce passage ; les anciennes gardent leur monde.</p>
<p>Les rats et l’eau sont animés. Les règles d’XP et les profils de combat sont conservés.
La réanimation EG-NEC-02 reste en attente de validation séparée.</p>
<details><summary>Voir les captures de la version intégrée</summary>'''
    for label, caption in [
        ('variant-1-street-entry', 'Trappe sur le trottoir · véritable session jouable'),
        ('variant-1-arrival', 'Arrivée dans les égouts · véritable session jouable'),
        ('variant-1-refuge', 'Refuge et groupe de cultistes · scène de contrôle du rendu livré'),
    ]:
        section += f'<figure><figcaption>{escape(caption)}</figcaption><div class="native"><img loading="lazy" src="../sewer-playtest-04/native/{label}.png" width="1218" height="844" alt="{escape(caption)}"></div></figure>'
    section += '''</details><p class="small">Contrôles : 23 tests ciblés réussis,
compilation de développement et vérification release réussies. Entrée, retour et
reprise de sauvegarde contrôlés dans le moteur pour les trois variantes.
Les scènes de contrôle du rendu ne constituent pas un essai d’équilibrage.</p></section>''' + end
    html = html.replace('</h1>', '</h1>' + section, 1)
    html = html.replace('Ces changements restent dans l’aperçu.',
                        'Ces changements sont maintenant intégrés pour l’essai en jeu.')
    class Refs(HTMLParser):
        def handle_starttag(self, tag, attrs):
            for key, value in attrs:
                if key in ('href', 'src', 'data-src'):
                    url = urlsplit(value)
                    if not url.scheme and url.path:
                        assert (path.parent / unquote(url.path)).resolve().exists(), value
    Refs().feed(html)
    path.write_text(html, encoding='utf8')


def record():
    review.habitat.accepted_unchanged()
    agreement = review.read(DOC / 'acceptance.json')
    for file, digest in agreement['accepted_source_sha256'].items():
        assert review.sha(ROOT / file) == digest, file
    checks = review.read(OUT / 'engine/checks.json')
    assert checks['integration'] and checks['generation'] == 162
    assert [v['monsters'] for v in checks['variants']] == [85, 104, 78]
    assert all(v['entry_and_return'] and v['snapshot_restore'] for v in checks['variants'])
    assert not checks['resurrection_active']
    captures = sorted((OUT / 'engine').glob('variant-*.png'))
    assert len(captures) == 18
    (OUT / 'native').mkdir(exist_ok=True)
    for label in ('variant-1-street-entry', 'variant-1-arrival', 'variant-1-refuge'):
        with Image.open(OUT / 'engine' / f'{label}.png') as im:
            # Windows may clamp the requested client height to the work area.
            # Crop without resampling; the complete native game viewport fits.
            assert im.width == 1920 and im.height >= 894, im.size
            im.crop((334, 50, 1552, 894)).save(OUT / 'native' / f'{label}.png')
    runtime = ROOT / 'assets/maps/prison-sewer-01'
    review.write(DOC / 'integration.json', dict(
        review='EG-COMP-04 / EG-POP-04', date='2026-10-20', status='integrated_for_playtest',
        generation=162, entry=[2, 14], runtime=review.rel(runtime),
        layout_policy='one of the three accepted layouts, selected deterministically by world seed',
        ordinary_profiles_unchanged=True, xp_unchanged=True, resurrection_active=False,
        legacy_worlds_unchanged=True, launcher='Jouer.cmd',
        engine_checks=checks, engine_capture_count=len(captures),
        capture_size_px=list(Image.open(captures[0]).size),
        verification=dict(cargo_build_locked=True, cargo_check_locked_release=True,
                          cargo_fmt_check=True, git_diff_check=True,
                          cryogenic_tests_passed=16, cryogenic_gpu_test_ignored=1,
                          sewer_tests_passed=7),
        runtime_sha256={review.rel(f): review.sha(f) for f in sorted(runtime.rglob('*')) if f.is_file()},
        capture_sha256={review.rel(f): review.sha(f) for f in captures},
        accepted_sources_unchanged=True,
    ))
    for path in (ROOT / 'artifacts/street-defects-review-01/index.html',
                 ROOT / 'artifacts/sewer-review-04/index.html'):
        refresh_page(path)
    print('Integration recorded: 23 targeted tests, three real entry/return sessions, 18 captures.')


if __name__ == '__main__':
    record()
