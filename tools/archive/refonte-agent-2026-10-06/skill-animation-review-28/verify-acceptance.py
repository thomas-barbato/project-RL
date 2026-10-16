import hashlib
import json
from pathlib import Path
root = Path(__file__).resolve().parent
project = root.parents[2]
receipt_path = root/'visual-acceptance-01.json'
receipt = json.loads(receipt_path.read_text(encoding='utf-8'))
for name,expected in receipt['artifact_hashes'].items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest() == expected, name
for name,expected in receipt['dependency_hashes'].items():
    assert hashlib.sha256((project/name).read_bytes()).hexdigest() == expected, name
review = json.loads((root/'review.json').read_text(encoding='utf-8'))
assert review['status'] == 'visually_approved'
assert all(c['status']=='visually_approved' for c in review['candidates'])
print(json.dumps({'accepted':[c['variant'] for c in review['candidates']],'hashes_verified':len(receipt['artifact_hashes'])+len(receipt['dependency_hashes'])}))
