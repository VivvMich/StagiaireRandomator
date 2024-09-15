
mod load;
mod add;
mod trainee;
mod shuffle;
//mod shuffle;


use std::collections::HashMap;
use std::fs::File;
use clap::{Parser, Subcommand};
use crate::trainee::Trainee;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Commands,
}



#[derive(clap::Subcommand)]
enum Commands{
    Add{
        name: String,
        skill_value: u8
    },
    // Shuffle{},
    // Purge {}
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let mut file = File::open("data.json")?;
    let mut trainees: Vec<Trainee> = load::load(&file)?; // posession, load

    match args.subcommand {
        Commands::Add {name, skill_value } => add::add(name, skill_value, &mut trainees, file)?//add::add(name, skill_value, &mut list, &mut file)?,
    //     Commands::Shuffle {} => print!("Shuffle !"),
    //     Commands::Purge {} => print!("Purge !")
    }

    Ok(())
}
