use std::{io::{stdout, Read, Write}, net::{TcpListener, TcpStream}, sync::Arc};
use clap::Parser;
use slog::{Drain, Logger,o,info};
use slog_term::PlainSyncDecorator;

#[derive(Clone,Copy)]
enum Engine{
    Kvs,
    Sled
}

impl From<Engine> for String{
    fn from(value: Engine) -> Self {
        match value{
            Engine::Kvs => "Kvs".to_string(),
            Engine::Sled => "Sled".to_string()
        }
    }
}

fn handle_listener(stream: &mut TcpStream) -> String{
    let mut buf: String = String::new();
    let _ = stream.read_to_string(&mut buf);
    let _ = stream.flush();
    println!("{}",buf);
    buf
}



#[derive(Parser,Debug)]
#[command(version, about)]
struct Args {
    #[arg(short,long)]
    address: String,

    #[arg(short,long, default_value_t=String::from("Kvs"))]
    engine: String
}


fn main() {
    // Structured Logging
    let plain = PlainSyncDecorator::new(stdout());

    let logger = Logger::root(
            slog_term::FullFormat::new(plain)
                .build()
                .fuse(),
        o!("version" => "0.1")
    );


    let args = Args::parse();



    // Initial logging
    info!(logger,
        "Application started";
        "started_at" => format!("{}", args.address)
    );

    let listener = TcpListener::bind(args.address).unwrap();

    for stream in listener.incoming(){
        
        handle_listener(&mut stream.expect("Error"));
        info!(logger,
            "Listener Incoming",
        );
    }
 
    
    
}
