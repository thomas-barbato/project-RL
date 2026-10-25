# Construire et utiliser le paquet joueur

`Jouer.cmd` à la racine compile et lance désormais la version Release du projet.
Les ressources restent lues depuis le dépôt pour ce lancement de développement.

`Construire_distribution.cmd` compile le Release et prépare `dist/Project-RL/`.
Ce dossier contient le jeu autonome : son `Jouer.cmd` n’utilise ni Rust ni Python.
Le dossier entier peut être déplacé. Les préférences et sauvegardes conservent
leur emplacement habituel ; les sources graphiques, captures, outils et caches
de compilation ne font pas partie du paquet.

Les données externes sont maintenant regroupées dans **`data.rlpak`**, une
archive ZIP standard, versionnée par le manifeste `runtime-assets.json`.
Le jeu lit et décompresse uniquement la ressource demandée, directement depuis
l’archive, sans extraction temporaire et sans chargement de tout le paquet en RAM.
Les catalogues de règles et de textes ainsi que les mods livrés sont dans
l’archive. Le dossier `mods/` reste disponible pour les mods supplémentaires ;
les règles de dépendance et de doublons de paquets restent les mêmes.

En développement, les fichiers restent accessibles dans le dépôt. Les images,
polices et le son déjà incorporés à l’exécutable y restent : cette étape
regroupe les ressources qui étaient auparavant livrées en fichiers externes.
Une nouvelle image ou un nouveau son ne demande aucun changement du format :
il suffit de l’ajouter à l’inventaire et de reconstruire la distribution.

Pour reconstruire depuis PowerShell :

```powershell
python tools/distribution/package_release.py --build
```

Le même dossier est réutilisé après vérification de son contenu précédent.
Un fichier géré modifié ou un fichier utilisateur supplémentaire bloque la
reconstruction pour éviter de l’écraser. Dans ce cas, conserver ce dossier et
choisir une autre destination avec `--output`.

La liste des scènes, fichiers et répertoires externes est versionnée dans
`tools/distribution/runtime-assets.json`. Une dépendance manquante ou une
référence de production à `assets/source-art/` fait échouer la préparation.
Les images déjà incorporées au programme ne sont pas recopiées, sauf lorsqu’un
chargeur externe en a besoin. Les PNG identiques partagent une seule copie grâce
aux alias ; la recompression est sans perte et ne réécrit jamais les sources.
Chaque entrée de l’archive est compressée sans perte si cela réduit sa taille ;
sinon, ses octets sont stockés tels quels. Les noms, l’ordre et les métadonnées
sont fixés pour que les mêmes entrées produisent la même archive.

La construction prépare et vérifie le nouveau paquet dans un dossier temporaire
avant de remplacer les fichiers générés. Une ancienne distribution en fichiers
séparés peut être reconstruite au même emplacement après sa vérification :
seuls ses fichiers gérés sont remplacés et ses répertoires devenus vides retirés.
Avec `--build` (donc aussi avec `Construire_distribution.cmd`), l’exécutable
préparé doit également réussir `--verify-assets` depuis ce dossier temporaire
avant la publication du paquet. Ce contrôle couvre la lecture de l’archive et
le chargement réel des catalogues, indépendamment des fichiers du dépôt.

Deux contrôles sont disponibles :

```powershell
python tools/distribution/package_release.py --verify dist/Project-RL
& ./dist/Project-RL/project-rl.exe --verify-assets
```

Le premier contrôle les empreintes SHA-256 de l’exécutable, de l’archive et de
chaque entrée, ainsi que les CRC ZIP et la validité des PNG. Le second vérifie
l’inventaire, les tailles et les CRC des entrées puis charge les catalogues de
contenu, sans ouvrir de fenêtre. Le programme détecte ses données à côté de l’exécutable.
`PROJECT_RL_DATA_DIR` permet d’imposer un autre emplacement pour un contrôle ;
une erreur dans ce répertoire ne déclenche pas de repli sur le dépôt.
Une archive manquante, invalide ou incomplète ne déclenche aucun repli sur des
copies de fichiers séparés. Le format ne chiffre pas les ressources.

Le contrôle natif `project-rl.exe --smoke-assets DOSSIER_DE_RAPPORT` ouvre le
moteur, décode et envoie les PNG externes au GPU, initialise les atmosphères,
charge le menu, les catalogues et le son de dialogue, puis quitte.

Les captures et sorties de diagnostic sont jetables une fois le contrôle fini.
Conserver uniquement le bilan utile et les décisions dans `docs/reviews/`,
l’aperçu courant encore utilisé et les données de travail de `artifacts/map-editor/`.
Le cache Debug et les programmes de test peuvent être retirés après validation :

```powershell
cargo clean --profile dev
```

Cette opération impose une recompilation pour les prochains tests Debug. Elle
conserve le profil Release et le paquet autonome.
