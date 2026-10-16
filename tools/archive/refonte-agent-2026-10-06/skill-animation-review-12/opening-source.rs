//! ANIM-OVER03: recover painted breadth at a corridor's opening into a room.
//! Additional visual sight starts at the opening, never through a solid wall.
use super::*;

fn exit_source(stage: &Stage) -> Option<Vec2> {
    let map = &stage.fixture.map;
    let narrow = |x| {
        map.blocks_vision(GridPos::new(x, ORIGIN.y - 1))
            && map.blocks_vision(GridPos::new(x, ORIGIN.y + 1))
    };
    // Only relax the projection of walls that actually line the emitting corridor.
    // Open rooms, isolated blocks and corridors with a transverse wall keep A07.
    if !narrow(ORIGIN.x) {
        return None;
    }
    for x in ORIGIN.x + 1..stage.width {
        if map.blocks_vision(GridPos::new(x, ORIGIN.y)) {
            return None;
        }
        if !narrow(x) {
            let at = vec2(x as f32 * TILE + 1.0, EMITTER.y);
            return clear_ray(at, &stage.walls()).then_some(at);
        }
    }
    None
}
fn visible(point: Vec2, walls: &[Rect], exit: Option<Vec2>) -> bool {
    clear_ray(point, walls)
        || exit.is_some_and(|source| {
            point.x >= source.x && !walls.iter().any(|&wall| intersects(source, point, wall))
        })
}
pub(super) fn visibility(stage: &Stage) -> Image {
    let walls = stage.walls();
    let exit = exit_source(stage);
    let size = stage.size(0);
    let mut image = Image::gen_image_color(size.x as u16, size.y as u16, BLACK);
    for y in 0..image.height as u32 {
        for x in 0..image.width as u32 {
            if visible(vec2(x as f32 + 0.5, y as f32 + 0.5), &walls, exit) {
                image.set_pixel(x, y, WHITE);
            }
        }
    }
    image
}
pub(crate) async fn run() {
    super::run_variant(true).await;
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    super::capture_variant(output, true).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drawn_sides_are_visible_after_the_opening_but_never_on_corridor_walls() {
        let stage = Stage::new(0);
        let walls = stage.walls();
        let exit = exit_source(&stage);
        let painted_side = vec2(320.0, 132.0);
        assert!(!clear_ray(painted_side, &walls));
        assert!(visible(painted_side, &walls, exit));
        assert!(!visible(vec2(160.0, 160.0), &walls, exit));
        for case in 1..5 {
            assert!(exit_source(&Stage::new(case)).is_none());
        }
    }
    #[test]
    fn room_wall_still_stops_the_reexpanded_drawing() {
        let mut stage = Stage::new(0);
        stage.fixture.map =
            Map::from_ascii("#######\n###.#.#\n###.#.#\n#...#.#\n###.#.#\n###.#.#\n#######")
                .unwrap();
        let walls = stage.walls();
        let exit = exit_source(&stage);
        assert!(exit.is_some());
        assert!(visible(vec2(220.0, 219.0), &walls, exit));
        assert!(!visible(vec2(352.0, 219.0), &walls, exit));
    }
}
