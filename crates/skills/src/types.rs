use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub schema_version: u32,
    pub package_id: String,
    pub version_id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub files: Vec<SkillFile>,
    pub package_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    pub path: String,
    pub size: u64,
    pub executable: bool,
    pub classification: FileClassification,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileClassification {
    Text,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub source_path: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillSourceKind {
    Home,
    Repo,
    Bundled,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillProvider {
    Codex,
    Claude,
    AgentSkills,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub providers: Vec<SkillProvider>,
    pub source_kind: SkillSourceKind,
    pub source_label: String,
    pub root_path: std::path::PathBuf,
    pub skill_file_path: std::path::PathBuf,
    pub installed: bool,
}
