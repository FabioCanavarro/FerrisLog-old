use std::{error::Error, fmt::Display, io::Write, net::TcpStream, panic::panic_any, process::exit};
use clap::{Parser, Subcommand};

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

            let mut stream = match TcpStream::connect(cli.address){
                Ok(stream) => {stream},
                Err(e) => {
                    panic!("{}",e);
                },
            };
            match stream.write_all(format!("Get {}",key).into_bytes().as_ref()){
                Ok(_) => (),
                Err(e) => panic!("{}",e)
            }



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
















