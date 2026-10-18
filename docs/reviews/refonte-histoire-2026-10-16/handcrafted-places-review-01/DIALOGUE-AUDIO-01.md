# DIALOGUE-AUDIO-01 — son de frappe du terminal

18 octobre 2026. Le joueur propose un petit son pendant l’écriture, sans
savoir le décrire. Trois essais originaux sont préparés dans la revue courante :

- **01 — Terminal feutré** : petits clics électroniques amortis ; proposition préférée.
- **02 — Relais mécanique** : texture mécanique plus grave et un peu plus longue.
- **03 — Impulsions numériques** : micro-impulsions tonales descendantes.

Les essais ont la même cadence que le texte (40 caractères par seconde),
avec moins de sons que de caractères, sans impulsions sur les espaces ni la
ponctuation. Une légère variation d’intensité et de timbre évite une répétition
strictement identique. Durée : 8,625 secondes ; silence après la fin.

Ce sont des échantillons d’écoute indépendants, sans lecture automatique.
Aucun son n’est intégré au jeu avant le choix du joueur. Après choix, le son
doit accompagner seulement la frappe, s’arrêter immédiatement à la fermeture
et rester silencieux lors d’une relecture affichée directement en entier.
Le raccordement réel au lecteur de texte et aux réglages audio reste à effectuer
après la sélection. Aucun bruit de parole ni nouvelle réplique ne sont ajoutés.

Sources reproductibles : `tools/art/prepare_dialogue_audio_review.py`.
Aucun enregistrement externe. WAV PCM mono 48 kHz, 16 bits ; crête limitée à
0,14 sans écrêtage. Les empreintes et durées sont conservées dans
`dialogue-audio-01.json` ; les sorties d’écoute restent sous `artifacts/`.

## Choix provisoire et essai en jeu

Le joueur choisit le 03 provisoirement et demande à l’éprouver en jeu. Il ne
le considère pas encore comme définitivement accepté. L’autorisation précise
est conservée dans `dialogue-audio-selection-01.json`.

`Tester_dialogue_terminal.cmd` lance une vraie partie d’essai du client, au camp,
à côté de l’officier. E ouvre sa bulle terminal r02 avec la frappe progressive
et le son 03 choisi, sans en changer les pixels ni le niveau de l’échantillon.
Clic ou Échap ferment la bulle et coupent la lecture. Le son ne se relance
pas à chaque image ; il est coupé à la fin de la frappe. Le mouvement réduit
affiche le texte complet et reste silencieux. Les services restent accessibles
avec leur interface existante.

L’essai utilise `dialogue-terminal-trial.json`, une sauvegarde distincte de
la partie normale. Les notifications consultées sont retenues pendant cet
essai ; leur persistance définitive reste à raccorder après validation.
Le fichier audio embarqué vient de `assets/audio/ui/` et non d’une capture.
La fonctionnalité Cargo optionnelle `dialogue-audio-trial` active le vrai
backend audio de Macroquad uniquement pour les compilations de cet essai.
Les lancements ordinaires conservent leur adaptateur de dialogue existant.

## Raccordement au lancement habituel

Le joueur précise qu’il veut retrouver le terminal en lançant son jeu habituel.
Le présentateur déjà essayé est donc raccordé à la boucle principale de
`cargo run` et de `Jouer.cmd`, pour le dialogue de l’officier du camp. Il garde
la même frappe, le même curseur, la même fermeture et les notifications lues.
Le son 03 reste provisoire ; son activation audio est désormais une option
Cargo activée par défaut (`dialogue-audio`). `--no-default-features` permet
une compilation silencieuse. L’ancien nom `dialogue-audio-trial` reste un alias
pour le lanceur d’essai facultatif. Les autres dialogues et services conservent
leur adaptateur complet actuel. Aucun texte narratif, règle de combat ou
format de sauvegarde n’est modifié. Le choix et le raccordement sont conservés
dans `dialogue-normal-launch-01.json`.
