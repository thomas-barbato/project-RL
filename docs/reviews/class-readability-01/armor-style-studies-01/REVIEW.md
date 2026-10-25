# ARMOR-STYLE-STUDIES01 — reprise de la direction visuelle

21 octobre 2026 — deux études, en attente de revue.

Le joueur n'apprécie finalement pas l'esthétique des armures de l'ensemble,
y compris celles du Bastion. Ce retour remet en revue la direction visuelle.
Les anciens accords et dessins restent conservés comme historique ; ils ne
valent pas validation de ces deux propositions.

## Ce qui est proposé

**01 · Vétéran industriel — BA-STYLE01-r01.** Cuirasse cuivre sombre,
épaule métallique asymétrique, pantalon renforcé et bottes usées. Les
matières et l'asymétrie remplacent l'accumulation de petites plaques.
La tête reste ouverte avec son augmentation et un bandeau technique ; la
source n'a pas produit la coiffe ouverte demandée dans le prompt.

**02 · Briseur cybernétique — BA-STYLE02-r01.** Protection ocre plus
anguleuse sur une combinaison sombre, épaules ardoise, avant-bras et tibias
marqués par de grandes masses. Coiffe ajustée et visage partiellement visible.

Les deux dessins sont générés entiers avec le tool intégré `image_gen`.
La référence CL01-r04 guide l'identité du Bastion, sans imposer son ancienne
tenue. Les prompts exacts et chemins de génération figurent dans `prompts.json`.
Les originaux et exports 64 × 64 sont conservés sous
`assets/source-art/class-readability-01/armor-style-studies-01/`.

## Comparaison et limites

La planche montre l'ancien Bastion 03 puis les candidats 01 et 02, à taille
native, sur fond sombre, clair et sur le même morceau de rue. Le zoom entier
×3 est séparé. L'ancien témoin garde ses pixels exacts.

Les dessins et matières changent entre les deux propositions. Le premier est
conditionné avec Lanczos et le second avec Nearest, sans étirement, en
conservant l'alpha d'origine. Cette comparaison ne permet donc pas d'attribuer
une préférence au seul filtrage. Le second est une proposition générée dans
un traitement pixelisé, pas un dessin de 64 px retouché manuellement pixel par pixel.

Ces essais recherchent le style d'une tenue intermédiaire de mêlée. Ils ne
constituent ni une gamme légère à lourde, ni des rigs d'équipement : tête,
corps et mains sont dessinés ensemble. Après sélection du style, préparer les
calques, masques de remplacement de tête, repères anatomiques et prises avec
les armes communes. L'ancre sol de revue est [32, 61] ; les raccords des bras
et casques ne sont pas encore mesurés ou validés.

La règle acceptée du torse fournissant la tête par défaut et du casque la
remplaçant reste applicable. Les variantes de couleur sont différées.
Le fond de rue est un montage sur une capture acceptée, sans simulation
nouvelle de l'éclairage. L'intégration moteur et sa capture restent à venir.

## Vérification

Deux images RGBA de 64 × 64, marges et pieds à y=61 contrôlés. Les 626 images
et manifestes d'acceptation antérieurs recensés dans `prior-images.json`
gardent leur empreinte. La galerie courante conserve les anciennes planches
dans une section repliée. Aucun dessin actif, règle ou code Rust n'est modifié.
Les contrôles de conditionnement figurent dans `checks.json` ; la vérification
du navigateur est enregistrée séparément dans `ui-checks.json`.
