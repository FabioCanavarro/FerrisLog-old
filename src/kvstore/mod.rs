use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader, Seek, SeekFrom, Write}, path::{Path, PathBuf}
};
pub mod command;
pub mod error;

use command::Command;
use error::{KvError, KvResult};

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    table: HashMap<String, u64>,
}

impl KvStore {
    pub fn new(path: PathBuf) -> KvStore {
        KvStore {
            path,
            table: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) -> KvResult<()> {
        let cmd = Command::set(key.clone(), val.clone());

        let mut f = File::options()
            .read(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        let start_pos = f.seek(SeekFrom::End(0)).unwrap();
        let _ = serde_json::to_writer(&mut f, &cmd);
        let _ = f.write_all(b"\n");
        self.table.insert(
            key,
            start_pos
        );

        Ok(())
    }

    pub fn get(&self, key: String) -> KvResult<Option<String>> {
        let val = self.table.get(&key);
        match &val {
            Some(_) => (),
            None => return Ok(None)
        }
        
        let file = File::options()
            .read(true)
            .open(&self.path)
            .unwrap();

        let mut f = BufReader::new(file);

        // Seek from val to the \n
        let _ = f.seek(SeekFrom::Start(*val.unwrap()));
        let mut line = String::new();
        let _ = f.read_line(&mut line);
        let res = serde_json::from_str::<Command>(&line.to_string());
        match res {
            Ok(re) => {
                match re {
                    Command::Set{key: _, val} => return Ok(Some(val)),
                    _ => return Ok(None)

                }
            },
            Err(_) => Err(KvError::ParseError)
        }

        
    }

    pub fn remove(&mut self, key: String) -> KvResult<()> {
        let cmd = Command::rm(key.clone());

        let mut f = File::options()
            .read(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        let _ = serde_json::to_writer(&mut f, &cmd);
        let _ = f.write_all(b"\n");



        todo!();


        match res {
            Some(_) => Ok(()),
            None => Err(KvError::RemoveError),
        }
    }



    pub fn open(path: impl Into<PathBuf> + AsRef<Path> + Copy) -> KvResult<KvStore> {
        let f = match File::open(path.into().join("log.txt")) {
            Ok(f) => f,
            Err(_) => {
                let _ = File::create(path.into().join("log.txt"));
                File::open(path.into().join("log.txt")).unwrap()
            }
        };
        let mut hash: HashMap<String, u64> = HashMap::new();
        let mut buffer = BufReader::new(&f);
        let mut pos = buffer.seek(SeekFrom::Start(0)).unwrap();

        loop{
            let mut line = String::new();

            let length = buffer.read_line(&mut line).unwrap();
            if length == 0{
                break;
            }
            let res = serde_json::from_str::<Command>(&line.to_string());

            match res {
                Ok(re) => {
                    if let Command::Set{key, val: _} = re {hash.insert(key, pos);}
                },

                Err(_) => return Err(KvError::ParseError)
            }

            pos = buffer.seek(SeekFrom::Start(pos + length as u64)).unwrap();

        }

        



        Ok(KvStore {
            path: path.into().join("log.txt"),
            table: hash,
        })
    }
}
