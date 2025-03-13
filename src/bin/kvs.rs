extern crate clap;
use std::{fs::File, path::PathBuf};

use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    set {
        key: String,
        val: String,
    },
    get {
        key: String,
    },
    rm {
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::open(&PathBuf::from("log.txt"));
    // Your implementation here
    match &cli.command.unwrap() {
        Commands::get { key } => {
            let val = store.get(key.to_string());
            println!("{}",val.unwrap().unwrap())
        }
        Commands::rm { key } => {
            println!("{}", key);
            store.remove(key.to_string());
        }
        Commands::set { key, val } => {
            store.set(key.to_string(), val.to_string());
        }
    }
}
