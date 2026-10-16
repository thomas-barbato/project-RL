//! Optional shared artwork for native reviews; the accepted bank remains the default.
pub(crate) fn mark_reticle_preview_path() -> Option<std::path::PathBuf> {
    #[cfg(debug_assertions)]
    if std::env::args().any(|arg| arg == "--apercu-viseur-seul") {
        return Some(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("artifacts/refonte-agent-2026-10-06/skill-icon-review-55/T01-C03-64.png"),
        );
    }
    None
}
