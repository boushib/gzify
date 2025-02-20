# Gzify
A simple, yet very efficient Rust file multi-threaded compressor/decompressor.

# Usage

## File Compression
```
cargo run -q -- --compress [source] [target]
```
Example:
```
cargo run -q -- --compress data.txt
```

## File Decompression
```
cargo run -q -- --decompress [source]
```
Example:
```
cargo run -q -- --decompress data.txt.gz
```

## Installation
### MacOS
Build the project
```
cargo build --release
```

Move the binary to a global path
```
sudo mv target/release/gzify /usr/local/bin/
```

Usage example
```
gzify -compress data.txt
```
