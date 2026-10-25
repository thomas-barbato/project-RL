// Selected ACTOR-READABILITY01 paintings and existing material variants.
fn readability_bytes(name: &str) -> Option<&'static [u8]> { Some(match name {
    "VIS-007-0" => include_bytes!("../assets/sprites/actor-readability-02/007-0.png"),
    "VIS-008-0" => include_bytes!("../assets/sprites/actor-readability-02/008-0.png"),
    "VIS-010-0" => include_bytes!("../assets/sprites/actor-readability-02/010-0.png"),
    "VIS-008-2" => include_bytes!("../assets/sprites/actor-readability-02/008-2.png"),
    "VIS-008-3" => include_bytes!("../assets/sprites/actor-readability-02/008-3.png"),
    "VIS-008-4" => include_bytes!("../assets/sprites/actor-readability-02/008-4.png"),
    _ => return None,
}) }
