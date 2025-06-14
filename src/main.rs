use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::panic;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// positional argument
    note: Option<Vec<String>>,
    #[arg(short = 'l', long)]
    list: bool,
    #[arg(short = 'd', long)]
    delete: bool,
    #[arg(short = 'v', long)]
    visual: bool,
    #[arg(short = 'f', long)]
    filepath: Option<PathBuf>,
}

struct _Note {}

fn save_note(note: Vec<String>, path: &PathBuf) {
    // check if file exists
    let file = match fs::metadata(&path).is_ok() {
        true => OpenOptions::new().append(true).open(path),
        false => {
            println!("created file: {:?}", &path);
            File::create(&path)
        }
    };
    let Ok(mut file) = file else {
        panic!("ERROR: can't open file");
    };
    let note = note.join(" ");
    file.write_all(note.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

fn list_note(path: &PathBuf) {
    let content = fs::read_to_string(&path).unwrap();
    let lines = content.lines();
    for line in lines {
        println!("{line}");
    }
}

fn delete_note(_path: &PathBuf) {
    todo!()
}

fn main() {
    let args = Args::parse();

    // get custom filepath
    let path = match args.filepath {
        Some(path) => path,
        None => PathBuf::from("./notes"),
    };

    if let Some(note) = args.note {
        save_note(note, &path);
    }

    if args.list {
        list_note(&path);
    } else if args.delete {
        delete_note(&path);
    } else if args.visual {
        println!("visual?");
    }
}
