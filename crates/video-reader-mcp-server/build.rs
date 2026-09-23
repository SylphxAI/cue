use std::path::Path;

fn main() {
    let version = product_version();
    println!("cargo:rustc-env=CUE_PRODUCT_VERSION={version}");
}

fn product_version() -> String {
    println!("cargo:rerun-if-env-changed=CUE_PRODUCT_VERSION");
    if let Ok(version) = std::env::var("CUE_PRODUCT_VERSION") {
        if !version.is_empty() {
            return version;
        }
    }
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let package_json = Path::new(&manifest_dir).join("../../package.json");
    println!("cargo:rerun-if-changed={}", package_json.display());
    read_version(&package_json)
}

fn read_version(path: &Path) -> String {
    let text = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("\"version\"") else {
            continue;
        };
        let rest = rest.trim().trim_start_matches(':').trim();
        let version = rest.trim_matches(|c| c == '"' || c == ',');
        if !version.is_empty() {
            return version.to_string();
        }
    }
    panic!("no version field in {}", path.display());
}
