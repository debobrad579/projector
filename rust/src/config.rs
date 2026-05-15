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

#[derive(Debug)]
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
        None => std::env::var("XDG_CONFIG_HOME")
            .context("unable to get XDG_CONFIG_HOME")
            .map(|path| PathBuf::from(path).join("projector/projector.json")),
    }
}
