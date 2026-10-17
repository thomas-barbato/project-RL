# Palettes des démons — revue 07

Le 17 octobre 2026, le joueur valide explicitement le lot présenté sur six
planches natives : « Valider et intégrer le lot ».

Les **176 nouvelles variantes rouge-orangé et vertes sur 88 autres modèles**
rejoignent les quatre palettes déjà acceptées des modèles 144 et 150. Les
dessins A/B restent identiques. Le catalogue réunit désormais 422 apparences,
dont deux boss uniquement proposés en revue ; la banque du jeu comporte
240 dessins natifs et 180 palettes de matériaux sur 129 modèles.

Les masques par modèle protègent les vêtements, les os, les armes et les parties
métalliques. Les régions tracées de la cage du 101 et du crâne tenu par le 138
séparent les matériaux qui partagent certaines couleurs avec la chair.
Les vêtements marron du 156 restent distincts de sa peau. Les contours, l’alpha,
les tailles et les points d’appui restent identiques aux sprites approuvés.

La génération 151 active ces palettes sur les nouvelles apparitions des rôles
déjà jouables. La génération 150 garde ses choix d’origine, vérifiés sur
768 affectations figées avant ce changement. L’identité visuelle enregistrée
reste stable après sauvegarde ; le plateau, l’inspection et le corps au sol
utilisent la même texture mise en cache. Les dessins de lanceurs de sorts
réservés restent en attente de leurs pouvoirs.

Cet accord porte sur les couleurs. Il n’ajoute ni dégâts élémentaires,
ni résistance, ni compétence, ni nom. Les futurs pouvoirs et leurs chiffres
restent à définir et à valider séparément.

Les sources durables, cartes RGB explicites, masques et exports sont dans
`assets/`. Le jeu ne lit aucun fichier sous `artifacts/`. La galerie courante
est `artifacts/monster-material-palettes-07/index.html` ; le catalogue complet
est `artifacts/npc-monsters-review-01/catalogue/index.html`.

Contrôles reproductibles :

```text
python tools/art/prepare_monster_material_palettes.py --check
python tools/art/build_campaign_bestiary.py --check
python tools/art/validate_campaign_bestiary.py --after-cleanup
project-rl.exe --ui-cold-monster-palettes artifacts/monster-material-palettes-07/runtime
python tools/art/validate_monster_material_palettes.py
cargo test --locked --lib
cargo test --locked --bin project-rl game_art::campaign
```

`asset-validation.json` conserve les empreintes et les comparaisons pixel par
pixel avec les 180 exports du rendu réel. `validation.json` consigne les tests,
les compilations et leurs limites. Les anciennes sorties d’audit devenues
inutiles sont retirées après ces contrôles ; les sources et les six planches
acceptées restent disponibles.
