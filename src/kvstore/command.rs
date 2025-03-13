#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Command {
    Set { key: String, val: String },
    Get { key: String },
    Remove { key: String },
}

impl Command {
    pub fn set(key: String, val: String) -> Command {
        Command::Set { key, val }
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
