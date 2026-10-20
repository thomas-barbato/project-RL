# COMBAT-START-01 / FIRE-NATIVE-01 — retour de l'essai des égouts

Le 20 octobre, le joueur signale que les monstres n'attaquent pas ou ne le
touchent jamais, que le lance-flammes semble encore agrandi depuis 32 px, et
demande d'activer les monstres prévus dans la ville.

## Diagnostic et périmètre

Le test isolé conserve les règles, statistiques et défenses initiales réelles
du Bastion. À découvert, le profil de mêlée de niveau 1 porte vingt coups sur
vingt-quatre tentatives ; le profil de tir en porte neuf sur neuf. Les chances
calculées sont respectivement de 79 et 87 %. Les dégâts sont intégralement
absorbés par l'armure. Il ne s'agit donc pas d'une absence de tirage de précision
dans ce cas. Les murs et les cases d'arrivée protégées gardent leurs règles.

La proposition initiale COMBAT-START-01 augmentait les dégâts de base des
quatre profils ordinaires : mêlée 2 → 4 et tir 1 → 5 au niveau 1 ; mêlée 3 → 5
et tir 2 → 6 au niveau 2. Après un accord pour essai, le joueur remplace cette
approche : « non ce qu'on va faire :) c'est déjà mettre les armures de base du
joueur à 0 :) elles serviront que de "skin" au départ :) ». La hausse des dégâts
ennemis n'est donc pas activée. `proposal.json` conserve le candidat historique.

STARTER-SKIN-02 applique ce choix aux cinq tenues de départ : protection 0,
malus d'esquive 0, mêmes identités, images et palettes. Le +1 d'armure propre
à la classe Bastion est conservé. La génération 164 utilise ces vêtements
cosmétiques ; les campagnes antérieures reconstruisent leurs anciennes valeurs
de protection et leur ancien malus d'esquive. Les autres armures ne changent pas.
`starter-skin-02.json` mesure les attaques ennemies originales avec les deux
configurations de défense, sans modification de la précision ni de l'IA.

Le lance-flammes et les flammes génériques passent encore par un raster de
16 × 16 dans `terminal_elements.rs`. FIRE-NATIVE-01 compare ce dessin avec
l'atlas détaillé accepté `skill-animation-review-06/flame-atlas.png`, dont les
images de 128 × 128 sont réduites dans les cases de 64 px. Les quatre directions
et l'emprise sont calculées depuis l'attaque réelle du lance-flammes. Ce candidat
est accepté explicitement : « Valider les flammes détaillées ». L'activation
normale utilise ce rendu pour la famille Flame (lance-flammes et flammes
génériques), sans changer les autres familles d'effets, la portée ni les dégâts.

## Ville activée

Le troisième point autorise l'activation des cinq groupes déjà essayés, sous
U01-POP-01 : quinze monstres et six cases de rayon libre à l'arrivée. La
génération 163 les place à la première matérialisation de la rue normale. Les
retours et sauvegardes conservent les acteurs au lieu de les recréer. Le test
interactif initial commençait au portail sud de cette rue,
avant les premiers groupes, avec la trappe des égouts accessible en X 2, Y 14.
Le dessin des gardes et leurs futures attaques restent des travaux distincts.

Les sources et l'accord U01-POP-01 d'origine restent inchangés. Les campagnes de
génération 162 ou antérieure gardent leur peuplement historique. Les aperçus sont
dans `artifacts/combat-fixes-01/`, les résultats et références utiles dans cette
revue. Les dégâts ennemis restent ceux d'origine ; l'accord visuel active
séparément les flammes détaillées.

## Dégâts du joueur : audit OFFENSE-START-02

Le joueur signale ensuite des dégâts trop élevés face aux PV des monstres,
particulièrement pour les compétences actives. L'audit utilise les armes et
attributs réels des cinq classes au niveau 1, sans équipement de démonstration.
Les attaques sont exécutées sur une cible inerte de 1 000 PV sans armure,
avec esquive désactivée pour observer les dégâts sans les masquer par une
esquive ou une mort prématurée. Les commandes, variances d'arme et bonus de
classe passent par le moteur normal ; 32 graines indépendantes sont observées.
Les montants observés ne constituent pas des bornes exhaustives garanties.

Les exemples supplémentaires non appris au départ sont signalés distinctement.
La mesure porte sur les dégâts appliqués pendant la commande, pas sur la somme
future des brûlures, les invocations ou une transformation complète. Le coût
observé d'énergie est net de l'éventuelle récupération pendant cette action.
Les données sont dans `offense-audit-02.json`. Ce diagnostic ne modifie ni les
dégâts du joueur, ni les PV ennemis, ni l'XP. Une correction chiffrée demande
encore une proposition et une validation séparées.

Résultats de départ, sans critique : Bastion 12 dégâts en attaque ordinaire et
17 avec Frappe de rupture ; Traqueur 6 au fusil ; Technomancien et Métamorphe
3 à l'arme de départ. L'Arcaniste n'inflige que 1 avec son couteau, mais Braise
applique 11 dégâts au cours de sa commande (impact et premier effet de brûlure
compris). Ces mesures montrent un décalage qui dépasse les seules compétences
actives, face aux profils ordinaires de 7–9 PV utilisés provisoirement.
Marquage balistique ne fait pas directement de dégâts : il prépare un tir
ultérieur, absent de cette mesure. Les critiques et les effets combinés
figurent dans les observations JSON sans être transformés en nouvelles règles.
