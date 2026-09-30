# Première faune souterraine — Ver cuirassé

27 septembre 2026 — génération 110.

## Rencontre

Le **Ver cuirassé** rejoint les régions générées de maintenance (profondeurs
1–2) et de production (1–3). C'est un animal à anneaux rigides, pas un robot.
Son nom a été [validé avec la première liste](NOMS_DU_BESTIAIRE.md).

- Présence dans les plaques de boue et d'eau peu profonde, hors des villes et
  des zones protégées. Les passages, acteurs, installations et butins existants
  restent réservés par la génération.
- Une à deux tentatives de rencontre solitaire, pour un budget maximal de deux
  individus par région. L'absence d'habitat disponible peut empêcher le tirage.
  Un grand bestiaire n'impose pas toutes ses espèces sur chaque carte.
- Apparition à au moins 20 cases de distance de Manhattan des passages. Le Ver
  ne poursuit pas une cible hors d'un rayon de cinq cases autour de son territoire.
- Niveau de référence 5, dans la bande proposée 5–8. Ce premier profil ne tire
  pas encore un niveau individuel aléatoire et ne suit pas le niveau du joueur.

La surface, les populations humanoïdes et les machines existantes ne sont pas
modifiées. Les villes restent distinctes des régions sauvages ; le secteur
scénarisé du relais ne reçoit pas automatiquement cette population régionale.

## Balayage annoncé

Le Ver s'oriente vers une position réellement perçue, annonce sa zone d'attaque,
puis frappe dans cette direction à sa prochaine occasion d'action. La visée ne
suit pas le joueur. Son court cône porte à deux cases : une case étroite devant
lui, puis un éventail. Reculer hors de portée ou quitter le cône permet l'esquive.

Après le coup, même dans le vide, il reste immobile pendant deux occasions
d'action. Un déplacement forcé pendant la préparation annule le coup. Les
obstacles présents au moment de l'attaque et les zones protégées restent
respectés. Une longue action du joueur peut laisser passer plusieurs actions
ennemies ; l'annonce ne suspend pas la simulation.

Valeurs de base d'essai : 14 PV, 2 d'armure, 4 dégâts cinétiques, 1 de pénétration,
10 XP avant les règles de progression. Pas d'équipement porté ni de butin
humain généré sur cet animal. Les restes animaux utiles restent à définir ;
les butins de lieux restent indépendants de sa mort.

## Présentation et compatibilité

Le glyphe affiché est `v`, distinct du `M` des marchands. Le nom et le niveau
figurent dans le panneau de cible ; inspection, légende et CAPTEURS reconnaissent
le Ver. La zone annoncée vient du calcul d'attaque, pas d'un cône dessiné à part.
Ni un ennemi invisible ni une case cachée ne sont révélés par cet avertissement.
L'état « Balayage imminent » laisse ensuite place à la récupération immobile.

L'identifiant stable est `core:armored_worm`, la famille de travail
`core:annelids`. Le comportement `telegraphed_sweeper` est ajouté à la fin des
variantes sérialisées ; son engagement réutilise l'état persistant `Aiming`.

Les reprises 109 et antérieures retirent uniquement le nouveau Ver de ses
deux profils régionaux avant génération et calcul d'empreinte. Les nouvelles
parties utilisent la génération 110 ; une ancienne partie n'est pas repeuplée.
Les empreintes 100, 101, 102, 103, 108 et 109 relevées avant l'ajout servent de
références de non-régression.

Diagnostics isolés, sans modification des sauvegardes du joueur :
`--ui-cold-armored-worm` et `--ui-cold-armored-worm-recovery`.

## Vérifications

Les tests couvrent l'empreinte annoncée et les dégâts uniques par cible,
l'esquive sans poursuite de la visée, la récupération immobile, le déplacement
forcé, le couvert, les cases protégées et la simulation hors de la zone active.
Les soigneurs humanoïdes ne soignent pas cette faune.

Le placement est vérifié sur 32 graines par biome. Une comparaison de cartes
complètes sur huit graines par biome confirme que retirer les nouveaux Vers
redonne exactement les anciens acteurs, terrains, butins et menaces ; les
installations et décors restent identiques. Toutes les villes du catalogue
sont vérifiées sans Ver.

Les instantanés conservent l'annonce et la récupération, puis reproduisent
les mêmes événements sur quatre actions supplémentaires. Les suspensions
109 et 110 sont reprises par instantané et par rejeu. Les avertissements
n'affichent ni acteurs cachés ni cases invisibles.

Les trois captures natives du 27 septembre (annonce, récupération et légende)
ont été inspectées ; leur contrôle automatique de contraste passe également.
Validation du lot 110 : 709 tests moteur et 348 tests client réussis ; le test
manuel de lecture d'une suspension externe reste ignoré. Compilation,
`cargo check --locked --all-targets`, formatage et `git diff --check` passent.

## Suite

Le lot suivant intègre l'[Anémone des caves](ANEMONE_DES_CAVES.md) en génération
111 ; son nom remplace « Agrippeur » après validation explicite.
**Hurleur des failles et Sentinelle** sont maintenant intégrés en génération
112 dans leurs milieux plus profonds ; voir le [rapport du lot](INTEGRATION_RENCONTRES_SOUTERRAINES.md).
La définition des butins animaux et la relecture des 27 autres noms restent
ouvertes. Aucun système d'artisanat ni de réputation n'est introduit.
