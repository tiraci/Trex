use crate::types::{SkillFile, SkillManifest};
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub fn parse_manifest(path: &Path) -> Result<SkillManifest> {
    let content = fs::read_to_string(path)?;
    let manifest: SkillManifest = serde_json::from_str(&content)?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

fn validate_manifest(manifest: &SkillManifest) -> Result<()> {
    if manifest.schema_version != 1 {
        anyhow::bail!("Unsupported schema version: {}", manifest.schema_version);
    }
    if manifest.files.is_empty() {
        anyhow::bail!("Manifest has no files");
    }
    if manifest.files.len() > 512 {
        anyhow::bail!("Too many files: {}", manifest.files.len());
    }
    Ok(())
}

pub fn compute_digest(files: &[SkillFile]) -> String {
    let mut hasher = Sha256::new();
    let mut sorted = files.to_vec();
    sorted.sort_by(|a, b| a.path.cmp(&b.path));
    for file in &sorted {
        hasher.update(file.path.as_bytes());
        hasher.update(file.size.to_le_bytes());
    }
    format!("{:x}", hasher.finalize())
}
