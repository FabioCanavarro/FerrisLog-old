use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};
pub mod command;
pub mod error;

use command::Command;
use error::{KvError, KvResult};

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    table: HashMap<String, String>,
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

        let mut f = File::options()
            .read(true)
            .append(true)
            .open(&self.path)
            .unwrap();

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
        let val = self.table.get(&key);
        Ok(val.cloned())
    }

    pub fn remove(&mut self, key: String) -> KvResult<()> {
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf> + AsRef<Path> + Copy) -> KvResult<KvStore> {
        let mut f = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(KvError::OpenError),
        };
        let mut hash: HashMap<String, String> = HashMap::new();
        let buffer = BufReader::new(&f);

        let temp = serde_json::Deserializer::from_reader(buffer);
        let mut stream = temp.into_iter::<Command>();

        // For write we make vector from commmands we print vec to file

        for i in stream {
            match i.unwrap() {
                Command::Set { key, val } => {
                    hash.insert(key, val);
                }
                _ => (),
            }
        }
        Ok(KvStore {
            path: Into::into(path),
            table: hash,
        })
    }
}
