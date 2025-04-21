use std::io::stdout;
use chrono::Local;
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
        "started_at" => format!("{}", Local::now().format("%Y-%m-%d_%H-%M-%S"))
    );

    
    
}
