"""Package square mine footprints with the preferred larger NPC feedback."""
import json
from pathlib import Path
import package_mine_feedback_readability as pack

OUT = pack.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-66'
pack.OUT = OUT
pack.package()
for name in ('index.html','cote-a-cote.html'):
    path=OUT/name
    page=path.read_text(encoding='utf8').replace('Mines élémentaires · repères agrandis','Mines élémentaires · zones carrées')
    page=page.replace('</h1>','</h1><p>Les zones sont désormais des carrés de <strong>3 × 3 cases</strong> au rayon d’une case, avec les quatre coins inclus. Prévisualisation et explosion utilisent cette même aire ; les murs bloquent les cases concernées.</p>',1)
    page=page.replace('</main>','<p><a href="../skill-animation-review-65/cote-a-cote.html">Voir les trois résidus sur cette même zone carrée</a></p></main>',1)
    path.write_text(page,encoding='utf8')
record=json.loads((OUT/'review.json').read_text(encoding='utf8'))
record.update(review_id='MINE-AREA-SQUARE01-NATIVE',geometry='square3x3, all4corners included, same area for preview and damage; blocked floor excluded',reference='mine-square-footprint-direction-decisions.json',numeric_timelines_identical_to_previous_fixture=True)
(OUT/'review.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
(OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
