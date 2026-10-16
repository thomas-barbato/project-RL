import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
OUT=BASE/'skill-animation-review-73'
data=json.loads((OUT/'proposal.json').read_text(encoding='utf8'))
data.update(numeric_profile_status='approved',catalogue_changed=True,shared_rule_integrated=True,
    approval='visual-and-numeric-acceptance-01.json',
    catalogue_bindings=['SG00','SM00','A02','FE04','FE05','FR02','FR05','FR06','HY07','TR64:pulse'])
(OUT/'accepted-rule.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
for name in ('index.html','cote-a-cote.html'):
    path=OUT/name
    path.write_text(path.read_text(encoding='utf8').replace('href="proposal.json"','href="accepted-rule.json"'),encoding='utf8')
CURRENT=BASE/'skill-area-damage-integration-01'
CURRENT.mkdir(exist_ok=True)
source=BASE/'skill-spatial-review-01/progressions.json'
try: source_text=source.read_text(encoding='utf8')
except UnicodeDecodeError:
    source_text=source.read_text(encoding='cp1252')
    json.loads(source_text)
    source.write_text(source_text,encoding='utf8')
progressions=json.loads(source_text)
(CURRENT/'progressions.json').write_text(json.dumps(progressions,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('Relevés de validation actualisés.')
