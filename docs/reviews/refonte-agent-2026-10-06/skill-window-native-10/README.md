# SKILLS-NATIVE10 — Froid validé

Le 11 octobre 2026, le joueur valide IC31 : « je valide on continue ».
Les cinq compactes exactes sont intégrées sans retouche :

- 54 — FR01-C01 — Aiguille de givre
- 55 — FR02-C01 — Nova de givre
- 56 — FR04-C01 — Vague de froid
- 57 — FR05-C03 — Blizzard
- 58 — FR06-C02 — Explosion cristalline

Accord figé dans `../skill-icon-review-31/visual-acceptance-01.json`.
Mur de glace et les illustrations précédentes sont conservés. La banque
compte 56 compactes et 16 illustrations, soit 72 dessins.

Contrôles : 565 empreintes préexistantes inchangées ; 25 exports et cinq
masters approuvés vérifiés ; les cinq nouveaux PNG actifs correspondent
exactement aux exports approuvés de 64 px. Les 94 fiches, leurs textes,
paramètres et règles sont conservés.

Les trois tests ciblés de `agent_skill_window::tests`, la compilation debug,
le formatage et le contrôle du diff passent. Le compteur du test du catalogue
est actualisé à 56 compactes. Les cinq captures réelles sont produites en
960 × 540 px avec code 0. La comparaison des icônes rendues reste sous la
tolérance GPU de 15/255 : écart moyen maximal de 11,329/255.

`index.html?skill=FR05` présente le rendu réel de Blizzard. Les nouveaux
effets de combat ne sont pas activés par cette intégration visuelle.
