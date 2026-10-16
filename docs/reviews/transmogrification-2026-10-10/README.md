# EQ-TRANSMOG01

Slot de transmogrification demandé puis validé le 10 octobre 2026.

Le torse de ce slot sélectionne uniquement la tenue visible et sa palette.
L'objet réel continue à définir l'armure et les bonus. Les armes et leur pose
proviennent toujours des emplacements actifs.

Depuis l'inventaire, sélectionner un torse puis cliquer sur le slot placé en
haut à droite, ou sur l'action **Transmogrification**. La croix retire le choix.
Réappliquer le même objet retire également son apparence. Le choix ne crée pas
d'objet supplémentaire : il référence une instance possédée et reste sauvegardé.

Les graphismes actifs sont réutilisés. Les nouvelles propositions d'armures et
leurs corrections de raccords restent soumises à leur revue séparée.

Les scènes de capture isolées sont `--ui-cold-terminal-inventory-live-transmog`
et `--ui-cold-terminal-inventory-live-transmog-empty`. Ajouter `-small` ou
`-largeui` permet de vérifier les tailles compactes et l'échelle d'interface.
Le diagnostic ne charge ni ne remplace les sauvegardes du joueur.

## Vérification

- Formatage, `cargo check --locked --all-targets`, build debug et contrôle des
  espaces Git : réussis.
- Trois tests moteur et un test d'interface de transmogrification : réussis.
  Ils couvrent l'absence de bonus mécaniques, la conservation de l'équipement
  réel et des armes actives, la sauvegarde, les refus atomiques, le dépôt de
  l'objet et la conservation de la sélection dans la liste.
- Test des bornes et des clics des emplacements, en fenêtre compacte et à
  grande échelle d'interface : réussi.
- Quatre captures réelles : slot vide, slot rempli, 960 × 540 et interface à
  150 %. Les rapports `terminal-validation.json` indiquent tous
  `state_unchanged: true`. Le torse réel reste l'instance 15, tandis que
  l'apparence utilise l'instance 19. Les images `world.png` montrent le
  changement de tenue sur le plateau, avec les mêmes défenses, PV et énergie.

La suite complète a terminé : 821 tests moteur réussis ; côté client,
624 réussis, 28 échoués et 15 ignorés. Les résultats sont dans `tests.log`.
27 échecs correspondent aux scénarios déjà répertoriés dans
`docs/reviews/joueur-quatre-prises-2026-10-05/failed-client-tests.txt`.
L'autre concerne `ux_resource_gauges_follow_heat_bandwidth_and_energy_after_real_actions` :
le test exige un bas de jauge inférieur à 69 alors que la géométrie actuelle
de `src/hud_resources.rs` place ce bas à 73. Il échoue aussi isolément ; cette
géométrie n'a pas été modifiée pour la transmogrification.
Les tests ciblés ont été relancés avec succès après les corrections de
sélection et d'infobulle compacte. La suite globale n'est pas entièrement verte.
