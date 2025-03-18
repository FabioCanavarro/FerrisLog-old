extern crate clap;
use std::{env::current_dir, process::exit};

use clap::{Parser, Subcommand};
use kvs::kvstore::KvStore;

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
    list_key
}

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::open(current_dir().unwrap().as_path()).unwrap();

    // Your implementation here
    match &cli
        .command
        .expect("ERROR: There is no commands found in the following input")
    {
        Commands::get { key } => {
            let val = store.get(key.to_string());
            match val.unwrap() {
                Some(d) => println!("{}", d),
                None => println!("Key not found"),
            }
        }
        Commands::rm { key } => {
            let res = store.remove(key.to_string());
            match res {
                Ok(_) => (),
                Err(_) => {
                    println!("Key not found");
                    exit(1);
                }
            }
        }
        Commands::set { key, val } => {
            let _ = store.set(key.to_string(), val.to_string());
        },
        Commands::list_key => {
            store.list_key();
        }
    }
}
