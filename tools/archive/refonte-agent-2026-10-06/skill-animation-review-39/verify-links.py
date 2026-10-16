import json
import re
from pathlib import Path
root = Path(__file__).resolve().parent
links = re.findall(r'href="([^"]+)"', (root / 'index.html').read_text(encoding='utf-8'))
missing = [name for name in links if not (root / name).is_file()]
assert not missing, missing
print(json.dumps({'links': len(links), 'missing': missing}))
