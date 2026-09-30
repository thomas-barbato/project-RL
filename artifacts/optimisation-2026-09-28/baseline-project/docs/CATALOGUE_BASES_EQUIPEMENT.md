# Catalogue des bases d’équipement

25 septembre 2026 — catalogue de conception ; **dix-huit bases sont désormais intégrées**.
Le [premier raccordement jouable](GENERATION_EQUIPEMENT.md) concerne les couteaux,
fusils et vestes P1 à P6 : six modèles par famille, couvrant les six couches
accessibles, surface comprise. Les autres familles restent des propositions.

## Périmètre

**144 bases nommées dans 24 familles**, chacune déclinée sur six paliers de puissance :
84 armes et modules offensifs, 60 protections et modules utilitaires. Les noms
sont proposés pour durer, pas des « arme niveau 1/2/3 ». Leur validation
individuelle et l’équilibrage restent à faire. Un nom de matériau ou de phénomène
ne promet pas automatiquement une mécanique particulière.

Ce catalogue ne transforme pas les 56 exemplaires du laboratoire en 144 nouveaux
objets jouables. Les nouvelles parties peuvent trouver les dix-huit bases dans
les régions habitées de surface, les caches souterraines et les
[marchands avec paris](COMMERCE_EQUIPEMENT.md). Les [Artilleurs et Soigneurs](EQUIPEMENT_DES_ENNEMIS.md)
portent aussi une arme récupérable depuis la génération 108. Les autres ennemis
et les anciennes parties restent inchangés.
144 n'est pas un objectif obligatoire : une base supplémentaire doit apporter
une différence utile, pas recopier chaque combinaison de bonus.

Le mélange humain, technique étrange, organique et vivant est validé par
l’utilisateur. **La provenance est également obligatoire : un robot ne donne pas
d’équipement humain.** Le fait qu’un objet soit organique ne permet pas non plus
de le faire tomber de n’importe quel animal.

Source structurée : [equipements.json](catalogues/equipements.json).
Règles de tirage et noms composés : [Affixes et provenance](AFFIXES_ET_PROVENANCE_EQUIPEMENT.md).

## Paliers, pas niveaux du personnage

| Palier | Repère | Intention |
|---|---|---|
| P1 | Usuel | Matériel courant, formes et usages immédiatement reconnaissables. |
| P2 | Spécialisé | Fabrications adaptées à un métier, un terrain ou un rôle de combat. |
| P3 | Avancé | Matériaux composites et mécanismes élaborés ; silhouettes encore familières. |
| P4 | Exotique | Structures inhabituelles, géométries et phénomènes étrangers. |
| P5 | Aberrant | Objets difficiles à interpréter ; fonction de combat encore lisible. |
| P6 | Singulier | Extrême profondeur : formes impossibles et fonctionnement non humain. |

Ces six paliers sont extensibles et ne fixent ni le niveau maximum du joueur,
ni une correspondance obligatoire avec une couche, ni un niveau requis pour
équiper l’objet. La profondeur et le danger orienteront les probabilités ;
une trouvaille supérieure au palier habituel restera possible.

Une base de haut palier doit apporter un intérêt réel sans effacer les
différences entre familles. Dégâts, portée, récupération, précision, ressources
et contraintes seront réglés ensemble. Ajouter seulement des PV aux ennemis
pour absorber l’inflation d’équipement n’est pas la méthode retenue.

## Sources compatibles

| Source | Équipements admissibles | Règle |
|---|---|---|
| Humanoïde équipé | 120 bases portables ou vestimentaires ; sélection affinée selon son anatomie et ses capacités | Possessions attribuées avant le combat et réellement restantes à sa mort |
| Robot | 24 modules mécaniques ou blindages de châssis | Aucune botte, veste, épée ou arme de poing humaine ; modèle et montage à vérifier |
| Créature organique non équipée | Aucun équipement générique pour l’instant | Restes utiles éventuels séparés ; son corps ne devient pas automatiquement une arme |
| Être anormal sans possessions | Aucun équipement automatique | Matérialité et utilité d’un éventuel reste à définir |
| Réserve humanoïde / dépôt mécanique | Le groupe correspondant au lieu | Contenu indépendant de la mort du gardien ; pas de deuxième récompense créée au décès |

La provenance d’un objet et son porteur possible sont distincts de son matériau.
Une veste vivante reste un vêtement destiné à une anatomie compatible ; elle ne
tombe pas spontanément d’un animal parce que les deux sont organiques.
Un nouvel ennemi équipé ou un organisme portant un élément réellement récupérable
pourra recevoir un profil précis. Aucune exception universelle n’est activée ici.

Les pièces de machine ne supposent ni un atelier d’adaptation, ni de l’artisanat,
ni une compatibilité automatique avec le joueur. Leur usage concret fait partie
du travail d’intégration déclaré ci-dessous.

## Familles et bases

### 1. Couteaux

Contact précis, faible impact par frappe, créneaux d'action courts.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Six bases P1 à P6 intégrées en génération 106 ; équilibrage provisoire.

Portée, précision et récupération sont paramétrables ; ne pas promettre un critique ou un dos vulnérable sans mécanique dédiée.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Couteau de camp | Fabriqué |
| P2 | Couteau de sapeur | Fabriqué |
| P3 | Couteau céramique | Fabriqué |
| P4 | Couteau de chitine | Organique |
| P5 | Couteau à dent vivante | Vivant |
| P6 | Couteau du dernier seuil | Étrange |

### 2. Épées

Mêlée polyvalente et aptitude à la parade ; portée courte.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Profil de contenu envisageable avec les primitives actuelles ; valeurs et intégration à réaliser.

La parade est une capacité du matériel exploitée par une compétence apprise, pas une compétence accordée par l'objet.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Épée de patrouille | Fabriqué |
| P2 | Épée de garde | Fabriqué |
| P3 | Épée composite | Fabriqué |
| P4 | Épée nervurée | Organique |
| P5 | Épée à veine vive | Vivant |
| P6 | Épée de l'intervalle | Étrange |

### 3. Haches

Fort impact de contact contre une disponibilité plus contraignante.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Profil de contenu envisageable avec les primitives actuelles ; valeurs et intégration à réaliser.

Impact et récupération existent ; pas de bris d'armure ou de destruction de décor gratuits.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Hache de chantier | Fabriqué |
| P2 | Hache d'assaut | Fabriqué |
| P3 | Hache laminée | Fabriqué |
| P4 | Hache de carapace | Organique |
| P5 | Hache de rupture | Étrange |
| P6 | Hache de l'entaille noire | Étrange |

### 4. Masses

Impact cinétique concentré ; force exploitée dans la limite du matériau.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Profil de contenu envisageable avec les primitives actuelles ; valeurs et intégration à réaliser.

Pousser ou étourdir n'est pas un résultat automatique de la famille : cela exige une technique ou un effet déclaré.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Masse de chantier | Fabriqué |
| P2 | Masse de brèche | Fabriqué |
| P3 | Masse inertielle | Fabriqué |
| P4 | Masse de résonance | Étrange |
| P5 | Masse à cœur battant | Vivant |
| P6 | Masse du point mort | Étrange |

### 5. Lances

Contact à allonge, tenue d'un couloir et placement.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

Valider une vraie allonge de mêlée, les diagonales et les interactions avec parade/interception ; pas un fusil renommé.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Lance de veille | Fabriqué |
| P2 | Lance de rempart | Fabriqué |
| P3 | Lance télescopique | Fabriqué |
| P4 | Lance d'os spiralé | Organique |
| P5 | Lance à tige vive | Vivant |
| P6 | Lance de l'horizon clos | Étrange |

### 6. Armes de poing

Tir ponctuel à portée modérée et faible engagement.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Profil de contenu envisageable avec les primitives actuelles ; valeurs et intégration à réaliser.

Définir des ressources et une récupération distinctes du fusil ; aucune règle de double maniement implicite.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Pistolet de service | Fabriqué |
| P2 | Pistolet de vigie | Fabriqué |
| P3 | Pistolet à aiguilles | Fabriqué |
| P4 | Pistolet à impulsion | Étrange |
| P5 | Pistolet de rémanence | Étrange |
| P6 | Pistolet du point aveugle | Étrange |

### 7. Fusils

Précision à moyenne/longue portée ; ligne de tir importante.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Six bases P1 à P6 intégrées en génération 106 ; équilibrage provisoire.

Tir automatique seulement pour les modèles qui déclarent la capacité, sans accorder la compétence associée.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Fusil de patrouille | Fabriqué |
| P2 | Fusil de guetteur | Fabriqué |
| P3 | Fusil à induction | Fabriqué |
| P4 | Fusil de parallaxe | Étrange |
| P5 | Fusil à nerf tendu | Vivant |
| P6 | Fusil de l'horizon fendu | Étrange |

### 8. Fusils à dispersion

Zone courte et large ; placement face à un groupe.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

La géométrie en cône existe ; concevoir la résolution de la dispersion avant de simuler plusieurs plombs comme des attaques indépendantes.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Fusil à gerbe | Fabriqué |
| P2 | Fusil de tranchée | Fabriqué |
| P3 | Fusil à fléchettes | Fabriqué |
| P4 | Fusil à gerbe d'épines | Organique |
| P5 | Fusil à dispersion creuse | Étrange |
| P6 | Fusil de l'éventail noir | Étrange |

### 9. Arbalètes

Tir mécanique appuyé, disponibilité plus espacée, sans dépense électrique par défaut.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

Un délai de récupération est possible ; le cycle de rechargement et ses ressources doivent être définis avant intégration.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Arbalète de chasse | Fabriqué |
| P2 | Arbalète de treuil | Fabriqué |
| P3 | Arbalète composite | Fabriqué |
| P4 | Arbalète de tendons | Organique |
| P5 | Arbalète sans corde | Étrange |
| P6 | Arbalète du fil rompu | Étrange |

### 10. Projecteurs thermiques

Zone thermique visible, couloirs et persistance explicitement déclarée.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Profil de contenu envisageable avec les primitives actuelles ; valeurs et intégration à réaliser.

Les cônes, brûlures et champs au sol existent ; leur présence fait partie de la base seulement si elle est déclarée.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Brûleur de défense | Fabriqué |
| P2 | Lance-flammes de sape | Fabriqué |
| P3 | Projecteur thermique | Fabriqué |
| P4 | Projecteur à chambre solaire | Étrange |
| P5 | Projecteur à gorge de braise | Vivant |
| P6 | Projecteur du brasier muet | Étrange |

### 11. Projecteurs électriques

Dégâts électriques ; position et terrain conducteur selon le profil.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

Le type électrique et Conduction existent ; ne pas appliquer la propagation à toutes les bases par défaut. Consommation et rendu primaire à définir.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Émetteur d'arc | Fabriqué |
| P2 | Émetteur de contention | Fabriqué |
| P3 | Projecteur capacitif | Fabriqué |
| P4 | Projecteur à nœuds conducteurs | Organique |
| P5 | Projecteur d'orage clos | Étrange |
| P6 | Projecteur de l'éclair immobile | Étrange |

### 12. Canons d'impact

Tir lourd, impact important et fenêtre d'indisponibilité marquée.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

Pas de promesse d'explosion ou de recul automatique ; vérifier la préparation, l'impact principal, les ressources et la récupération.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Canon à boulons | Fabriqué |
| P2 | Canon de démolition | Fabriqué |
| P3 | Canon inertiel | Fabriqué |
| P4 | Canon de compression | Étrange |
| P5 | Canon à vertèbres | Organique |
| P6 | Canon du centre effondré | Étrange |

### 13. Vestes protectrices

Protection légère ; budget plutôt orienté vers les caractéristiques et les réserves.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Six bases P1 à P6 intégrées en génération 106 ; équilibrage provisoire.

La distinction légère/lourde est une orientation de budget, pas un bonus d'esquive déjà implémenté.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Veste matelassée | Fabriqué |
| P2 | Veste de veille | Fabriqué |
| P3 | Veste à fibres croisées | Fabriqué |
| P4 | Veste de membranes | Organique |
| P5 | Veste de peau seconde | Vivant |
| P6 | Veste de la seconde ombre | Étrange |

### 14. Plastrons

Protection intermédiaire et régulière.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Profil de contenu envisageable avec les primitives actuelles ; valeurs et intégration à réaliser.

Le moteur sait équiper une protection de torse ; les nouvelles bases restent hors distribution.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Plastron de récupération | Fabriqué |
| P2 | Plastron de garde | Fabriqué |
| P3 | Plastron stratifié | Fabriqué |
| P4 | Plastron de nacre | Organique |
| P5 | Plastron sans envers | Étrange |
| P6 | Plastron de l'intervalle | Étrange |

### 15. Armures lourdes

Protection physique élevée ; contrepartie fonctionnelle à tester.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

Ne pas ajouter seulement davantage d'armure. Fixer une contrepartie réelle et lisible avant d'établir sa supériorité ou ses coûts.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Armure rivetée | Fabriqué |
| P2 | Armure de siège | Fabriqué |
| P3 | Armure à plaques composites | Fabriqué |
| P4 | Armure de carapaces | Organique |
| P5 | Armure à plaques vivantes | Vivant |
| P6 | Armure de l'espace fermé | Étrange |

### 16. Casques

Protection de tête ; affixes de perception, traitement ou défense possibles.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Emplacement et interface à intégrer et à valider.

Le profil d'armure accepte un emplacement déclaratif ; ajouter cet emplacement au personnage et à l'interface demande une décision distincte.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Casque de chantier | Fabriqué |
| P2 | Casque de vigie | Fabriqué |
| P3 | Casque à coque composite | Fabriqué |
| P4 | Casque de chitine | Organique |
| P5 | Casque sans face | Étrange |
| P6 | Casque de l'œil fermé | Étrange |

### 17. Gantelets

Protection des mains ; affixes de maniement ou de caractéristiques possibles.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Emplacement et interface à intégrer et à valider.

Aucun bonus de vitesse, de parade ou de puissance intrinsèque n'est accordé par le seul nom de famille.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Gantelets de travail | Fabriqué |
| P2 | Gantelets de garde | Fabriqué |
| P3 | Gantelets articulés | Fabriqué |
| P4 | Gantelets de tendons tressés | Organique |
| P5 | Gantelets aux jointures inversées | Étrange |
| P6 | Gantelets de la prise impossible | Étrange |

### 18. Bottes

Protection des pieds ; futurs profils de terrain à concevoir séparément.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Emplacement et interface à intégrer et à valider.

Pas de vitesse supplémentaire ou d'immunité aux terrains sans support moteur et coût de puissance explicites.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Bottes de marche | Fabriqué |
| P2 | Bottes de traverse | Fabriqué |
| P3 | Bottes renforcées | Fabriqué |
| P4 | Bottes de mailles réticulées | Étrange |
| P5 | Bottes à semelles vivantes | Vivant |
| P6 | Bottes du pas absent | Étrange |

### 19. Ceintures

Support défensif ou utilitaire ; réserve et dissipation parmi les affixes compatibles.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Emplacement et interface à intégrer et à valider.

Une ceinture pourra devenir un accessoire plutôt qu'une armure ; ne pas lui inventer un point d'armure obligatoire pour contourner le modèle actuel.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Ceinture de route | Fabriqué |
| P2 | Ceinture de service | Fabriqué |
| P3 | Ceinture compartimentée | Fabriqué |
| P4 | Ceinture de fibres nerveuses | Organique |
| P5 | Ceinture à boucle ouverte | Étrange |
| P6 | Ceinture du cercle incomplet | Étrange |

### 20. Boucliers

Protection directionnelle et parade potentielle.

**Source :** humanoïde équipé ou réserve compatible.
**État :** Travail de règles ou de mécanique requis avant intégration.

L'occupation d'une main, les armes compatibles et le blocage ne sont pas décidés ; aucun emplacement de main secondaire n'est créé par ce catalogue.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Bouclier de récupération | Fabriqué |
| P2 | Bouclier de rempart | Fabriqué |
| P3 | Bouclier composite | Fabriqué |
| P4 | Bouclier de nacre | Organique |
| P5 | Bouclier à peau mouvante | Vivant |
| P6 | Bouclier du revers absent | Étrange |

### 21. Modules de coupe

Outil de coupe fixé à une machine ; impact de contact.

**Source :** robot ou dépôt mécanique.
**État :** Travail de règles ou de mécanique requis avant intégration.

Valider la récupération et la compatibilité d'équipement du joueur sans ajouter implicitement une recette ou un atelier.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Disque de coupe | Fabriqué |
| P2 | Module de cisaille | Fabriqué |
| P3 | Module à dents céramiques | Fabriqué |
| P4 | Module à tranchant vibratoire | Étrange |
| P5 | Module de coupe sans axe | Étrange |
| P6 | Module du plan séparé | Étrange |

### 22. Modules balistiques

Armement balistique de châssis ; tir et ressources propres.

**Source :** robot ou dépôt mécanique.
**État :** Travail de règles ou de mécanique requis avant intégration.

La pièce utilisée par le robot est celle qui pourra être récupérée, pas un pistolet humain tiré après sa mort.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Module lance-rivets | Fabriqué |
| P2 | Module lance-aiguilles | Fabriqué |
| P3 | Module balistique à induction | Fabriqué |
| P4 | Module à chambre prismatique | Étrange |
| P5 | Module de tir sans recul | Étrange |
| P6 | Module du projectile absent | Étrange |

### 23. Blindages de châssis

Protection structurelle d'une machine, distincte d'un plastron humain.

**Source :** robot ou dépôt mécanique.
**État :** Travail de règles ou de mécanique requis avant intégration.

La compatibilité avec le corps du joueur reste à décider ; une plaque intégrée n'est pas automatiquement une armure portable.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Blindage boulonné | Fabriqué |
| P2 | Blindage articulé | Fabriqué |
| P3 | Blindage céramique | Fabriqué |
| P4 | Blindage alvéolaire inversé | Étrange |
| P5 | Blindage sans jointure | Étrange |
| P6 | Blindage de l'enveloppe close | Étrange |

### 24. Modules thermiques

Module de réserve ou de dissipation ; pas une pièce vestimentaire.

**Source :** robot ou dépôt mécanique.
**État :** Travail de règles ou de mécanique requis avant intégration.

Famille utilitaire : le modèle actuel d'armure impose une protection positive. Ne pas ajouter d'armure fictive pour contourner cette limite.

| Palier | Nom proposé | Nature |
|---|---|---|
| P1 | Dissipateur à ailettes | Fabriqué |
| P2 | Module de refroidissement | Fabriqué |
| P3 | Module caloporteur | Fabriqué |
| P4 | Module à circulation inverse | Étrange |
| P5 | Module de chaleur captive | Étrange |
| P6 | Module du puits froid | Étrange |

## Vérification et suite

Le validateur contrôle les identifiants uniques, les six paliers de chaque
famille, les sources compatibles, les noms, les accords et les affixes.
La distinction « compatible » ne suffit pas à garantir un drop : présence
réelle, consommation, destruction et conditions de récupération restent nécessaires.

Ordre d’intégration : propriétés nommées persistantes par exemplaire ; noms et
fiches du laboratoire ; premières familles aux comportements vérifiables ;
profils d’équipement des ennemis ; distribution et économie. Les familles
qui nécessitent une nouvelle règle doivent passer leur propre essai avant diffusion.

Répartition actuelle : 72 bases fabriquées, 15 organiques, 10 vivantes et 47 étranges.
