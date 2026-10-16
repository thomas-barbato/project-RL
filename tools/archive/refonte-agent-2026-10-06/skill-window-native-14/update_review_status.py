from pathlib import Path
OUT = Path(__file__).resolve().parent
REVIEW = OUT.parent / 'skill-icon-review-38'
page = REVIEW / 'index.html'
text = page.read_text(encoding='utf-8')
assert 'les quatre icônes à valider' in text
text = text.replace('PROPOSITIONS 65–68','VALIDÉES 65–68')
text = text.replace('les quatre icônes à valider','les quatre icônes validées')
text = text.replace('Les quatre autres icônes restent proposées à la revue.','Les quatre autres icônes sont maintenant validées et intégrées.')
text = text.replace('Puissance métamorphe intégrée en jeu','Les transformations intégrées en jeu').replace('../skill-window-native-13/index.html','../skill-window-native-14/index.html')
page.write_text(text,encoding='utf-8')
page = REVIEW / 'README.md'
page.write_text(page.read_text(encoding='utf-8')+'\n## Validation du 11 octobre 2026\n\nLe joueur valide les quatre icônes 65–68 par « ok on peut continuer ». Elles sont intégrées sans retouche. Accord dans visual-acceptance-01.json ; contrôles de préservation et capture réelle dans skill-window-native-14. Les paragraphes précédents décrivent la préparation de la proposition.\n',encoding='utf-8')
print('IC38 acceptance status updated; drawings unchanged')
