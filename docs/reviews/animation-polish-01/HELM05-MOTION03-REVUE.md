# HELM05-MOTION03 — proportions et netteté des têtes

Le joueur signale que la tête complète MOTION02 est trop grosse pour la
morphologie du personnage, puis qu'elle paraît plus floue que l'armure.
MOTION02 n'est donc pas accepté. Le principe de tête casquée complète est
conservé ; cette reprise concerne sa taille et son échantillonnage.

## Diagnostic

Le module était réduit une première fois à 17 px, puis redimensionné par
`helmets::place` avec un filtre bilinéaire et les facteurs de porteur/armure.
Sa hauteur effective allait d'environ 15,5 à 20,9 px. Le deuxième passage
interpolait aussi des positions de cou fractionnaires sur les côtés.
Les têtes peintes de face du pilote mesurent environ 12–13 px jusqu'au repère
de tête précédent ; ce repère ne comprend pas toute la zone de raccord de cou.
Le précédent choix de 17 px multiplié par la carrure amplifiait trop le volume.

## Proposition 03

Les mêmes dessins haute résolution de tête casquée sont conditionnés directement
à leur taille finale pour chacun des vingt couples personnage/armure.
La base de calibrage passe de 17 à 12,5 px, avec les facteurs de carrure et
d'armure précédents, arrondi au pixel entier, minimum 12 px. Le casque intégral
05 dispose d'un pixel supplémentaire de coque ; la gamme finale est 12–16 px.
Ces dimensions sont proposées pour la revue, sans prétendre qu'un calcul suffit
à valider les proportions artistiques.

Chaque vue subit une seule réduction depuis son dessin source, en alpha
prémultiplié. La sélection alpha finale est binaire au seuil 128 afin de ne
pas conserver une frange de détourage partiellement transparente. Aucun
filtre d'accentuation ni aucune repeinture RGB n'est appliqué.

Le moteur sélectionne ensuite la couche déjà dimensionnée et la copie pixel
pour pixel au cou arrondi, avec le même filtrage de texture au plus proche
que les corps. Il n'y a plus de mise à l'échelle ni d'interpolation de cette
couche pendant la préparation des phases. L'ensemble tête–casque reste une
seule pièce, la même dans les six phases d'une orientation ; l'ancienne tête
peinte dans le corps reste masquée comme dans MOTION02.

Les 400 vues natives sont dans cinq atlas fixes de 128 × 640 px, soit environ
1,6 Mio de RGBA décodé au maximum pour ces sources. La préparation des corps
et textures reste hors du dessin et bornée à un degré d'armure dans le diagnostic.
Les sources nouvelles sont dans
`assets/source-art/animation-polish-01/helmet-head-fit-r03/` et leur manifeste
dans `HELM05-MOTION03-SOURCES.json`. Les dessins de MOTION02 et les icônes
acceptées sont conservés sans modification.

## Revue et périmètre

L'aperçu courant oppose **02 trop grande** à **03 taille et netteté**, à taille
native et au zoom entier ×2, avec les mêmes corps et phases. Le détail ×4 suit
le casque et le degré d'armure sélectionnés. Le témoin MOTION02 est conservé
dans `current/motion03/before/` pour cette comparaison.

Le pilote reste limité aux quatre classes, cinq degrés d'armure, cinq casques,
quatre orientations et mitrailleuse 31 en couleur d'origine. Aucune intégration
au jeu normal, modification d'objet, de règle ou de sauvegarde n'est effectuée.
Le rendu et les proportions attendent l'avis du joueur.

## Reproduction

```powershell
python tools/art/prepare_helmet_native_fit.py
cargo test --locked --bin project-rl animation_polish_review::worn_helmets::tests:: -- --nocapture
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-helmet-worn artifacts/animation-polish-01/helmet-worn-r01/current
python tools/art/build_animation_polish_review.py
```

## Vérifications du 31 octobre

Les six tests portés passent. Le nouveau contrôle vérifie 160 placements
et compare les octets RGBA de chaque pixel avec la couche source : ni
interpolation ni création de couleurs au placement. Le contrôle de
remplacement de l’ancien casque peint passe toujours, ainsi que celui
du déplacement avec le torse et de la préservation des sources normales.
La compilation Debug et le contrôle du binaire Release passent ; la suite
complète du projet n’a pas été exécutée.

Le moteur produit 140 planches ; les 140 lignes témoins et les 2 800 extraits
contrôlés des bras/armes et du bas du corps restent identiques en RGB.
Les anciens dessins et icônes sont inchangés. Les 60 couches frontales et
latérales du modèle 02 conservent au moins un pixel de verre cyan dans la
bande de l’œil, hors du mât. Le navigateur vérifie les quatre vues, le détail
selon l’armure, les gabarits fins/lourds, la pause et le repos, sans erreur de
console. Ces vérifications ne remplacent pas l’avis sur les proportions et
la netteté. Voir `HELM05-MOTION03-VALIDATION.json`.
