# LATERAL01 — restauration de la marche acceptée

Le signalement du joueur est reproduit sur les quatre classes : les six vues
latérales du précédent aperçu donnaient une seule image distincte par côté.
Le changement provenait de `approved_walking_layers` : une pose explicite
WALK12/ARM14 passait par `render_layers`, qui consulte l'ancien contexte
WALK04 resté volontairement désactivé. La composition revenait donc au dessin
frontal statique, différent des figures latérales acceptées et du repos VERT18.

Le correctif transmet directement la pose au cycle accepté. Il partage ensuite
la seule finition d'ombre, avec le vrai index de la couche d'arme après les
manches séparées. Les six figures entières, leurs têtes, tenues et prises restent
celles de WALK12/ARM14. Les paramètres des fusils 02, les vues verticales, le
repos frontal, les durées et les règles de déplacement ne sont pas modifiés.
WALK04 reste désactivé ; aucun dessin nouveau n'est créé.

## Vérifications

- Test normal sans activation d'un contexte de revue : quatre classes × cinq
  armures × cinq équipements × deux directions, soit 200 cycles. Chaque cycle
  possède six images distinctes et des jambes qui changent effectivement.
- Le rendu normal du moteur est comparé à 896 compositions préparées. Les
  64 cycles latéraux contrôlés ont six phases distinctes, y compris dans la
  région des jambes. Les 512 planches de face, de dos et de repos sont identiques
  aux précédentes ; les 384 planches de marche latérale remplacent l'image fixe.
- Les 32 retours au repos, huit portraits et 18 contrôles d'autres acteurs ou
  objets passent. Les 427 vérifications de sources conservent le périmètre RF02.
- Deux vrais déplacements d'une case dans l'application, avec fusil d'assaut
  et durée de 400 ms, produisent les phases 0–2 vers la droite puis 3–5 vers la
  gauche. Le retour frontal et l'absence de mutation de simulation par le
  dessin sont vérifiés. Sept captures complètes sont conservées ; les captures
  devenues 1 × 1 pendant la réduction de la fenêtre sont explicitement exclues.
- Les comparaisons du camp utilisent désormais une cible graphique fixe de
  640 × 610, avec les vrais matériaux et la passe d'éclairage. La réduction de
  la fenêtre ne peut plus transformer cette preuve en une image vide.

Les anciennes égalités de pixels de FIREARM02 comparaient deux chemins utilisant
le même montage fautif. Elles prouvaient leur égalité, pas l'existence de six
phases de marche. Le présent contrôle de variation s'ajoute à la comparaison.

Preuves dans `artifacts/animation-polish-01/lateral-regression-r01/` :
`before.json`, `before-frozen.png`, `rifles/checks.json`,
`rifles/restoration-parity.json` et `live/checks.json`.
Les aperçus de fusils, armes lourdes et éclairage sont recapturés avec ce montage.
La suite complète n'est pas relancée. Aucune campagne utilisateur n'est chargée.

La construction Release et le vrai `Jouer.cmd --verify-assets` passent depuis
un autre dossier, avec le contrôle négatif attendu sur des données absentes.
Les quatre JSON utilisateur et le manifeste sont inchangés. Les 2 576 planches
des trois comparatifs courants sont actualisées ; leurs images sont servies
avec une nouvelle version de cache pour éviter la réutilisation des vues figées.
Une animation de preuve montre les quatre classes en 64 px avec leur repos,
leur marche gauche/droite et leur dos : `marche-corrigee-native.webp`.
Formatage Rust, scripts Python, JavaScript publié, JSON et `git diff --check`
sont vérifiés. Les sorties redondantes de capture sont retirées ; les rapports
et une planche représentative par groupe restent conservés.
