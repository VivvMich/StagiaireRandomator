mod add;
//mod shuffle;
//mod show;


use std::collections::HashMap;
use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Commands,
}

//
// #[derive(Serialize, Deserialize, Debug)]
// struct Trainee {
//     name: String,
//     skill_value: u8
// }
//

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

    let mut list: HashMap<String, u8> = HashMap::new();

    match args.subcommand {
        Commands::Add {name, skill_value } => add::add(name, skill_value, &mut list)?,
    //     Commands::Shuffle {} => print!("Shuffle !"),
    //     Commands::Purge {} => print!("Purge !")
    }

    Ok(())
}
