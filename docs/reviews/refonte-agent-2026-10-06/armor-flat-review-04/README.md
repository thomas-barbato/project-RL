# ARMOR-FLAT04

Revue des raccords de couches, préparée le 10 octobre 2026. Les six tenues
conservent leurs sources peintes, les trois palettes de la revue précédente
et les seize configurations du gabarit frontal validé.

Le nom **Tablier de cuir** est explicitement demandé par le joueur pour
`AR-08` / `core:patched_plating` et appliqué à la traduction française active.
Les corrections d'assemblage restent des propositions visuelles à valider.

Les lignes et triangles gris provenaient de fentes transparentes au raccord
corps/bras. Le nouveau recalage local des poignets est nul aux frontières des
couches. Les masques du corps et des bras utilisent le même espace de
coordonnées ; le tablier ne participe pas à la couche mobile des gants.

## Vérification

- 288 compositions avec doublement exact, sans recadrage des armes.
- 468 vérifications de recouvrement gants/armes.
- 1 608 sondes de continuité aux frontières des couches.
- Tête et bas du corps identiques entre poses ; alpha conservé entre palettes.
- Points d'attache et transformations des armes inchangés.
- Les 1 552 fichiers figés de ARMOR-FLAT02 et huit atlas actifs sont préservés.
- Les zones de perte d'alpha de l'ancien assemblage ont été contrôlées sur le
  corps fixe, en complément des contrôles de couleurs antérieurs.

Ces contrôles portent sur le prototype visuel. Ils ne constituent pas une
validation de la compilation complète du jeu, dont le code n'a pas été modifié
pour cette correction.

`audit/overview.png` compare avant/après. `audit/equipment-64-V*.png` montre
chaque configuration en agrandissement exact. Les captures `browser-*.png`
montrent les images réellement affichées sur fond clair dans le navigateur.

Reproduction : lancer `build_armor_flat_review_04.py`, puis
`prepare_armor_flat_gallery_04.py` depuis le dossier parent.
Les anciens prototypes `prototype_armor_seams_04.py` et
`prototype_armor_continuous_04.py` sont des essais de diagnostic ; le module
retenu est `armor_continuous_registration.py`.
