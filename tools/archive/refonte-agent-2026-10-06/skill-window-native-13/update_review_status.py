from pathlib import Path
OUT = Path(__file__).resolve().parent
REVIEW = OUT.parent / 'skill-icon-review-37'
page = REVIEW / 'index.html'
text = page.read_text(encoding='utf-8')
assert 'Nouvelle icône à valider.' in text
text = text.replace('Nouvelle icône à valider.','Icône validée et intégrée. <a href="../skill-window-native-13/index.html">Voir la fenêtre du jeu</a>.')
text = text.replace('M03-C05 · proposition','M03-C05 · validée')
text = text.replace('La banque conserve le dernier dessin validé.','Ce symbole est maintenant utilisé en jeu.')
page.write_text(text,encoding='utf-8')
page = REVIEW / 'README.md'
page.write_text(page.read_text(encoding='utf-8')+'\n## Validation du 11 octobre 2026\n\nLe joueur valide « oui ca me va :) ». M03-C05 est intégrée sans retouche. Accord dans visual-acceptance-01.json ; vérifications et capture réelle dans skill-window-native-13. Les paragraphes précédents décrivent la préparation de la proposition.\n',encoding='utf-8')
print('IC37 acceptance status updated; drawings unchanged')
