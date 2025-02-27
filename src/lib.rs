use std::{collections::HashMap, path::PathBuf};

#[derive(env)]
pub enum KvError{
    WriteError,
    ReadError,
}

pub type KvResult<T> = Result<T,crate::KvError>;


#[derive(Debug)]
pub struct KvStore {
    dict: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            dict: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, val: String) -> KvResult<()>{
        self.dict.insert(key, val);
        Ok(())
    }

    pub fn get(&self, key: String) -> KvResult<Option<String>> {
        Ok(self.dict.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> KvResult<()>{
        self.dict.remove(&key);
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> KvResult<KvStore>{
        todo!()
    }


}

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}
