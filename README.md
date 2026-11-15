### ```notes-rs``` allows you to quickly save ideas or notes right in your comfy terminal.

Use case is for really quickly writing something down without needing to open vim or something
(I actually use it lol)

#### Example:
just write anything down
```bash
notes push notes app
```
display your notes
```bash
notes
push notes app
```
if your shell (zsh, maybe others) has problems with special characters just wrap your note in quotation marks:
```bash
notes "pacman -Syu today?!"
```
#### Installation:
Rust needs to be installed -> see rust-lang.org

Linux / macOS:
```
git clone https://github.com/FailC/notes.git
cd notes
cargo build --release
mv ./target/release/notes /usr/local/bin
// or use cargo install to build and install:
cargo install --path .
```
Windows:
```bash
git clone https://github.com/FailC/notes.git
cd notes
cargo build --release
move ".\target\release\notes.exe" to a folder that is in your path
```

It creates a hidden file in the home directory
`~/.notes-rs`
after running for the first time.
You must edit and recompile (cargo build) the code for a user-defined path, or use the -f flag for custom filename
