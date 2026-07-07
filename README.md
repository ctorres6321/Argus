# Argus

A command-line search tool written in Rust that recursively scans directories and finds matching text in files.

## Features
- Recursive directory traversal
- Literal text search
- Case-insensitive search with `-c`
- Skips `.git` and `target` directories
- Prints file path, line number, and matching line

## Usage

### For case-sensitive search for specific directory

```bash
cargo run -- "<query>" <directory>
```

### For case-insensitive search

```bash
cargo run -- "<query>" <directory> -c
```

## Future Improvements

- Support regular expressions for advanced pattern matching
- Ignore files using .gitignore rules
