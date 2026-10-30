# CREATION-UI02-C — lecture compacte

Proposition native du 30 octobre 2026, à la demande du joueur de réduire les
images pour éviter le défilement vertical. Cette révision reste en aperçu ;
la version 02-B est désormais active dans le lancement normal et `Jouer.cmd`.

Les portraits passent à 48 px ; les icônes d'équipement et de compétences à
40 ou 48 px selon la hauteur disponible. Les cartes mesurent 56 ou 64 px.
Les sources graphiques sont inchangées. Le lecteur est plus large, son en-tête
moins haut et les espaces entre paragraphes sont réduits. Le texte des fiches
reste à 14 px, avec les mêmes descriptions et valeurs. Les fondus de 02-B et
le réglage de mouvement réduit sont conservés.

Le calcul du défilement utilise la dernière ligne dessinée, sans compter une
seconde fois l'espace prévu par le lecteur. La mention de molette apparaît
uniquement lorsqu'un défilement est réellement nécessaire.

Le diagnostic natif sélectionne les quatre classes puis chacune de leurs
deux cartes d'équipement et deux compétences, avec les vraies entrées de
clic et le rendu du moteur :

- **1280 × 800 :** les 16 fiches et toutes les cartes tiennent sans défilement.
- **960 × 540 :** toutes les cartes et 15 fiches tiennent sans défilement.
  La longue description de Mue prédatrice conserve un défilement de 208 px.
- L'empreinte de la partie, le tour et l'état du générateur aléatoire sont
  identiques avant et après les deux séries de contrôle.

Les captures actuelles sont dans
`artifacts/character-creation-review-02/current/compact-02C/` : `metamorphe/`,
`arcaniste-small/`, `audit-1280/` et `audit-960/`.
Les mesures et empreintes sont dans `PROPOSITION-02C.json`.
L'activation normale de 02-B est consignée dans `integration-02B.json`.

Les quatre dossiers d'une première passe de capture ont été envoyés dans la
Corbeille après refus de leur suppression définitive par le contrôle automatique.
La Corbeille n'a pas été vidée. Seule la dernière passe reste dans l'aperçu.
