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
    }
}


fn main() {
    let args = Cli::parse();
    match args.subcommand {
        Commands::Add {name, skill_value } => print!("{}", name)
    }
}
