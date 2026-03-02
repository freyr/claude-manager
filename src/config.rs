/// User configuration loaded from `~/.config/jigolo/config.toml`.
///
/// All fields are optional so a partial config file works. Missing file
/// returns `Config::default()` (all `None`).
use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

/// User preferences persisted across sessions.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Color theme: `"dark"` or `"light"`.
    #[serde(default)]
    pub theme: Option<String>,
    /// Default directories to scan when no CLI paths are provided.
    #[serde(default)]
    pub default_paths: Option<Vec<PathBuf>>,
    /// Default maximum scan depth (overrides the built-in default of 3).
    #[serde(default)]
    pub default_depth: Option<usize>,
}

/// Returns the default config file path using the `HOME` environment
/// variable.
pub fn config_path() -> Option<PathBuf> {
    let home = env::var("HOME").ok()?;
    Some(config_path_in(&PathBuf::from(home)))
}

/// Returns the config file path relative to a given home directory.
pub fn config_path_in(home: &Path) -> PathBuf {
    home.join(".config").join("jigolo").join("config.toml")
}

/// Loads configuration from the default path, returning `Config::default()`
/// if the file does not exist.
pub fn load_config() -> Result<Config> {
    match config_path() {
        Some(path) => load_config_from(&path),
        None => Ok(Config::default()),
    }
}

/// Loads configuration from a specific path. Returns `Config::default()`
/// if the file does not exist.
pub fn load_config_from(path: &Path) -> Result<Config> {
    match fs::read_to_string(path) {
        Ok(contents) => {
            let config: Config = toml::from_str(&contents)
                .with_context(|| format!("failed to parse {}", path.display()))?;
            Ok(config)
        }
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(Config::default()),
        Err(err) => Err(anyhow::anyhow!(
            "failed to read {}: {}",
            path.display(),
            err
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn default_config_has_all_none() {
        let config = Config::default();
        assert_eq!(config.theme, None);
        assert_eq!(config.default_paths, None);
        assert_eq!(config.default_depth, None);
    }

    #[test]
    fn config_path_in_returns_expected_path() {
        let home = PathBuf::from("/home/testuser");
        let path = config_path_in(&home);
        assert_eq!(
            path,
            PathBuf::from("/home/testuser/.config/jigolo/config.toml")
        );
    }

    #[test]
    fn load_config_from_missing_file_returns_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nonexistent.toml");
        let config = load_config_from(&path).unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn load_config_from_parses_all_fields() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(
            &path,
            r#"
theme = "light"
default_paths = ["/a", "/b"]
default_depth = 5
"#,
        )
        .unwrap();

        let config = load_config_from(&path).unwrap();
        assert_eq!(config.theme.as_deref(), Some("light"));
        assert_eq!(
            config.default_paths,
            Some(vec![PathBuf::from("/a"), PathBuf::from("/b")])
        );
        assert_eq!(config.default_depth, Some(5));
    }

    #[test]
    fn load_config_from_parses_partial_config() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "theme = \"dark\"\n").unwrap();

        let config = load_config_from(&path).unwrap();
        assert_eq!(config.theme.as_deref(), Some("dark"));
        assert_eq!(config.default_paths, None);
        assert_eq!(config.default_depth, None);
    }

    #[test]
    fn load_config_from_empty_file_returns_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "").unwrap();

        let config = load_config_from(&path).unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn load_config_from_invalid_toml_returns_error() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, "not valid toml {{{").unwrap();

        let result = load_config_from(&path);
        assert!(result.is_err());
    }
}
