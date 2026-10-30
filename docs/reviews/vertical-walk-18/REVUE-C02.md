# VERT18-C02 — trois autres tenues de base

Préparé le 29 octobre 2026 à la suite de l'accord sur **VERT18-C01**.
Le joueur valide ensuite ce lot par « je valide :) ». Les fichiers exacts sont
figés dans `acceptance-C02.json` ; la généralisation à tous les équipements est
suivie dans `COUVERTURE-EQUIPEMENT.json`.
Le premier cycle et ses fichiers exacts restent figés dans `acceptance-C01.json`.
Cette nouvelle revue concerne trois déclinaisons, **sans intégration au jeu normal** :

1. Bastion, body 3, tenue légère grise et pantalon olive.
2. Traqueuse, body 5, tenue bleue avec son bras découvert et sa manche asymétrique.
3. Métamorphe, body 7, forme humaine et tenue olive/brune.

Chaque profil possède six poses frontales et six vrais dos, dessinés comme
figures entières. Le conditionnement conserve une échelle uniforme par profil,
le niveau des pieds et le gabarit natif du dessin existant. Les sources initiales
de la Traqueuse et du Métamorphe montraient des plaques de genou à l'arrière :
les versions r02 corrigent ce défaut avant présentation. La manche et l'optique
de la Traqueuse restent du même côté anatomique quand le personnage tourne.

La cadence, l'alternance des appuis et le retour au dessin frontal existant
reprennent le mécanisme de C01. Les animations de présentation sont sans perte,
avec comparaison des pixels après décodage. Le zoom ×3 est signalé ; le plateau
et la séquence du camp restent à 64 px par case avec les matériaux du moteur.

## Reproduction

```powershell
python tools/art/pack_vertical_walk_roster18.py
cargo build --locked
& ./target/debug/project-rl.exe --ui-cold-vertical-roster artifacts/vertical-walk-review-18/current/roster-C02/engine
python tools/art/publish_vertical_walk_roster18.py
```

La revue est dans `artifacts/vertical-walk-review-18/current/roster-C02/apercu.html`.
Elle réutilise le serveur de C01 et conserve un lien vers le premier lot validé.
Les captures restent des sorties de travail, sans dépendance du jeu à ces fichiers.
Les sources sont dans `assets/source-art/vertical-walk-review-18/`, les planches
natives dans `candidate-C02/` et les prompts complets dans `prompts-C02.json`.
Mode de génération : **outil imagegen intégré**, avec alpha transparent.

## Portée de cette revue

Ces trois personnages sont montrés **sans arme, sans casque ajouté et sans
armure portée supplémentaire**. Le programme refuse de préparer leurs profils
avec une arme : les repères de paume provisoires ne sont pas une validation de
prise. Les armes, casques et autres armures feront l'objet des prochains lots.

Les trois autres PNJ du camp et les archétypes urbains restent à dessiner avec
leurs propres vêtements. Le Technomancien retiré de la création conserve sa
compatibilité existante ; aucune identité ni sauvegarde n'est supprimée.
Les monstres et leurs morphologies ne reçoivent pas cette marche humaine.

La validation C01 ne préjuge pas de ces nouveaux dessins. Leur acceptation doit
être consignée séparément après présentation native, selon `AGENTS.md`.
L'intégration de VERT18 et la revue DUAL17-C02 restent deux décisions distinctes.
