use crate::types::{DiscoveredSkill, SkillMetadata, SkillProvider, SkillSourceKind};
use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::debug;

pub fn discover_skills(home_dir: &Path, project_dir: Option<&Path>) -> Vec<DiscoveredSkill> {
    let mut skills = Vec::new();

    // Home directory skills
    let home_skills_dir = home_dir.join(".agents").join("skills");
    if let Ok(entries) = std::fs::read_dir(&home_skills_dir) {
        for entry in entries.flatten() {
            if let Ok(skill) = discover_one(&entry.path(), SkillSourceKind::Home, "home") {
                skills.push(skill);
            }
        }
    }

    // Project directory skills
    if let Some(project) = project_dir {
        let repo_skills_dir = project.join(".agents").join("skills");
        if let Ok(entries) = std::fs::read_dir(&repo_skills_dir) {
            for entry in entries.flatten() {
                if let Ok(skill) = discover_one(&entry.path(), SkillSourceKind::Repo, "repo") {
                    skills.push(skill);
                }
            }
        }
    }

    skills
}

fn discover_one(
    path: &Path,
    source_kind: SkillSourceKind,
    source_label: &str,
) -> Result<DiscoveredSkill> {
    let skill_file = path.join("SKILL.md");
    if !skill_file.exists() {
        anyhow::bail!("No SKILL.md in {}", path.display());
    }

    let metadata = parse_skill_metadata(&skill_file)?;
    let id = format!(
        "{}-{}",
        source_label,
        metadata.name.replace(' ', "-").to_lowercase()
    );

    Ok(DiscoveredSkill {
        id,
        name: metadata.name,
        description: metadata.description,
        providers: vec![SkillProvider::AgentSkills],
        source_kind,
        source_label: source_label.to_string(),
        root_path: path.to_path_buf(),
        skill_file_path: skill_file,
        installed: true,
    })
}

fn parse_skill_metadata(skill_file: &Path) -> Result<SkillMetadata> {
    let content = std::fs::read_to_string(skill_file)?;

    // Parse YAML frontmatter
    let (frontmatter, _) = parse_frontmatter(&content);
    let name = extract_field(&frontmatter, "name").unwrap_or_else(|| {
        // Fallback: first heading
        content
            .lines()
            .find(|l| l.starts_with('#'))
            .map(|l| l.trim_start_matches('#').trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    });
    let description = extract_field(&frontmatter, "description").unwrap_or_default();

    Ok(SkillMetadata {
        name,
        description,
        source_path: skill_file.to_path_buf(),
    })
}

fn parse_frontmatter(content: &str) -> (String, &str) {
    if content.starts_with("---") {
        let rest = &content[3..];
        if let Some(end) = rest.find("---") {
            let fm = rest[..end].to_string();
            let body = &rest[end + 3..];
            return (fm, body);
        }
    }
    (String::new(), content)
}

fn extract_field(frontmatter: &str, field: &str) -> Option<String> {
    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix(&format!("{}:", field)) {
            let value = value.trim();
            let value = value.trim_matches('"').trim_matches('\'');
            return Some(value.to_string());
        }
    }
    None
}
