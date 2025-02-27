use std::{collections::HashMap, path::PathBuf};

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

    pub fn set(&mut self, key: String, val: String) {
        self.dict.insert(key, val);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.dict.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.dict.remove(&key);
    }

    pub fn open<E>(path: impl Into<PathBuf>) -> Result<KvStore,E>{
        todo!()
    }


}

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}
