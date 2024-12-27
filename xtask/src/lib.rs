use anyhow::{Context, Result};
use cargo_toml::Manifest;

pub fn get_cargo_version() -> Result<String> {
    let dir = std::env::current_dir()?;

    let manifest_bytes = std::fs::read(dir.join("Cargo.toml"))?;
    let manifest = Manifest::from_slice(&manifest_bytes)
        .context("manifest should be present")?;
    let version = manifest.package.unwrap().version;

    let version_string = match version {
        cargo_toml::Inheritable::Set(v) => v,
        cargo_toml::Inheritable::Inherited { .. } => {
            unreachable!();
        },
    };
    Ok(version_string.to_string())
}
