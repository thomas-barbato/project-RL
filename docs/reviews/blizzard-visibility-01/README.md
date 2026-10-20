# BLIZZARD-VIS-01 — visibilité de Blizzard sur sol foncé

**Accord du 20 octobre 2026 :** le joueur valide le rendu plus visible, puis le rythme ralenti et son application à toutes les compétences. L’accord, sa portée et le contrôle du raccordement sont dans [`../animation-policy-01/`](../animation-policy-01/). Le contenu ci-dessous conserve la proposition présentée avant cet accord.

Le 20 octobre 2026, le joueur signale que Blizzard est trop peu visible sur
sol foncé. Cette proposition ajoute trois comparaisons natives à l’aperçu
courant, sans considérer son « ok » comme une validation des autres réglages.

- **BLIZZARD-VIS-01-01** : avertissement avant les premiers impacts.
- **BLIZZARD-VIS-01-02** : tempête active sur sol libre.
- **BLIZZARD-VIS-01-03** : tempête active près d’une cloison physique.

Les trois candidats restent en attente de validation visuelle avant
intégration, comme les précédents lots de cadence et de formes naturelles.

## Proposition

Réutiliser les six dessins du Blizzard existant. Rendre les traits clairs de
neige et de glace plus pâles, avec des accents bleu glacé presque blancs,
en renforçant moins la brume pour conserver la lecture du sol et des acteurs.
L’opacité de revue passe de 0,28 à 0,42 pendant l’avertissement et de 0,68 à
0,94 pendant la tempête ; la brume reçoit une atténuation de 24 % à 0 % selon
la clarté de sa matière. Le dessin entier déborde naturellement de 10 % sur
le sol libre, avec un masque distinct des cases de dégâts. Les cloisons et
les cellules hors de vue restent protégées.

La couleur et l’opacité sont les seuls paramètres nouveaux de lisibilité.
Aucun nouvel impact, flocon comptant comme un coup, marqueur de dégâts ou
déclenchement de Gel n’est inventé. La boucle garde ses six dessins par
seconde ; le bouton commun de revue peut allonger le cycle visuel des deux
versions de 30 % pour les comparer. Ce réglage reste une proposition de présentation.

## État réel et portée

Chaque scène initialise un Arcaniste isolé connaissant FR05 au rang un et
exécute la vraie commande de lancement. La scène active exécute ensuite
une vraie attente pour passer de l’avertissement à la tempête. Le rendu
actuel appelle les fonctions de production avec cet état réel. Après cette
préparation, l’état moteur ne change plus pendant les 72 phases capturées.
La boucle visuelle continue sans faire avancer les tours.

Les chances d’impact de 50 % par case et par tour, dégâts, Gel, délai
d’avertissement, durée, portée, zone et coûts restent ceux du jeu. Aucune
sauvegarde de campagne n’est lue ou écrite. Les personnages du diagnostic
servent de repères de lecture ; aucune population de campagne n’est simulée.

Le contrôle compare des éclairages de revue sombre et plus clair, avec et
sans cloison. Il vérifie le contraste de l’effet par rapport à une scène
sans Blizzard, la variation de la boucle, le débordement effectif et
l’absence de pixels d’effet sur les murs ou dans les cellules protégées.
Le contraste cumulé mesuré est un diagnostic de pixels, pas une mesure
perceptive universelle. Les résultats vont dans `validation.json`.

## Reproduction

```powershell
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-blizzard-readability artifacts/refonte-agent-2026-10-06/blizzard-visibility-01
python tools/reviews/build_blizzard_visibility_review.py
```

L’aperçu reste `artifacts/refonte-agent-2026-10-06/apercu-animations.html`.
Il démarre sur la tempête active et conserve les quatre comparaisons
précédentes dans le sélecteur. Les sources embarquées restent dans `assets/`.
