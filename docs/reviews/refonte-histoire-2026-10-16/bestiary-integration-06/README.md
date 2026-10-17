# Intégration du bestiaire — 17 octobre 2026

Autorisation et limites : `integration.json`. Les visuels et couleurs sont
intégrés aux nouvelles parties de génération 150. Les anciennes générations
conservent leurs identités et leurs règles ; aucun champ n'est ajouté aux acteurs
ou aux résidus sauvegardés. Les tags existants conservent modèle, palette et MM21.

La banque contient 129 dessins et 240 variantes natives, plus quatre palettes
de matériaux. Treize dessins de lanceurs de sorts sont stockés en réserve ; leurs
pouvoirs et comportements ne sont pas inventés. Les 116 autres dessins utilisent
les rôles de combat déjà présents : cultistes en surface, possédés plus bas,
démons dans les profondeurs. Aucun changement de PV, dégâts, butin ou durée
d'action ne découle du dessin. Les cultistes à deux mains 164 et 165 ont été
explicitement validés à taille native et suivent la prise de l'arme portée.

MM21 ajoute 30 points de résistance physique, plafonnés à 75 %, après l'armure,
uniquement sur les composantes physiques. Les esprits utilisent ce trait fixe.
Les mots-clés affichés proviennent des attaques et défenses réelles, sans doublon.
Les autres pouvoirs chiffrés restent en revue. Une palette ne confère pas encore
un pouvoir élémentaire non validé. Les traits apparaissent dans Examiner avec
leurs explications au survol. La vie au survol d'un adversaire visible est
activable dans les options graphiques.

Les corps au sol reprennent le dessin et la palette de l'adversaire, selon la
convention actuelle de corps tourné, avec ombre séparée et butin au-dessus. Il
ne s'agit pas d'une nouvelle banque de dessins de cadavres. Les empreintes de
source et chaque pixel opaque de la planche native sont contrôlés par le
validateur. Les originaux sont dans `assets/source-art/`, les atlas du jeu dans
`assets/sprites/campaign-monsters/` ; le jeu ne charge rien depuis `artifacts/`.

Les cinq modèles refusés sont exclus : 13, 23, 27, 91 et 163. Les boss de secteur
18 et 19 restent en revue. `cleanup-plan.json` conserve les correspondances des
anciens exports vers leurs copies durables avant retrait des sorties inutiles.
Les cartes de `artifacts/map-editor/`, les sauvegardes et les anciens assets
nécessaires aux parties historiques sont conservés.

Les résultats effectifs des contrôles et les limites des captures sont dans
`validation.json`, `asset-validation.json` et `capture-checks.json`.
