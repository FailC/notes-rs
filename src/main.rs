use clap::Parser;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::panic;
use std::path::PathBuf;
use std::process::Command;
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// new note
    note: Option<Vec<String>>,
    /// list notes
    #[arg(short = 'l', long)]
    list: bool,
    /// choose note to delete
    #[arg(short = 'd', long)]
    delete: bool,

    /// delete all notes
    #[arg(short = 'D', long)]
    delete_all: bool,

    /// open text editor
    #[arg(short = 'v', long)]
    visual: bool,
    /// use custom filepath
    #[arg(short = 'f', long)]
    filepath: Option<PathBuf>,
}

fn open_editor(path: &PathBuf) {
    // TODO: test on windows lmao
    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL")) // sometimes VISUAL is used
        .unwrap_or_else(|_| {
            if cfg!(windows) {
                "notepad".to_string()
            } else {
                "vim".to_string()
            }
        });

    let _status = Command::new(editor)
        .arg(path)
        .status()
        .expect("can't open editor, check $EDITOR");
}

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
    file.write_all(note.as_bytes())
        .expect("ERROR: can't write to file");
    file.write_all("\n".as_bytes())
        .expect("ERROR: can't write to file");
}

fn list_note(path: &PathBuf) {
    let content = fs::read_to_string(&path).expect("ERROR: can't read content");
    let lines = content.lines();
    for line in lines {
        println!("{line}");
    }
}

fn delete_all(path: &PathBuf) {
    // check if file exist, if not don't create new file
    // does happen in main
    let _file = File::create(&path).expect("can't delete notes");
    // println!("deleting all notes..");
}

fn delete_note(path: &PathBuf) {
    // read file
    let mut lines: Vec<String> = fs::read_to_string(&path)
        .expect("ERROR: can't open file")
        .lines()
        .map(String::from)
        .collect();

    // list file
    lines
        .iter()
        .enumerate()
        .for_each(|(i, line)| println!("{}: {}", i + 1, line));

    print!("Select note to delete: ");
    io::stdout().flush().expect("can't flush buffer");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("can't read input");

    // delete line
    let mut numbers: Vec<usize> = input
        .split(" ")
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .filter(|x| *x <= lines.len() && *x != 0)
        .collect();
    numbers.sort_by(|a, b| b.cmp(a));

    numbers.iter().for_each(|num| {
        lines.remove(num - 1);
    });

    // save file
    let file = File::create(&path);

    let Ok(mut file) = file else {
        eprintln!("ERROR: can't open file");
        exit(1);
    };

    let content = lines.join("\n");
    if content.is_empty() {
        return;
    }
    file.write_all(content.as_bytes())
        .expect("can't write to file");
    file.write_all("\n".as_bytes())
        .expect("can't write to file");
}

fn main() {
    let args = {
        let mut raw_args: Vec<String> = env::args().collect();

        // use --list if no argument provided
        if raw_args.len() == 1 {
            raw_args.push("--list".to_string());
        }

        Args::parse_from(raw_args)
    };

    // get custom filepath
    let path = match args.filepath {
        Some(path) => path,
        // default file path
        None => PathBuf::from("/home/felix/.new_notes"),
    };

    // check filepath
    if !path.is_file() {
        println!("INFO: file doesnt exist");
        println!("INFO: creating file: {:?}", &path);
        File::create(&path).expect("created file");
    }

    if let Some(note) = args.note {
        save_note(note, &path);
    }
    if args.list {
        list_note(&path);
    } else if args.delete {
        delete_note(&path);
    } else if args.delete_all {
        delete_all(&path);
    } else if args.visual {
        open_editor(&path);
    }
}
