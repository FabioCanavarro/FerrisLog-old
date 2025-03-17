# kvs-rs

A minimal, persistent key-value store implemented in Rust, featuring a CLI interface and log-based data storage. Built for educational purposes and lightweight data storage needs.

![Rust](https://img.shields.io/badge/Rust-1.72%2B-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Features

- **CRUD Operations**: `set`, `get`, and `rm` commands
- **Persistence**: Operations logged to `log.txt` (JSON format)
- **Crash Recovery**: Rebuilds state from log file on startup
- **CLI Interface**: Built with `clap` for command parsing
- **Error Handling**: Custom error types for storage operations

### Planned
- **Multi Threaded Operations**
- **Server and client KVS**
- **Time-To-Live**
- **Snapshotting**

## Installation

```bash
# Clone repository
git clone https://github.com/yourusername/kvs-rs.git
cd kvs-rs

# Build with Cargo
cargo build --release

# Install globally
cargo install --path .


## Usage

### Basic Commands

```bash
# Set key-value pair
kvs set username ferris

# Get value
kvs get username  # Output: "ferris"

# Remove key
kvs rm username

# Attempt to get removed key
kvs get username  # Output: "Key not found"
```

### File Structure

```
.
├── Cargo.toml
├── src/
│   ├── lib.rs        # Core library
│   ├── kvstore/      # Storage implementation
│   │   ├── mod.rs
│   │   ├── command.rs
│   │   └── error.rs
│   └── kvs.rs        # CLI entry point
└── log.txt           # Persistent command log
```

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amzing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request
```
