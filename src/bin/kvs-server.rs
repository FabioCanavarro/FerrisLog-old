use clap::Parser;
use ferris::kvstore::command;
use slog::{info, o, warn, Drain, Logger};
use slog_term::PlainSyncDecorator;
use std::{
    error::Error,
    fmt::Display,
    io::{stdout, Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Clone, Copy)]
enum Engine {
    Kvs,
    Sled,
}

#[derive(Debug)]
enum ServerError {
    UnableToReadFromStream,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::UnableToReadFromStream => writeln!(f, "Unable to read from stream"),
        }
    }
}

impl Error for ServerError {}

impl From<Engine> for String {
    fn from(value: Engine) -> Self {
        match value {
            Engine::Kvs => "Kvs".to_string(),
            Engine::Sled => "Sled".to_string(),
        }
    }
}

struct Header {
    command: u8,
    keysize: u8,
    valuesize: u8
}

impl Header{
    fn new(command: u8,keysize: u8,valuesize: u8) -> Header{
        Header { command, keysize, valuesize }
    }
}

fn handle_listener(stream: &mut TcpStream) -> Result<Vec<u8>, ServerError> {
    let mut buf: [u8;3] = [0,0,0];

    let _ = stream.flush();

    match stream.read_exact(&mut buf){
        Ok(_) => (),
        Err(e) => ()
    }

    let header = Header::new(buf[0], buf[1], buf[2]);
    let key: &[u8] = &[];
    Ok(buf.to_vec())
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    address: String,

    #[arg(short,long, default_value_t=String::from("Kvs"))]
    engine: String,
}

fn main() {
    // Structured Logging
    let plain = PlainSyncDecorator::new(stdout());

    let logger = Logger::root(
        slog_term::FullFormat::new(plain).build().fuse(),
        o!("version" => "0.1"),
    );

    let args = Args::parse();

    // Initial logging
    info!(logger,
        "Application started";
        "started_at" => format!("{}", args.address)
    );

    let listener = TcpListener::bind(args.address).unwrap();

    for stream in listener.incoming() {
        let command = handle_listener(&mut stream.expect("Error"));

        match command {
            Ok(log) => info!(logger,
                        "Incoming Message";
                        "Command" =>  format!("{:?}",log)
            ),

            Err(e) => warn!(logger,
                        "StreamError";
                        "Error:" => format!("{}",e)
            ),
        }
    }
}
