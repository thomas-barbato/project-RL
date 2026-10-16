from pathlib import Path
import json

OUT = Path(__file__).resolve().parent
BASE = OUT.parent
receipt = BASE / 'skill-icon-review-50/visual-acceptance-01.json'
approval = json.loads(receipt.read_text('utf-8'))
approval['user_approval'] = "Concernant 84 : fait juste une batterie. \nConcernant 85 : fait une silhouette (dans le meme style que la skill d'invisibilité) mais avec uniquement la silhouette :) \n\nje valide les autres"
receipt.write_text(json.dumps(approval, ensure_ascii=False, indent=2) + '\n', 'utf-8')
page = BASE / 'skill-icon-review-50/index.html'
html = page.read_text('utf-8')
html = html.replace('Ces huit dessins attendent leur validation.', 'Les six icônes de Mines (86 à 91) sont validées et intégrées. Les candidats 84 et 85 sont repris dans la revue suivante. <a href="../skill-icon-review-51/index.html">Deux corrections de Furtivité</a>.')
page.write_text(html, 'utf-8')
print('Review status updated; accepted drawings preserved.')
