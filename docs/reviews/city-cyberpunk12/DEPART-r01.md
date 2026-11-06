# Carte de départ · retouches r01

Le joueur préfère le décor de départ en jeu. Cette proposition repart de sa
carte et applique trois retouches indépendantes.

- **DEP-I01** reprend l’intérieur de la maison visitable choisi dans la revue
  [Conduites r02](CONDUITES-r02.md), à sa position exacte sous le toit.
- **DEP-B01** différencie la petite maison à l’est par un toit en zinc avec
  verrière basse et une façade d’atelier. Les deux maisons sont visibles
  ensemble dans le comparatif. La toiture et la façade ont leur propre couche.
- **DEP-M01** prolonge le mur du portail jusqu’au cadre est et masque les deux
  portes dessinées de la ruine. Les cases au sud de la ruine forment déjà une
  ligne bloquante dans la carte. Le raccord visuel suit cette ligne.

Les études ont été réalisées avec l’outil ImageGen intégré. Le
[manifeste](../../../assets/source-art/city-cyberpunk12/depart-r01/manifest.json)
associe chaque couche à son dessin source et à son prompt. Voir les prompts
[du toit](DEPART-r01-MAISON-EST-PROMPT.txt), [de la façade](DEPART-r01-FACADE-EST-PROMPT.txt)
et [du mur](DEPART-r01-MUR-PROMPT.txt).

Le joueur a prévalidé le comparatif à 64 px par case le 5 novembre 2026. Le
fond exact de la proposition est intégré dans la rue agrandie. La version de
la rue avec la trappe reprend les mêmes retouches dans leurs masques. Les émetteurs
`ruin-west` et `ruin-east` ont été retirés avec les anciennes portes et leur
masque de lentille. L’éclairage ambiant et les autres lumières utilisent les
mêmes données. La validation finale se fait en jeu. [Accord et empreintes](ACCORD-DEPART-r01.json)

Les contrôles du trajet camp–rue, des portes, du toit et de la population
ont réussi. Le scénario isolé dans le moteur a confirmé le rendu des retouches
sur la rue avec trappe, l’ouverture et la fermeture de la maison et le retour
au camp. Les parcours de capture en campagne peuplée se sont interrompus
avant la vérification visuelle complète.
