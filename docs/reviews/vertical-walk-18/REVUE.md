# VERT18-C01 — marche verticale du joueur et des PNJ

Premier lot du 29 octobre 2026, **validé visuellement** par « ca me plai bien :) ».
Les sources exactes et la portée de l'accord sont dans `acceptance-C01.json`. Il est
désactivée dans le jeu normal, en Debug comme en Release. La demande porte sur
une marche de dos vers le haut, de face vers le bas, puis le retour au dessin
de face habituel à chaque arrêt, pour le joueur et les PNJ.

## Premier lot présenté

- Arcaniste sans armure portée : six phases de face et six phases de dos.
- Même Arcaniste avec le pistolet 104 et le couteau 70, composés séparément.
- PNJ du camp NPC-01 : six phases de face et six phases de dos, avec son
  brassard, sa sangle, son casque et son holster conservés du même côté anatomique.

Chaque phase vient d'une figure entière dessinée. Le conditionnement recadre
les figures, applique une échelle uniforme par profil et conserve le niveau des
pieds du dessin existant. Il ne fabrique aucune pose par greffe, cisaillement ou
déplacement de bandes. La case reste de 64 px ; les toiles de 128 px fournissent
les marges transparentes. Le vrai dos n'est pas un miroir du dessin frontal.

Le mécanisme commun de déplacement distingue le signe de Y pendant la marche,
conserve trois phases par pas et alterne les appuis sur deux pas. La durée du
prototype est de 400 ms par case. L'arrivée ou l'annulation libère la pose de
marche : le dessin frontal existant revient sans modifier les règles de jeu.
Le mouvement réduit conserve le comportement actuel.

## Revue et fichiers

Lancer `cargo build --locked`, puis :

```powershell
& ./target/debug/project-rl.exe --ui-cold-vertical-walk artifacts/vertical-walk-review-18/current/engine
python tools/art/publish_vertical_walk_review18.py
```

La page `artifacts/vertical-walk-review-18/current/apercu.html` présente des
captures du moteur en natif64, un zoom entier ×3 et une scène dans le camp.
Les pauses de 650 ms dans les animations servent à examiner le retour de face ; elles
ne changent pas la durée du déplacement du jeu.

Les animations WebP sont sans perte : leurs pixels décodés sont comparés à
chaque image du moteur. Un premier export GIF a été écarté parce que sa palette
limitée dénaturait les quelques pixels du manteau au milieu du décor détaillé.

Les sources et le manifeste sont dans
`assets/source-art/vertical-walk-review-18/`. Les générations utilisent l'outil
**imagegen intégré**, avec transparence, puis un conditionnement local. Les
prompts complets, références, variantes et empreintes sont dans `prompts.json`.
Les dessins sélectionnés sont Arcaniste r01 et NPC-01 r02. Les captures restent
des sorties reproductibles sous `artifacts/` ; le prototype n'en dépend pas.

## Limites avant généralisation

Ce lot ne couvre pas encore les autres classes, armures, casques ou PNJ. Les
prises de cette revue concernent les armes à une main montrées. Les prises à
deux mains et les autres équipements nécessitent leurs propres dessins et
contrôles de profondeur, de côté anatomique et de contact. Aucun corps de
monstre ne reçoit automatiquement cette marche humanoïde.

Après accord sur ce premier cycle, décliner les tenues de classe et les PNJ
par lots, avec contrôle natif de l'équipement, des lumières, des appuis et du
retour de face. Conserver des profils séparés pour les identités et vêtements
des PNJ ; les armures de classe restent réservées au joueur.

L'accord reçu concerne **VERT18-C01**. Il ne valide pas rétroactivement
DUAL17-C02, qui reste une proposition séparée. Le rendu latéral accepté DUAL15
et le rendu normal des profils non couverts restent en place.

## Préparation et vérifications

Les sources sont lues/décodées et les compositions préparées avant l'affichage
de la revue ; les textures sont créées sur le thread graphique. Les caches de
sources et de figures préparées sont bornés à 4 et 8 Mio. Le dessin ne lit et
ne décode aucun fichier. Le décor et ses lumières sont préparés séparément avant
la séquence du camp, puis libérés avec cette capture isolée.

Le test ciblé vérifie Nord/Sud, alternance des appuis, arrivée, annulation,
changement vers un pas horizontal, nettoyage et mouvement réduit à 120, 200 et
400 ms, pour deux entités. Les tests existants de retour frontal et de marche
verticale conservent leur résultat. Les contrôles du moteur sont consignés
dans `ANALYSE.json`, avec les limites du lot présenté.
