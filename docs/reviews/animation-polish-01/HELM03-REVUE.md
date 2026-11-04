# HELM03 — nouvelles icônes de casque, premier lot

Suite du même 31 octobre : HELM04 complète la gamme à cinq modèles selon les
degrés de poids visuel d'armure. Le casque devra garder un dessin commun, fidèle
à son icône, sur toutes les classes ; son ajustement suit la carrure et l'armure.
Cette décision remplace les adaptations de dessin par classe envisagées plus
bas. Les trois recherches HELM03 et leurs identifiants restent conservés.

Le 31 octobre 2026, le joueur juge la capuche peu réussie et les cinq autres
casques trop semblables à distance de jeu. Il demande de les redessiner avec
une identité moins générique, puis précise la priorité : les icônes d'abord,
les visuels portés adaptés à la classe ensuite. Il suggère un clignotement
léger des visières colorées. Les citations exactes sont dans
`HELM03-PROPOSITION.json`.

## Trois directions à examiner

- **H03-I01 / 01 — Coque industrielle** : couronne basse et large, front en
  gradins, mâchoire anguleuse ; fer, cuivre mat et visière ambre.
- **H03-I02 / 02 — Optique asymétrique** : coiffe plus effilée, optique cyan
  décentrée, petit capteur latéral et protection inférieure en pointe.
- **H03-I03 / 03 — Coque organique** : silhouette plus verticale, grandes
  plaques segmentées, crête courte et masque inférieur fendu ; os/ocre et olive.

Ce lot propose trois familles visuelles distinctes plutôt qu'une série de
niveaux sur une coque commune. Ce nombre et ces dessins restent des candidats.
Les intitulés servent à la revue : ce ne sont pas de nouveaux noms d'objets.
Aucune affectation des six objets actuels à ces trois pistes n'est décidée.

Les icônes sont présentées à 64 px, sur fonds clair et sombre, avec un zoom
entier pour les détails. Leurs sources générées et leur conditionnement natif
restent dans `assets/source-art/animation-polish-01/helmet-icons-r01/`.
L'aperçu courant réunit ce lot et les six icônes existantes.

## H03-P01 — essai de visière animée

La revue compare la même icône fixe avec une légère modulation de luminosité
sur son matériau de visière uniquement. La coque, la position, les dimensions
et l'alpha restent constants. Un creux lissé de 400 ms revient toutes les
3,6 secondes, jusqu'à 62 % de luminosité. Les masques inspectés contiennent
respectivement 16, 39 et 8 pixels ; voir `HELM03-PULSE.json`.
L'accord sur une icône ne vaudra pas accord
sur l'animation. L'essai n'ajoute aucune règle de jeu.

L'aperçu propose un fond clair ou sombre, un arrêt de l'animation et un bouton
pour rejouer immédiatement le clignotement. Les trois icônes sont visibles
ensemble à taille native, puis chacune est comparée fixe/animée. Les 17 niveaux
de luminosité sont préparés une fois après chargement des six petites images
source/masque ; la lecture des fichiers et la construction des pixels restent
hors de la boucle d'affichage.

## Suite portée conservée

Après le choix des icônes, le casque équipé devra s'accorder à l'identité de
chaque classe et à la silhouette complète avec l'armure portée. Les volumes,
ouvertures, matières, visières et détails asymétriques devront rester logiques
de face, de dos et des deux côtés, pendant la marche et au repos. La taille
restera stable pour un même équipement. Les proportions 04 de HELM01 restent
la référence antérieure acceptée ; les futurs ajustements selon l'armure et
les nouveaux dessins nécessitent leur propre revue.
Chaque famille retenue devra disposer de ses adaptations pour toutes les
classes concernées ; aucune de ces icônes n'est réservée à une classe.

La production des visuels portés n'est pas engagée dans ce lot d'icônes. Aucun
de ces nouveaux dessins ou de ces effets n'est intégré au rendu normal avant
l'accord sur les candidats concrets, conformément à `AGENTS.md`.

## Sources et reproduction

Les trois appels utilisent le mode intégré **imagegen**, avec fond transparent.
Les prompts complets et les chemins de chaque source brute et native sont dans
[`HELM03-PROMPTS.json`](HELM03-PROMPTS.json). Les empreintes et le conditionnement
(recadrage alpha, réduction proportionnelle sur une case 64 px, sans repeinture)
sont dans `HELM03-SOURCES.json`. Les fichiers sont :

- `assets/source-art/animation-polish-01/helmet-icons-r01/H03-I01-generated.png`
  et `H03-I01-native64.png` ;
- `assets/source-art/animation-polish-01/helmet-icons-r01/H03-I02-generated.png`
  et `H03-I02-native64.png` ;
- `assets/source-art/animation-polish-01/helmet-icons-r01/H03-I03-generated.png`
  et `H03-I03-native64.png`.

```powershell
python tools/art/prepare_helmet_icons_review.py
python tools/art/build_animation_polish_review.py
```

La page reste `artifacts/animation-polish-01/apercu.html#nouvelles-icones`.
Les PNG et WebP de présentation sont reproductibles sous
`artifacts/animation-polish-01/helmet-icons-r01/current/` ; le jeu n'en dépend pas.
