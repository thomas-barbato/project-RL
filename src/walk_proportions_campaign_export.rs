//! Export canonical CPU references; no game state or GPU context is created.
use super::*;

#[test]
#[ignore = "Explicit campaign reference audit; RL_PROPORTIONS_OUTPUT required"]
fn audit_all_campaign_proportions_diagnostic() {
    let output = std::path::PathBuf::from(
        std::env::var("RL_PROPORTIONS_OUTPUT").expect("audit output required"),
    )
    .join("campaign");
    std::fs::create_dir_all(&output).unwrap();
    assert!(!walking::corrections_enabled());
    let candidate = std::env::var_os("RL_PROPORTIONS_CANDIDATE").is_some();
    let _review = candidate.then(walking::review_corrections);
    let _proportions = walking::review_proportions(candidate);
    let pairs: std::collections::BTreeSet<_> = project_rl::monster_art::MONSTER_ART
        .iter()
        .map(|a| (a.number, a.variant))
        .chain(project_rl::monster_palettes::all_palettes().map(|a| (a.number, a.variant)))
        .collect();
    let mut references = Vec::new();
    for (number, variant) in pairs {
        let art = project_rl::monster_art::art(number, variant).unwrap();
        let reference = campaign::reference_image(number, variant).unwrap();
        let file = format!("MON-{number:03}-{variant}-front.png");
        reference.export_png(output.join(&file).to_str().unwrap());
        references.push(serde_json::json!({"number":number,"variant":variant,"name":art.name,"role":format!("{:?}",art.role),
            "side":art.side,"foot":art.foot,"image_size":[reference.width,reference.height],"file":file,
            "normal_cycle_active":walking::creature_selection(Visual::CampaignMonster{number,variant},walking::Pose{facing:1,frame:1}).is_some()}));
    }
    assert_eq!(references.len(), 471);
    std::fs::write(
        output.join("references.json"),
        serde_json::to_vec_pretty(
            &serde_json::json!({"exported_pngs_need_vertical_flip":true,"references":references}),
        )
        .unwrap(),
    )
    .unwrap();
}
