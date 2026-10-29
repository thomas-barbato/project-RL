//! SOL-CONT01: opt-in continuous floor candidates; normal rendering unchanged.
use std::cell::Cell;
thread_local! {static MODE:Cell<u8>=const{Cell::new(0)};}
pub fn enabled() -> bool {
    MODE.get() == 1
}
pub fn set_enabled(value: bool) {
    MODE.set(u8::from(value));
}
pub fn set_interior(value: bool) {
    MODE.set(if value { 3 } else { 0 });
}
pub fn set_future(value: bool) {
    MODE.set(if value { 4 } else { 0 });
}
pub fn set_corridor_grate(value: bool) {
    MODE.set(if value { 5 } else { 0 });
}
pub fn set_corridor_wear(value: bool) {
    MODE.set(if value { 6 } else { 0 });
}
pub fn camp_path() -> Option<&'static str> {
    match MODE.get() {
        1 => Some("assets/source-art/floor-continuous-01/camp-floor.png"),
        3 => Some("assets/source-art/floor-continuous-01/interior-r03/floor.png"),
        4 => Some("assets/source-art/floor-future-01/floor.png"),
        5 => Some("assets/source-art/floor-future-01/corridor-grate-01/floor.png"),
        6 => Some("assets/source-art/floor-future-01/corridor-wear-07/floor.png"),
        _ => None,
    }
}
pub fn structure_path() -> Option<&'static str> {
    match MODE.get() {
        3 => Some("assets/source-art/floor-continuous-01/interior-r03/structure.png"),
        4 => Some("assets/source-art/floor-future-01/structure.png"),
        5 => Some("assets/source-art/floor-future-01/corridor-grate-01/structure.png"),
        6 => Some("assets/source-art/floor-future-01/corridor-wear-07/structure.png"),
        _ => None,
    }
}
pub fn street_path(source: &str) -> Option<&'static str> {
    if !enabled() {
        return None;
    }
    match source {
        "assets/maps/prison-street-expanded-01/street.png" => {
            Some("assets/source-art/floor-continuous-01/street-0.png")
        }
        "assets/maps/prison-sewer-01/art/background-afe7b21dff4c.png" => {
            Some("assets/source-art/floor-continuous-01/street-1.png")
        }
        "assets/maps/prison-sewer-01/art/background-b3fe8948652d.png" => {
            Some("assets/source-art/floor-continuous-01/street-1.png")
        }
        _ => None,
    }
}
