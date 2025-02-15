extern crate clap;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Set{key: String, val: String},
    Get{key: String},
    Remove{key:String}

}

fn main() {
    let cli = Cli::parse();
    // Your implementation here
    match &cli.command.unwrap() {
        Commands::Get { key } => println!("{}",key),
        Commands::Remove { key } => todo!(),
        Commands::Set { key, val } => todo!()
        
    }
}
