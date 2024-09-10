mod add;

use std::collections::HashMap;
use clap::{Parser, Subcommand};

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
    Shuffle{
    }
}

fn main() {
    let args = Cli::parse();
    let mut list: HashMap<String, u8> = HashMap::new();
    match args.subcommand {
        Commands::Add {name, skill_value } => add::add(name, skill_value, &mut list),
        Commands::Shuffle {} => print!("Shuffle !")
    }
}
