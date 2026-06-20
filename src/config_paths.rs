use crate::model::Project;
use std::path::{Path, PathBuf};

pub const DOCUMENT_DIR_KEY: &str = "document_dir";
pub const ROADMAP_ACTIVE_PATH_KEY: &str = "roadmap_active_path";
pub const ROADMAP_ARCHIVE_PATH_KEY: &str = "roadmap_archive_path";

pub const DEFAULT_DOCUMENT_DIR: &str = "docs/designs";
pub const DEFAULT_ROADMAP_ACTIVE_PATH: &str = "ROADMAP_ACTIVE.md";
pub const DEFAULT_ROADMAP_ARCHIVE_PATH: &str = "ROADMAP_ARCHIVE.md";

pub fn document_dir(project: &Project) -> PathBuf {
    configured_path(project, DOCUMENT_DIR_KEY, DEFAULT_DOCUMENT_DIR)
}

pub fn roadmap_active_path(project_root: &Path, project: &Project) -> PathBuf {
    project_path(
        project_root,
        configured_path(
            project,
            ROADMAP_ACTIVE_PATH_KEY,
            DEFAULT_ROADMAP_ACTIVE_PATH,
        ),
    )
}

pub fn roadmap_archive_path(project_root: &Path, project: &Project) -> PathBuf {
    project_path(
        project_root,
        configured_path(
            project,
            ROADMAP_ARCHIVE_PATH_KEY,
            DEFAULT_ROADMAP_ARCHIVE_PATH,
        ),
    )
}

fn configured_path(project: &Project, key: &str, default: &str) -> PathBuf {
    project
        .config
        .get(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(default))
}

fn project_path(project_root: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        project_root.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Project;
    use indexmap::IndexMap;

    fn project_with_config(config: IndexMap<String, String>) -> Project {
        Project {
            name: "Test".to_string(),
            description: String::new(),
            version: "0.1.0".to_string(),
            milestones: Vec::new(),
            archived_milestones: Vec::new(),
            backlog_path: "roadmap/backlog.toml".to_string(),
            config,
        }
    }

    #[test]
    fn missing_config_uses_current_default_paths() {
        let project = project_with_config(IndexMap::new());
        let root = Path::new("/repo");

        assert_eq!(document_dir(&project), PathBuf::from("docs/designs"));
        assert_eq!(
            roadmap_active_path(root, &project),
            PathBuf::from("/repo/ROADMAP_ACTIVE.md")
        );
        assert_eq!(
            roadmap_archive_path(root, &project),
            PathBuf::from("/repo/ROADMAP_ARCHIVE.md")
        );
    }

    #[test]
    fn configured_paths_override_defaults() {
        let mut config = IndexMap::new();
        config.insert(DOCUMENT_DIR_KEY.to_string(), "docs/specs".to_string());
        config.insert(
            ROADMAP_ACTIVE_PATH_KEY.to_string(),
            "generated/active.md".to_string(),
        );
        config.insert(
            ROADMAP_ARCHIVE_PATH_KEY.to_string(),
            "/tmp/archive.md".to_string(),
        );
        let project = project_with_config(config);
        let root = Path::new("/repo");

        assert_eq!(document_dir(&project), PathBuf::from("docs/specs"));
        assert_eq!(
            roadmap_active_path(root, &project),
            PathBuf::from("/repo/generated/active.md")
        );
        assert_eq!(
            roadmap_archive_path(root, &project),
            PathBuf::from("/tmp/archive.md")
        );
    }
}
