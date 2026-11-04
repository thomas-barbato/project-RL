# LATERAL02-r01 — première reprise d’identité latérale

## Demande et périmètre

Le joueur constate que les figures latérales ne sont pas les mêmes que les
vues face/dos. La correction LATERAL01 rétablit la lecture des six phases,
mais conserve les anciens dessins WALK12 ; elle ne corrige pas leur écart
d’identité avec les vues VERT18.

Le 1er novembre 2026, « ok tu peux commencer » autorise cette première reprise.
Le candidat 01 concerne uniquement la Traqueuse en armure lourde : corps 5,
armure 101, profil `traqueuse-armor3`, tenue technique 13. Il propose six
figures de marche et un vrai repos par direction, gauche et droite. Les
figures sont entières. Le casque ancien n’est pas greffé sur la nouvelle tête.
Les références face/dos VERT18 sont conservées.

Cette revue porte sur l’identité, la tenue, les proportions et la marche,
sans arme. Le raccord avec les armes partagées et leurs prises, les autres
tenues et les autres classes ne sont pas couverts. Aucune règle ni statistique
d’équipement n’est modifiée. L’ancienne animation reste active dans le jeu.

## Préparation et vérification

- Source générée et prompt conservés ; une seule échelle commune, puis
  translations des figures entières. Aucun déplacement isolé de membre.
- Validation de six images distinctes et de six bas de corps distincts par
  direction, repos distinct, repère supérieur horizontal stable à moins de
  1,01 px et ligne d’appui commune. Ces contrôles ne valent pas validation
  esthétique ou anatomique.
- Compilation Debug verrouillée et capture native dans le moteur via
  `--ui-cold-lateral-identity` : 14 comparatifs, deux cycles de six images
  distinctes, camp accepté, lumière LIGHT01 Discret (gain joueur 1,18).
- Ressources préparées avant le dessin ; aucune lecture de fichier dans la
  fermeture de rendu. Le module de capture est réservé à la compilation Debug.
- L’aperçu conserve les pixels de la scène : seuls les titres sont ajoutés
  dans les 32 lignes réservées en haut. L’affichage principal reste natif ;
  le zoom facultatif est un agrandissement entier ×4.
- Vérification dans l’aperçu courant : chargement des deux directions,
  lecture/pause et sélection du repos distinct. Le panneau étroit dispose
  les quatre vues en deux rangées sans redimensionner les personnages.
  Aucun avertissement ni erreur relevé dans la console du navigateur.
- Syntaxes Python et JavaScript vérifiées ; format Rust ciblé et
  `git diff --check` réussis. Les fichiers VERT18 de référence n’ont aucun
  changement Git. Empreintes de source et références conservées dans
  `LATERAL02-PROPOSITION.json`.

## État

`awaiting_visual_review`, sans intégration au rendu normal. La validation de
la silhouette, du mouvement et des proportions reste à obtenir du joueur.
La règle de `AGENTS.md` impose : « Présenter les nouveaux candidats numérotés
à taille native avant leur intégration, sauf si le joueur a déjà autorisé
explicitement l'intégration de ces candidats. » L’accord pour commencer ne
valide pas encore les dessins nouvellement produits.

Sources : `assets/source-art/animation-polish-01/lateral-identity-02/`.
Preuves de rendu : `artifacts/animation-polish-01/lateral-identity-02/current/`.
Aperçu courant : `animation-polish-01/apercu.html?revision=LATERAL02-r01#identite-laterale`.
