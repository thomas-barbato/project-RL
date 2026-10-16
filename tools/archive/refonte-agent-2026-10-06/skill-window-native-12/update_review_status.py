from pathlib import Path

OUT = Path(__file__).resolve().parent
REVIEW = OUT.parent / 'skill-icon-review-33'
page = REVIEW / 'index.html'
text = page.read_text(encoding='utf-8')
assert 'Proposition 60 en attente de validation visuelle.' in text
text = text.replace('Nouvelle proposition à examiner à taille réelle.', 'Dessin validé et intégré sans retouche.')
text = text.replace('Proposition 60 en attente de validation visuelle.', 'Dessin 60 validé et intégré. <a href="../skill-window-native-12/index.html?skill=A02">Voir la fenêtre du jeu</a>.')
page.write_text(text,encoding='utf-8')
page = REVIEW / 'README.md'
page.write_text(page.read_text(encoding='utf-8')+'\n## Validation du 11 octobre 2026\n\nLe joueur répond « parfait on continue :) » à cette proposition. A02-C02 est validée et intégrée ; accord figé dans visual-acceptance-01.json, contrôles dans skill-window-native-12. Les paragraphes précédents conservent le contexte de préparation.\n',encoding='utf-8')
print('IC33 current acceptance status updated; images unchanged')
