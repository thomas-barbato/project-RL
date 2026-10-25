# Optimisation des assets — bilan du 21 octobre 2026

Intervention sur `fc5f5632`, sans commit ni publication. Les résultats chiffrés
utiles sont conservés dans `validation.json`. Les captures, programmes et
diagnostics temporaires de cette intervention sont retirés après contrôle.

| Mesure | Avant | Après |
|---|---:|---:|
| Dossier de développement, tailles logiques | 16,55 Gio | **9,84 Gio** |
| Stockage Git | 7,65 Gio | **6,55 Gio** |
| Cache de compilation | 1,95 Gio lors de l’audit | **0,79 Gio**, profil Release conservé |
| Paquet joueur autonome | Aucun paquet sélectionné | **89,9 Mio** |
| Pic mémoire échantillonné au menu, Debug | 504,0 Mio | **300,5 Mio** |

La diminution du dossier est d’environ **6,70 Gio**. Elle comprend des sorties
envoyées à la Corbeille : cette partie n’est pas encore libérée sur le disque.
La mesure mémoire correspond à deux lancements du même contrôle de menu, avec
échantillonnage du working set toutes les 40 ms. Elle ne mesure ni toute une
partie ni la VRAM, et ne démontre pas un gain de FPS. Les captures de menu ont
été inspectées ; leur mise en page est conservée, avec les animations habituelles.

**Distribution et chargement**

- Paquet construit à partir d’un inventaire explicite, comprenant 602 chemins
  logiques, 573 fichiers de données et 29 alias. Les données font 35,48 Mio ;
  l’exécutable, le manifeste et le lanceur complètent les 89,9 Mio.
- 1 397 634 octets de copies PNG et 496 260 octets de compression sans perte
  évités dans ce paquet. Ces gains concernent uniquement ses ressources externes.
- Chemins résolus à côté de l’exécutable, ou dans le dépôt pour les lancements
  Cargo. Un répertoire imposé reste strict, sans repli silencieux.
- Dessins du camp copiés à l’identique vers ses ressources de production.
  Les 10 383 assets d’origine ont été comparés : aucun manquant ; seul le
  manifeste du camp change, pour ses chemins. Les dessins sources sont conservés.
- Décodage différé des banques d’images ; partage des textures d’animation
  identiques ; caches de finition et de monstres limités respectivement à
  16 Mio de pixels CPU et 32 Mio de textures RGBA8, hors autres banques.
  Une entrée dépassant seule le budget peut rester seule pour permettre son rendu.

**Contrôles réalisés**

- Compilations Debug et Release réussies.
- 50 tests graphiques réussis, un test déjà ignoré ; deux tests de chemins,
  deux tests de cache et trois tests de préparation du paquet réussis.
- Vérification des SHA-256, des alias et des PNG du paquet ; reconstruction
  dans la même destination réussie. Les tests vérifient aussi le refus d’écraser
  des données utilisateur ou un paquet modifié.
- Déplacement temporaire hors du dépôt, dans un chemin contenant des espaces,
  puis lancement depuis un autre répertoire : vérification réussie. Un fichier
  délibérément masqué est détecté ; le contrôle repasse après restauration.
- Contrôle natif du Release déplacé : contenu, menu et audio chargés,
  **194 PNG décodés et envoyés au GPU**, sept configurations d’atmosphère
  initialisées ; réutilisation d’une même texture d’animation vérifiée.
- Contrôles natifs du camp et du HUD réussis avec les données du paquet.
  Les trois variantes d’égouts valident l’entrée, le retour et la restauration
  d’état dans leur bilan final. Le processus d’égouts a dépassé l’attente initiale
  de 45 secondes, puis a terminé et produit ce bilan.
- `git fsck --full` réussi avant et après maintenance ; références Git et index
  préservés. Recompression et maintenance ordinaires, sans réécriture d’historique
  ni expiration forcée des reflogs. La sauvegarde temporaire a été retirée après
  ces contrôles ; les objets récents de récupération restent conservés par Git.

**Limite du contrôle de promenade dans la rue**

Les captures `prison-street` et `street-integration` s’arrêtent sur
« Joueur absent » dans le trajet automatique. L’échec est reproduit aussi avec
les données du dépôt, sans utiliser le paquet. Le chargement natif des textures
et atmosphères de rue passe ; le trajet complet de ces scénarios n’est pas validé.
Les règles de combat et de déplacement n’ont pas été modifiées pour contourner
cet échec.

**Nettoyage effectué**

Les anciens dossiers de captures et les nouveaux diagnostics sont retirés,
ainsi que les caches Python et l’archive de sorties jetables du 20 octobre.
`cargo clean --profile dev` retire **1 340 fichiers, environ 1,4 Gio**, tout en
conservant le Release et la distribution. Un seul aperçu d’animations est gardé,
avec ses données encore utilisées ; l’aperçu de déplacement en attente et les
données de `artifacts/map-editor/` sont préservés. Les 60 fichiers nécessaires
à l’aperçu d’animations ont été vérifiés avant le tri.

Le contrôle automatique a refusé les suppressions définitives de fichiers.
Les captures et archives concernées ont donc été envoyées à la Corbeille,
sans créer de nouvelle archive de captures. Leur espace sera récupéré lorsque
la Corbeille sera vidée.

Le [guide de distribution](../../DISTRIBUTION.md) décrit la reconstruction et
les contrôles du paquet. Les grandes cibles de rendu des cartes n’ont pas été
redécoupées dans cette intervention ; leur consommation GPU reste à profiler.
