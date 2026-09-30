# Première couche : pression au contact et équipement ordinaire

Date : 27 septembre 2026. Génération 120, inchangée.

## Question et protocole

Ce complément aux [essais de parcours](ESSAIS_PREMIERE_COUCHE.md) recherche la cause des dégâts importants et compare des équipements sans affixe. Il ne change ni les monstres, ni les soins, ni le butin, ni les sauvegardes de campagne.

Commande : `cargo test --locked --bin project-rl first_layer_plain_equipment_pressure_sample -- --ignored --nocapture`.

Échantillon fixé : graines 0 et 1 × trajets court et long × trois kits × mêlée, distance et évitement, soit 36 essais, sans remplacement des morts. Le pilote conserve les décisions des 24 essais historiques : soins à 12 PV ou moins, repli à 6 PV ou moins sans soin. Il connaît le terrain et le trajet. Il ne dépense pas de points de compétence, ne se rééquipe pas avec le butin rencontré et ne visite pas les services urbains. Une vie par essai.

- **A :** couteau de camp, fusil de patrouille, aucune armure portée ajoutée.
- **B :** même équipement avec une veste matelassée ordinaire.
- **C :** couteau de sapeur, même fusil et même veste, toujours sans bonus magique.

Tous partent avec 20 PV, douze munitions et deux soins de six PV. La veste est équipée par une vraie commande dans le refuge de diagnostic : cela ajoute un tour de préparation aux kits B et C. Le couteau de sapeur est une autre base d'objet, pas un affixe ajouté au couteau de camp. Ces kits sont des cas contrôlés, pas une distribution représentative des équipements aléatoires obtenus en campagne.

Les événements de dégâts mesurent séparément le plus gros impact réellement subi, le total subi après une commande et le nombre de sources distinctes ayant infligé des dégâts après une commande. Les sources environnementales sans identifiant ne comptent pas comme un ennemi supplémentaire. Le total compte les dégâts effectivement retirés aux PV, pas un potentiel théorique au-delà du coup fatal. Chaque maximum peut provenir d'une action différente.

Les traces détaillent commande, PV précédents, repli, visibilité, profil et état de poursuite du responsable, attaque et événement de dégâts. La valeur brute du slot 0 est un repère diagnostique après la commande ; elle ne remplace pas les événements et ne doit pas être interprétée comme la cause de tous les effets secondaires.

## Résultats

Les 36 essais se terminent sans arrêt technique ni action refusée.

| Kit | Aller-retour complet | Repli avant destination | Mort |
|---|---:|---:|---:|
| A — sans veste | 5 | 1 | 6 |
| B — avec veste | 5 | 3 | 4 |
| C — couteau de sapeur et veste | 5 | 3 | 4 |

Les douze résultats du kit A retrouvent les résultats 120 déjà consignés. Les équipements modestes facilitent certains replis, mais ne suffisent pas à rendre ces combats de mêlée favorables. Les kits B et C ont les mêmes nombres d'issues, pas nécessairement les mêmes coûts ni les mêmes suites de jets. Ce tableau n'est pas un taux de victoire de joueurs.

### Origine des pics

1. **Production : un vrai coup de huit PV, pas un double comptage.** Le profil de poursuivant au contact a trois dégâts de base et une puissance de 7. L'impact physique disponible et son plafond valent 14, contre une référence de 10 : cela ajoute quatre dégâts, pour un total de sept. La variation de niveau applique ensuite jusqu'à 110 %, avec arrondi supérieur ; 103 % suffit déjà à passer de sept à huit. La pénétration annule ici l'armure du corps du joueur sans veste. Il n'y a ni arme enchantée ni critique dans ces traces.
2. **Soigner au contact peut aggraver la situation.** Sur le trajet court de la graine 0, le pilote passe de 20 à 12 PV, utilise un soin de six PV puis subit huit dégâts et termine à 10 PV ; le soin suivant le laisse à 8 PV après la riposte. Le seuil de repli tardif ne s'active donc pas avant le coup fatal. Cela met en cause à la fois le coût du contact et cette politique du pilote, pas seulement la quantité de soins.
3. **Les dégâts d'une action peuvent venir de plusieurs ennemis.** Sur le trajet court de la graine 1, le kit A subit jusqu'à dix PV après une action, avec deux sources distinctes et des impacts individuels plafonnant à cinq. La veste abaisse le maximum observé à huit dans l'approche d'évitement, sans supprimer la mort.
4. **Pas de poursuite infinie démontrée.** Les traces fatales de production montrent un compteur de poursuite qui décroît normalement. Le joueur meurt avant son épuisement. Les marges au placement et les limites de poursuite ne garantissent pas un échange de coups supportable.
5. **L'itinéraire long de la graine 1 reste très calme.** Les approches de mêlée et d'évitement y terminent sans blessure, quel que soit le kit. La différence entre parcours demeure importante ; augmenter uniformément les récompenses ne corrigerait pas cette disparité.

## Relevé complet

| Graine | Trajet | Kit | Approche | Résultat | Tours | PV min. | Coup max. | Dégâts max./action | Sources max./action |
|---|---|---|---|---|---:|---:|---:|---:|---:|
| 0 | Court | A | Mêlée | Mort | 172 | 0 | 8 | 8 | 1 |
| 0 | Court | A | Distance | Aller-retour | 620 | 12 | 8 | 8 | 1 |
| 0 | Court | A | Évitement | Aller-retour | 607 | 8 | 8 | 8 | 1 |
| 0 | Court | B | Mêlée | Mort | 174 | 0 | 7 | 7 | 1 |
| 0 | Court | B | Distance | Aller-retour | 621 | 13 | 7 | 7 | 1 |
| 0 | Court | B | Évitement | Aller-retour | 606 | 13 | 7 | 7 | 1 |
| 0 | Court | C | Mêlée | Mort | 174 | 0 | 7 | 7 | 1 |
| 0 | Court | C | Distance | Aller-retour | 619 | 16 | 4 | 4 | 1 |
| 0 | Court | C | Évitement | Aller-retour | 606 | 13 | 7 | 7 | 1 |
| 0 | Long | A | Mêlée | Mort | 129 | 0 | 8 | 8 | 1 |
| 0 | Long | A | Distance | Mort | 165 | 0 | 8 | 8 | 1 |
| 0 | Long | A | Évitement | Mort | 129 | 0 | 8 | 8 | 1 |
| 0 | Long | B | Mêlée | Repli | 221 | 4 | 7 | 7 | 1 |
| 0 | Long | B | Distance | Repli | 280 | 4 | 7 | 7 | 1 |
| 0 | Long | B | Évitement | Mort | 131 | 0 | 7 | 7 | 1 |
| 0 | Long | C | Mêlée | Repli | 221 | 4 | 7 | 7 | 1 |
| 0 | Long | C | Distance | Repli | 276 | 4 | 7 | 7 | 1 |
| 0 | Long | C | Évitement | Mort | 131 | 0 | 7 | 7 | 1 |
| 1 | Court | A | Mêlée | Repli | 56 | 4 | 4 | 8 | 2 |
| 1 | Court | A | Distance | Mort | 131 | 0 | 5 | 8 | 2 |
| 1 | Court | A | Évitement | Mort | 85 | 0 | 5 | 10 | 2 |
| 1 | Court | B | Mêlée | Repli | 58 | 5 | 3 | 6 | 2 |
| 1 | Court | B | Distance | Mort | 131 | 0 | 4 | 6 | 2 |
| 1 | Court | B | Évitement | Mort | 120 | 0 | 4 | 8 | 2 |
| 1 | Court | C | Mêlée | Repli | 58 | 5 | 3 | 6 | 2 |
| 1 | Court | C | Distance | Mort | 134 | 0 | 4 | 6 | 2 |
| 1 | Court | C | Évitement | Mort | 120 | 0 | 4 | 8 | 2 |
| 1 | Long | A | Mêlée | Aller-retour | 760 | 20 | 0 | 0 | 0 |
| 1 | Long | A | Distance | Aller-retour | 779 | 12 | 4 | 4 | 1 |
| 1 | Long | A | Évitement | Aller-retour | 760 | 20 | 0 | 0 | 0 |
| 1 | Long | B | Mêlée | Aller-retour | 761 | 20 | 0 | 0 | 0 |
| 1 | Long | B | Distance | Aller-retour | 780 | 11 | 3 | 3 | 1 |
| 1 | Long | B | Évitement | Aller-retour | 761 | 20 | 0 | 0 | 0 |
| 1 | Long | C | Mêlée | Aller-retour | 761 | 20 | 0 | 0 | 0 |
| 1 | Long | C | Distance | Aller-retour | 776 | 17 | 3 | 3 | 1 |
| 1 | Long | C | Évitement | Aller-retour | 761 | 20 | 0 | 0 | 0 |

## Repli après la première blessure

Commande : `cargo test --locked --bin project-rl first_layer_plain_kit_wounded_retreat_sample -- --ignored --nocapture`.

Même génération, mêmes graines et itinéraires, kit A et approche de mêlée. Le pilote décide cette fois de rentrer après le premier dégât subi. Pendant le repli, il privilégie le déplacement avant le soin lorsqu'un hostile visible est adjacent ; il ne gagne aucune vitesse ni action supplémentaire. En l'absence de blessure, il peut achever l'aller-retour normal. Il connaît toujours le terrain : ce n'est pas une garantie de sortie pour un joueur découvrant la carte.

Un premier contrôle limité à la première région avait donné quatre retours, mais une seule blessure. Il ne rencontrait pas les poursuivants de production responsables des coups de huit PV. Le protocole a donc été étendu jusqu'à la première blessure sur le parcours ; les graines n'ont pas été remplacées. Ce contrôle reste distinct du repérage sans combat et de la matrice de 36 essais.

| Graine | Trajet | Résultat | Tours | PV min. | PV au retour | Soins |
|---|---|---|---:|---:|---:|---:|
| 0 | Court | Repli | 291 | 4 | 16 | 2 |
| 0 | Long | Repli | 217 | 12 | 18 | 1 |
| 1 | Court | Repli | 53 | 12 | 18 | 1 |
| 1 | Long | Aller-retour sans blessure | 760 | 20 | 20 | 0 |

Les quatre reviennent. Sur le trajet court de la graine 0, le premier déplacement de repli reste dans la portée de l'ennemi : un second coup de huit laisse seulement quatre PV. Le repli fonctionne dans ce cas précis, mais sa marge est très faible. Dans les ateliers de la graine 1, deux poursuivants infligent quatre dégâts chacun sur la même action ; le pilote se dégage puis se soigne et rentre à dix-huit PV.

Cela confirme l'importance de la décision de repli et des cases choisies, sans annuler le problème de brutalité des échanges. Il ne faut pas exiger du joueur une lecture parfaite pour compenser un équipement sans affixe.

## Validation de cette passe

- Matrice de 36 parcours exécutée explicitement : aucun arrêt de pilote ni action refusée.
- Quatre reconnaissances au premier hostile aperçu rejouées : quatre retours sans blessure ; quatre parcours de réaction après blessure terminés comme ci-dessus.
- Nouveau test des trois kits : 20 PV, douze munitions, deux soins, aucun bonus magique, armure réellement équipée et reprise identique après snapshot. Le contrôle compare aussi la reprise au premier contact dans les diagnostics et l'absence de recharge ou soin gratuits aux transitions.
- Cinq tests moteur ciblés sur l'équilibrage par couche réussis ; `cargo check --locked --all-targets`, formatage et contrôle des différences réussis. La suite complète n'a pas été relancée pour ces modifications de tests et de documentation. Aucune validation visuelle requise ou revendiquée : l'interface est inchangée.

## Portée pour l'équilibrage

Aucun affixe obligatoire ne doit être introduit pour résoudre ces difficultés. La prochaine passe de réglage doit cibler le coût réel des ennemis courants au contact, en tenant compte de l'addition puissance + dégâts de base + variation de niveau, puis de leur action à plusieurs. Il faut préserver les rôles et distinguer ces ennemis des grosses attaques annoncées et évitables.

Ces essais ne justifient pas encore un pourcentage universel de réduction : ils ne couvrent que deux graines, des pilotes simples et trois kits proches. Le rééquilibrage devra être comparé sur les mêmes cas, puis sur d'autres graines, d'autres familles d'armes et des tirages conservés même lorsqu'ils sont médiocres. La première couche n'est pas déclarée équilibrée.
