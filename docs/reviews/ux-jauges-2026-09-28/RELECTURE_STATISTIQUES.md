# Statistiques et jauges — textes à relire

Chaque entrée ci-dessous reprend le texte de l'onglet F1 « Statistiques et jauges ». La numérotation sert à commenter chaque description individuellement ; elle n'apparaît pas dans le jeu.

Les réserves, attributs, seuils et rythmes de récupération chiffrés sont ceux d'une partie de diagnostic. Dans le jeu, ils sont lus dans la partie en cours ; les anciennes parties peuvent avoir d'autres règles. Ces chiffres illustrent l'affichage et ne constituent pas de nouvelles propositions d'équilibrage.

## Jauges et ressources

### 01. Points de vie (PV)

Votre état actuel sur votre maximum. Les dégâts retirent des PV ; à zéro, la partie est terminée. Augmenter le maximum avec un bonus d'équipement n'est pas un soin. Les effets du passage de niveau sont indiqués dans Progression.

### 02. Énergie (E)

Réserve commune aux techniques et aux équipements alimentés : 100 / 100 actuellement. Vous récupérez 1 point d'énergie à chaque tour terminé, sans dépasser la capacité. Attendre fait aussi agir le monde. Ouvrir un menu ne recharge rien. À zéro, seules les actions demandant de l'énergie sont bloquées ; cela ne tue pas le personnage.

### 03. Munitions

Stock commun aux armes qui utilisent des projectiles. Chaque tir ou projectile d'une salve prélève son coût, même s'il manque sa cible. Le coût de l'arme active est indiqué près de son nom. Changer d'arme ne recharge pas le stock. La barre utilise une échelle visuelle par tranches de 40 : ce n'est pas une capacité maximale. Les anciennes parties à chargeurs affichent plutôt le stock et la capacité de l'arme.

### 04. Bande passante (B)

Capacité de contrôle partagée par vos drones, certaines liaisons et certains effets maintenus. La jauge montre la part OCCUPÉE : plus elle est pleine, moins il reste de place. Une réservation est libérée lorsque son effet ou sa liaison prend fin ; attendre ne libère pas une liaison toujours active. Apprendre une technique ne réserve rien à lui seul. Actuellement : 0 occupés, 4 libres, capacité 4.

### 05. Chaleur (H) et dissipation

Chaleur accumulée par certaines techniques et certains modules. Elle redescend à la fin des tours grâce à la dissipation. Moins il y en a, mieux c'est. La barre va jusqu'au seuil critique, mais la valeur peut le dépasser : ce seuil n'est pas une capacité maximale. Actuellement : 0 ; alerte à 40 ; seuil critique à 80 ; dissipation de 4 par tour terminé.

### 06. Alerte thermique et seuil critique

La jauge devient ambre au seuil d'alerte, puis rouge au seuil critique. Pour la réserve personnelle affichée ici, franchir ces seuils signale le danger sans infliger, à lui seul, une perte de PV. Les conséquences dépendent du module ou de l'effet : un module surcadencé peut s'user au-dessus de son seuil sûr et refuser une attaque qui dépasserait sa limite. Une surchauffe hostile appliquée à un système électronique possède ses propres dégâts thermiques.

## Attributs primaires

### 07. PUISSANCE

Renforce vos frappes physiques au corps à corps et votre capacité à repousser les adversaires. Améliore la charge que vous pouvez transporter et la maîtrise du recul, dans les limites de votre équipement. N’augmente pas les dégâts des projectiles. Votre valeur actuelle : 6.

### 08. COORDINATION

Améliore la précision de vos attaques et de vos lancers, ainsi que votre capacité à esquiver et à rester discret. N’augmente pas votre vitesse d’action. Votre valeur actuelle : 6.

### 09. RÉSILIENCE

Augmente vos points de vie maximaux et améliore votre Stabilité, pour mieux résister aux interruptions et à certaines perturbations. N’améliore pas votre armure et ne restaure pas les points de vie perdus. Votre valeur actuelle : 6.

### 10. PERCEPTION

Améliore votre capacité à repérer les présences dissimulées et les indices discrets, ainsi qu’à analyser vos observations. Contribue également à la précision de vos tirs. N’augmente pas la portée de vos capteurs et ne permet pas de voir à travers les murs. Votre valeur actuelle : 5.

### 11. TRAITEMENT

Améliore l’efficacité de vos intrusions et votre résistance aux attaques logicielles. Facilite l’analyse des informations disponibles. N’augmente ni vos réserves d’énergie ni votre bande passante. Votre valeur actuelle : 5.

## Défenses et résistances

### 12. Armure

Réduit les dégâts physiques des impacts compatibles, après prise en compte de la pénétration et de la fragilisation. Les dégâts cinétiques, perforants et explosifs rencontrent cette protection. L'armure n'est ni de l'esquive ni une réserve de PV.

### 13. Fragilisation

Réduit temporairement l'armure prise en compte. La valeur et la durée de l'effet indiquent la protection perdue ; elles ne représentent pas des dégâts immédiats.

### 14. Esquive

Score opposé à la précision des attaques ciblées qui peuvent être évitées. Une valeur de 10 n'est pas une probabilité de 10 %. Une zone dangereuse ou une attaque sans jet d'esquive suit ses propres règles.

### 15. Stabilité

Aide à résister aux interruptions et aux perturbations qui testent cette défense. Ce score n'est pas une réduction générale des dégâts et ne remplace pas l'armure.

### 16. Défense numérique

Réduit les chances qu'une intrusion ou un programme hostile franchisse les protections logicielles d'un système compatible. Ne protège pas des décharges électriques et ne retire pas un programme déjà implanté.

### 17. Résistance thermique

Réduit les dégâts de feu et les autres dégâts thermiques. Elle ne vide pas la jauge de chaleur et n'améliore pas la dissipation.

### 18. Résistance électrique

Réduit les dégâts électriques. Elle ne remplace pas la défense numérique contre un programme hostile.

### 19. Résistance chimique

Réduit les dégâts chimiques. Les autres effets d'une substance gardent leurs propres conditions.

### 20. Résistance aux radiations

Réduit la composante Radiation des dégâts reçus.

### 21. Résistance à la corruption

Réduit la composante Corruption des dégâts reçus.

### 22. Pourcentages de résistance

Une valeur positive réduit les dégâts du type indiqué ; une valeur négative les augmente. Zéro n'apporte pas de réduction. Chaque composante d'un impact est traitée selon son type.

## Attaques, armes et délais

### 23. Précision et chance de toucher

La précision de l'arme, les attributs, les états et la défense de la cible participent au jet. Le pourcentage Touche de la fiche de personnage est une référence sans cible, pas une garantie contre chaque adversaire.

### 24. Dégâts et types

La fiche indique les dégâts avant les protections de la cible. Un impact peut combiner plusieurs types. KIN : cinétique ; PIR : perforant ; EXP : explosif ; THR : thermique ; ELE : électrique ; CHM : chimique ; RAD : radiation ; COR : corruption.

### 25. Pénétration d'armure

Ignore une partie de l'armure pour cet impact. Elle ne détruit pas l'équipement et ne diminue pas les résistances spécialisées.

### 26. Impact disponible, transmis et plafond

Au corps à corps, la Puissance contribue à l'impact disponible. L'arme n'en transmet que ce que son matériau supporte. L'impact transmis intervient dans les dégâts physiques ; augmenter la Puissance au-delà du plafond de cette arme n'augmente pas cette part.

### 27. Portée, rayon et zone

La portée fixe la distance maximale de l'action. Le rayon et la forme décrivent les cases affectées. Visibilité, obstacles, trajectoire et conditions de ciblage restent applicables ; la précision n'allonge pas la portée.

### 28. Préparation, action et récupération

La préparation précède l'effet ; l'action le résout ; la récupération est le délai imposé ensuite. Une attente ou un déplacement peut faire avancer la récupération si l'action est autorisée. Les valeurs indiquent la durée propre à la technique ou à l'arme.

### 29. Durée, recharge et entretien

La durée indique combien de temps un effet persiste. La recharge indique le délai avant de réutiliser une technique. Un coût d'entretien est prélevé pendant le maintien, en plus du coût initial lorsqu'il existe. Consulter la fiche pour les conditions d'arrêt.

### 30. E, H et B dans un coût

E est l'énergie dépensée ; H la chaleur ajoutée ; B la bande passante réservée. Les munitions et autres consommables sont indiqués séparément. Ces nombres décrivent des usages différents et ne s'additionnent pas en une seule réserve.

## Équipement, perception et compagnons

### 31. Base, bonus et valeur actuelle

Base désigne les attributs du personnage ; les bonus viennent de l'équipement et des effets applicables. La valeur actuelle tient compte de ces contributions et de leurs limites. Un bonus de capacité ne remplit pas automatiquement la réserve concernée.

### 32. Durabilité

État propre d'un équipement, d'un composant ou d'un objet destructible. Une pièce défaillante peut perdre ses fonctions. La réparer ne soigne pas automatiquement les PV de son porteur.

### 33. Places d'inventaire et quantité

Les places comptent les entrées et piles de votre sac, par rapport à sa capacité. La quantité est le nombre d'exemplaires dans une pile ; une pile peut contenir plusieurs objets. Les emplacements d'équipement déterminent où un objet compatible peut être porté.

### 34. Masse, ancrage et traction

La masse du corps et de sa charge, ainsi que l'ancrage, interviennent dans la résistance aux déplacements forcés. La traction limite la masse qu'un dispositif peut déplacer. Ces valeurs ne réduisent pas directement les dégâts.

### 35. Temps de déplacement

Temps nécessaire pour parcourir une case dans votre état actuel. Certains états ou terrains peuvent le modifier ; les autres acteurs continuent d'agir pendant ce temps. La Coordination n'accélère pas toutes les actions.

### 36. Portée des capteurs

Distance maximale d'observation par vos capteurs. Les obstacles et le canal utilisé restent déterminants. La Perception améliore les observations compatibles, mais agrandir la fenêtre ou augmenter cet attribut ne permet pas de voir à travers les murs.

### 37. Signature, discrétion et détection

Une signature est un indice émis, par exemple un bruit ou une silhouette. La discrétion et la détection sont comparées selon le canal et l'effet concernés. Masquer un canal ne rend pas invisible sur tous les autres, et n'efface pas une observation passée.

### 38. PV, batterie et liaison d'un compagnon

Les PV et la batterie appartiennent au compagnon, pas au joueur. La liaison conditionne les ordres à distance. Sa bande passante peut rester réservée pendant son contrôle ; attendre ne crée ni batterie ni actions supplémentaires pour lui.

### 39. Distance et informations inconnues

Les distances sont exprimées en cases. Les statistiques d'une cible ne sont affichées que lorsqu'elles sont connues, notamment après analyse. Une donnée inconnue ne signifie pas zéro.

## Progression

### 40. Niveau et expérience (XP)

L'expérience cumulée permet d'atteindre les seuils de niveau. La jauge XP représente l'avancement entre le seuil du niveau actuel et celui du suivant ; le compteur affiche l'expérience cumulée et le prochain seuil. Dans cette partie, chaque niveau gagné ajoute 3 PV au maximum. Le passage de niveau restaure aussi tous vos PV.

### 41. Points de compétence

Servent à apprendre les techniques et améliorations dont les conditions sont remplies. Le coût dépend des choix déjà effectués dans la discipline et apparaît avant l'achat. Vous pouvez conserver ces points pour plus tard.

### 42. Conditions et améliorations

Une technique peut demander un niveau, des attributs et un apprentissage préalable. Une amélioration modifie une technique connue. Les fiches indiquent les effets, coûts, limites et conditions d'utilisation de chaque choix.

