# Complément bleu — revue 08

Le joueur demande aussi des variantes bleues pour le gel. Après présentation
des six planches à taille native, il valide explicitement :
**« Valider et intégrer les 51 bleues »**, le 17 octobre 2026.

Les 51 nouvelles palettes **E** complètent les 39 modèles qui disposent déjà
d’un bleu, bleu-gris ou cyan dans leurs variantes acceptées. Les six planches
distinguent les nouveautés des bleus existants. Les noms et codes restent ceux
de la revue des silhouettes ; aucun nom élémentaire n’est inventé.

Le conditionnement reprend les masques de matériaux approuvés de la revue 07.
Il conserve les points d’appui, l’alpha, la géométrie et les pixels des autres
matériaux. La cage du 101, le crâne tenu par le 138, les armes, les tissus et
les parties osseuses identifiées restent protégés. La variante bleue du 156
existe déjà et garde ses vêtements marron.

Les apparences supplémentaires sont disponibles pour les rôles déjà intégrés
dans les nouvelles parties de **génération 152**. Les affectations des
générations 150 et 151 restent identiques, contrôlées sur 768 apparitions chacune.
Sauvegarde, inspection et corps au sol conservent le modèle et sa couleur.
Les lanceurs de sorts réservés attendent la définition de leurs pouvoirs.

Les ressources et sources sont durables dans `assets/`. Le jeu charge les
cartes RGB générées et les masques de matériaux ; il ne dépend pas des planches
de revue sous `artifacts/`. Le catalogue complet comporte **473 apparences**,
dont deux boss conservés uniquement en revue ; la banque comprend 240 dessins
natifs et 231 palettes de matériaux sur 129 modèles.

Le bleu prépare l’identité visuelle du gel. Aucun dégât de froid, pouvoir ou
bonus chiffré n’est ajouté par cette passe visuelle.

Contrôles reproductibles :

```text
python tools/art/prepare_monster_material_palettes.py --check
python tools/art/prepare_monster_blue_palettes.py --check
python tools/art/validate_campaign_bestiary.py --after-cleanup
project-rl.exe --ui-cold-monster-blue-palettes artifacts/monster-blue-palettes-08/runtime
python tools/art/validate_monster_material_palettes.py --blue --runtime artifacts/monster-blue-palettes-08/runtime
cargo test --locked --lib monster_
cargo test --locked --bin project-rl game_art::campaign
```

L’accord et les cartes sont dans `review.json`. `asset-validation.json` conserve
les empreintes et la comparaison des 231 palettes avec le rendu GPU réel.
`validation.json` consigne les tests et leurs limites. Les dessins sources et
les planches natives acceptées restent disponibles.
