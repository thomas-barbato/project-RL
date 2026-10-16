import json
import re
from pathlib import Path
root=Path(__file__).resolve().parent
links=re.findall(r'href="([^"]+)"',(root/'index.html').read_text(encoding='utf-8'))
missing=[p for p in links if not (root/p).exists()]
assert not missing, missing
print(json.dumps({'links':len(links),'missing':missing}))
