"""Publish the armed visual review without altering the original attachment."""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
SITE=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')
fragment=(ROOT/'artifacts/player-walk-review-02/fragment.html').read_text('utf-8')
child='''<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src data:; connect-src 'none'; base-uri 'none'; form-action 'none'"><title>WALK02-r02 · armes entières</title><style>body{margin:16px;background:#181818;color:#eee;font:14px system-ui}</style></head><body>'''+fragment+'</body></html>'
page='''<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; frame-src 'self'; base-uri 'none'; form-action 'none'"><title>Project RL · armes et déplacements</title><style>body{margin:0;background:#181818;color:#eee;font:14px system-ui}nav{padding:12px 16px;border-bottom:1px solid #3b4949}a{color:#efb776}iframe{width:100%;height:calc(100vh - 50px);border:0;display:block}</style></head><body><nav><a href="catalogue.html">Catalogue de trois quarts</a> · Bras et armes entiers · revue r02</nav><iframe id="review" title="Bastion avec ses armes · dessins de trois quarts" sandbox="allow-scripts" referrerpolicy="no-referrer"></iframe><script>document.getElementById('review').srcdoc=__CHILD__;</script></body></html>'''
child=child.replace('</head>', '''<style>
:root{color-scheme:dark}*{box-sizing:border-box}body{font:14px/1.4 system-ui,sans-serif}.viz-controls{display:flex;gap:8px;align-items:end;flex-wrap:wrap;margin:10px 0}.form-label{display:grid;gap:5px;font-size:12px;color:#c5c7c2}.form-select,.btn{font:14px system-ui;color:#eee;background:#303633;border:1px solid #62675e;border-radius:5px;padding:8px 10px}.form-select{min-width:140px}.btn{cursor:pointer}.btn:hover{background:#424c44}.btn-primary{color:#191c19;background:#edc297}.viz-row{display:flex;gap:14px;justify-content:space-between;flex-wrap:wrap;margin-top:12px}.text-small{font-size:12px;color:#c1c5bf}#rl-walk-armed .armed-toolbar{padding-right:0}h3{font-size:16px}.form-select:focus-visible,.btn:focus-visible{outline:2px solid #edc297;outline-offset:2px}
</style></head>''')
page=page.replace("frame-src 'self';", "img-src data:; frame-src 'self';")
page=page.replace('__CHILD__',json.dumps(child,ensure_ascii=False).replace('</','<\\/'))
(SITE/'armes.html').write_text(page,'utf-8')
print('Updated armed review with opaque sandbox and restrictive parent/child CSP.')
