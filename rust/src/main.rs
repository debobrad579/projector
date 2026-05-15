use anyhow::Result;
use clap::Parser;
use rust::{config::Config, opts::Opts};

fn main() -> Result<()> {
    let config: Config = Opts::parse().try_into()?;
    println!("{:?}", config);

    Ok(())
}
