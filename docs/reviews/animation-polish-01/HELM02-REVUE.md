# HELM02 — inventaire et cohérence des casques

Le 31 octobre 2026, le joueur confirme trois étapes : voir tous les visuels de
casques, étudier une réduction du nombre de modèles visuels, puis adapter le
casque porté au style de la classe. Il ajoute que ce casque doit rester logique
de face, de dos et de côté. Les citations et la portée sont dans
`HELM02-REGLE.json`. Les proportions 04 / C06-HELM01-r01 restent acceptées.
Il précise ensuite que la taille doit tenir compte de la morphologie et de
l'armure portée, pour éviter un casque ridiculement petit. La silhouette
complète devient donc la référence de calibrage des prochains candidats.

## Inventaire actuel

Les six objets équipables du slot tête sont reliés aux modèles portés suivants :

| Repère | Objet | Modèle |
| --- | --- | --- |
| 01 | Calotte Cognefer | 115 |
| 02 | Capuche de mue | 116 |
| 03 | Casque Cerbère | 117 |
| 04 | Heaume Bastion | 118 |
| 05 | Masque Chrysalide | 119 |
| 06 | Visière Fantôme | 120 |

Ces repères sont ceux des six modèles, distincts du numéro de proposition 04
qui valide leurs proportions. Les affectations viennent des objets de contenu,
de `src/game_art_catalog.rs` et du catalogue d'icônes natif. Les têtes fournies
par les tenues de torse sont des références de style séparées : elles ne créent
pas d'objets équipables supplémentaires.

La page courante reçoit la section `#inventaire-casques`. Une planche réunit les
six icônes et les quatre orientations portées. Elle permet de choisir Bastion,
Traqueuse, Arcaniste ou Métamorphe, tenue de base ou lourde, et chacune des six
phases de marche. Les figures sont copiées à taille native depuis les captures
du moteur de HELM01, ligne des proportions 04 acceptées. Un zoom entier ×2 et
une planche des coques face/dos agrandies ×4 permettent d'examiner les détails.
Les couleurs sont celles d'origine. Le Technomancien historique, toujours
présent dans les ressources de classe, ne dispose pas de captures verticales
dans ce pilote à quatre classes et n'y est pas présenté.

## Construction actuelle des orientations

La face et le dos utilisent deux banques de coques distinctes sous
`assets/animations/vertical-18/heads/`. Les vues latérales reprennent la banque
frontale de `assets/sprites/class-armors-01/helmets.png`, projetée par
`turned_helmet` puis calée sur la tête peinte en marche. Ce ne sont pas deux
dessins latéraux autonomes. Les six coques sont communes aux classes ; les
proportions 04 changent leur gabarit, sans créer une identité visuelle propre
à chaque classe.

L'examen doit suivre la forme de la coque, la visière, les ouvertures, la nuque,
les matières et les détails asymétriques sur toutes les orientations. Il inclut
les occultations et les transitions de marche/repos. Une projection cohérente
ou un point d'attache correct ne prouve pas à lui seul la qualité du dessin.

## Choix suivants

Le nombre de modèles à conserver n'est pas fixé. Aucun regroupement, nouvelle
silhouette ou variante de classe n'est encore accepté. La réduction concerne
le nombre de modèles visuels ; cet inventaire ne supprime ni objets, ni noms,
ni règles d'équipement. Les futurs dessins seront présentés ensemble sur leurs
porteurs, dans toutes les orientations et dans le décor accepté, avant leur
intégration selon `AGENTS.md`.

La demande de calibrage selon l'armure fait évoluer le facteur constant entre
les tenues de HELM01. Les proportions 04 restent la base acceptée et les
planches de cet inventaire en montrent exactement le rendu. Les prochains
candidats pourront adapter le volume au couple porteur/armure ; leur taille
devra rester constante pendant la marche et entre les angles pour ce même
équipement. Les valeurs et les nouveaux dessins restent à montrer. Une simple
largeur d'épaulières ne suffit pas à déterminer automatiquement la taille.

## Reproduction

Après les captures HELM01 existantes :

```powershell
python tools/art/build_animation_polish_review.py
```

Le rapport `HELM02-INVENTAIRE.json` conserve les affectations, la couverture et
les empreintes des entrées. Les planches reproductibles vont dans
`artifacts/animation-polish-01/helmet-inventory/current/`. Aucun dessin source
ni chemin de rendu normal n'est modifié par cet inventaire.
