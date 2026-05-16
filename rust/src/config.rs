use std::path::PathBuf;

use anyhow::{Context, Error, Result, anyhow};

use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = Error;

    fn try_from(value: Opts) -> Result<Self> {
        Ok(Config {
            operation: value.args.try_into()?,
            pwd: get_pwd(value.pwd)?,
            config: get_config(value.config)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        match value.as_slice() {
            [] => Ok(Operation::Print(None)),
            [key] => Ok(Operation::Print(Some(key.clone()))),
            [cmd, key, value] if cmd == "add" => Ok(Operation::Add(key.clone(), value.clone())),
            [cmd, key] if cmd == "remove" => Ok(Operation::Remove(key.clone())),
            [cmd, ..] if cmd == "add" => Err(anyhow!("operation add expects 2 arguments")),
            [cmd, ..] if cmd == "remove" => Err(anyhow!("operation remove expects 1 argument")),
            [..] => Err(anyhow!("operation print expects 0 or 1 arguments")),
        }
    }
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    match pwd {
        Some(path) => Ok(path),
        None => std::env::current_dir().context("failed to get pwd"),
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    match config {
        Some(path) => Ok(path),
        None => match std::env::var("XDG_CONFIG_HOME") {
            Ok(path) => Ok(PathBuf::from(path).join("projector/projector.json")),
            Err(_) => std::env::var("HOME")
                .context("failed to get config path")
                .map(|path| PathBuf::from(path).join(".projector.json")),
        },
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::{
        config::{Config, Operation},
        opts::Opts,
    };

    fn test_config(args: Vec<String>, operation: Operation) -> Result<()> {
        let config: Config = Opts {
            args,
            config: None,
            pwd: None,
        }
        .try_into()?;
        assert_eq!(config.operation, operation);
        Ok(())
    }

    fn test_error(args: Vec<String>) {
        let result: Result<Config> = Opts {
            args,
            config: None,
            pwd: None,
        }
        .try_into();
        assert!(result.is_err())
    }

    #[test]
    fn print_all() -> Result<()> {
        test_config(vec![], Operation::Print(None))
    }

    #[test]
    fn print() -> Result<()> {
        test_config(vec!["foo".into()], Operation::Print(Some("foo".into())))
    }

    #[test]
    fn add() -> Result<()> {
        test_config(
            vec!["add".into(), "foo".into(), "bar".into()],
            Operation::Add("foo".into(), "bar".into()),
        )
    }

    #[test]
    fn remove() -> Result<()> {
        test_config(
            vec!["remove".into(), "foo".into()],
            Operation::Remove("foo".into()),
        )
    }

    #[test]
    fn print_error() {
        test_error(vec!["foo".into(), "bar".into()])
    }

    #[test]
    fn add_error() {
        test_error(vec!["add".into(), "foo".into()])
    }

    #[test]
    fn remove_error() {
        test_error(vec!["remove".into(), "foo".into(), "bar".into()])
    }
}
