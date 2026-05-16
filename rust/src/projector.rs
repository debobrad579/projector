use std::{collections::HashMap, fs::File, iter, path::PathBuf};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

impl TryFrom<Config> for Projector {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self> {
        match std::fs::exists(&config.config) {
            Ok(true) => Ok(Projector {
                data: serde_json::from_str(&std::fs::read_to_string(&config.config)?)?,
                config,
            }),
            Ok(false) => File::create(&config.config).map(|_| {
                Ok(Projector {
                    data: Data {
                        projector: HashMap::new(),
                    },
                    config,
                })
            })?,
            Err(e) => Err(e.into()),
        }
    }
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<String, String> {
        iter::successors(Some(self.config.pwd.as_path()), |p| p.parent())
            .filter_map(|p| self.data.projector.get(p))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .cloned()
            .fold(HashMap::new(), |mut acc, m| {
                acc.extend(m);
                acc
            })
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        iter::successors(Some(self.config.pwd.as_path()), |p| p.parent())
            .find_map(|p| self.data.projector.get(p).and_then(|v| v.get(key)))
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Option<String> {
        self.data
            .projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key.to_string(), value.to_string())
    }

    pub fn remove_value(&mut self, key: &str) -> Option<String> {
        self.data
            .projector
            .get_mut(&self.config.pwd)
            .map(|x| x.remove(key))?
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{collections::HashMap, fs, path::PathBuf};
    use tempfile::TempDir;

    fn write_config() -> (TempDir, String) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("projector.json");

        let data: HashMap<PathBuf, HashMap<String, String>> = [
            (
                "/".into(),
                [
                    ("foo".to_string(), "bar1".to_string()),
                    ("bar".to_string(), "baz".to_string()),
                ]
                .into(),
            ),
            (
                "/foo".into(),
                [("foo".to_string(), "bar2".to_string())].into(),
            ),
            (
                "/foo/bar".into(),
                [("foo".to_string(), "bar3".to_string())].into(),
            ),
        ]
        .into();

        fs::write(
            &path,
            serde_json::to_string(&serde_json::json!({ "projector": data })).unwrap(),
        )
        .unwrap();

        (dir, path.to_string_lossy().to_string())
    }

    fn get_projector(pwd: &str) -> Projector {
        let (_dir, config_path) = write_config();
        Config {
            operation: crate::config::Operation::Print(None),
            pwd: PathBuf::from(pwd),
            config: PathBuf::from(config_path),
        }
        .try_into()
        .unwrap()
    }

    #[test]
    fn test_get_value_all() {
        let expected: HashMap<String, String> = vec![
            ("foo".to_string(), "bar3".to_string()),
            ("bar".to_string(), "baz".to_string()),
        ]
        .into_iter()
        .collect();

        assert_eq!(get_projector("/foo/bar").get_value_all(), expected);
    }

    #[test]
    fn test_get_value() {
        let projector = get_projector("/foo/bar");

        assert_eq!(projector.get_value("foo"), Some(&"bar3".to_string()));
        assert_eq!(projector.get_value("bar"), Some(&"baz".to_string()));
    }

    #[test]
    fn test_set_value() {
        let mut projector = get_projector("/foo/bar");

        projector.set_value("foo", "bar");
        assert_eq!(projector.get_value("foo"), Some(&"bar".to_string()));

        projector.set_value("bar", "baz2");
        assert_eq!(projector.get_value("bar"), Some(&"baz2".to_string()));

        projector.set_value("baz", "bar");
        assert_eq!(projector.get_value("baz"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_remove_value() {
        let mut projector = get_projector("/foo/bar");

        projector.remove_value("foo");
        assert_eq!(projector.get_value("foo"), Some(&"bar2".to_string()));

        projector.remove_value("bar");
        assert_eq!(projector.get_value("bar"), Some(&"baz".to_string()));
    }
}
