# Technomancien — reprise des formes, témoin 01

21 octobre 2026 — revue `TECHNOMANCIEN-W02-ID02`, candidat **01 / TE-W02-ID02-r01**.
**Témoin validé en 64 × 64** le 21 octobre par « d'accord ca me va ».
L’accord `TECHNOMANCIEN-W02-ID02-ACCEPT01` conserve la composition exacte et
ses calques dans `acceptance.json`, avec les exports figés sous
`assets/source-art/class-readability-01/technomancien-heads-01/accepted/`.
Cette validation concerne ce témoin précis ; aucune intégration au jeu.

## Retour du joueur et portée

Le joueur interrompt la première progression de cinq coiffes : « non mais c'est
les memes formes que ceux du bastion, tu dois faire different . ». Cette direction
est abandonnée. Les trois sources produites avant l’arrêt et les cinq prompts
prévus sont conservés dans `prompts.json` et dans le sous-dossier de sources
`rejected/`. Les deux autres coiffes n’ont pas été conditionnées ni retenues.

La reprise porte d’abord sur **un seul témoin de silhouette**, avant une nouvelle
gamme. Elle conserve le manteau technique turquoise et les prothèses cuivrées
comme direction de matière, mais change la forme de la tête et du col : capuche
souple profonde, sommet décalé, ouverture du visage et col drapé asymétrique.
Le masque inférieur fait partie de cette coiffe ; aucun objet de casque n’est créé.

Le témoin est composé sur le corps intermédiaire TE-W02-ID01-r01. Ce corps était
une proposition appréciée dans son orientation, sans validation présumée des cinq
anciennes tenues. Les autres classes et les dessins acceptés du Bastion ne sont
pas modifiés. Aucun nouvel effet mécanique ou nom d’objet n’est introduit.

## Fabrication et couches

L’outil intégré **image_gen** produit un module isolé de capuche et de col sur
fond réellement transparent. La face de base n’est pas redessinée ; le module
la couvre selon son alpha explicite. Son placement a été ajusté à taille native
pour laisser apparaître le visage cybernétique.

Le conditionnement se limite au cadrage, à la réduction isotrope Lanczos, à
l’assemblage et aux planches. Le module est limité à Y=[0,27) et les pixels RGBA
du corps sous Y=27 restent strictement identiques au témoin précédent. Les sources
et références antérieures sont vérifiées par empreintes avant et après génération
des exports de revue. La tête et le corps avec col ont des calques séparés à
Y=14 ; leur réassemblage restitue exactement la composition proposée.

La tête et le col sont fournis par l’armure de torse. Un casque équipé remplace
uniquement le domaine de tête, en conservant le col du corps. Les futurs casques
équipables et leurs raccords restent une revue distincte, comme les prises d’armes.

## Présentation et suite

La section `#technomancien-heads-01` de la galerie montre le Bastion accepté,
l’ancien témoin et la nouvelle proposition 01 sur trois fonds, à taille native.
Un zoom entier ×3, un détail de tête et de col ×6 et un montage sur un sol de rue
complètent la revue. Le montage ne valide pas le rendu dans le moteur.

Après validation de ce témoin, le joueur demande de préparer toutes les armures
des cinq classes, puis de présenter l’ensemble à taille réelle pour choisir
les modèles à garder ou à modifier. Cette nouvelle cadence remplace la revue
classe par classe ; elle ne valide pas les déclinaisons à produire.
Voir `../class-armors-all-01/`. Aucune variante de couleur n’est produite. L’intégration
reste différée jusqu’aux armures, têtes par défaut, casques et positions de mains
préparés et revus.

## Fichiers

- Sources : `assets/source-art/class-readability-01/technomancien-heads-01/`.
- Prompt exact, références et cadrage : `identity-prompt.json`.
- Dessin courant, calques, empreintes et contrôles : `identity-candidate.json`.
- Revue navigateur : `ui-checks.json`.
- Aperçu : `artifacts/class-armor-review-01/technomancien-heads-01/`.
- Script : `tools/art/prepare_technomancien_cowl_01.py`.

Les sources retenues sont conservées dans le projet. Le jeu ne dépend d’aucune
sortie de cette galerie et aucun fichier Rust n’est modifié.
