use super::*;

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
