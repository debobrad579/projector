use anyhow::Result;
use clap::Parser;
use rust::{
    config::{Config, Operation},
    opts::Opts,
    projector::Projector,
};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    let mut projector: Projector = Projector::from(&config);

    match &config.operation {
        Operation::Print(None) => {
            println!("{}", serde_json::to_string(&projector.get_value_all())?)
        }
        Operation::Print(Some(key)) => {
            if let Some(value) = projector.get_value(key) {
                println!("{}", value)
            }
        }
        Operation::Add(key, value) => {
            projector.set_value(key, value);
            projector.save()?
        }
        Operation::Remove(key) => {
            projector.remove_value(key);
            projector.save()?
        }
    };

    Ok(())
}
