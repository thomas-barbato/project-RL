# Rencontres combinées : première passe

27 septembre 2026 · génération 116 inchangée.

Après les [intégrations du bestiaire](INTEGRATION_BESTIAIRE_LOT_2.md), cette
passe ajoute des scènes natives et cinq tests permanents. Aucun changement
de statistiques, de tables de population, de butin ou de sauvegarde.

## Trois situations

| Milieu | Groupe | Réponse vérifiée |
|---|---|---|
| Surface | Deux Chauves-souris des ruines et un Cracheur des mares | Quitter la case visée ; les piqués et le jet ne suivent pas le joueur. La paire se replie vers son nid commun sans superposition ni nouvelle attaque pendant le repli. |
| Recherche, couche 2 | Écailleux des cavernes et Hurleur des failles | Reculer devant l'Écailleux pendant la préparation du Hurleur, sans entrer dans son onde. |
| Corruption, couche 5 | Gueule du vide et Spectre | Sortir des premières marques, puis reculer encore avant la couronne extérieure. Une seule esquive ne suffit pas nécessairement. |

Les créatures sont produites par les générateurs du catalogue avec leurs
attaques, sens, corps, effectifs et comportements réels, puis équilibrées
pour leur couche. Les tirages sont filtrés et les positions imposées uniquement
dans ces fixtures. Le test ne prétend pas que ces groupes apparaissent à ces
endroits dans toutes les parties. La paire de chauves-souris conserve le nid
commun choisi par le générateur.

## Résultats et garde-fous

Sur **16 graines pour chacun des trois groupes**, attendre la première annonce
puis reculer deux fois vers l'ouest ne coûte aucun PV. Les attaques annoncées
s'exécutent réellement : le test ne passe pas simplement parce que toutes
les attaques sont annulées. Le Hurleur dispose de sa préparation plus longue.

Un essai témoin reste immobile quatre tours, avec le corps de départ et sans
équipement ajouté :

| Groupe | PV perdus sur les 16 graines |
|---|---:|
| Surface | 3–12 |
| Recherche | 11–16 |
| Corruption | 0–14 |

Le zéro en corruption correspond à deux jets d'attaque ratés, pas à une case
sûre. Chaque essai témoin vérifie au moins deux tentatives contre le joueur.
Ces chiffres décrivent des ouvertures de combat précises ; ce ne sont ni des
moyennes de difficulté par couche ni un combat complet avec progression.

Autres contrôles :

- Huit tours d'attente sur une arrivée protégée pour chacun des groupes :
  aucune attaque annoncée contre le joueur ni perte de PV.
- Mur continu entre le joueur et le duo profond : pas d'acquisition à travers
  le mur ni de dégâts à distance.
- Paire de chauves-souris sur 16 graines : positions distinctes, repli sans
  attaque et retour au nid ou à sa case voisine occupable.
- Sauvegarde de diagnostic pendant les préparations simultanées : après
  restauration, six commandes produisent les mêmes événements et états.

## Rejouer

```powershell
cargo test --locked --bin project-rl mixed_encounters -- --nocapture
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-mixed-surface target/ui-mixed-surface
.\target\debug\project-rl.exe --ui-cold-mixed-research target/ui-mixed-research
.\target\debug\project-rl.exe --ui-cold-mixed-corrupted target/ui-mixed-corrupted
```

Choisir des dossiers de capture nouveaux. Ces commandes de diagnostic ne
modifient pas la partie du joueur et n'ajoutent aucun menu de campagne.

Les trois captures `target/ui-mixed-*-20260927/cold-start.png` ont été
inspectées : populations CAPTEURS, marques simultanées, nid et garde latérale.
Le cadre reste celui du diagnostic isolé ; son en-tête de ville et son objectif
de départ ne représentent pas une expédition réelle.

Validation locale : les cinq nouveaux tests `mixed_encounters` et les cinq
tests `bestiary_batch` passent. `cargo check --locked --all-targets`, compilation
native, contrôle release, formatage et vérification du diff réussissent.
Le contrôle release signale trois méthodes de chargement inutilisées, hors
de cette modification. La suite globale n'a pas été relancée pour ce lot de
diagnostics ; les totaux de la précédente intégration ne sont pas un nouveau
résultat. Aucun commit ni push.

## Ce qui reste

Ces scènes ouvertes prouvent des possibilités de réponse, **pas** que toute
combinaison générée possède toujours une sortie sûre. Elles ne mesurent pas
encore les couloirs encombrés, une fuite prolongée, les soins et munitions
consommés, ni les équipements réellement trouvés en plusieurs secteurs.
La prochaine passe utile est cette progression courte avec ressources finies,
avant de retoucher les budgets de population ou de gonfler les PV des ennemis.

Une [sortie courte à vie unique](EXPEDITION_SURVIE.md) couvre maintenant un
aller-retour persistant, les réserves finies, une veste réellement tirée puis
ramassée et équipée, le soin d'une blessure et le refus d'agir après la mort.
Elle reste un parcours ouvert contrôlé, pas une campagne procédurale validée.
