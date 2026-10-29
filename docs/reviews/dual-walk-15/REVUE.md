# DUAL15-r01 — 01, marche avec deux armes

28 octobre 2026. **01 validée** par « oui ca me semble bien mieux :) ».

Le joueur demande d'améliorer l'animation lorsqu'un personnage marche avec
deux armes actives. La méthode DUAL15 complète maintenant WALK12 / ARM14 pour
les deux armes à une main actives. Les dessins existants sont conservés.
Le comparatif historique reste un diagnostic Debug isolé, sans campagne
chargée ni préférence personnelle écrite.

## Problème et proposition

ARM14 utilise une pose de bras compacte fixe, translatée avec le torse. En
ambidextrie, les deux bras restent ainsi figés ensemble pendant que les jambes
alternent. Les deux armes et les deux gants sont aussi composés au-dessus du
corps. Les pistolets conservent l'orientation relevée de leur pose frontale.

01 sépare les deux assemblages épaule / bras / arme / gant. Le bras éloigné
passe derrière le torse, et le bras proche devant. Chaque assemblage reçoit
un petit mouvement opposé, synchronisé sur les six phases déjà acceptées.
Pour les pistolets, la main et l'arme sont d'abord orientées ensemble au
poignet, puis portées un peu plus bas pendant le pas. Le rendu réutilise les
pixels existants ; aucun nouveau dessin de personnage ou d'arme n'est généré.

Le corps, les six dessins de jambes, les protections et les casques restent
ceux de WALK12 / ARM14. À l'arrivée, le personnage reprend sa vraie pose
frontale. Une case dure toujours 400 ms par défaut ; les pauses de lecture
de l'aperçu ne sont pas des délais de jeu. Les prises à une arme, les armes
à deux mains et le mouvement vertical gardent leur rendu existant. Aucun
identifiant, équipement, bonus, collision, règle de combat ou de sauvegarde
n'est modifié.

## Comparaison conservée du rendu validé

[Aperçu courant](../../../artifacts/dual-walk-review-15/apercu.html).

- [Deux pistolets, natif64](../../../artifacts/dual-walk-review-15/r01/pistols-natif64.gif).
- [Deux armes de mêlée, natif64](../../../artifacts/dual-walk-review-15/r01/blades-natif64.gif).
- [Lame + pistolet, natif64](../../../artifacts/dual-walk-review-15/r01/mixed-natif64.gif).
- [Pistolet + lame, natif64](../../../artifacts/dual-walk-review-15/r01/mixed-reversed-natif64.gif).
- [Deux armures lourdes, natif64](../../../artifacts/dual-walk-review-15/r01/armures-lourdes-natif64.gif).
- [Détail des pistolets ×3](../../../artifacts/dual-walk-review-15/r01/pistols-detail-x3.gif).

Les quatre premières animations montrent les quatre classes jouables dans
leur tenue de départ, dans les deux directions. Le complément montre le
Bastion et la Traqueuse en armure lourde. Chaque ligne « actuel » emploie le
rendu antérieur ARM14, chaque ligne « 01 » le rendu DUAL15 validé. Les images proviennent du
moteur avec le traitement des acteurs et de l'éclairage actif. Aucun
redimensionnement des pixels du jeu n'est appliqué aux GIF natifs.

## Fabrication et contrôles

Le compositeur source contrôle neuf tenues : les cinq de la Traqueuse, les
Bastion de départ et lourd, puis l'Arcaniste et le Métamorphe de départ. Il
croise quatre paires d'armes, deux directions et six phases. Le diagnostic
du moteur couvre les quatre tenues de départ et les deux lourdes, avec ces
mêmes paires, dans deux conditions d'éclairage. Il vérifie aussi que le
rendu normal revient à l'octet près après chaque comparaison et que le
retour frontal est identique.

Les transformations sont rigides autour de l'épaule ou du poignet, sans
étirement des armes ni des bras. Le contrôle des contacts main / arme et
des pixels du corps complète la revue visuelle ; il ne la remplace pas.
Les sources existantes, paramètres exacts et résultats avant accord sont
consignés dans [validation.json](validation.json). L'accord et les références
exactes sont dans [acceptance.json](acceptance.json) et
`assets/source-art/dual-walk-15/accepted-r01/`. Le contrôle du rendu normal,
des contacts et des compilations est dans [integration.json](integration.json).

La même méthode d'attache s'applique aux tenues compatibles, sans nouveau
dessin. Les références compactes du bras et de la prise sont mises en cache
par tenue, direction et arme. Ce cache borné à 8 Mio évite de les recomposer
à chaque phase ; les dessins obtenus restent identiques à l'aperçu accepté.

```powershell
$env:RL_DUAL15_INTEGRATION_OUTPUT = 'artifacts/dual-walk-review-15/integration/source'
cargo test --locked --bin project-rl validate_dual15_integration -- --ignored --nocapture
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-dual-walk-integration artifacts/dual-walk-review-15/integration/engine
```

Le contrôle natif compare le rendu normal au rendu accepté, avec les deux
éclairages et le retour frontal. Le drapeau `--ui-cold-dual-walk` conserve
la comparaison historique ARM14 / DUAL15 si elle doit être reproduite.
Le générateur initial des références est figé après accord.

## Accord

[AGENTS.md](../../../AGENTS.md) demande de présenter les candidats numérotés
à taille native avant intégration et de faire valider explicitement les
visuels révisés. Le guide
[project-rl-lateral-walk-review](C:/Users/User/.codex/memories/skills/project-rl-lateral-walk-review/SKILL.md)
demande également : « Obtain explicit review before integration ».
L'accord reçu autorise cette méthode montrée. Les ressources et accords
WALK12 / ARM14 restent conservés ; les autres démarches et les règles de
jeu ne font pas partie de cette modification.

## Bilan de l’intégration

Le rendu normal reproduit exactement les 36 planches sources et les 48
planches natives du moteur conservées après accord. Les 39 modèles à une main
sont contrôlés dans les deux mains, les deux sens et les six phases sur les
25 tenues techniques : **23 400 contacts, aucun échec**. Les cinq tenues de
l’ancien Technomancien restent des ressources de compatibilité ; sa classe
n’est pas rétablie. Les 475 contrôles hors ambidextrie animée conservent
exactement les pixels antérieurs.

Les huit tests de rig et les quatre tests du lecteur de marche passent,
ainsi que le contrôle des casques. Un ancien contrôle de rigidité comparait
une manche au lieu de la couche d’arme et ignorait la translation du torse ;
il est corrigé et repasse séparément. Il ne s’agit pas d’une exécution de
la suite complète du jeu.

La compilation Release et le contrôle du diff passent. Les 300 PNG bruts
reproductibles sont déplacés dans la Corbeille, sans vidage ; les 84 planches
acceptées, les dessins sources, les GIF et les comparatifs natifs courants
restent conservés. Voir [cleanup-integration.json](cleanup-integration.json).
Pour republier la comparaison historique après ce rangement :

```powershell
.\target\debug\project-rl.exe --ui-cold-dual-walk artifacts/dual-walk-review-15/r01/engine
python tools/art/publish_dual_walk_review_15.py
```
