# Armes de tir, mains et couleurs individuelles

Les dessins industriels et palettes approuvés sont intégrés dans le client. Les trois prises de pistolet ont été reprises après le retour du joueur : gant plus étroit, doigts recourbés et séparés autour de la poignée, poignet sous la prise. Pistolet seul, deux pistolets et épée + pistolet disposent chacun de leurs 17 figures complètes. Les pistolets restent des références graphiques : aucun nouvel objet de pistolet ou de pistolet mitrailleur n'est ajouté au butin dans cette étape.

Les nouvelles parties de génération 145 attribuent à chaque nouvelle arme ou armure une des sept palettes : origine, rouge, bleu, violet, turquoise, jaune ou rose. La couleur ne crée ni nouveau modèle ni nouveau nom, et ne modifie ni statistiques ni rareté. Elle est tirée une seule fois par un flux cosmétique distinct du combat et des propriétés aléatoires. La tenue et les deux armes ont des couleurs indépendantes.

La couleur est conservée au ramassage, équipement, dépôt, amélioration, vente/rachat, transport par drone, voyage et sauvegarde/reprise. Les parties de génération 144 et antérieures gardent leurs règles et la couleur d'origine. L'instantané binaire passe à RLWS v12 ; les caches binaires antérieurs sont refusés avant décodage et restent couverts par le mécanisme de rejeu du client. Les anciennes données JSON sans palette restent lisibles.

Le torse continue de choisir une tenue complète de même hauteur, 30 pixels dans une case de 32 pixels. Le cadre de 48 pixels permet aux armes de dépasser sans changer les collisions. Les casques, gants et bottes conservent leurs illustrations et effets individuels dans l'inventaire ; leur couleur individuelle n'ajoute pas de calque au dessin porté.

Les 61 armes de tir gardent leurs modèles, identifiants, noms et statistiques. Huit dessins de famille remplacent leurs anciennes illustrations et icônes ; le fusil porté adopte le dessin industriel commun approuvé. Les dessins de mêlée et les poses sans arme sont conservés.

## Résultats visuels

- [Mains réellement embarquées : portrait, prise agrandie et corps natif](mains-pistolet-final.png).
- [Capture réelle du rendu et des palettes indépendantes](capture-palettes-final/cold-start.png).
- [Inventaire réel avec survol d'une armure équipée](capture-inventory-final/cold-start.png).
- [Carte et inspection d'une arme au sol](capture-world-final/cold-start.png).
- [Toutes les tenues et poses assemblées](tenues-assemblees.png).
- [Validation et limites](VALIDATION.md).

## Production et reproduction

Mode graphique : **imagegen intégré**, éditions avec références locales. Les prompts exacts sont dans [PROMPTS.md](PROMPTS.md) et [PROMPTS-PRISE-PISTOLET.md](PROMPTS-PRISE-PISTOLET.md). Les sources retenues sont `tenues-two-handed.png`, `tenues-raised.png`, `tenues-dual-raised.png` et `tenues-dual-mixed.png`. Les premières prises sont conservées dans les fichiers `*-prise-v1.png`.

`tools/art/build_player_outfits.py` conditionne les dessins par découpe, alpha et réduction nearest, sans dessiner les mains ou assembler des calques d'arme. `tools/art/build_ranged_families.py` conditionne les dessins de tir approuvés. Les deux scripts se vérifient avec `--check` ; Python n'est pas requis en jeu. La recoloration est un rendu de matériaux en Rust, avec textures mises en cache, sans changement de coordonnées ni d'alpha.

La source antérieure est copiée dans `avant/src` et `avant/cyberpunk`. `verify_preservation.py` compare les fichiers à cette étape ; il ne restaure ni ne modifie le code. Les anciens dossiers de propositions et les modifications utilisateur préexistantes restent conservés. Aucun commit, push ou déploiement.
