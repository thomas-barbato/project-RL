//! Client-only controls: one binding per action, independent of game rules.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use macroquad::prelude::{
    KeyCode, MouseButton, get_keys_pressed, is_key_down, is_mouse_button_pressed, mouse_position,
    mouse_wheel, screen_height, screen_width,
};
use serde::{Deserialize, Serialize};

const GAME: u8 = 1;
const INVENTORY: u8 = 2;
const SKILLS: u8 = 4;
const REPORT: u8 = 8;
const SETTINGS: u8 = 16;
const ALL: u8 = GAME | INVENTORY | SKILLS | REPORT | SETTINGS;

macro_rules! actions {
    ($( $id:ident, $label:literal, $key:literal, $contexts:expr; )*) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum Action { $($id,)* }
        impl Action {
            pub const ALL: &'static [Self] = &[$(Self::$id,)*];
            pub const MOVEMENT: &'static [Self] = &[
                Self::MoveNorth,
                Self::MoveEast,
                Self::MoveSouth,
                Self::MoveWest,
            ];
            pub fn name(self) -> &'static str { match self { $(Self::$id => $label,)* } }
            fn default_key(self) -> &'static str { match self { $(Self::$id => $key,)* } }
            fn contexts(self) -> u8 { match self { $(Self::$id => $contexts,)* } }
        }
    };
}

actions! {
    MoveNorth, "Se déplacer vers le nord", "W", GAME;
    MoveSouth, "Se déplacer vers le sud", "S", GAME;
    MoveWest, "Se déplacer vers l'ouest", "A", GAME;
    MoveEast, "Se déplacer vers l'est", "D", GAME;
    Wait, "Attendre un tour", "Space", GAME;
    Attack, "Attaquer la cible", "F", GAME;
    CycleTarget, "Cible suivante", "Tab", GAME;
    Interact, "Interagir / ramasser", "E", GAME;
    Inventory, "Ouvrir / fermer l'inventaire", "I", ALL;
    Character, "Ouvrir / fermer le personnage", "J", ALL;
    Skills, "Ouvrir / fermer les compétences", "K", ALL;
    QuickTechniques, "Ouvrir les techniques actives", "U", GAME;
    Report, "Ouvrir / fermer le dossier", "O", ALL;
    QuestJournal, "Ouvrir / fermer le journal de quêtes", "N", ALL;
    Legend, "Afficher / masquer la légende", "F1", GAME;
    NpcVision, "Afficher / masquer les champs de vision", "F2", GAME;
    Restart, "Nouvelle partie", "R", GAME;
    Analyze, "Analyse de cible", "C", GAME;
    Traces, "Lecture de traces", "L", GAME;
    Walls, "Examen des parois", "B", GAME;
    Threat, "Profil de menace", "P", GAME;
    Multiple, "Analyse multiple", "M", GAME;
    Corrosion, "Essai : corrosion", "H", GAME;
    Pulse, "Essai : impulsion", "G", GAME;
    Slot1, "Canal 1 : sélectionner / équiper", "Key1", GAME | INVENTORY;
    Slot2, "Canal 2 : sélectionner / équiper", "Key2", GAME | INVENTORY;
    Slot3, "Canal 3 : sélectionner / équiper", "Key3", GAME | INVENTORY;
    MenuUp, "Menus / dossier : ligne précédente", "Up", INVENTORY | SKILLS | REPORT | SETTINGS;
    MenuDown, "Menus / dossier : ligne suivante", "Down", INVENTORY | SKILLS | REPORT | SETTINGS;
    MenuLeft, "Valeur / discipline précédente", "Left", SKILLS | SETTINGS;
    MenuRight, "Valeur / discipline suivante", "Right", SKILLS | SETTINGS;
    Learn, "Apprendre / réattribuer une commande", "Enter", SKILLS | SETTINGS;
    Use, "Utiliser la sélection", "U", INVENTORY | SKILLS;
    Drop, "Déposer la sélection", "X", INVENTORY;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Azerty,
    Qwerty,
}

impl Layout {
    pub fn name(self) -> &'static str {
        match self {
            Self::Azerty => "AZERTY",
            Self::Qwerty => "QWERTY",
        }
    }
    pub fn other(self) -> Self {
        match self {
            Self::Azerty => Self::Qwerty,
            Self::Qwerty => Self::Azerty,
        }
    }
}

/// Miniquad emits physical US positions on Windows/macOS, layout symbols on Linux.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeySemantics {
    Physical,
    Logical,
}

impl KeySemantics {
    pub fn native() -> Self {
        if cfg!(any(target_os = "windows", target_os = "macos")) {
            Self::Physical
        } else {
            Self::Logical
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Binding {
    Key(String),
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

macro_rules! keys {
    ($($key:ident),*) => {
        const KEYS: &[(&str, KeyCode)] = &[$((stringify!($key), KeyCode::$key)),*];
    };
}
keys!(
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Enter,
    Tab,
    Space,
    Backspace,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpEnter,
    KpAdd,
    KpSubtract,
    KpMultiply,
    KpDivide,
    KpDecimal,
    Semicolon,
    Comma,
    Period,
    Slash,
    Apostrophe,
    Backslash,
    LeftBracket,
    RightBracket,
    Minus,
    Equal,
    GraveAccent,
    World2
);

impl Binding {
    pub fn key(name: &str) -> Self {
        Self::Key(name.to_owned())
    }
    fn valid(&self) -> bool {
        match self {
            Self::Key(name) => KEYS.iter().any(|(n, _)| *n == name),
            _ => true,
        }
    }
    pub fn label(&self, layout: Layout, semantics: KeySemantics) -> String {
        let name = match self {
            Self::Key(name) => name.as_str(),
            Self::MouseLeft => return "Clic gauche".to_owned(),
            Self::MouseRight => return "Clic droit".to_owned(),
            Self::MouseMiddle => return "Clic milieu".to_owned(),
        };
        if layout == Layout::Azerty && semantics == KeySemantics::Physical {
            let localized = match name {
                "W" => Some("Z"),
                "Z" => Some("W"),
                "A" => Some("Q"),
                "Q" => Some("A"),
                "Semicolon" => Some("M"),
                "M" => Some(","),
                "Comma" => Some(";"),
                "Period" => Some(":"),
                "Slash" => Some("!"),
                "Apostrophe" => Some("ù"),
                "Key1" => Some("& (1)"),
                "Key2" => Some("é (2)"),
                "Key3" => Some("\" (3)"),
                "Key4" => Some("' (4)"),
                "Key5" => Some("( (5)"),
                "Key6" => Some("- (6)"),
                "Key7" => Some("è (7)"),
                "Key8" => Some("_ (8)"),
                "Key9" => Some("ç (9)"),
                "Key0" => Some("à (0)"),
                "Minus" => Some(")"),
                "Equal" => Some("="),
                "GraveAccent" => Some("²"),
                "LeftBracket" => Some("^"),
                "RightBracket" => Some("$"),
                "Backslash" => Some("*"),
                "World2" => Some("<"),
                _ => None,
            };
            if let Some(label) = localized {
                return label.to_owned();
            }
        }
        match name {
            "Space" => "Espace",
            "Escape" => "Échap",
            "Enter" => "Entrée",
            "Up" => "Haut",
            "Down" => "Bas",
            "Left" => "Gauche",
            "Right" => "Droite",
            "Semicolon" => ";",
            "Comma" => ",",
            "Period" => ".",
            "Slash" => "/",
            _ => name.strip_prefix("Key").unwrap_or(name),
        }
        .to_owned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Controls {
    version: u8,
    pub layout: Layout,
    semantics: KeySemantics,
    bindings: BTreeMap<Action, Binding>,
}

impl Controls {
    pub fn preset(layout: Layout, semantics: KeySemantics) -> Self {
        let bindings = Action::ALL
            .iter()
            .map(|action| {
                let name = match (layout, semantics, action) {
                    (Layout::Azerty, KeySemantics::Physical, Action::Multiple) => "Semicolon",
                    (Layout::Azerty, KeySemantics::Logical, Action::MoveNorth) => "Z",
                    (Layout::Azerty, KeySemantics::Logical, Action::MoveWest) => "Q",
                    _ => action.default_key(),
                };
                (*action, Binding::key(name))
            })
            .collect();
        Self {
            version: 2,
            layout,
            semantics,
            bindings,
        }
    }

    pub fn binding(&self, action: Action) -> &Binding {
        &self.bindings[&action]
    }
    pub fn label(&self, action: Action) -> String {
        self.binding(action).label(self.layout, self.semantics)
    }
    /// Changing the preset never silently destroys customized bindings.
    pub fn change_layout(&mut self, layout: Layout) -> Result<(), String> {
        let previous_defaults = Self::preset(self.layout, self.semantics);
        let next_defaults = Self::preset(layout, self.semantics);
        let mut next = self.clone();
        next.layout = layout;
        for action in Action::ALL {
            if self.binding(*action) == previous_defaults.binding(*action) {
                next.bindings
                    .insert(*action, next_defaults.binding(*action).clone());
            }
        }
        next.validate()?;
        *self = next;
        Ok(())
    }
    pub fn pressed(&self, action: Action, frame: &InputFrame) -> bool {
        frame.pressed.contains(self.binding(action))
    }
    pub fn held(&self, action: Action, frame: &InputFrame) -> bool {
        frame.held.contains(self.binding(action))
    }
    pub fn rebind(&mut self, action: Action, binding: Binding) -> Result<(), String> {
        if !binding.valid() {
            return Err(
                "Touche non prise en charge (Échap reste réservé au menu pause).".to_owned(),
            );
        }
        for (other, current) in &self.bindings {
            if *other != action && *current == binding && other.contexts() & action.contexts() != 0
            {
                return Err(format!(
                    "Déjà attribué : {}. Aucun changement effectué.",
                    other.name()
                ));
            }
        }
        self.bindings.insert(action, binding);
        Ok(())
    }
    fn validate(&self) -> Result<(), String> {
        if self.version != 2 {
            return Err("Version des commandes incompatible.".to_owned());
        }
        if self.bindings.len() != Action::ALL.len() {
            return Err("Commandes incomplètes.".to_owned());
        }
        let mut checked = self.clone();
        for action in Action::ALL {
            let binding = self.bindings.get(action).ok_or("Commande manquante.")?;
            checked.rebind(*action, binding.clone())?;
        }
        Ok(())
    }
    pub fn decode(source: &str) -> Result<Self, String> {
        let mut document: serde_json::Value =
            serde_json::from_str(source).map_err(|error| error.to_string())?;
        // v1 had a remappable close action. Escape now owns hierarchical pause navigation.
        // Keep all other custom bindings, and do not rewrite a user's file on load.
        if document["version"] == 1 {
            document["version"] = serde_json::json!(2);
            if let Some(bindings) = document["bindings"].as_object_mut() {
                bindings.remove("close");
            }
        }
        // Older profiles had separate pickup (E) and interaction (V) keys.
        // Keep a personalized key when one exists; otherwise use the new E default.
        if let Some(bindings) = document["bindings"].as_object_mut() {
            if let Some(pickup) = bindings.remove("pick_up") {
                let old_interact = bindings.get("interact").cloned();
                let pickup_custom = pickup != serde_json::json!({"type":"key", "value":"E"});
                let interact_custom = old_interact.as_ref().is_some_and(|binding| {
                    *binding != serde_json::json!({"type":"key", "value":"V"})
                });
                if pickup_custom || !interact_custom {
                    bindings.insert("interact".to_owned(), pickup);
                }
            }
        }
        let mut result: Self =
            serde_json::from_value(document).map_err(|error| error.to_string())?;
        // Add actions introduced after a user's file was saved without resetting
        // custom bindings. Prefer each new default, then the first free key.
        // Loading a legacy file never writes it back automatically.
        let defaults = Self::preset(result.layout, result.semantics);
        for action in [
            Action::Interact,
            Action::Legend,
            Action::NpcVision,
            Action::Character,
            Action::QuickTechniques,
            Action::QuestJournal,
        ] {
            if result.bindings.contains_key(&action) {
                continue;
            }
            let preferred = defaults.binding(action).clone();
            for binding in
                std::iter::once(preferred).chain(KEYS.iter().map(|(name, _)| Binding::key(name)))
            {
                if result.rebind(action, binding).is_ok() {
                    break;
                }
            }
        }
        result.validate()?;
        Ok(result)
    }
    pub fn load(path: &Path, detected: Option<Layout>) -> (Self, String) {
        let fallback = Self::preset(detected.unwrap_or(Layout::Qwerty), KeySemantics::native());
        match std::fs::read_to_string(path) {
            Ok(source) => match Self::decode(&source) {
                Ok(saved) if saved.semantics == KeySemantics::native() => (saved, "Commandes personnelles chargées.".to_owned()),
                Ok(_) => (fallback, "Commandes d'un autre type de clavier système : préréglage utilisé, fichier conservé. Vérifier les touches avant de réenregistrer.".to_owned()),
                Err(error) => (
                    fallback,
                    format!("Réglages invalides, fichier conservé : {error}"),
                ),
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let message = if detected.is_some() {
                    "Disposition détectée ; Échap → Options → Commandes."
                } else {
                    "Disposition non détectée : repli QWERTY. Échap → Options → Commandes."
                };
                (fallback, message.to_owned())
            }
            Err(error) => (
                fallback,
                format!("Lecture des commandes impossible : {error}"),
            ),
        }
    }
    pub fn save(&self, path: &Path) -> Result<(), String> {
        use std::io::Write;
        self.validate()?;
        let source = serde_json::to_string_pretty(self).map_err(|error| error.to_string())?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let temporary = path.with_extension("json.tmp");
        // A failed write must not truncate an existing user's configuration.
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        let result = file
            .write_all(source.as_bytes())
            .and_then(|()| file.sync_all());
        drop(file);
        let result = result.and_then(|()| std::fs::rename(&temporary, path));
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result.map_err(|e| e.to_string())
    }
}

#[derive(Default)]
pub struct InputFrame {
    pub pressed: BTreeSet<Binding>,
    /// Keyboard bindings currently held. Mouse buttons deliberately remain
    /// edge-triggered: holding a click must never manufacture repeated actions.
    pub held: BTreeSet<Binding>,
    pub pause: bool,
    pub pointer: Option<(f32, f32)>,
    pub viewport: Option<(f32, f32)>,
    pub wheel_y: f32,
}

impl InputFrame {
    pub fn capture() -> Self {
        let keys = get_keys_pressed();
        let mut pressed: BTreeSet<_> = KEYS
            .iter()
            .filter(|(_, key)| keys.contains(key))
            .map(|(name, _)| Binding::key(name))
            .collect();
        let held = KEYS
            .iter()
            .filter(|(_, key)| is_key_down(*key))
            .map(|(name, _)| Binding::key(name))
            .collect();
        for (button, binding) in [
            (MouseButton::Left, Binding::MouseLeft),
            (MouseButton::Right, Binding::MouseRight),
            (MouseButton::Middle, Binding::MouseMiddle),
        ] {
            if is_mouse_button_pressed(button) {
                pressed.insert(binding);
            }
        }
        Self {
            pressed,
            held,
            pause: keys.contains(&KeyCode::Escape),
            pointer: Some(mouse_position()),
            viewport: Some((screen_width(), screen_height())),
            wheel_y: mouse_wheel().1,
        }
    }
}

/// Turns a held movement binding into paced movement intents. The engine still
/// receives one ordinary command per returned action, preserving turn order,
/// collisions and deterministic replay.
#[derive(Clone, Copy, Debug, Default)]
pub struct MovementRepeater {
    active: Option<Action>,
    repeat_at: f64,
}

impl MovementRepeater {
    pub const INITIAL_DELAY_SECONDS: f64 = 0.28;
    pub const INTERVAL_SECONDS: f64 = 0.085;

    pub fn clear(&mut self) {
        self.active = None;
        self.repeat_at = 0.0;
    }

    /// The initial press remains in `InputFrame::pressed`, so this only returns
    /// subsequent repeats. At most one action is produced per rendered frame.
    pub fn poll(&mut self, controls: &Controls, frame: &InputFrame, now: f64) -> Option<Action> {
        if let Some(action) = Action::MOVEMENT
            .iter()
            .copied()
            .find(|action| controls.pressed(*action, frame))
        {
            self.active = Some(action);
            self.repeat_at = now + Self::INITIAL_DELAY_SECONDS;
            return None;
        }

        let action = self.active?;
        if !controls.held(action, frame) {
            self.clear();
            return None;
        }
        if now < self.repeat_at {
            return None;
        }
        self.repeat_at = now + Self::INTERVAL_SECONDS;
        Some(action)
    }
}

pub fn config_path() -> PathBuf {
    let root = if cfg!(target_os = "windows") {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
    };
    root.unwrap_or_else(|| PathBuf::from("config"))
        .join("ProjectRL")
        .join("controls.json")
}

#[cfg(target_os = "windows")]
pub fn detect_layout() -> Option<Layout> {
    // Read the game thread's actual layout, never the UI language or locale.
    #[link(name = "user32")]
    unsafe extern "system" {
        fn GetKeyboardLayout(thread: u32) -> *mut std::ffi::c_void;
        fn MapVirtualKeyExW(code: u32, map_type: u32, layout: *mut std::ffi::c_void) -> u32;
    }
    // SAFETY: the system owns the non-null handle; these calls only inspect it.
    unsafe {
        let layout = GetKeyboardLayout(0);
        if layout.is_null() {
            return None;
        }
        match (
            MapVirtualKeyExW(0x10, 3, layout),
            MapVirtualKeyExW(0x11, 3, layout),
        ) {
            (0x41, 0x5a) => Some(Layout::Azerty),
            (0x51, 0x57) => Some(Layout::Qwerty),
            _ => None,
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn detect_layout() -> Option<Layout> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_are_single_binding_and_match_backend_semantics() {
        for semantics in [KeySemantics::Physical, KeySemantics::Logical] {
            for layout in [Layout::Azerty, Layout::Qwerty] {
                let controls = Controls::preset(layout, semantics);
                assert_eq!(controls.validate(), Ok(()));
                assert_eq!(
                    controls.binding(Action::MoveNorth).label(layout, semantics),
                    if layout == Layout::Azerty { "Z" } else { "W" }
                );
                assert_eq!(
                    controls.binding(Action::MoveWest).label(layout, semantics),
                    if layout == Layout::Azerty { "Q" } else { "A" }
                );
                assert_eq!(
                    controls.binding(Action::Multiple).label(layout, semantics),
                    "M"
                );
                assert_eq!(controls.binding(Action::Report), &Binding::key("O"));
                assert_eq!(controls.binding(Action::Legend), &Binding::key("F1"));
                assert_eq!(controls.binding(Action::NpcVision), &Binding::key("F2"));
            }
        }
    }

    #[test]
    fn held_movement_repeats_after_a_delay_and_respects_rebinding() {
        let mut controls = Controls::preset(Layout::Azerty, KeySemantics::Physical);
        controls
            .rebind(Action::MoveNorth, Binding::key("Up"))
            .unwrap();
        let held = InputFrame {
            pressed: [Binding::key("Up")].into(),
            held: [Binding::key("Up")].into(),
            ..Default::default()
        };
        let mut repeat = MovementRepeater::default();
        assert_eq!(repeat.poll(&controls, &held, 10.0), None);

        let held = InputFrame {
            held: [Binding::key("Up")].into(),
            ..Default::default()
        };
        assert_eq!(repeat.poll(&controls, &held, 10.27), None);
        assert_eq!(
            repeat.poll(&controls, &held, 10.28),
            Some(Action::MoveNorth)
        );
        assert_eq!(repeat.poll(&controls, &held, 10.30), None);
        assert_eq!(
            repeat.poll(&controls, &held, 10.365),
            Some(Action::MoveNorth)
        );

        repeat.clear();
        assert_eq!(repeat.poll(&controls, &held, 20.0), None);
    }

    #[test]
    fn movement_repeat_stops_as_soon_as_the_binding_is_released() {
        let controls = Controls::preset(Layout::Qwerty, KeySemantics::Physical);
        let pressed = InputFrame {
            pressed: [Binding::key("D")].into(),
            held: [Binding::key("D")].into(),
            ..Default::default()
        };
        let mut repeat = MovementRepeater::default();
        assert_eq!(repeat.poll(&controls, &pressed, 1.0), None);
        assert_eq!(repeat.poll(&controls, &InputFrame::default(), 2.0), None);

        let held_again = InputFrame {
            held: [Binding::key("D")].into(),
            ..Default::default()
        };
        assert_eq!(repeat.poll(&controls, &held_again, 3.0), None);
    }

    #[test]
    fn rebinding_replaces_old_key_and_rejects_conflicts_atomically() {
        let mut controls = Controls::preset(Layout::Azerty, KeySemantics::Physical);
        controls.rebind(Action::Report, Binding::key("F3")).unwrap();
        for (binding, expected) in [
            ("O", false),
            ("Enter", false),
            ("Escape", false),
            ("F3", true),
        ] {
            let frame = InputFrame {
                pressed: [Binding::key(binding)].into(),
                ..Default::default()
            };
            assert_eq!(controls.pressed(Action::Report, &frame), expected);
        }
        let before = controls.clone();
        assert!(controls.rebind(Action::Report, Binding::key("K")).is_err());
        assert!(
            controls
                .rebind(Action::Report, Binding::key("Escape"))
                .is_err()
        );
        assert_eq!(controls, before);
        controls
            .rebind(Action::MoveNorth, Binding::key("Up"))
            .unwrap(); // disjoint contexts
        controls
            .rebind(Action::Report, Binding::MouseRight)
            .unwrap();
        assert_eq!(controls.binding(Action::Report), &Binding::MouseRight);
    }

    #[test]
    fn saved_preferences_override_detection_and_invalid_files_are_not_overwritten() {
        let folder = std::env::temp_dir().join(format!(
            "project-rl-controls-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let path = folder.join("controls.json");
        let mut controls = Controls::preset(Layout::Azerty, KeySemantics::native());
        controls
            .rebind(Action::Report, Binding::MouseMiddle)
            .unwrap();
        controls.save(&path).unwrap();
        assert_eq!(Controls::load(&path, Some(Layout::Qwerty)).0, controls);
        controls.rebind(Action::Report, Binding::key("F3")).unwrap();
        controls.save(&path).unwrap();
        assert_eq!(Controls::load(&path, None).0, controls);
        std::fs::write(&path, "invalid").unwrap();
        assert!(Controls::load(&path, None).1.contains("invalides"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "invalid");
        std::fs::remove_file(&path).unwrap();
        std::fs::remove_dir(&folder).unwrap();
    }

    #[test]
    fn changing_layout_keeps_custom_bindings_and_checks_new_conflicts() {
        let mut controls = Controls::preset(Layout::Qwerty, KeySemantics::Physical);
        controls
            .rebind(Action::Report, Binding::MouseRight)
            .unwrap();
        controls.change_layout(Layout::Azerty).unwrap();
        assert_eq!(controls.label(Action::Multiple), "M");
        assert_eq!(
            controls.binding(Action::Multiple),
            &Binding::key("Semicolon")
        );
        assert_eq!(controls.binding(Action::Report), &Binding::MouseRight);
        controls.rebind(Action::Report, Binding::key("M")).unwrap();
        let before = controls.clone();
        assert!(controls.change_layout(Layout::Qwerty).is_err());
        assert_eq!(controls, before);
        let mut document = serde_json::to_value(&controls).unwrap();
        document["bindings"]["report"] = serde_json::json!({"type":"key", "value":"OOPS"});
        assert!(Controls::decode(&document.to_string()).is_err());
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("report");
        assert!(Controls::decode(&document.to_string()).is_err());
    }

    #[test]
    fn v1_preferences_migrate_without_losing_customizations_or_an_escape_alias() {
        let mut controls = Controls::preset(Layout::Azerty, KeySemantics::native());
        controls.rebind(Action::Report, Binding::key("F3")).unwrap();
        let mut document = serde_json::to_value(&controls).unwrap();
        document["version"] = serde_json::json!(1);
        document["bindings"]["close"] = serde_json::json!({"type":"key", "value":"Escape"});
        assert_eq!(Controls::decode(&document.to_string()).unwrap(), controls);
        document["bindings"]["move_north"] = serde_json::json!({"type":"key", "value":"Escape"});
        assert!(Controls::decode(&document.to_string()).is_err());
        assert!(controls.rebind(Action::Report, Binding::key("F1")).is_err());
    }

    #[test]
    fn split_pickup_and_interaction_bindings_migrate_to_one_context_key() {
        let controls = Controls::preset(Layout::Qwerty, KeySemantics::Physical);
        let mut document = serde_json::to_value(&controls).unwrap();
        document["bindings"]["pick_up"] = serde_json::json!({"type":"key", "value":"E"});
        document["bindings"]["interact"] = serde_json::json!({"type":"key", "value":"V"});
        assert_eq!(
            Controls::decode(&document.to_string())
                .unwrap()
                .binding(Action::Interact),
            &Binding::key("E")
        );
        document["bindings"]["interact"] = serde_json::json!({"type":"key", "value":"F4"});
        assert_eq!(
            Controls::decode(&document.to_string())
                .unwrap()
                .binding(Action::Interact),
            &Binding::key("F4")
        );
        document["bindings"]["pick_up"] = serde_json::json!({"type":"key", "value":"F3"});
        assert_eq!(
            Controls::decode(&document.to_string())
                .unwrap()
                .binding(Action::Interact),
            &Binding::key("F3")
        );
    }

    #[test]
    fn legacy_controls_gain_interact_without_overwriting_a_custom_v_binding() {
        let mut document =
            serde_json::to_value(Controls::preset(Layout::Azerty, KeySemantics::native())).unwrap();
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("interact");
        document["bindings"]["report"] = serde_json::json!({"type":"key", "value":"V"});
        let migrated = Controls::decode(&document.to_string()).unwrap();
        assert_eq!(migrated.binding(Action::Report), &Binding::key("V"));
        assert_ne!(migrated.binding(Action::Interact), &Binding::key("V"));
        migrated.validate().unwrap();
    }

    #[test]
    fn legacy_controls_gain_legend_without_overwriting_a_custom_f1_binding() {
        let mut document =
            serde_json::to_value(Controls::preset(Layout::Azerty, KeySemantics::native())).unwrap();
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("legend");
        document["bindings"]["report"] = serde_json::json!({"type":"key", "value":"F1"});
        let migrated = Controls::decode(&document.to_string()).unwrap();
        assert_eq!(migrated.binding(Action::Report), &Binding::key("F1"));
        assert_ne!(migrated.binding(Action::Legend), &Binding::key("F1"));
        migrated.validate().unwrap();
    }

    #[test]
    fn legacy_controls_gain_npc_vision_without_overwriting_a_custom_f2_binding() {
        let mut document =
            serde_json::to_value(Controls::preset(Layout::Azerty, KeySemantics::native())).unwrap();
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("npc_vision");
        document["bindings"]["report"] = serde_json::json!({"type":"key", "value":"F2"});
        let migrated = Controls::decode(&document.to_string()).unwrap();
        assert_eq!(migrated.binding(Action::Report), &Binding::key("F2"));
        assert_ne!(migrated.binding(Action::NpcVision), &Binding::key("F2"));
        migrated.validate().unwrap();
    }

    #[test]
    fn legacy_controls_gain_character_without_overwriting_a_custom_j_binding() {
        let mut document =
            serde_json::to_value(Controls::preset(Layout::Azerty, KeySemantics::native())).unwrap();
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("character");
        document["bindings"]["report"] = serde_json::json!({"type":"key", "value":"J"});
        let migrated = Controls::decode(&document.to_string()).unwrap();
        assert_eq!(migrated.binding(Action::Report), &Binding::key("J"));
        assert_ne!(migrated.binding(Action::Character), &Binding::key("J"));
        migrated.validate().unwrap();
    }

    #[test]
    fn legacy_controls_gain_quick_techniques_without_overwriting_a_custom_u_binding() {
        let mut document =
            serde_json::to_value(Controls::preset(Layout::Azerty, KeySemantics::native())).unwrap();
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("quick_techniques");
        document["bindings"]["attack"] = serde_json::json!({"type":"key", "value":"U"});
        let migrated = Controls::decode(&document.to_string()).unwrap();
        assert_eq!(migrated.binding(Action::Attack), &Binding::key("U"));
        assert_ne!(
            migrated.binding(Action::QuickTechniques),
            &Binding::key("U")
        );
        migrated.validate().unwrap();
    }

    #[test]
    fn legacy_controls_gain_quest_journal_without_overwriting_a_custom_n_binding() {
        let mut document =
            serde_json::to_value(Controls::preset(Layout::Azerty, KeySemantics::native())).unwrap();
        document["bindings"]
            .as_object_mut()
            .unwrap()
            .remove("quest_journal");
        document["bindings"]["report"] = serde_json::json!({"type":"key", "value":"N"});
        let migrated = Controls::decode(&document.to_string()).unwrap();
        assert_eq!(migrated.binding(Action::Report), &Binding::key("N"));
        assert_ne!(migrated.binding(Action::QuestJournal), &Binding::key("N"));
        migrated.validate().unwrap();
    }
}
