# Rust `cp` Command Implementation

A simple but functional implementation of the Unix `cp` command in Rust.

## Features

- Copy files and directories
- Recursive directory copying with `-r` flag
- Force overwrite with `-f` flag
- Preserve file attributes with `-p` flag
- Copy multiple files to a directory
- Proper error handling

## Usage

```bash
# Copy a file
cp source.txt destination.txt

# Copy a directory recursively
cp -r source_dir destination_dir

# Force overwrite existing files
cp -f source.txt destination.txt

# Preserve file attributes
cp -p source.txt destination.txt

# Copy multiple files to a directory
cp file1.txt file2.txt destination_dir/

# Combined options
cp -rp source_dir destination_dir
```

## Building and Running

```bash
# Build the project
cargo build

# Run with arguments
cargo run -- source.txt destination.txt

# Build release version
cargo build --release
```

## Implementation Details

- Uses standard library I/O operations
- 8KB buffer for efficient file copying
- Cross-platform support (Unix and Windows)
- Proper error handling for edge cases