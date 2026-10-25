# Bastion — têtes par défaut des cinq armures

Revue `BASTION-W01-HEAD02`, **validée le 21 octobre 2026** par la réponse
« ok j evalide ». L’accord `BASTION-W01-HEAD02-ACCEPT01` conserve les cinq têtes
et leurs cinq compositions montrées en 64 × 64 dans `acceptance.json`.
Les exports exacts et les calques sont figés dans
`assets/source-art/class-readability-01/bastion-heads-02/accepted/`.
L’accord antérieur sur les corps du Bastion reste conservé. Aucune intégration
au jeu et aucune nouvelle variante colorée ne sont effectuées.

## Propositions

- **01 — BA-W01-H01-r01** : couvre-chef rembourré très léger, visage découvert.
- **02 — BA-W01-H02-r01** : casque léger ouvert, petites protections latérales.
- **03 — BA-W01-H03-r01** : casque de combat ouvert, couverture latérale renforcée.
- **04 — BA-W01-H04-r01** : casque fermé, large bande de visière sombre.
- **05 — BA-W01-H05-r01** : casque fermé renforcé, fente de visière orange.

Chaque protection appartient à l’apparence par défaut de son armure de torse.
Il ne s’agit pas de cinq nouveaux objets de casque à ajouter au catalogue.
Les corps associés sont P01-r01, P02-r01, P03-r01, P04-r01 et P05-r03.

## Composition et conservation

Les éléments sont générés séparément par `image_gen`, puis cadrés et réduits
isotropiquement dans une case de 64 × 64. Les ouvertures des modèles 01 à 03
laissent apparaître la tête acceptée sous le calque ; aucun visage n’est redessiné.
Les modèles fermés masquent le visage. La tête d’origine reste disponible sous
la protection et les sources ne sont pas retouchées par le script.

Le calque de protection possède exclusivement les lignes Y=[0,12). Le raccord
avec le corps est à Y=12. Le cadrage des modèles fermés prolonge la protection
jusqu’au raccord, où la limite du calque conserve la propriété des pixels du corps.
Tous les pixels RGBA du corps Y=[12,64) restent strictement identiques aux exports
acceptés. Les vingt compositions et leurs sources déjà acceptées, ainsi que leur
fichier d’acceptation, sont vérifiées par empreintes avant et après la préparation.

Le script produit aussi un exemple de priorité : la protection 05 remplace celle
du torse 01, sans cumul ; la retirer rétablit exactement la composition 01.
Cet exemple illustre uniquement la règle `CLASS-READ01-HEAD02` et ne crée aucun
objet ni règle de jeu supplémentaire. Les casques équipables restent à dessiner
et à valider séparément.

## Revue native

La galerie courante ouvre la section `#bastion-heads-02`. La planche principale
présente les cinq compositions en 64 × 64 sur trois fonds. Une comparaison
avant/après, un zoom entier ×3 et des détails ×6 complètent la revue ; la taille
native reste la référence. Le montage sur le sol de la rue ne constitue pas une
validation de l’éclairage ou de la composition dans le moteur.

La validation visuelle explicite s’ajoute aux contrôles techniques ; elle ne
valide pas par extension les autres classes, les casques équipables, les prises
d’armes ou le rendu moteur.
L’intégration attend les armures, les têtes par défaut, les casques et les bonnes
positions des mains avec les armes, selon la demande du joueur.

## Sources et sorties

- Sources conservées : `assets/source-art/class-readability-01/bastion-heads-02/`.
- Prompts exacts et cadrages : `prompts.json` dans ce dossier de revue.
- Identités, sources, empreintes et contrôles : `candidates.json`.
- Contrôle du navigateur : `ui-checks.json`.
- Aperçu courant : `artifacts/class-armor-review-01/bastion-heads-02/`.
- Préparation : `tools/art/prepare_bastion_heads_02.py`.
- Enregistrement de l’accord : `tools/art/record_bastion_heads_acceptance_02.py`.

Aucun code Rust, catalogue de butin, statistique ou format de sauvegarde n’est
modifié par cette préparation. Les sorties de revue ne sont pas utilisées par le jeu.
