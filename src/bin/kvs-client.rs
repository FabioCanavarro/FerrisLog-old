use std::{error::Error, fmt::Display, io::Write, net::TcpStream, process::exit};
use clap::{Parser, Subcommand};



// Error enum
#[derive(Debug)]
enum ClientError{
    UnableToWriteToStream,
    UnableToConnectToStream
}

impl Display for ClientError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ClientError::UnableToWriteToStream => writeln!(f, "Unable to write to stream"),
            ClientError::UnableToConnectToStream => writeln!(f, "Unable to connect to stream")
        }
    }
}

impl Error for ClientError{}

// Cli Parser
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short,long)]
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

fn main(){
    let cli = Cli::parse();

    if cli.command.is_none() {
        Cli::parse_from(["kvs", "--help"]);
        return;
    }

    // Your implementation here
    match &cli.command.unwrap() {
        Commands::get { key } => {

            let mut stream = TcpStream::connect(cli.address).expect("Cant Connect to the address");
            let _ = stream.write_all(format!("Get {}",key).into_bytes().as_ref());



        }

        Commands::rm { key } => {
            let res: Result<&str, Box<dyn Error>> = Ok("s");
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
            println!("Key set succesfully");
        }
    }
}
















