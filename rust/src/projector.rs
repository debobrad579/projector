use std::{collections::HashMap, iter, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Data {
    projector: HashMap<PathBuf, HashMap<String, String>>,
}

#[derive(Debug)]
pub struct Projector<'a> {
    config: &'a Config,
    data: Data,
}

impl<'a> From<&'a Config> for Projector<'a> {
    fn from(config: &'a Config) -> Self {
        match std::fs::exists(&config.config) {
            Ok(true) => Projector {
                data: serde_json::from_str(
                    &std::fs::read_to_string(&config.config).unwrap_or_default(),
                )
                .unwrap_or_default(),
                config,
            },
            Ok(false) | Err(_) => Projector {
                data: Data::default(),
                config,
            },
        }
    }
}

impl<'a> Projector<'a> {
    pub fn save(&self) -> Result<(), std::io::Error> {
        if let Some(parent) = self.config.config.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&self.config.config, serde_json::to_string(&self.data)?)
    }

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

        fs::write(
            &path,
            serde_json::to_string(&serde_json::json!({
                "projector": {
                    "/": {
                        "foo": "bar1",
                        "bar": "baz"
                    },
                    "/foo": {
                        "foo": "bar2"
                    },
                    "/foo/bar": {
                        "foo": "bar3"
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();

        (dir, path.to_string_lossy().to_string())
    }

    fn make_config(config_path: &str, pwd: &str) -> Config {
        Config {
            operation: crate::config::Operation::Print(None),
            pwd: PathBuf::from(pwd),
            config: PathBuf::from(config_path),
        }
    }

    fn make_projector<'a>(config: &'a Config) -> Projector<'a> {
        Projector::from(config)
    }

    #[test]
    fn get_value_all() {
        let (_dir, config_path) = write_config();
        let config = make_config(&config_path, "/foo/bar");
        let projector = make_projector(&config);

        let expected: HashMap<String, String> = vec![
            ("foo".to_string(), "bar3".to_string()),
            ("bar".to_string(), "baz".to_string()),
        ]
        .into_iter()
        .collect();

        assert_eq!(projector.get_value_all(), expected);
    }

    #[test]
    fn get_value() {
        let (_dir, config_path) = write_config();
        let config = make_config(&config_path, "/foo/bar");
        let projector = make_projector(&config);

        assert_eq!(projector.get_value("foo"), Some(&"bar3".to_string()));
        assert_eq!(projector.get_value("bar"), Some(&"baz".to_string()));
    }

    #[test]
    fn set_value() {
        let (_dir, config_path) = write_config();
        let config = make_config(&config_path, "/foo/bar");
        let mut projector = make_projector(&config);

        projector.set_value("foo", "bar");
        assert_eq!(projector.get_value("foo"), Some(&"bar".to_string()));

        projector.set_value("bar", "baz2");
        assert_eq!(projector.get_value("bar"), Some(&"baz2".to_string()));

        projector.set_value("baz", "bar");
        assert_eq!(projector.get_value("baz"), Some(&"bar".to_string()));
    }

    #[test]
    fn remove_value() {
        let (_dir, config_path) = write_config();
        let config = make_config(&config_path, "/foo/bar");
        let mut projector = make_projector(&config);

        projector.remove_value("foo");
        assert_eq!(projector.get_value("foo"), Some(&"bar2".to_string()));

        projector.remove_value("bar");
        assert_eq!(projector.get_value("bar"), Some(&"baz".to_string()));
    }
}
