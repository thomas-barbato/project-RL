use super::*;

#[test]
fn random_sewer_templates_always_join_water_and_keep_dry_routes() {
    let mut variants = std::collections::BTreeSet::new();
    for seed in 0..1000 {
        let index = variant(seed);
        let connection = connected_connection(seed, 1);
        assert!(connected_id(connection.destination.id.as_str()));
        assert_eq!(
            index_for_id(connection.destination.id.as_str()),
            Some(index)
        );
        variants.insert(index);
        let p = canal_candidate(index);
        let map = canal_candidate_map(index).unwrap();
        assert!(map.is_walkable(pos(p.spawn)), "seed {seed}");
        for member in p.groups.iter().flat_map(|g| &g.members) {
            assert!(
                map.is_walkable(pos(member.at)),
                "seed {seed}, actor {:?}",
                member.at
            );
        }
    }
    assert_eq!(variants.len(), 3);
}

fn small_channel_fixture(rows: &[&str], spawn: [i32; 2]) -> Plan {
    serde_json::from_value(serde_json::json!({"seed":1,"width":rows[0].len(),"height":rows.len(),"spawn":spawn,"layout":rows,"groups":[],"views":[],"blocked":[],"rats":[]})).unwrap()
}

#[test]
fn construction_rejects_an_isolated_pool_and_a_diagonal_join() {
    for rows in [
        vec!["#######", "#.....#", "#.~.~.#", "#.....#", "#######"],
        vec![
            "#######", "#.....#", "#.~...#", "#..~..#", "#.....#", "#######",
        ],
    ] {
        let p = small_channel_fixture(&rows, [1, 1]);
        assert!(
            construct_map(&p, true)
                .unwrap_err()
                .contains("disconnected water")
        );
    }
}

#[test]
fn connected_water_needs_a_walkable_crossing_for_both_banks() {
    let isolated = small_channel_fixture(
        &["#######", "#..~..#", "#..~..#", "#..~..#", "#######"],
        [1, 1],
    );
    assert!(
        construct_map(&isolated, true)
            .unwrap_err()
            .contains("dry route")
    );
    let bridged = small_channel_fixture(
        &["#######", "#..~..#", "#..=..#", "#..~..#", "#######"],
        [1, 1],
    );
    let map = construct_map(&bridged, true).unwrap();
    assert!(map.is_walkable(GridPos::new(3, 2)));
    assert!(!map.is_walkable(GridPos::new(3, 1)));
    assert!(map.is_walkable(GridPos::new(5, 2)));
}

#[test]
fn scenery_cannot_isolate_a_player_a_monster_or_any_dry_tile() {
    for blocked in [
        vec![[2, 1], [1, 2]],
        vec![[6, 3], [7, 2]],
        vec![[1, 1]],
        vec![[7, 3]],
    ] {
        let mut p = small_channel_fixture(
            &[
                "#########",
                "#..~....#",
                "#..=....#",
                "#..~....#",
                "#########",
            ],
            [1, 1],
        );
        p.groups.push(Group {
            id: 1,
            family: "test".into(),
            members: vec![Member {
                at: [7, 3],
                number: 14,
                variant: 0,
                level: 1,
            }],
        });
        p.blocked = blocked;
        assert!(construct_map(&p, true).is_err());
    }
}

#[test]
fn connected_layout_ids_do_not_change_historical_collisions() {
    for index in 1..=3 {
        let old_id = IDS[index - 1];
        let new_id = CONNECTED_IDS[index - 1];
        assert_eq!(index_for_id(old_id), index_for_id(new_id));
        let old_map = construct_map(plan_for_id(old_id).unwrap(), false).unwrap();
        let new_map = construct_map(plan_for_id(new_id).unwrap(), true).unwrap();
        let mut additions = 0;
        for (y, row) in connected_plan(index).layout.iter().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                if c == b'~' && plan(index).layout[y].as_bytes()[x] == b'.' {
                    let at = GridPos::new(x as i32, y as i32);
                    assert!(old_map.is_walkable(at));
                    assert!(!new_map.is_walkable(at));
                    additions += 1;
                }
            }
        }
        assert!(additions > 0);
    }
}

#[test]
fn sewer_variants_retain_approved_geometry_counts_and_safe_arrival() {
    for (i, count) in [(1, 85), (2, 104), (3, 78)] {
        let p = plan(i);
        assert_eq!((p.width, p.height), (47, 44));
        assert_eq!(
            p.groups.iter().map(|g| g.members.len()).sum::<usize>(),
            count
        );
        assert_eq!(
            p.groups
                .iter()
                .flat_map(|g| &g.members)
                .filter(|m| m.number == 132)
                .count(),
            2
        );
        let mut seen = std::collections::BTreeSet::new();
        for m in p.groups.iter().flat_map(|g| &g.members) {
            assert!(seen.insert(m.at));
            assert!(
                m.at[0]
                    .abs_diff(p.spawn[0])
                    .max(m.at[1].abs_diff(p.spawn[1]))
                    > 6
            );
            assert!(matches!(
                p.layout[m.at[1] as usize].as_bytes()[m.at[0] as usize],
                b'.' | b'='
            ));
            assert!(!p.blocked.contains(&m.at));
        }
    }
    let variants: std::collections::BTreeSet<_> = (0..100).map(variant).collect();
    assert_eq!(variants.len(), 3);
}

#[test]
fn sewer_runtime_manifests_never_depend_on_review_outputs() {
    fn inspect(v: &serde_json::Value) {
        match v {
            serde_json::Value::String(s) => {
                assert!(!s.contains("artifacts/"));
                if s.ends_with(".png") {
                    assert!(
                        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                            .join(s)
                            .is_file(),
                        "{s}"
                    );
                }
            }
            serde_json::Value::Array(a) => {
                for v in a {
                    inspect(v)
                }
            }
            serde_json::Value::Object(o) => {
                for v in o.values() {
                    inspect(v)
                }
            }
            _ => {}
        }
    }
    for json in [
        include_str!("../assets/maps/prison-sewer-01/entry.json"),
        include_str!("../assets/maps/prison-sewer-01/variant-1/street.json"),
        include_str!("../assets/maps/prison-sewer-01/variant-2/street.json"),
        include_str!("../assets/maps/prison-sewer-01/variant-3/street.json"),
    ] {
        inspect(&serde_json::from_str(json).unwrap());
    }
}
