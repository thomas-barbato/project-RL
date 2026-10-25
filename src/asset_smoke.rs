//! Native installation check, available in Release without the review/capture banks.
use macroquad::prelude::*;

pub async fn run(output: &std::path::Path) -> Result<(), String> {
    let files = project_rl::data_paths::verify()?;
    let manifest = project_rl::data_paths::manifest_at(project_rl::data_paths::root())?;
    let app = crate::ascii_app::AsciiApp::new()?;
    let _dialogue = crate::ascii_app::TerminalDialogue::load().await?;
    app.draw();
    next_frame().await;
    let mut pngs = 0;
    let mut atmospheres = 0;
    for (index, file) in manifest.files.iter().enumerate() {
        if file.path.ends_with(".png") {
            let bytes = project_rl::data_paths::read(&file.path)?;
            let image = Image::from_file_with_format(&bytes, Some(ImageFormat::Png))
                .map_err(|e| format!("{}: {e}", file.path))?;
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            pngs += 1;
            drop(texture);
        } else if file.path.starts_with("assets/maps/") && file.path.ends_with(".json") {
            let value: serde_json::Value =
                serde_json::from_slice(&project_rl::data_paths::read(&file.path)?)
                    .map_err(|e| e.to_string())?;
            if let Some(config) = value.get("atmosphere") {
                let config: crate::camp_atmosphere::Config =
                    serde_json::from_value(config.clone()).map_err(|e| e.to_string())?;
                let width = value["width"].as_u64().ok_or("map width")? as u32 * 64;
                let height = value["height"].as_u64().ok_or("map height")? as u32 * 64;
                let _atmosphere = crate::camp_atmosphere::Atmosphere::new(&config, width, height);
                atmospheres += 1;
            }
        }
        // Flush deferred texture destruction and keep the window responsive.
        if index % 8 == 0 {
            next_frame().await;
        }
    }
    let bytes =
        include_bytes!("../assets/animations/agent/skill-animation-review-06/flame-atlas.png");
    let first = crate::effect_geometry::LazyTexture::fitted(bytes, FilterMode::Linear);
    let second = crate::effect_geometry::LazyTexture::fitted(bytes, FilterMode::Linear);
    if !std::ptr::eq::<Texture2D>(&*first, &*second) {
        return Err("Animation texture was uploaded twice".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let report = serde_json::json!({"status":"passed", "root":project_rl::data_paths::root(), "packed":project_rl::data_paths::source()?.is_packed(), "files":files, "pngs_decoded_and_uploaded":pngs, "atmospheres":atmospheres, "shared_animation_texture":true, "content_and_menu":true, "dialogue_audio_loaded":true});
    std::fs::write(
        output.join("native-assets.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
