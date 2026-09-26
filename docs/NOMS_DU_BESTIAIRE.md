# Noms du bestiaire et des habitants — première revue

26 septembre 2026 — première liste validée, noms de surface appliqués en jeu.

## Direction retenue

Les noms du catalogue ont été jugés trop artificiels, y compris ceux de la
surface. La nouvelle direction est confirmée : des noms courts, faciles à
prononcer et évocateurs, comme s'ils avaient été donnés par les habitants.
« Sentinelle » remplace la proposition « Gardien de confinement » pour la
machine envisagée. Après présentation, les cinq noms d'animaux de surface,
les quatre noms souterrains et les cinq prénoms des habitants ci-dessous ont
été validés. Cet accord ne couvre pas les 27 autres entrées du catalogue.

- Nommer une créature d'après un trait reconnaissable de son corps, de son
  habitat ou de son comportement. Ne pas lui inventer une capacité pour
  justifier son nom.
- Garder certains noms déjà parlants : tout ne nécessite pas un remplacement.
- Séparer le nom courant, la famille, le régime alimentaire et le tempérament.
  « Herbivore » reste une information utile, sans devoir être le nom de l'espèce.
- Réserver les noms propres aux individus. Afficher le métier d'un habitant
  séparément ; un surnom peut exister, sans être la règle pour tout le monde.
- Pour une machine, employer une fonction compréhensible. Un numéro de modèle
  peut figurer dans son inspection, sans alourdir systématiquement son nom.
- L'étrangeté croissante vient des formes et des comportements, pas d'une
  succession de mots pseudo-scientifiques.

Les familles et les 36 identités du
[catalogue de conception](BESTIAIRE_ET_RENCONTRES.md) restent structurées comme
avant. Leurs anciennes appellations servent de correspondance de travail,
pas de noms à intégrer tels quels. Aucun habitat, niveau, comportement,
butin, peuple ou lien de parenté n'est changé par cette revue.

## 1. Les cinq animaux déjà jouables

Les noms validés sont appliqués à l'inspection, au ciblage et à la légende.
Il s'agit des mêmes animaux, sans modification de leurs comportements.

| Ancien nom affiché | Correspondance du catalogue | Nom retenu | Pourquoi |
|---|---|---|---|
| Grignoteur de gravats | Vaurin cendré | **Grignoteur** | Petit nom courant ; le milieu peut rester dans sa description. |
| Mordeur des friches | Vaurin fauve | **Mordeur des friches** | Nom déjà concret, qui distingue le prédateur de meute. |
| Brise-os | Vaurin ossuaire | **Brise-os** | Court et reconnaissable ; conserver cette identité. |
| Fouisseur vibrant | Talvène pâle | **Fouisseur pâle** | Reprend l'apparence proposée sans laisser croire que l'animal émet des vibrations. |
| Herbivore à carapace | Ostrèle des mousses | **Dos-rond** | Évoque sa coque bombée ; « herbivore » reste dans la description. |

La pâleur du Fouisseur vient de sa fiche de conception, pas d'une nouvelle
propriété de simulation. « Dos-rond » est son nom courant,
pas une nouvelle famille et pas un retour à « Brouteur ».

## 2. Les quatre rencontres souterraines envisagées

Ces quatre entrées ne sont pas encore intégrées. Leur sélection ne les place
pas toutes au premier étage : habitats et bandes de danger restent ceux du
catalogue, à confronter à la progression avant intégration.

| Ancienne proposition | Nom retenu | Trait reconnaissable prévu |
|---|---|---|
| Néride annelée | **Ver cuirassé** | Corps épais à anneaux rigides. |
| Hymèle à vrilles | **Agrippeur** | Masse ancrée au sol, qui saisit avec ses vrilles. |
| Lithère creuse | **Hurleur des failles** | Coque minérale creuse et onde annoncée par une résonance. |
| Prévôt S-12 | **Sentinelle** | Machine de défense qui prépare une attaque en cône. |

« Sentinelle » est déjà employé par un ancien profil générique du prototype.
L'intégration devra distinguer les identités concernées : ce nom n'autorise
ni à transformer tous ces acteurs en robots, ni à attribuer le nouveau
comportement à toute IA de garde. Les identifiants techniques restent stables.

Les 27 autres entrées du catalogue restent à relire dans cette même direction.
Elles ne sont ni supprimées ni validées par défaut.

## 3. Habitants et adversaires humains

Les prénoms validés sont appliqués aux individus fixes, avec leur activité
affichée séparément :

| Ancien nom | Prénom retenu | Activité existante affichée séparément |
|---|---|---|
| Orme | **Elias** | Informations sur les chemins du quartier. |
| Sève | **Nora** | Soins à la clinique. |
| Rivet | **Milo** | Récupération au relais. |
| Opérateur du tri | **Basile** | Tri au dépôt. |
| Éclaireuse de la lisière | **Lina** | Reconnaissance des friches. |

Ces prénoms ne fixent ni culture, ni peuple, ni passé de campagne. Les rôles et
les dialogues restent ceux des prototypes ; ce document ne réécrit pas les
services, les quêtes ou leur statut de démonstration.

Les marchands et habitants générés ne doivent pas tous recevoir le même prénom
par remplacement d'une traduction générique. Une future attribution de noms
individuels devra être persistante et éviter les doublons gênants dans un lieu.

Pour les adversaires anonymes, des intitulés de rôle restent utiles. Piste
distincte non appliquée dans ce lot :
**Tireur** est proposé à la place d'« Artilleur » pour le porteur de fusil,
et **Soigneur** à la place de « Soigneur de terrain ». Cela ne donne pas un nom
de peuple, une faction ou une hostilité à tous ceux qui exercent ce métier.

## 4. Périmètre d'intégration

Les noms de surface changent dans les textes français et la présentation du
client : inspection, ciblage, légende, interactions, dialogues, journal et
messages narratifs. L'objectif de l'enquête et son compte rendu citent Elias.
Les prénoms ne remplacent pas les rôles génériques de tous les marchands ou
habitants procéduraux.

Les identifiants d'acteurs, de contenu, de textes et de quêtes sont conservés,
notamment `core:orme`, `core:seve`, `core:rivet` et `core:moss_grazer`. Aucune
modification de population, de règles, de butin ou de version de génération.
Les textes localisés sont résolus lors de l'affichage, pas incorporés aux
empreintes de simulation. Les anciennes lignes de journal déjà enregistrées
restent historiques ; on ne réécrit pas les messages conservés d'une partie.

La compatibilité est couverte par un test qui crée des suspensions avec les
anciennes traductions aux générations 100, 103 et 109, puis les reprend avec
les nouveaux noms par instantané et par rejeu inter-compilations. D'autres
tests vérifient les noms des cinq animaux et la cohérence dialogue, interaction,
donneur de quête et textes de mission. Les diagnostics natifs permettent de
contrôler la légende, le Dos-rond, Lina et le journal d'Elias.

Les quatre rencontres souterraines restent à implémenter ; leurs noms sont
désormais fixés. Les 27 autres entrées seront relues avant leur intégration.
