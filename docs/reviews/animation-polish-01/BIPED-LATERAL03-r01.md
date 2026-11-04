# BIPED-LATERAL03-r01 — ailes et prises à deux mains

Après la revue BIPED-LATERAL02-r01, le joueur répond « ok on peut continuer ».
Le lot précédent (MON-005, 014, 156) est figé par son accord séparé.
Le démon MON-144-r03 conserve également ses sources et son atlas acceptés.

Ce nouveau lot prépare trois silhouettes de la banque courante :

- 01 MON-145-0, ailé d’obsidienne : corps bleu-gris élancé, deux bras,
  deux appuis, deux ailes portées et une queue. Conserver sa masse propre.
- 02 MON-164-0, cultiste au fusil : capuche rouge et tabard fendu ; arme
  d’origine à deux prises, crosse, canon et avant-bras continus.
- 03 MON-165-0, cultiste à la hache : même identité que son frontal courant,
  lame et hampe d’origine rigides, deux prises distinctes sur le manche.

Les huit poses du Bastion CLASS-LATERAL02-r02 fournissent la même succession
contact, appui, passage et repose. Les figures sont redessinées entières,
avec leur anatomie et leurs matériaux propres. Sur les deux cultistes armés,
le port à deux mains remplace le balancement des bras de la référence ;
les deux mains restent attachées à la même arme. Aucun morceau de jambe,
main ou tête n’est déplacé seul ; le conditionnement aligne et réduit la
figure complète avec filtrage nearest. Pas de miroir d’une figure asymétrique.

La cadence reste **400 ms par pas, 800 ms par cycle**. Un vrai frontal courant
est affiché à l’arrêt. La pause de revue de 400 ms ne constitue pas une règle
de jeu. Les deux orientations sont disponibles séparément et en aller-retour.
Le camp, les cases de 64 px et le traitement lumineux courant de chaque acteur
restent communs au comparatif natif. Le joueur de référence garde sa taille.

04–07 proposent les quatre monstres déjà acceptés comme repères, avec les
atlases d’origine réutilisés sans modification et leurs empreintes contrôlées.
Le comparatif propose joueur/monstre synchronisés, frontal actuel/nouvelle
marche et trois colonnes. Les captures de la sélection sont préparées avant
lecture ; le lecteur conserve au plus deux banques de 66 images.

Les silhouettes 01–03 et les nouvelles prises restent à examiner à taille
native et zoom entier. Les armes du joueur, les autres palettes, les autres
monstres, la marche verticale et l’intégration normale sont hors de ce lot.
Les identités, règles de jeu, sauvegardes et préférences sont conservées.

Sources : `assets/source-art/animation-polish-01/biped-lateral-03/r01/`.
Génération intégrée image_gen, prompts conservés dans
`BIPED-LATERAL03-PROMPTS-r01.md`. Reproduction :
`tools/art/prepare_biped_armed03.py`,
`--ui-cold-biped-armedlot OUTPUT`,
`tools/art/build_biped_armed03_review.py`,
`tools/art/check_biped_armed03.py`.

Vérifications : compilation Debug réussie, 462 captures du moteur et
448 correspondances de phase joueur/monstre contrôlées. Les 12 empreintes
des sources/atlas acceptés correspondent aux accords ; les 264 captures des
quatre repères restent identiques pixel par pixel dans la région des acteurs.
Chaque aller-retour WebP dure exactement 2 400 ms (deux cycles de 800 ms et
deux pauses de revue de 400 ms). Contrôles Python, rustfmt ciblé et diff réussis.

Le navigateur charge les sept banques de 66 images. Affichage natif
320 × 192, trois colonnes 480 × 192 et zoom entier 960 × 384 contrôlés.
Repos frontal et passage du pied vérifiés manuellement ; lecture active
à 400 ms et aucune erreur console observée. Preuves conservées sous
`artifacts/animation-polish-01/biped-lateral-03/r01/` dans
`native-validation.json`, `browser-checks.json` et `browser-proof.png`.
Ces contrôles techniques ne remplacent pas le jugement visuel sur les prises.
