from pathlib import Path
OUT = Path(__file__).resolve().parent
REVIEW = OUT.parent / 'skill-icon-review-41'
page = REVIEW / 'index.html'
text = page.read_text(encoding='utf-8')
assert 'PROPOSITION 69' in text
text = text.replace('PROPOSITION 69','ICÔNE 69 VALIDÉE')
text = text.replace('Une décharge d’énergie plus compacte, sans rouge, proposée à la revue.','La décharge verte sans rouge est validée et intégrée.')
text = text.replace('Dessins proposés à la revue.','Dessin validé et intégré. <a href="../skill-window-native-15/index.html">Voir la fenêtre du jeu</a>.')
text = text.replace('../skill-icon-review-39/index.html','../skill-icon-review-42/index.html')
page.write_text(text,encoding='utf-8')
page = REVIEW / 'README.md'
page.write_text(page.read_text(encoding='utf-8')+'\n## Validation du 11 octobre 2026\n\nLe joueur valide « ok je valide ceci » avec la pièce jointe verte, dont les pixels correspondent exactement au master M02-C03. Cette icône est intégrée sans retouche. Accord dans visual-acceptance-01.json ; tests et capture réelle dans skill-window-native-15. Les autres propositions restent à valider.\n',encoding='utf-8')
print('IC41 accepted; artwork unchanged')
