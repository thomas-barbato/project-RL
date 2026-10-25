# Nettoyage du25 octobre2026

Le joueur demande de retirer les aperçus et autres éléments devenus inutiles
du dossier du jeu. La compilation Release et l'inventaire des ressources sont
contrôlés avant l'opération, puis de nouveau après.

## Sorties retirées

- 7 080 fichiers,466 712 235 octets, passent dans la Corbeille : anciens exports
de marche, banques de comparaison, montages de casques, programmes de diagnostic,
captures répétées et caches Python. La Corbeille n'est pas vidée ; ces octets
restent sur le disque jusqu'à son vidage.
- Le nettoyage Cargo retire1 611 fichiers Debug, soit1 652 798 395 octets.
Le Release et le paquet joueur restent disponibles.

Les sorties retirées représentent2 119 510 630 octets bruts. L'éditeur ouvert
a ensuite reconstruit automatiquement167 834 704 octets de cache de vérification
et son répertoire `target/flycheck0`. Son contrôle Dev réussit en1min13s ; ces
fichiers actifs ne sont pas supprimés en boucle.

Après ces régénérations et ajout du registre, le dossier passe d'environ6,96Go
à5,01Go hors historique Git, soit environ1,94Go de réduction nette. La réduction
logique du cache Debug sur le disque est d'environ1,48Go ; les466,7Mo de sorties
envoyées à la Corbeille restent sur le disque jusqu'à son vidage.

## Conservation

Les dessins sources, ressources de compilation et ressources du jeu sont conservés.
Les accords et règles restent dans `docs/reviews/`, avec des vues natives finales
représentatives. La revue des casques garde un aperçu d'ensemble courant ; les
exports détaillés restent régénérables depuis le compositeur Rust.

Les cartes, préférences et récupérations de `artifacts/map-editor/` sont préservées,
ainsi que les sauvegardes et préférences utilisateur. Les sources, le contenu,
les outils, le Release, le paquet autonome et l'état Git ont des empreintes avant
nettoyage. `preservation-after.json` consigne la comparaison des25 144 fichiers.

Les aperçus encore utiles de politique d'animation et de trajet souris, ainsi que
les données documentaires de l'étude d'itemisation, restent à leur emplacement.
Les propositions de dessins non validées restent dans leurs sources et revues.

## Vérification et registre

- Release avant : compilation réussie,0,27s ; après : réussie,0,30s.
- Contrôle Dev automatique de l'éditeur après nettoyage : réussi,1min13s.
- Inventaire de production :1 813 chemins de ressources présents.
- Intégrité du paquet autonome conservé : réussie.
- Exécutable Release actuel avec `--verify-assets` sur les données du paquet :
573 fichiers et catalogues vérifiés, sans ouvrir de fenêtre ni reprendre de partie.
- Les chemins absolus, ancêtres, jonctions, tailles et empreintes de toutes les
cibles sont contrôlés avant le premier déplacement vers la Corbeille. Les fichiers
ajoutés ou modifiés depuis le plan bloquent cette opération.

`plan.json` détaille les cibles et leurs raisons ; `recycled.json` et
`recycle-result.json` enregistrent l'opération réelle. `cargo-clean.json` mesure
le cache retiré. `preserved-before.json` et `preservation-after.json` vérifient
les fichiers conservés. `result.json` regroupe le bilan final.

Les anciennes références à des captures retirées se retrouvent dans le registre.
Le jeu ne dépend d'aucune de ces sorties. `Jouer.cmd` utilise toujours le Release ;
les prochains tests ou lancements Debug devront reconstruire leur cache.
