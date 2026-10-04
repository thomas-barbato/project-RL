# Rapport — intégration des rencontres souterraines

27 septembre 2026 · génération 112.

État historique du lot 112. La passe suivante est décrite dans
[Équilibrage par couche — génération 113](EQUILIBRAGE_PAR_COUCHE.md) : elle
remplace les niveaux fixes des nouvelles rencontres par des bandes liées à la
profondeur et ajuste leur puissance. Les chiffres ci-dessous restent les
références du lot initial, pas une mesure des statistiques finales en v113.

## Ce qui est terminé

Le lot des **quatre noms souterrains validés** est intégré. Le Ver et l'Anémone
étaient les deux premières étapes ; cette dernière passe ajoute le Hurleur
et la Sentinelle, avec leur génération, comportement, présentation et compatibilité.
Il ne s'agit pas de l'intégration des 36 entrées du catalogue : les 27 autres
noms restent à relire, conformément à la direction retenue.

| Rencontre | Milieu et profondeur | Niveau de référence | Ce que le joueur doit lire |
|---|---|---|---|
| Ver cuirassé (`v`) | Boue/eau peu profonde, maintenance 1–2 et production 1–3 | 5 | Court balayage annoncé, puis récupération immobile. |
| Anémone des caves (`e`) | Eau peu profonde, recherche 2–4 | 8 | Fixée au sol ; annonce les huit cases voisines, frappe et entrave les cibles touchées. |
| Hurleur des failles (`h`) | Sol sec de ruines/gravier, recherche 2–4 | 12 | Prépare une onde autour de lui ; reculer ou passer derrière un mur protège. |
| Sentinelle (`p`) | Secteurs de sécurité 3–6 | 18 | Machine de garde ; charge un cône électrique fixe, tire puis recharge. |

Les villes restent sans ces rencontres hostiles. Chaque espèce dispose d'une
identité stable ; les noms retenus ne sont pas remplacés par des noms provisoires.
La nouvelle **Sentinelle · machine** est distinguée des anciennes sentinelles
génériques. Ces dernières ne deviennent pas des robots.

## Les deux ajouts de cette passe

### Hurleur des failles

- Créature minérale, sans système électronique ni arme portée.
- Onde cinétique de rayon trois, annoncée trois occasions d'action ennemies
  avant sa libération. Elle ne suit pas une cible qui s'éloigne.
- Les murs, portes fermées et angles fermés arrêtent les rayons de l'onde.
  Cacher la cible initiale n'annule pas la partie encore exposée de l'onde.
- Poursuite limitée à quatre cases de son territoire ; immobile pendant la
  préparation puis trois occasions d'action de récupération.
- Animation propre de fronts de pression et poussière, distincte des éclairs.
- Valeurs de base d'essai : 22 PV, armure 2, dégâts cinétiques 5, pénétration 1,
  20 XP avant les règles de progression.

Dans la recherche, une seule rencontre de faune est tirée : famille radiale
de l'Anémone de poids 100, famille minérale du Hurleur de poids 25. Cela donne
20 % de Hurleurs **si les deux familles ont un habitat admissible** ; ce n'est
pas un taux universel. Les habitats, cellules réservées et le budget filtrent
les choix avant tirage. L'arrivée est éloignée d'au moins 20 cases de Manhattan.

### Sentinelle

- Machine électronique dans le tirage des populations, pas dans celui de la faune.
- Garde sa position ; prépare pendant deux occasions d'action un cône de portée
  cinq, puis tire dans la direction annoncée. Trois occasions de récupération.
- Détruire le projecteur empêche le tir ; un déplacement forcé annule la charge.
- Trois composants réels : assemblage locomoteur (12), projecteur électrique
  (10), capteur optique (6). Les chiffres sont leurs durabilités maximales.
- Valeurs de base d'essai : 24 PV, armure 3, dégâts électriques 6, 28 XP avant
  les règles de progression. Les protections physiques n'absorbent pas l'électricité.
- Une tentative solitaire sur un tirage de zéro à un groupe par secteur de
  sécurité ; jamais une armée identique sur chaque carte. Arrivée éloignée
  d'au moins 24 cases de Manhattan.

La carcasse contient les composants réellement survivants, avec leur usure.
La récupération utilise la compétence d'ingénierie existante, produit le
composant récupéré existant et ne peut pas être répétée sur une pièce déjà prise.
Un projecteur détruit n'est pas recréé au moment de la mort. Aucun équipement
humain n'est généré sur cette machine.

## Lisibilité et sauvegardes

Les noms, niveaux, glyphes, légende et CAPTEURS reconnaissent les deux ajouts.
Le panneau de cible affiche le délai avant l'attaque puis la récupération.
Les annonces sont courtes : « Éloignez-vous ou abritez-vous ! » / « Sortez du cône ! ».

La zone dessinée vient du même calcul que l'attaque. Les avertissements ne
révèlent ni ennemis cachés ni cases invisibles ; ils ne dépendent pas de F2.
Une longue action peut consommer plusieurs occasions ennemies : l'avertissement
ne met pas la simulation en pause. Aucun nouvel effet persistant au sol n'est
créé par ces deux attaques.

Les nouvelles parties utilisent la génération 112. Les parties plus anciennes
gardent leurs anciens profils : retirer les nouveaux ennemis restaure également
le budget de recherche historique. Les noms, animations et traductions ne
modifient pas les empreintes de simulation. Le cache moteur reste v6 ; les
variantes sérialisées ont été ajoutées sans déplacer les variantes historiques.

## Vérifications et essai manuel

Validation finale du lot : **723 tests moteur et 355 tests client réussis**,
zéro échec. Un test manuel de reprise d'une suspension externe reste ignoré.
`cargo build --locked`, `cargo check --locked --all-targets`, le contrôle de
formatage et `git diff --check` passent. Aucun commit ni push effectué.

La première passe complète a révélé 14 références de tests historiques qui
incluaient encore la nouvelle population de machines. Leurs catalogues de
référence excluent maintenant explicitement le lot 112 ; les empreintes
historiques enregistrées n'ont pas été remplacées. Les 76 tests ciblés de
versions puis la suite client complète ont été rejoués avec succès.

Les tests couvrent les délais, la sortie de zone depuis le contact, les dégâts
sans double impact, le couvert, les zones protégées, les déplacements forcés,
le projecteur détruit, la récupération et la continuation hors écran.
Les instantanés conservent la préparation et reproduisent les événements suivants.
Les empreintes historiques 100, 101, 102, 103, 108, 109, 110 et 111 restent
comparées à leurs valeurs relevées avant modification. Les suspensions
109 à 112 sont reprises par instantané et par rejeu.

Le tirage recherche/sécurité est contrôlé sur 64 graines par biome : déterminisme,
rareté relative, habitats, composants, provenance et arrivées protégées.
Les villes sont vérifiées avec le catalogue courant. Les tests des deux
premières rencontres restent conservés sur leur catalogue historique.

Captures locales inspectées (contraste automatique également validé) :
[onde du Hurleur](../target/ui-howler-resonance-20260927/cold-start.png),
[cône annoncé](../target/ui-sentinel-20260927/cold-start.png),
[tir électrique](../target/ui-sentinel-impact-20260927/cold-start.png),
[légende](../target/ui-legend-deep-20260927/cold-start.png).
Les deux états de récupération ont aussi été contrôlés. Ces captures sont des
sorties locales de diagnostic dans `target`, pas des assets de campagne.

Diagnostics natifs isolés, sans lire ni modifier la partie du joueur :

- `--ui-cold-fault-howler` et `--ui-cold-sentinel-projector` : préparation.
- Ajouter `-impact` : animation réelle à la libération de l'attaque.
- Ajouter `-recovery` : état après le coup.
- `--ui-cold-legend` : légende mise à jour.

Pour revoir ces rencontres naturellement, créer une **nouvelle partie** puis
explorer les biomes indiqués. Leur présence reste aléatoire : ne pas chercher
les quatre dans la ville ni dans chaque secteur. Les captures des diagnostics
utilisent une petite arène, pas un nouveau lieu de campagne.

## Ce qui n'est pas prétendu terminé

- Les niveaux individuels ne sont pas encore tirés dans les bandes du catalogue
  (5–8, 8–12, 12–16, 18–23) : ce lot utilise les quatre niveaux de référence.
- Les chiffres de combat nécessitent encore une passe d'équilibrage en vraie
  progression, notamment face aux équipements et aux compétences du joueur.
- Les animaux/minéraux ne lâchent toujours pas d'objets arbitraires. Leurs restes
  utiles attendent la définition de ressources ayant un usage réel.
- Pas d'artisanat, de réputation, de nouvelles quêtes canoniques ou de nouveaux
  peuples. Les 27 autres fiches ne sont pas intégrées sans revue des noms.
- Le rendu illustré final et les sons propres aux créatures restent à produire.
  Le lot fournit les glyphes et effets du rendu terminal.

Prochaine étape conseillée : jouer cette tranche, ajuster densité et difficulté,
puis relire le prochain groupe du catalogue avec ses butins avant son intégration.
