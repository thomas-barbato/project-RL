# CLASS-ARMOR-INTEGRATION01 — armures des cinq classes

Le joueur accepte les 25 dessins et demande leur intégration, sous condition
de compatibilité avec toutes les armes. Les cinq accords exacts sont référencés
dans `source-checks.json` et dans le manifeste de production
`assets/sprites/class-armors-01/manifest.json`. Les acceptations figées ne sont
pas réécrites par le bilan d’intégration.

Chaque classe dispose de cinq tenues, de très légère à très lourde. L’objet de
torse sélectionne le même stade sur toutes les classes ; son dessin porté reste
propre à la classe. Sans torse, le premier stade fournit la tenue de base, sans
créer d’objet. Aucun identifiant de contenu, bonus, dégât ou format de sauvegarde
ne change. L’intitulé affiché de Traqueuse est féminisé ; son identifiant reste
`core:traqueur`.

| Modèle d’illustration de torse | Stade visuel |
| --- | --- |
| Aucun, 108, 112, 113, anciens 200–204 | 01 — Très légère |
| 110, 114 | 02 — Légère |
| 102, 105, 106, 111 | 03 — Intermédiaire |
| 101, 104, 107, 109 | 04 — Lourde |
| 103 | 05 — Très lourde |

Ce classement est visuel et ne change pas les protections ou conditions
d’équipement. Les objets historiques retirés restent reconnus dans les anciennes
sauvegardes ; ils ne sont pas réintroduits dans le butin.

Le torse fournit son apparence de tête par défaut. Un casque équipé la remplace,
puis son retrait restaure la tête de la tenue. Les six casques existants sont
réutilisés ; les deux casques ouverts montrent la tête découverte de référence
de la classe, y compris sur les tenues à masque fermé par défaut. Traqueuse et
Métamorphe conservent donc chacune leur propre visage sous un casque ouvert.
Ce lot ne crée aucun nouveau casque.

Les 113 dessins d’armes actuels restent communs aux classes. Les bras proviennent
des nouvelles sources acceptées ; les mains fermées et les doigts reprennent
les prises déjà peintes et acceptées FIN-01 et FIN-14 à FIN-17. La petite face
des phalanges de FIN-15 est isolée du même dessin de main. L’ordre de composition
est corps/bras/main, arme, doigts : aucune manche ou plaque ne repasse devant
l’arme. Le pistolet utilise une main fermée, alignée sur sa poignée.

Le retour suivant valide le reste mais refuse encore la position des pistolets.
La correction `pistol-raised-02/` replie l’avant-bras vers le haut : un ou deux
pistolets sont portés à hauteur de tête, canons vers le haut, avec une marge
autour du visage. Les autres configurations sont comparées aux pixels de ce
premier rendu pour garantir leur conservation exacte.

Les attaches des six épées verticales 88–93 sont ramenées sur leur poignée.
La hache 66 utilise l’axe de son manche à y=60 au lieu de l’ancien centre de
poing trois pixels plus bas. Les armes, palettes d’armes et règles de prise
restent inchangées. Les marteaux 97 et 98 restent à une main, comme demandé ;
les nouveaux marteaux à deux mains sont un lot de propositions séparé :
`docs/reviews/two-hand-hammers-01/`.

Les trois dessins de ce lot sont ensuite validés par le joueur. L’accord visuel
est conservé dans son `acceptance.json` ; aucun nouveau texte ou chiffre d’objet
n’est ajouté par la correction de position des pistolets.

Les couleurs originales des 25 tenues sont conservées. Les variantes de couleur
des nouvelles armures restent reportées à la demande du joueur. Les palettes
existantes des icônes et des armes ne sont pas supprimées.

Les dessins natifs de 64 × 64 sont placés sans agrandissement dans une toile
transparente de composition de 128 px, afin de garder les débordements d’armes.
Les mêmes couches servent au plateau, aux portraits et au menu de classe.
Les monstres et PNJ ne sont pas modifiés par cette sélection de tenue.

## Reproduction des contrôles

```powershell
python tools/art/build_class_armor_assets_01.py
cargo test --locked --bin project-rl game_art:: -- --quiet
cargo test --locked --bin project-rl class_start -- --quiet
cargo test --locked --lib -- --quiet
$env:RL_CLASS_ARMOR_OUTPUT = Join-Path (Get-Location) 'artifacts/class-armor-integration-01/compositor'
cargo test --locked --bin project-rl class_armor_native_diagnostic -- --ignored --nocapture
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-class-armor-integration artifacts/class-armor-integration-01/gpu
python tools/art/verify_class_armor_integration_01.py
```

La vérification Python redresse les exports CPU une seule fois et compare les
25 bases embarquées aux pixels acceptés. La planche interactive permet de choisir
chacune des cinq classes et chacune des 113 armes, avec cinq stades à taille
native, plus les paires et casques de contrôle. Elle reste un aperçu reproductible
sous `artifacts/` ; le jeu n’en dépend pas.

`validation.json` conserve les résultats réels, la portée des tests et les limites
de validation. La couverture automatique des attaches ne vaut pas une validation
artistique humaine de chacune des milliers de combinaisons.
