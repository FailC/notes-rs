### ```notes-rs``` allows you to quickly save ideas or notes right in your comfy terminal.

Use case is for quickly writing something down without needing to open any program

#### Example:
just write anything down
```bash
notes update arch linux
```

display your notes with
```bash
notes
```

display all options with
```bash
notes -h
```

#### Installation:
Rust needs to be installed -> see rust-lang.org

Linux / macOS:
```
git clone https://github.com/FailC/notes-rs.git
cd notes-rs
cargo build --release
mv ./target/release/notes /usr/local/bin
// or use cargo install to build and install:
cargo install --path .
```
Windows:
```bash
git clone https://github.com/FailC/notes-rs.git
cd notes-rs
cargo install --path .
```

It creates a hidden text file in the home directory
`~/.notes-rs`
after running for the first time.
You must edit and recompile (cargo build) the code for a user-defined path, or use the -f flag for custom filename
