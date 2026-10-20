//! Old agent-v14 caches are migrated only after complete historical proof.
//! Every fixture is in memory; no user suspension is read, written or removed.
use super::*;

fn old_agent_v14_fixture() -> (Suspension, Vec<u8>) {
    let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let class_id: CharacterClassId = "core:arcaniste".parse().unwrap();
    let classes = ascii_character_class_catalog().unwrap();
    let class = classes.get(&class_id).unwrap();
    class
        .apply_to_rules(&mut rules, class.recommended_attributes())
        .unwrap();
    weapon_supplies_app::ensure_class_weapon_supplies(
        &mut rules,
        CRYOGENIC_CAMP_GENERATION_VERSION,
    );
    let mut app = AsciiApp::from_seed_version(
        INITIAL_SEED,
        rules,
        texts,
        loot,
        expeditions,
        CRYOGENIC_CAMP_GENERATION_VERSION,
    )
    .unwrap();
    app.character_class = Some(class_id);
    // App rules are a content template; the actual engine owns the versioned
    // hands/loadout. Preserve that distinction exactly as a real campaign does.
    let mut old_engine_rules = app.game.rules().clone();
    old_engine_rules.statuses =
        project_rl::skills::agent::legacy_v14_status_definitions(&old_engine_rules.statuses);
    app.rules.statuses =
        project_rl::skills::agent::legacy_v14_status_definitions(&app.rules.statuses);
    let engine = project_rl::skills::agent::legacy_v14_fixture_bytes(&app.game).unwrap();
    app.game = WorldState::from_recovery_snapshot_bytes(&engine, old_engine_rules.clone()).unwrap();
    crate::cryogenic_camp::restore_obstructions(&mut app.game).unwrap();
    let mut saved = app.suspension().unwrap();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(saved.recovery.as_ref().unwrap().snapshot_state().unwrap())
        .unwrap();
    let mut cache: AppRecoverySnapshot = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(&bytes)
        .unwrap();
    cache.engine = engine.clone();
    let cache = bincode::serialize(&cache).unwrap();
    saved.recovery = Some(
        RecoveryPayload::snapshot(
            saved.commands.len(),
            base64::engine::general_purpose::STANDARD.encode(cache),
        )
        .unwrap(),
    );
    saved.build = format!(
        "{:016x}",
        u64::from_str_radix(env!("PROJECT_RL_BUILD_FINGERPRINT"), 16).unwrap() ^ 1
    );
    saved.validate().unwrap();
    // Check the packed app cache independently before exercising migration.
    let decoded = AsciiApp::decode_recovery_snapshot(
        saved.recovery.as_ref().unwrap().snapshot_state().unwrap(),
        old_engine_rules,
    )
    .unwrap();
    let expected_debug = format!("{:#?}", app.game);
    let decoded_debug = format!("{:#?}", decoded.game);
    let first_difference = expected_debug
        .lines()
        .zip(decoded_debug.lines())
        .enumerate()
        .find(|(_, (expected, actual))| expected != actual);
    assert_eq!(
        suspension::fingerprint(&decoded.game),
        saved.state,
        "historical fixture must retain its exact engine rules and complete state; first difference: {first_difference:?}; live fingerprint: {}",
        suspension::fingerprint(&app.game)
    );
    (saved, engine)
}

fn restore(saved: &Suspension, rules: GameRules) -> Result<AsciiApp, String> {
    let (_, texts, loot, expeditions) = ascii_game_content().unwrap();
    AsciiApp::restore_suspension(saved, rules, texts, loot, expeditions)
}

#[test]
fn interim_empty_obstruction_hash_is_verified_for_snapshot_and_replay() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app =
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 108).unwrap();
    let saved = project_rl::world::with_explicit_empty_obstructions(|| app.suspension().unwrap());
    assert_ne!(saved.state, suspension::fingerprint(&app.game));
    let original = serde_json::to_vec(&saved).unwrap();
    for replay in [false, true] {
        let mut candidate = saved.clone();
        if replay {
            candidate.build = "0000000000000000".into();
        }
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let restored = restore(&candidate, rules).unwrap();
        assert_eq!(
            restored.game.recovery_snapshot_bytes().unwrap(),
            app.game.recovery_snapshot_bytes().unwrap()
        );
        candidate.state ^= 1;
        let (rules, _, _, _) = ascii_game_content().unwrap();
        assert!(restore(&candidate, rules).is_err());
    }
    assert_eq!(serde_json::to_vec(&saved).unwrap(), original);
}

#[test]
fn agent_v14_resume_verifies_old_rules_and_state_then_adds_only_builtin_definitions() {
    let (saved, engine) = old_agent_v14_fixture();
    let before = serde_json::to_vec(&saved).unwrap();
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let restored = restore(&saved, rules).unwrap();
    assert_eq!(
        project_rl::skills::agent::legacy_v14_fixture_bytes(&restored.game).unwrap(),
        engine,
        "all actor, inventory, progression, RNG and zone bytes remain identical"
    );
    assert!(
        restored
            .game
            .rules()
            .statuses
            .contains(&"agent:frozen".parse().unwrap())
    );
    assert!(
        restored
            .game
            .rules()
            .statuses
            .contains(&"agent:mine_frozen".parse().unwrap())
    );
    assert!(
        restored
            .game
            .rules()
            .statuses
            .contains(&"agent:provoked".parse().unwrap())
    );
    assert_ne!(
        rules_fingerprint_for_version(&restored.rules, saved.version),
        saved.rules
    );
    assert_eq!(
        serde_json::to_vec(&saved).unwrap(),
        before,
        "input suspension is immutable"
    );
}

#[test]
fn agent_v14_resume_refuses_a_modified_status_definition_in_current_content() {
    let (saved, _) = old_agent_v14_fixture();
    let (mut rules, _, _, _) = ascii_game_content().unwrap();
    let id = "agent:frozen".parse().unwrap();
    rules.statuses = rules.statuses.without_id(&id);
    rules
        .statuses
        .register(
            project_rl::status::StatusDefinition::new(
                id,
                Some(99),
                project_rl::status::StatusStacking::KeepExisting,
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    assert!(
        matches!(restore(&saved,rules),Err(ref e)if e=="Les règles ou les mods ont changé ; suspension conservée.")
    );
}

#[test]
fn agent_v14_resume_refuses_a_falsified_state_hash_without_replaying() {
    let (mut saved, _) = old_agent_v14_fixture();
    saved.state ^= 1;
    let before = serde_json::to_vec(&saved).unwrap();
    let (rules, _, _, _) = ascii_game_content().unwrap();
    assert!(
        matches!(restore(&saved,rules),Err(ref e)if e=="L'état reconstruit ne correspond pas à la suspension ; fichier conservé.")
    );
    assert_eq!(serde_json::to_vec(&saved).unwrap(), before);
}

#[test]
fn agent_v14_resume_requires_the_exact_old_engine_schema_and_a_snapshot() {
    let (mut saved, _) = old_agent_v14_fixture();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(saved.recovery.as_ref().unwrap().snapshot_state().unwrap())
        .unwrap();
    let mut cache: AppRecoverySnapshot = bincode::deserialize(&bytes).unwrap();
    cache.engine[4] = 15;
    saved.recovery = Some(
        RecoveryPayload::snapshot(
            saved.commands.len(),
            base64::engine::general_purpose::STANDARD.encode(bincode::serialize(&cache).unwrap()),
        )
        .unwrap(),
    );
    let (rules, _, _, _) = ascii_game_content().unwrap();
    assert!(restore(&saved, rules.clone()).is_err());
    saved.recovery = Some(RecoveryPayload::replay(saved.commands.len()).unwrap());
    assert!(restore(&saved, rules).is_err());
}
