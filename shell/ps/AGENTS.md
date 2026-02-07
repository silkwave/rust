# AGENTS.md - Rust PS 프로젝트 가이드라인

## 프로젝트 개요
`/proc` 파일 시스템을 읽어오는 ps 명령어의 Rust 구현입니다. 에러 처리 및 CLI 파싱을 포함한 리눅스 시스템 프로그래밍을 보여줍니다.

## 빌드 명령어

### 기본 명령어
```bash
cargo build
cargo build --release
cargo run
cargo run -- -f    # 전체 포맷
cargo run -- -a    # 모든 프로세스
```

### Testing Commands
```bash
# Run all tests (if any exist)
cargo test

# Run a single test (when tests are added)
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run specific test file
cargo test --bin rust-ps
```

### Linting and Formatting
```bash
# Check code with Clippy (if installed)
cargo clippy

# Format code with rustfmt
cargo fmt

# Format code and check
cargo fmt -- --check
```

## Code Style Guidelines

### Import Organization
- Group imports: external crates, std library, local modules
- Use specific imports over glob imports
- Keep imports sorted alphabetically

```rust
use clap::{Arg, Command};  // External crates first
use std::fs;              // std library
```

### Struct and Function Naming
- Use `PascalCase` for struct and enum names
- Use `snake_case` for function and variable names
- Use descriptive names that explain the purpose

```rust
#[derive(Debug)]
struct ProcessInfo {
    pid: u32,
    comm: String,
}

fn parse_stat(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    // implementation
}
```

### Type Usage
- Prefer explicit types over type inference in function signatures
- Use appropriate integer types (u32 for IDs, u64 for time/clocks)
- Use `String` for owned strings and `&str` for borrowed strings

```rust
fn get_process_list() -> Vec<ProcessInfo> {  // Explicit return type
    let mut processes = Vec::new();          // Type inference OK for locals
}
```

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Prefer `Box<dyn std::error::Error>` for generic error types
- Use `?` operator for error propagation
- Provide meaningful error messages

```rust
fn parse_stat(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
    let stat_path = format!("/proc/{}/stat", pid);
    let content = fs::read_to_string(&stat_path)?;  // Use ? operator
    
    if parts.len() < 15 {
        return Err("Invalid stat format".into());    // Meaningful error
    }
    // ...
}
```

### Comments and Documentation
- Use Korean comments for this project (as established)
- Add comments to explain complex logic or business rules
- Use doc comments (`///`) for public API documentation
- Keep comments concise and up-to-date

```rust
// 프로세스 정보 구조체
#[derive(Debug)]
struct ProcessInfo {
    pid: u32,        // 프로세스 ID
    comm: String,    // 프로세스 명령어 이름
}

/// /proc/[pid]/stat 파일을 파싱하여 프로세스 정보 추출
fn parse_stat(pid: u32) -> Result<ProcessInfo, Box<dyn std::error::Error>> {
```

### Function Organization
- Keep functions focused on a single responsibility
- Limit function length to ~50 lines when possible
- Extract complex logic into helper functions
- Use private helper functions for implementation details

```rust
fn main() {
    // Parse command line arguments
    let matches = parse_args();
    
    // Get and display processes
    print_processes(matches.full_format);
}

fn parse_args() -> CliArgs {
    // Argument parsing logic
}

fn print_processes(full_format: bool) {
    // Display logic
}
```

### String Formatting
- Use `format!` macro for complex string construction
- Use string interpolation for simple cases
- Be consistent with formatting patterns

```rust
let stat_path = format!("/proc/{}/stat", pid);
let tty = if process.tty_nr == 0 {
    "?".to_string()
} else {
    format!("pts/{}", process.tty_nr - 1)
};
```

### Constants and Magic Numbers
- Extract magic numbers into named constants
- Use descriptive names for constants
- Group related constants together

```rust
const MIN_STAT_FIELDS: usize = 15;
const CLOCK_TICK_FACTOR: u64 = 100;
const NO_TTY: u32 = 4194303;
```

## Project Structure
```
.
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── src/
│   └── main.rs         # Main application code
└── target/             # Build artifacts
```

## Dependencies
- `clap` (v4.0+) with `derive` feature for command-line argument parsing
- Only use dependencies when absolutely necessary
- Prefer std library solutions when available

## Platform Considerations
- This code is Linux-specific due to `/proc` filesystem usage
- Do not attempt to make it cross-platform without explicit requirements
- Handle filesystem errors gracefully when `/proc` is not available

## Performance Guidelines
- Minimize filesystem reads by reading only necessary data
- Use efficient string operations (avoid unnecessary allocations)
- Consider using buffered readers for large files
- Profile before optimizing critical paths

## Testing Strategy
- Add unit tests for parsing logic
- Add integration tests for command-line interface
- Test error conditions and edge cases
- Use property-based testing for complex parsing logic if needed

## Security Considerations
- Validate all input from filesystem
- Handle malformed stat files gracefully
- Avoid buffer overflows when parsing strings
- Sanitize output if used in security-sensitive contexts