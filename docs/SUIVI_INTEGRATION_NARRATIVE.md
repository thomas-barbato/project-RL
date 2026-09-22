# Suivi de l'intégration narrative

État du 22 septembre 2026. Les dialogues jouables d'Orme, Rivet et Sève ont été reformulés après le retour sur leur manque de naturel. Le [cahier de relecture](TEXTES_NARRATIFS_PROPOSES.md) reprend ces formulations ; les scènes non intégrées restent des propositions à revoir.

## Première tranche jouable : l'enquête du relais

Les parties de générations 87 et 88 proposent **Le chemin des absents** auprès d'Orme, alors rattaché à l'habitant qui circule entre son logement et la place. Depuis la génération 89, Orme est un contact distinct et immobile en ville, à X 11, Y 27. L'habitant conserve sa ronde et ses conversations ordinaires. Les sauvegardes 87 et 88 gardent leur donneur historique pour préserver le rejeu. Sève conserve les soins de la clinique et possède ses premiers sujets de conversation. Rivet occupe un petit relais garanti dans le secteur industriel procédural, accessible par le passage au sud-est des friches.

La touche **E** sert désormais à ramasser l'objet sous le joueur, à parler à un PNJ ou à utiliser un élément proche. Si plusieurs actions sont possibles, un menu permet de choisir avec ↑/↓ puis E ou Entrée, ou à la souris ; Échap annule. Ouvrir ce choix ne consomme pas de tour. Les anciens réglages de touches fusionnent les deux raccourcis : une touche personnalisée de ramassage est conservée en priorité, sinon celle de l'interaction, sinon E.

Le relais est placé sur un espace accessible du terrain généré, sans supprimer de couloir, de sortie, d'ennemi ou de butin existant. Sa position locale varie avec la carte. Depuis la génération 91, son site est une petite cour fermée : l'ancien accès est barricadé, tandis qu'une porte ordinaire au sud permet d'entrer par le service. Un plan mural à l'intérieur confirme ce trajet après qu'on l'a réellement parcouru. Le placement ne doit pas couper les autres chemins de la carte ; les anciennes générations gardent leur petit relais ouvert. La répartition des lieux narratifs sur des plans de couches complets reste à réaliser.

L'enquête peut être menée par conversation avec Rivet ou par consultation du registre. Les faits obtenus avant de parler à Orme sont conservés : accepter ensuite l'enquête permet de faire immédiatement son rapport. À partir des nouvelles parties de génération 88, le rapport accorde une seule fois **30 crédits et 12 XP**, valeurs provisoires d'équilibrage. Les 12 XP correspondent au montant des premières enquêtes du prototype. Le montant est défini dans `narrative_reward_experience` dans le fichier d'expédition ; les crédits restent dans `narrative.investigation.reward_credits`. Les personnages se souviennent du sujet de conversation en cours.

La génération 87 est reprise avec ses règles historiques (**30 crédits, 0 XP**), afin de conserver la vérification exacte de ses commandes et de sa sauvegarde. Aucune récompense rétroactive n'est injectée dans une ancienne partie. Le dialogue affiche désormais la récompense prévue, puis un récapitulatif persistant « QUÊTE TERMINÉE » avec les montants effectivement prévus par cette partie. Le journal d'une quête achevée indique « RÉCOMPENSE REÇUE ».

Depuis la génération 90, un panneau d'orientation est fixé près d'Orme, à X 10, Y 28. Il peut être lu avant l'enquête. Après le rapport, son indication change dans la ville : elle confirme l'occupation du relais et précise que l'ancien accès reste fermé. Le panneau n'affirme pas qu'un itinéraire praticable pour les voyageurs a été ouvert. Sa modification est un effet de quête enregistré avec l'état du monde ; elle reste visible après sauvegarde et reprise. Les générations 87 à 89 conservent leur ville et leur catalogue précédents.

Cette tranche remplace, dans les nouvelles parties, les quatre offres de démonstration du quartier. Ces offres et leurs anciennes règles restent disponibles pour reprendre les parties de générations 65 à 86. Les nouvelles conversations ne sont pas ajoutées rétroactivement à ces sauvegardes.

## Textes et fichiers

| Contenu en jeu | Blocs du cahier |
|---|---|
| Offre, indication, motivation et rapport d'Orme | ABS-D01, D02, D03, D14, D17 |
| Occupation du relais, motif de fermeture et envies de Rivet | ABS-D04, D05, D06, D08, D10 |
| Premiers échanges à la clinique | SEV-D01, D02, D05 |
| Registre, titre et suivi de l'enquête | ABS-A01, ABS-J01/J02, adaptés au suivi actuel |
| Panneau d'Orme avant et après le rapport | ABS-P01 |
| Plan mural de l'accès de service | ABS-E02 |

- **Tous les textes proposés**, y compris ceux encore à intégrer : [TEXTES_NARRATIFS_PROPOSES.md](TEXTES_NARRATIFS_PROPOSES.md).
- **Textes réellement affichés** : [fr.json5](../content/core/locales/fr.json5), clés `narrative.*`, `core:relay_testimony`, `core:relay_service_route_verified` et `core:orme_direction_board_*`. Les répliques jouables sont synchronisées avec les blocs correspondants du cahier ; le paramètre `{passage_coordinates}` est remplacé par les coordonnées du passage. Les libellés courts des choix et du journal sont adaptés aux interactions disponibles.
- **Enchaînements, conditions et actions des dialogues** : [expedition.json5](../content/core/worlds/expedition.json5), champ `narrative`.

## Repères de quête

Les repères sont affichés au-dessus des PNJ et des éléments concernés. Leur signification ne dépend pas de leur couleur :

| Symbole | Signification |
|---|---|
| `!` | Quête disponible auprès du PNJ |
| `…` | Quête acceptée, objectif encore en cours |
| `?` | Quête prête à rendre à ce PNJ |
| `*` | Indice, interlocuteur, objet ou cible utile à un objectif actif |

La légende du jeu rappelle ces symboles. Un élément hors de vue n'est jamais révélé par un repère, même s'il a déjà été aperçu. Une indication de quête n'accorde aucune découverte et ne fait pas avancer le temps. Les marqueurs des indices disparaissent lorsque les faits sont connus ; le marqueur du donneur devient alors `?`. Une quête achevée ne laisse pas de marqueur résiduel.

Les objectifs de livraison, de combat et de consultation d'archives utilisent aussi ce système. Les sites de relevé sont signalés lorsqu'ils sont visibles et portent encore une archive recherchée. Les passages vers des régions inconnues ne sont pas signalés comme s'ils avaient déjà été explorés.

Les repères de quête ont un fond sombre opaque, un cadre doré et un symbole ivoire de taille renforcée. Leur léger flottement respecte l'option de réduction des animations.

## Coordonnées et indication du relais

La position locale du joueur est affichée au-dessus de la minimap, ou sous la carte lorsque la fenêtre ne permet pas d'afficher le panneau latéral. X augmente vers l'est, Y vers le sud ; chaque carte possède ses propres coordonnées.

ABS-D02 et le journal donnent maintenant les coordonnées du **passage souterrain vers le secteur industriel**, lues dans la définition de l'expédition : actuellement X 176, Y 108 dans les friches. Il faut interagir avec ce passage pour descendre au secteur industriel (profondeur 1), puis y rechercher le relais ; atteindre l'entrée ne suffit pas à accomplir l'enquête. Le bandeau d'itinéraire rappelle explicitement de descendre. Il s'agit d'enquêter sur un relais, pas de ramasser un panneau. L'ancienne consigne de suivre des flèches absentes du jeu a été retirée du texte intégré ; le cahier conserve la proposition narrative à retoucher ultérieurement.

Dans le secteur industriel, la position du relais dépend de la génération. Le bandeau invite à le localiser, puis donne les coordonnées de son registre lorsqu'il est en vue. Ces indications n'explorent aucune case et ne révèlent aucun indice caché. Les coordonnées `{passage_coordinates}` des textes sont remplacées à l'affichage, sans modifier la sauvegarde ni le déroulement de l'enquête.

## Limites de cette tranche

Le contournement par la porte de service est maintenant un trajet physique, mais il n'est pas encore une branche de rapport distincte auprès d'Orme. La barricade ne peut pas encore être retirée ni forcée ; les unités qui attaqueraient les réserves ne sont pas encore liées à ce site. Le motif de la fermeture reste appris dans un témoignage ou un registre. Aucun dialogue ne prétend que ces autres actions sont déjà possibles.

Le panneau d'Orme réagit désormais au rapport, mais la nouvelle voie évoquée n'est pas encore un passage traversable entre le quartier et le relais. Les autres jalons de campagne, les passagers et les épilogues restent à intégrer. Cette tranche ne valide donc ni la durée finale ni l'équilibrage d'une campagne entière.

## Vérification

Un test d'intégration parcourt également la carte réelle depuis l'acceptation auprès d'Orme : passage souterrain, ouverture de la porte de service, lecture facultative du plan, présence de Rivet, consultation du registre, remontée et rapport récompensé. Il vérifie que l'entrée indiquée permet effectivement d'achever l'enquête, que le panneau change, se laisse lire et conserve son nouvel état après reprise.

Les contrôles couvrent la découverte anticipée, le rapport, la récompense unique, le rejet d'un dialogue périmé ou hors de portée, le rejeu des commandes, la reprise d'un instantané et les repères sans révélation hors champ. Le placement et la connectivité de la cour du relais sont contrôlés sur 25 graines, dont la graine de départ. Les coordonnées sont vérifiées après une acceptation réelle auprès d'Orme ; les indications ne modifient ni l'état du monde ni le terrain découvert. Les diagnostics visuels `--ui-cold-narrative`, `--ui-cold-narrative-markers`, `--ui-cold-narrative-directions`, `--ui-cold-narrative-journal` et `--ui-cold-narrative-route` permettent de vérifier l'interface sans toucher à une partie du joueur.
