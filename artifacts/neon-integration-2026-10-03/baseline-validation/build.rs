// Build provenance. Replay compatibility is checked through format, rules and
// the complete reconstructed state, so UI-only changes do not strand a run.
use std::{fs, path::Path};

fn sources(path: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(path).expect("read source directory") {
        let path = entry.expect("read source entry").path();
        if path.is_dir() {
            sources(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

fn main() {
    // Native visual diagnostics create several complete disposable app states.
    // MSVC's default 1 MiB stack is insufficient without optimization. Reserve
    // more virtual stack space only for the Windows debug executable.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
        && std::env::var("PROFILE").as_deref() == Ok("debug")
    {
        println!("cargo:rustc-link-arg-bin=project-rl=/STACK:8388608");
    }
    let mut files = Vec::new();
    sources(Path::new("src"), &mut files);
    sources(Path::new("vendor/miniquad/src"), &mut files);
    files.push("vendor/miniquad/Cargo.toml".into());
    files.push("vendor/miniquad/build.rs".into());
    files.extend(["Cargo.toml", "Cargo.lock", "build.rs"].map(std::path::PathBuf::from));
    files.sort();
    let mut hash = 0xcbf29ce484222325_u64;
    for path in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let name = path.to_string_lossy().replace('\\', "/");
        for byte in name
            .bytes()
            .chain(fs::read(&path).expect("read build input"))
        {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
    }
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rustc-env=PROJECT_RL_BUILD_FINGERPRINT={hash:016x}");
}
