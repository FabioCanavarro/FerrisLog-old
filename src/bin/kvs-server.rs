

use std::{env::current_dir, io::stdout, path::PathBuf, process::exit, str::FromStr};

use clap::{Parser, Subcommand};
use ferris::kvstore::KvStore;
use slog::{Drain, Logger,o};
use slog_scope::set_global_logger;
use slog_term::PlainSyncDecorator;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]

enum Commands {
    #[allow(non_camel_case_types)]
    /// Set a key-value pair
    set { key: String, val: String },

    /// Get the value for a key
    #[allow(non_camel_case_types)]
    get { key: String },
    
    /// Remove a key-value pair
    #[allow(non_camel_case_types)]
    rm { key: String },

    /// List all keys in the store
    #[allow(non_camel_case_types)]
    list_key,

    /// Count the number of keys in the store
    #[allow(non_camel_case_types)]
    count,

    /// Create a backup of the current database state
    #[allow(non_camel_case_types)]
    create_snapshot,

    /// Load a database from a snapshot file
    #[allow(non_camel_case_types)]
    load_snapshot { path: String },
}

fn main() {




    re"cmp.utils.feedkeys".run(33)
    djeet cli = Cli::parse();

    let mut store = KvStore::open(current_dir().unwrap().as_path()).unwrap();

    if cli.command.is_none() {
        Cli::parse_from(["kvs", "--help"]);
        return;
    }

    // Your implementation here
    match &cli.command.unwrap() {
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
            println!("Key removed succesfully");
        }

        Commands::set { key, val } => {
            let _ = store.set(key.to_string(), val.to_string());
            println!("Key set succesfully");
        }

        Commands::list_key => {
            store.list_key();
        }

        Commands::count => {
            println!("{}", store.count());
        }

        Commands::create_snapshot => {
            let snapshot_dir = store.create_snapshot();
            println!(
                "Snapshot Created at {}",
                snapshot_dir.unwrap().to_str().unwrap()
            );
        }

        Commands::load_snapshot { path } => {
            let pathb = PathBuf::from_str(path);
            match pathb {
                Ok(_) => (),
                Err(e) => {
                    println!("Path is invalid, error: {:?}", e);
                    println!("Path inputed {}", path);
                }
            }

            let _ = store.load_snapshot(pathb.unwrap());
            println!("Snapshot Loaded");
        }
    }
}
