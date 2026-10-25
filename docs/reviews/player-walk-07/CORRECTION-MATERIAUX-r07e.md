# WALK07-C11-r07e — continuité de la jambe de la Traqueuse

**Étape historique, remplacée par
[WALK07-C11-r07f](CORRECTION-MATERIAUX-r07f.md).** Le joueur demande ensuite
les deux jambes en métal. L'exigence d'un seul mollet métallique ci-dessous
ne s'applique donc plus au candidat courant.

Le 24 octobre 2026, le joueur indique que la démarche semble « ok », mais que
la jambe armurée change encore. Il rejette ensuite les deux planches r07c :
« non le probleme est toujours là .. (alternance de jambe armurée) ».
Cet avis conserve la cible de mouvement ; il ne valide ni les matériaux ni
une nouvelle intégration. Le candidat courant reste à examiner.

## Défaut et correction ciblée

r07c avait attribué une jambe différente aux deux groupes temporels de trois
images. Cette hypothèse était fausse : les positions des cuisses et des genoux
n'avaient pas changé de propriétaire. Déplacer seulement le métal au pas
suivant créait précisément le défaut signalé. Ce classement temporel est
retiré ; une moitié du cycle ne constitue pas un identifiant anatomique.

La rangée gauche r07b est reprise sans modification. Dans la vue droite, le
mollet métallique reste sur la jambe arrière issue du même côté du bassin,
au repos puis dans les six poses ; le mollet avant reste en tissu. Il n'y a
plus de changement de matériau imposé entre les images3 et4. La cible reste
la marche latérale entière du Bastion original, sans reprise des déplacements
verticaux ni du Métamorphe.

La retouche d'une rangée complète r07d n'avait pas respecté cette consigne dans
les premières images. Les poses1 et2 ont donc été retouchées individuellement,
comme **figures entières**. La pose3 complète correcte de cette rangée est
retenue, ainsi que les poses4–6 complètes déjà correctes. Le conditionnement
extrait, réduit uniformément et place chaque figure entière ; il ne colle ni
jambe, ni tête, ni plaque isolée. Les poses de repos et les points de contact
restent distincts des phases de marche. La revue doit encore confirmer leur
enchaînement et les petits écarts de dessin introduits par la retouche.

## Dessins sources et consignes

Outil utilisé : **image_gen intégré**, sans CLI de génération.
Les sources sélectionnées du candidat sont dans
`assets/source-art/player-walk-review-07/` :

- `BASTION-TRANSFER-10-r07b-left.png`, rangée gauche conservée.
- `BASTION-TRANSFER-10-r07e-right.png`, rangée droite conditionnée.
- `CLASS-10-r07e-right-pose-1.png` et `CLASS-10-r07e-right-pose-2.png`,
  retouches individuelles entières conservées comme sources.

`prosthesis-sources-r07e.json` consigne les fichiers et colonnes exacts retenus,
les facteurs d'échelle de figures entières et l'empreinte de la rangée finale.
Les prompts complets sont
`prompts/CLASS-10-prosthesis-r07e-right-1.txt`,
`prompts/CLASS-10-prosthesis-r07e-right-2.txt` et
`prompts/CLASS-10-prosthesis-r07d-right.txt`.
Les entrées et guides individuels sont dans `prosthesis-jobs-r07e.json` et
`references/CLASS-10-r07e-right-*`. Deux sorties à halo ont été écartées ;
elles ne sont pas utilisées dans la composition. r07c n'est pas retenu comme
cycle complet et sa règle d'alternance du matériau est rejetée.

## Contrôles et limite de validation

Les sept poses de chaque direction ont été examinées sur les planches entières
et avec le fusil à taille native. Le contrôle de matériau porte désormais sur
les **deux directions et le repos** : une zone de mollet identifiée reste
métallique, tandis que la zone de l'autre mollet reste sombre. Il ne déduit
jamais le côté anatomique du numéro de phase. Ces zones sont propres à ces
quatorze dessins ; elles ne constituent pas un détecteur universel d'anatomie.

Les exports Rust de recettes et de pixels passent pour les117 configurations
d'équipement de la Traqueuse. Le contrôle compare3276 corps source et
compositions des deux pilotes, protège les jambes de la découpe des bras et
vérifie les empreintes inchangées des cinq atlas WALK06 actifs. Les armes
gardent leurs dessins réels et leurs transformations rigides. Les comparaisons
du rendu JavaScript utilisent un DOM/canvas simulé et les fixtures exportées.
Les détails courants sont dans `bastion-transfer-checks-r07.json`,
`raster-audit.json` et `renderer-checks.json`.

Seul l'aperçu de la Traqueuse est actualisé. La Métamorphe, les autres tenues,
les monstres et les ressources du jeu restent inchangés par cette correction.
La tentative d'ouverture automatique du fichier local avait été refusée par
la politique d'accès du navigateur. Aucun contournement n'est utilisé ; cette
limite ne signifie pas que le navigateur du joueur est globalement bloqué.
Aucune exécution réelle de ce candidat en navigateur ou dans le jeu n'est
revendiquée. La revue visuelle et l'autorisation d'intégration restent ouvertes.
