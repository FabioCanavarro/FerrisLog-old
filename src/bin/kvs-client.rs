use clap::{Parser, Subcommand};
use std::{error::Error, fmt::Display, io::Write, net::TcpStream, panic::panic_any, process::exit};

// Cli Parser
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    address: String,
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
}

fn main() {
    let cli = Cli::parse();

    if cli.command.is_none() {
        Cli::parse_from(["kvs", "--help"]);
        return;
    }

    // Your implementation here
    match &cli.command.unwrap() {
        Commands::get { key } => {
            let mut stream = match TcpStream::connect(cli.address) {
                Ok(stream) => stream,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            match stream.write_all(format!("Get {}", key).into_bytes().as_ref()) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            // todo!()
        }

        Commands::rm { key } => {
            let mut stream = match TcpStream::connect(cli.address) {
                Ok(stream) => stream,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            match stream.write_all(format!("Rm {}", key).into_bytes().as_ref()) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            // todo!()
        }

        Commands::set { key, val } => {
            let mut stream = match TcpStream::connect(cli.address) {
                Ok(stream) => stream,
                Err(e) => {
                    panic!("{}", e);
                }
            };
            match stream.write_all(format!("Set {} {}", key, val).into_bytes().as_ref()) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            // todo!()
        }
    }
}
