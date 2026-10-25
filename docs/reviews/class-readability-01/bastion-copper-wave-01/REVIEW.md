# BASTION-COPPER-W01 — cinq degrés de protection du Bastion

Préparation le 21 octobre 2026 — **cinq tenues et têtes par défaut validées
le 22 octobre 2026**, par « je valide :) ».

`acceptance.json` conserve l'accord `BASTION-COPPER-W01-ACCEPT01`, les cinq
IDs, les empreintes et les copies exactes montrées en 64 × 64. Les sources,
exports et composants figés sont dans le sous-dossier `accepted/` des sources
de cette gamme. Les planches et preuves de revue y sont également conservées.

Après « c'est bien mieux là », le joueur autorise la continuation et demande
des armures plus épaisses et brutales à mesure que les versions montent.
Le choix de direction est conservé dans
`../armor-style-studies-02/direction-continuation-01.json`.

## Les cinq paliers

**01 · Très légère — BA-CU01-P01-r01.** Gilet textile ouvert, bras dégagés,
pantalon souple et bottes simples. Quelques larges renforts supérieurs,
sans cuirasse, épaulières ni jambières métalliques. Tête humaine découverte.

**02 · Légère — BA-STYLE01-r02.** Le Vétéran industriel apprécié : cuirasse
cuivre, une épaule métallique, protège-avant-bras et genouillères. Sa tenue
et sa tête natives restent exactement celles montrées dans la reprise r02.

**03 · Intermédiaire — BA-STYLE02-r02.** Le Briseur cuivre apprécié :
combinaison sombre, protections cuivre sur torse, avant-bras et jambes,
épaules grises et coiffe ouverte. Pixels natifs inchangés.

**04 · Lourde — BA-CU01-P04-r01.** Cuirasse épaissie, large plaque
abdominale, épaulières plus carrées, gros brassards et jambes renforcées.
La coiffe de torse devient un casque fermé à fente sombre.

**05 · Très lourde — BA-CU01-P05-r01.** Blindage massif : gros blocs
d'épaules, torse projeté en avant, avant-bras et poings blindés, hanches et
cuisses protégées, jambières épaisses et bottes larges. La tête demeure
compacte et s'insère dans l'ensemble d'armure.

Les numéros décrivent les paliers de cette gamme. Ils ne renumérotent pas les
études antérieures : l'étude 01 devient ici le palier 02, et l'étude 02 le
palier 03. Les IDs stables sont conservés. Ces paliers ne créent aucun objet,
type d'armure ou bonus de jeu ; leur association au catalogue reste à définir.

## Fabrication et vérification

Trois dessins entiers nouveaux, générés avec le tool intégré `image_gen`,
complètent les deux références retenues. Les prompts exacts sont dans
`prompts.json`. Les sources et composants sont conservés sous
`assets/source-art/class-readability-01/bastion-copper-wave-01/` ; les deux
sources de référence restent dans `armor-style-studies-02/`.

Tous les personnages mesurent 58 pixels opaques de haut dans une toile
transparente de 64 × 64, avec pieds à y=61. La réduction utilise Lanczos en
préservant l'alpha et les proportions. Aucune silhouette n'est étirée.
Les emprises maximales sont 33, 32, 34, 36 et 42 px : celle d'un vêtement
léger dépend aussi de la pose des bras nus. La montée de protection se lit
dans la couverture et l'épaisseur des pièces ; les trois derniers paliers
élargissent également la silhouette totale.

La planche montre toute la gamme à taille native sur fonds sombre, clair et
sur le même sol de rue. Le zoom entier ×3 est séparé. La rue reste un montage
sur une capture existante : l'éclairage moteur n'est pas simulé à nouveau.
Les 649 images et manifestes d'acceptation antérieurs recensés conservent
leur empreinte, ainsi que les deux références reprises dans cette gamme.

Les masques de tête sont des polygones locaux explicites, distincts des
épaules. Les composants tête et corps recomposent exactement le dessin natif.
Cette séparation sert à préparer les éléments : les masques de remplacement
de casque, les bras, les prises et les points d'attache d'armes ne sont pas
encore des rigs de production validés. L'ancre de sol est [32, 61].

## Suite de la revue

Les cinq dessins montrés et leurs têtes par défaut sont désormais validés.
Les anciens accords et dessins du Bastion restent conservés comme historique ;
la gamme cuivre est la référence courante. La tête présentée est celle fournie
par le torse ; un casque équipé la remplacera selon la règle acceptée.
Les casques équipables, les mains avec les armes et l'intégration attendent
toujours leur préparation et revue. Les variantes de couleur sont différées.
Les autres classes gardent leurs anciens candidats de revue et devront
préserver leur identité propre lorsqu'elles seront reprises.

`checks.json` contient les contrôles de fichiers et de conditionnement ;
`ui-checks.json` enregistre l'échelle vérifiée dans le navigateur.
