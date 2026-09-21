use crate::manifest::parse_manifest;
use crate::types::{SkillManifest, SkillSourceKind};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;
use tracing::info;

pub struct InstallResult {
    pub installed_path: PathBuf,
    pub manifest: SkillManifest,
}

pub fn install_skill(
    archive_path: &Path,
    target_dir: &Path,
    _source_kind: SkillSourceKind,
) -> Result<InstallResult> {
    let file = fs::File::open(archive_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    // Extract to temp dir first
    let temp_dir = tempdir::TempDir::new("skill-install")?;
    archive.unpack(temp_dir.path())?;

    // Find manifest
    let manifest_path = find_manifest(temp_dir.path())?;
    let manifest = parse_manifest(&manifest_path)?;

    // Validate digest
    let computed = crate::manifest::compute_digest(&manifest.files);
    if computed != manifest.package_digest {
        anyhow::bail!("Digest mismatch: expected {}, got {}", manifest.package_digest, computed);
    }

    // Install to target
    let skill_dir = target_dir.join(&manifest.name);
    if skill_dir.exists() {
        fs::remove_dir_all(&skill_dir)?;
    }
    fs::create_dir_all(&skill_dir)?;

    // Copy files from temp to target
    copy_dir_all(temp_dir.path(), &skill_dir)?;

    info!(
        name = %manifest.name,
        version = %manifest.version_id,
        path = %skill_dir.display(),
        "Skill installed"
    );

    Ok(InstallResult {
        installed_path: skill_dir,
        manifest,
    })
}

fn find_manifest(dir: &Path) -> Result<PathBuf> {
    let manifest_path = dir.join("manifest.json");
    if manifest_path.exists() {
        return Ok(manifest_path);
    }
    anyhow::bail!("No manifest.json found in {}", dir.display())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
