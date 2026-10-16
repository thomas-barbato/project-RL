//! Compatibility entry for the accepted shared AOE-FALLOFF01 resolver.
use project_rl::world::GridPos;

/// Square shells: full damage at centre, half at the unblocked outer edge.
/// The original radius is used even when walls remove some cells.
pub(super) fn damage(base: u16, centre: GridPos, target: GridPos, radius: u16) -> Option<u16> {
    project_rl::skills::area_damage::damage_at(
        base,
        centre,
        target,
        project_rl::skills::spatial::Area::Square { radius },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn square_corners_share_the_outer_shell_and_damage_is_integer() {
        let c = GridPos::new(4, 4);
        assert_eq!(damage(25, c, c, 1), Some(25));
        assert_eq!(damage(25, c, GridPos::new(3, 3), 1), Some(12));
        assert_eq!(damage(25, c, GridPos::new(4, 3), 1), Some(12));
        assert_eq!(damage(25, c, GridPos::new(4, 2), 1), None);
    }
    #[test]
    fn larger_zones_have_intermediate_shells_without_weakening_centre() {
        let c = GridPos::new(0, 0);
        assert_eq!(damage(100, c, c, 2), Some(100));
        assert_eq!(damage(100, c, GridPos::new(1, -1), 2), Some(75));
        assert_eq!(damage(100, c, GridPos::new(-2, 0), 2), Some(50));
        assert_eq!(damage(100, c, c, 0), Some(100));
        assert_eq!(damage(100, c, GridPos::new(1, 0), 0), None);
    }
}
