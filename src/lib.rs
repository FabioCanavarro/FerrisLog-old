use core::fmt;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::Deserialize;
use serde_json::error;
extern crate serde;
extern crate serde_json;

pub fn load_file(name: &PathBuf) -> Result<File, std::io::Error> {
    File::options().read(true).append(true).open(name)
}

#[derive(Debug, Deserialize)]
pub enum KvError {
    WriteError,
    ReadError,
    OpenError,
    ParseError,
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KvError::WriteError => writeln!(f, "Writing has failed!"),
            KvError::ReadError => Ok(()),
            KvError::OpenError => writeln!(f, "Opening has failed!"),
            KvError::ParseError => writeln!(f, "Parsing has failed!"),
        }
    }
}

impl Error for KvError {}

pub type KvResult<T> = Result<T, crate::KvError>;

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    table: HashMap<String, LogPosition>,
}

impl KvStore {
    pub fn new(path: PathBuf) -> KvStore {
        KvStore {
            path,
            table: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) -> KvResult<()> {
        let cmd = Command::set(key.clone(), val);
        let mut f = load_file(&self.path).unwrap();

        let _ = serde_json::to_writer(&mut f, &cmd);
        f.write_all(b"\n");
        /* let start_pos = f.seek(SeekFrom::End(0));
        let _ = serde_json::to_writer(&mut f, &cmd);
        let end_pos = f.seek(SeekFrom::End(0));
        if self.table.contains_key(&key) {
            let gen = self.table.get_key_value(&key).unwrap().1.gen + 1;
            self.table.insert(
                key,
                LogPosition {
                    gen,
                    start: start_pos.unwrap(),
                    end: end_pos.unwrap(),
                },
            );
        } else {
            self.table.insert(
                key,
                LogPosition {
                    gen: 1,
                    start: start_pos.unwrap(),
                    end: end_pos.unwrap(),
                },
            );
        } */

        Ok(())
    }

    pub fn get(&self, key: String) -> KvResult<Option<String>> {
        Ok(Some("".to_string()))
    }

    pub fn remove(&mut self, key: String) -> KvResult<()> {
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf> + AsRef<Path> + Copy) -> KvResult<KvStore> {
        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(KvError::OpenError),
        };
        let hash: HashMap<String, String> = HashMap::new();
        let buffer = BufReader::new(&f);

        let temp = serde_json::Deserializer::from_reader(buffer);
        let mut stream = temp.into_iter::<Command>();
        println!("{:?}", &stream.next());

        // For write we make vector from commmands we print vec to file
        for i in stream {
            match i.unwrap() {
                Command::Set { key, val } => println!("{key},{val}"),
                _ => (),
            }
        }
        Ok(KvStore {
            path: Into::into(path),
            table: HashMap::new(),
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Command {
    Set { key: String, val: String },
    Get { key: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, val: String) -> Command {
        Command::Set { key, val }
    }

    fn get(key: String) -> Command {
        Command::Get { key }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

#[derive(Debug)]
struct LogPosition {
    gen: u64,
    start: u64,
    end: u64,
}
