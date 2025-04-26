use bincode::{config::{self, Config}, encode_to_vec};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{io::Write, net::TcpStream, u8};

// Cli Parser
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    address: String,
}

#[derive(Subcommand, Serialize)]
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

impl From<Commands> for u8{
    fn from(value: Commands) -> Self {
        match value{
            Commands::set { key, val } => 0,
            Commands::get { key } => 1,
            Commands::rm { key } => 2
        }
    }
}


fn main() {
    let cli = Cli::parse();
    let config = config::standard();

    if cli.command.is_none() {
        Cli::parse_from(["kvs", "--help"]);
        return;
    }

    let mut stream = match TcpStream::connect(cli.address) {
        Ok(stream) => stream,
        Err(e) => {
            panic!("{}", e);
        }
    };

    let command = cli.command.unwrap();

    match &command {
        Commands::set { key, val } => {

            let Bytecommand = u8::from(command);
            let byteKey = encode_to_vec(val, config);


        }

        Commands::get { key } => {

            let command = Commands::get { key: key.to_string() };
            let bytes = serde_json::to_vec_pretty(&command).unwrap();

            match stream.write_all(&bytes) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            // todo!()
        }

        Commands::rm { key } => {

            let command = Commands::rm { key: key.to_string() };
            let bytes = serde_json::to_vec_pretty(&command).unwrap();

            match stream.write_all(&bytes) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            // todo!()
        }


    }
}
