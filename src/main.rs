mod add;
mod shuffle;
mod show;

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use serde_derive::{Deserialize, Serialize};
use serde_json::{to_writer};
use std::fs::File;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Commands,
}

#[derive(Serialize, Deserialize, Debug)]
struct Trainee {
    name: String,
    skill_value: u8
}

#[derive(clap::Subcommand)]
enum Commands{
    Add{
        name: String,
        skill_value: u8
    },
    Shuffle{},
    Purge {}
}

fn main() {
    let args = Cli::parse();

    // Open file
    let mut f = File::options()
        .read(true)
        .write(true)
        .open("data.json")
        .unwrap();

    // pour revenir ou debut du fichier
    //let _ = f.seek(std::io::SeekFrom::Start(0)).unwrap();

    // pour ecrire avec serde dans le fichier
    //serde_json::to_writer(f, &q).unwrap();

    let mut list: HashMap<&str, u8> = HashMap::new();

    list.insert("Coucou", 5);
    list.insert("Baba", 50);
    list.insert("Zie", 74);



    match args.subcommand {
        Commands::Add {name, skill_value } => print!("Add !"),
        Commands::Shuffle {} => print!("Shuffle !"),
        Commands::Purge {} => print!("Purge !")
    }
}
