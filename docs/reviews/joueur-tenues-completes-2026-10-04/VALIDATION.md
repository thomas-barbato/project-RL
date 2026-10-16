# Tenues complètes du joueur — 4 octobre 2026

## Résultat intégré localement

Une armure de torse choisit désormais un personnage complet : tête, corps, bras, mains, jambes et bottes. Les trois classes sans torse équipé portent déjà un casque. Les autres pièces conservent leurs effets, leurs slots, leurs illustrations et leurs tooltips ; elles ne remplacent plus une partie indépendante du dessin du joueur.

Les 14 modèles de torse existants ont trois poses : mêlée, tir à une main, tir à deux mains. Les couleurs incluent ivoire, bronze, violet, bleu, bleu nuit, brun, rouille, olive, rouge et ocre. Les coiffes ont des constructions différentes : casques ouverts/fermés, capuches, masque à gaz, masque de soudure, casque de chantier avec lampe, bonnet d'aviateur et turban. Les robes et tenues techniques interprètent les torses existants.

Selon la dernière décision de l'utilisateur, le joueur utilise seulement trois dessins d'armes portées : épée, pistolet et fusil. Tous les objets d'une catégorie partagent son dessin. Les noms, statistiques et illustrations d'inventaire restent individuels. Les trois dessins sont préparés une seule fois ; le rendu ne prépare plus 101 modèles. La pose à une main est contrôlée par le pistolet de référence, car le catalogue actuel n'a pas de pistolet jouable.

Les mains appartiennent directement aux figures. Le calque de premier plan recopie leurs pixels pour que les doigts couvrent la poignée, sans déplacer des mains indépendantes d'un vêtement. Les repères de prise sont relevés sur chaque dessin et l'arme suit ces repères. Le cadrage du fusil a été élargi pour conserver sa lecture entre les mains.

## Sources et prompts

Mode de production : **imagegen intégré**, puis cadrage, nettoyage alpha, séparation des doigts et rangement en atlas avec Pillow. Aucun dessin vectoriel de remplacement.

Sources finales conservées dans ce dossier :

- classes-casquees-poses.png : trois classes avec casque, trois poses.
- tenues-side-final.png : 14 tenues, pose de mêlée.
- tenues-two-handed-final.png : 14 tenues, pose de fusil.
- tenues-raised-final.png : 14 tenues, pose de tir à une main.

Prompts exacts : PROMPTS.md, PROMPTS-PALETTES.md et PROMPTS-TETES.md. Les variantes intermédiaires sont conservées pour retracer les corrections de bras, couleurs et têtes.

Assets chargés par le client : assets/sprites/cyberpunk/player-outfits-{side,two-handed,raised}-{32,128}.png et player-fingers-{side,two-handed,raised}-{32,128}.png. Repères et correspondances : player-outfit-grips.rs et player-outfits.json. Le code du client ne charge plus les anciennes banques de pièces portées, qui restent archivées.

Reproduction : python tools/art/build_player_outfits.py ; contrôle avec --check.

## Vérifications

- 10 tests graphiques ciblés passent : choix de la tenue par le torse, indépendance des autres pièces et de la classe, partage des dessins d'armes par catégorie, doigts issus des mêmes figures, prises connectées, fusil contenu dans la tuile et sens du pistolet.
- 2 tests player_art passent : arme active et déplacements clavier/souris sans changement des règles.
- cargo check --locked --all-targets, cargo build --locked et cargo build --locked --release réussissent.
- rustfmt --check sur les modules de rendu modifiés et git diff --check passent.
- build_player_outfits.py --check et build_cyberpunk_assets.py --check confirment les assets reproductibles.
- Captures natives en 1280 × 800 : player-outfits-review, combat-review, terminal-hud-live et terminal-inventory-live-equipped. Les sorties et les images ont été vérifiées.
- La fonction de fil de fer reste identique. Les pixels RGB du panneau d'inventaire (32, 30, 1248, 770) restent identiques à l'inventaire approuvé ; preuve dans preservation.json.

La suite complète du client et les 806 tests du moteur n'ont pas été relancés : cette validation est ciblée sur le rendu du joueur. Les avertissements de compilation sans rapport avec cette modification restent présents.

## Noms

NOMS-PROPOSES.md contient une proposition de noms adaptés aux silhouettes. Elle n'est pas appliquée aux textes du jeu. Aucun identifiant de contenu, effet d'équipement, fichier de sauvegarde, PNJ ou monstre n'est changé par cette étape. Travail local, sans commit ni publication.


## Mesure de rendu

Diagnostic debug --ui-cold-performance terminé, 120 images par scène sur 10 scènes, après les compilations.

- world : première image CPU 354.43 ms ; médiane CPU 8.61 ms.
- inventory : première image CPU 24.33 ms ; médiane CPU 11.70 ms.

Données complètes : performance/timings.json. Ce diagnostic du client debug ne constitue pas un benchmark contrôlé ni une mesure de FPS de la version release.
