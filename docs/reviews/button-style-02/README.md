# BUTTON-STYLE02 — Boutons d'abandon et de confirmation

Demande du joueur, le 26 octobre 2026 : modifier les boutons rouges pour une
présentation plus soignée, cohérente avec l'interface.

Le composant commun `UiTheme.button` remplace le bordeaux et les contours rouges
des actions `Danger` par les surfaces sombres des actions secondaires. Le focus
et le survol reprennent la couleur du thème ; le texte reste clair. Dans
`button_with_icon`, l'icône seule reçoit un accent cuivré discret.

Aucun libellé, emplacement, événement de clic, confirmation, choix initial,
réglage ou sauvegarde du joueur n'est modifié. Les alertes conservent leurs couleurs.

Contrôles natifs : captures `pause`, `new-run-confirmation-small` et
`new-run-confirmation-largeui-contrast`, toutes terminées avec code 0 et texte
présent dans les zones vérifiées. Les trois images ont été inspectées.
Elles sont conservées sous `artifacts/button-style-02/`.

Les builds Debug et Release verrouillés, le contrôle de formatage et
`git diff --check` ont réussi. La version utilisée par `Jouer.cmd` est à jour.
Aucun nouveau test unitaire n'est ajouté pour ce changement de présentation.
