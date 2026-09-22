use std::fs;
use std::path::Path;
use trex_skills::discovery::discover_skills;
use trex_skills::manifest::{compute_digest, parse_manifest};
use trex_skills::types::{FileClassification, SkillFile, SkillSourceKind};

fn write_skill_dir(root: &Path, name: &str, description: &str) {
    let dir = root.join(".agents").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("SKILL.md"),
        format!("---\nname: {}\ndescription: {}\n---\n\n# {}\n\nBody\n", name, description, name),
    )
    .unwrap();
}

#[test]
fn discovers_home_and_repo_skills() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let project = tmp.path().join("project");
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(&project).unwrap();

    write_skill_dir(&home, "Approval Flow", "review flow");
    write_skill_dir(&project, "Build", "build steps");

    let found = discover_skills(&home, Some(&project));
    assert_eq!(found.len(), 2);

    let approval = found.iter().find(|s| s.source_kind == SkillSourceKind::Home).unwrap();
    assert_eq!(approval.id, "home-approval-flow");
    assert_eq!(approval.name, "Approval Flow");
    assert_eq!(approval.description, "review flow");
    assert!(approval.installed);

    let build = found.iter().find(|s| s.source_kind == SkillSourceKind::Repo).unwrap();
    assert_eq!(build.id, "repo-build");
}

#[test]
fn discovers_none_when_no_skills_present() {
    let tmp = tempfile::tempdir().unwrap();
    assert!(discover_skills(tmp.path(), None).is_empty());
}

#[test]
fn digest_is_stable_regardless_of_file_order() {
    let files = vec![
        SkillFile {
            path: "b.txt".to_string(),
            size: 2,
            executable: false,
            classification: FileClassification::Text,
            sha256: String::new(),
        },
        SkillFile {
            path: "a.txt".to_string(),
            size: 1,
            executable: false,
            classification: FileClassification::Text,
            sha256: String::new(),
        },
    ];
    let forward = compute_digest(&files);
    let mut reversed = files.clone();
    reversed.reverse();
    assert_eq!(compute_digest(&reversed), forward);
    assert!(!forward.is_empty());
}

#[test]
fn install_extracts_manifest_and_files() {
    let tmp = tempfile::tempdir().unwrap();
    let source = tmp.path().join("src");
    let target = tmp.path().join("installed");
    fs::create_dir_all(&source).unwrap();

    fs::write(source.join("SKILL.md"), "# Demo\n\nBody\n").unwrap();
    let files = vec![SkillFile {
        path: "SKILL.md".to_string(),
        size: 9,
        executable: false,
        classification: FileClassification::Text,
        sha256: String::new(),
    }];
    let digest = compute_digest(&files);

    let manifest = serde_json::json!({
        "schema_version": 1,
        "package_id": "demo",
        "version_id": "1.0.0",
        "name": "demo",
        "description": "demo skill",
        "created_at": "2026-01-01T00:00:00Z",
        "files": [{
            "path": "SKILL.md",
            "size": 9,
            "executable": false,
            "classification": "Text",
            "sha256": ""
        }],
        "package_digest": digest,
    });
    fs::write(source.join("manifest.json"), serde_json::to_string_pretty(&manifest).unwrap()).unwrap();

    let archive_path = tmp.path().join("demo.tar.gz");
    let tar_file = fs::File::create(&archive_path).unwrap();
    let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(tar_file, flate2::Compression::default()));
    for entry in fs::read_dir(&source).unwrap() {
        let entry = entry.unwrap();
        tar.append_path_with_name(&entry.path(), entry.file_name()).unwrap();
    }
    let enc = tar.into_inner().unwrap();
    enc.finish().unwrap();

    let result = trex_skills::install::install_skill(&archive_path, &target, SkillSourceKind::Home).unwrap();
    assert!(result.installed_path.join("SKILL.md").exists());
    assert_eq!(result.manifest.name, "demo");
    assert_eq!(result.manifest.package_digest, digest);

    let manifest_path = result.installed_path.join("manifest.json");
    assert!(manifest_path.exists());
}

#[test]
fn install_rejects_bad_schema_version() {
    let tmp = tempfile::tempdir().unwrap();
    let source = tmp.path().join("src");
    let target = tmp.path().join("installed");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("bad.txt"), "x").unwrap();

    let manifest = serde_json::json!({
        "schema_version": 2,
        "package_id": "bad",
        "version_id": "1.0.0",
        "name": "bad",
        "description": "",
        "created_at": "2026-01-01T00:00:00Z",
        "files": [],
        "package_digest": "abc"
    });
    fs::write(source.join("manifest.json"), serde_json::to_string(&manifest).unwrap()).unwrap();

    let result = parse_manifest(&source.join("manifest.json"));
    assert!(result.is_err());
    let err = result.err().unwrap().to_string();
    assert!(err.contains("Unsupported schema version"));

    let archive_path = tmp.path().join("bad.tar.gz");
    let tar_file = fs::File::create(&archive_path).unwrap();
    let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(tar_file, flate2::Compression::default()));
    for entry in fs::read_dir(&source).unwrap() {
        let entry = entry.unwrap();
        tar.append_path_with_name(&entry.path(), entry.file_name()).unwrap();
    }
    let enc = tar.into_inner().unwrap();
    enc.finish().unwrap();

    assert!(trex_skills::install::install_skill(&archive_path, &target, SkillSourceKind::Home).is_err());
}