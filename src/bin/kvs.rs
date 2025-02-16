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
    set{key: String, val: String},
    get{key: String},
    rm{key:String}

}

fn main() {
    let cli = Cli::parse();
    let mut store = kvs::KvStore::new();
    // Your implementation here
    match &cli.command.unwrap() {
        Commands::get { key } => {store.get(key.to_string());},
        Commands::rm { key } => {println!("{}",key);store.remove(key.to_string());},
        Commands::set { key, val } => {store.set(key.to_string(), val.to_string());}
        
    }
}

