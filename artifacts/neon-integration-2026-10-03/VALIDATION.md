# Validation de la refonte graphique — 3 octobre 2026

Le rendu dessiné est intégré au jeu Rust/Macroquad. Les acteurs, murs, serveurs,
terminaux et portes restent dans leurs cases existantes. Aucun gros objet,
tuyau, obstacle ni placement supplémentaire n'est introduit. Les portes
reprennent les montants et panneaux des captures de l'éditeur ; l'ouverture
laisse le centre dégagé. Les couleurs du monde restent indépendantes des
quatre thèmes persistants de l'interface.

Les fonds du HUD sont presque noirs. Le relief décoratif du HUD et le contour
extérieur de la vue ont été retirés : aucune barre ne traverse le titre du
secteur, l'objectif ou le dessus des PV. Les zones et commandes du HUD restent
celles du jeu. Les corps dessinés remplissent aussi les cases aux zooms
intermédiaires ; le filtrage reste en pixels nets.

## Vérifications

- `cargo fmt --all --check`, `cargo check --locked`, `cargo build --locked` et
  `git diff --check` réussissent.
- Tests ciblés finaux : 56 réussites, aucun échec (`final-targeted.log`).
  Ils couvrent les préférences, les anciens réglages, les quatre thèmes,
  application/confirmation/retour arrière, navigation et clics sans tour,
  alignement caméra/souris, perception/mémoire et silhouettes des portes.
- Suite complète : moteur 791 réussites ; client 556 réussites, 18 échecs,
  15 tests ignorés (`tests.log`).
- Les 18 tests en échec ont été rejoués sur une copie isolée de l'état précédent
  en remettant exactement les fichiers sauvegardés avant cette refonte.
  Les 18 échouent également avant la refonte (`baseline-failures.log`).
  Ces écarts concernent des fixtures de génération, commerce, reprise et
  ancien texte d'objectif. Ils n'ont pas été corrigés dans ce travail visuel.
- Avertissement de compilation préexistant : `TerminalView::wall_joins` inutilisé.

## Captures natives

- `final-game-verified/cold-start.png` : exécutable final, laboratoire,
  thème violet, préférence de zoom 32 px (la caméra adapte le zoom à sa zone).
- `game-violet-wide/cold-start.png` et `expedition-violet-wide/cold-start.png` :
  jeu normal en 1920 × 1080, zoom compact 20 px.
- `expedition-small/cold-start.png` : disposition compacte 960 × 540.
- `graphics-small/cold-start.png` : choix de thème dans Affichage, 960 × 540.
- `door-route-violet/cold-start.png` : sélection d'une vraie porte fermée
  dans une fixture native et vérification du clic sans consommation de tour.
- `door-blue-wide`, `door-violet-wide`, `theme-green-wide`, `theme-red-wide` :
  même scène de laboratoire avec les quatre thèmes. Les suffixes « door »
  de ces quatre diagnostics utilisent la fixture standard du laboratoire,
  et ne constituent pas des captures de porte.
- `scenery-palette/cold-start.png` : aperçu des objets existants via le rendu
  natif à 64, 32 et 24 px ; ce diagnostic ne place aucun objet dans le jeu.

Les contrôles automatiques des captures ci-dessus confirment la présence de
texte dans les zones prévues. La lecture visuelle vérifie la disposition et
les trois emplacements de traits signalés. Ce constat ne vaut pas certification
générale de contraste de tous les écrans du jeu.

Les captures utilisent des réglages et sauvegardes de diagnostic isolés.
Aucune préférence personnelle, sauvegarde de partie, collision, règle ou
portée de perception n'a été changée. Un réglage de zoom déjà enregistré est
conservé ; les nouvelles valeurs compactes 16 et 20 px sont accessibles dans
Affichage. `integration.patch` contient uniquement cette refonte, comparée aux
snapshots `before/`, afin de préserver les modifications déjà présentes.
Aucun commit ni push effectué.

## Échecs reproduits avant la refonte

- `ascii_app::equipment_commerce::tests::shops_cover_every_layer_with_white_stock_and_current_tier_gambles`
- `ascii_app::equipment_generation::tests::accessory_models_preserve_version_133`
- `ascii_app::equipment_generation::tests::bonus_rarity_preserves_version_134`
- `ascii_app::equipment_generation::tests::firearm_models_preserve_version_129`
- `ascii_app::equipment_generation::tests::improvement_preserves_version_135`
- `ascii_app::equipment_generation::tests::melee_models_preserve_version_130`
- `ascii_app::equipment_generation::tests::reinforced_armor_preserves_version_132`
- `ascii_app::equipment_generation::tests::sword_models_preserve_version_131`
- `ascii_app::equipment_generation::tests::version_106_keeps_its_original_merchants_and_can_resume`
- `ascii_app::equipment_generation::tests::version_107_keeps_its_original_equipment_and_can_resume`
- `ascii_app::surface_density::tests::version_108_surface_fingerprint`
- `ascii_app::tests::equipment_merchant_keyboard_mouse_scroll_and_hidden_gambles_work_together`
- `ascii_app::tests::installed_security_alarm_is_visible_and_survives_replay`
- `ascii_app::tests::player_can_deliver_the_requested_material_and_replay_the_intervention`
- `ascii_app::tests::stored_version_fifteen_zero_command_run_keeps_its_exact_fingerprints`
- `ascii_app::tests::suspension_replays_buy_sell_and_weighted_gamble`
- `ascii_app::tests::suspension_replays_the_complete_run_and_is_consumed_only_once`
- `ascii_app::tests::ux_objective_card_opens_journal_without_a_turn_or_world_click`
