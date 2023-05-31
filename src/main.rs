use std::env::current_dir;
use std::fs::read_dir;
use std::io::StdoutLock;
use std::io::Write;
use std::io::stdout;
use std::path::Path;
use std::path::PathBuf;
use std::println;

use clap::value_parser;
use clap::command;
use clap::arg;
use clap::Arg;
use clap::ArgAction;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MATCHES: clap::ArgMatches = {
        command!()
            .arg(arg!(-r --recursive))
            .arg(Arg::new("depth").short('d').long("depth").requires("recursive").value_parser(value_parser!(usize)))
            .arg(Arg::new("show_files").short('f').long("show-files").action(ArgAction::SetTrue))
            .arg(Arg::new("show_symlinks").short('s').long("show-symlinks").action(ArgAction::SetTrue))
            .arg(arg!(-p --path <PATH>))
            .arg(arg!(-m --matches <REGEX>))
            .get_matches()
    };
    static ref RECURSIVE: bool = *MATCHES.get_one::<bool>("recursive").unwrap();
    static ref MAX_DEPTH: usize = *MATCHES.get_one::<usize>("depth").unwrap_or_else(
        || if *RECURSIVE {&usize::MAX} else {&0}
    );
    static ref SHOW_FILES: bool = *MATCHES.get_one::<bool>("show_files").unwrap();
    static ref SHOW_SYMLINKS: bool = *MATCHES.get_one::<bool>("show_symlinks").unwrap();
    static ref ROOT: PathBuf = PathBuf::from(MATCHES.get_one::<String>("path").unwrap_or(
        &current_dir().unwrap().into_os_string().into_string().unwrap())
    );
    static ref PATTERN: Option<Regex> = MATCHES.get_one::<String>("matches").map(|pat| Regex::new(&pat).expect("Regex error"));

}

fn main() {
    if ROOT.is_relative() {
        let mut abs = current_dir().unwrap();
        abs.extend(&*ROOT);
        println!("Displaying contents of \"{}\"", abs.into_os_string().into_string().unwrap());
    } else {
        println!("Displaying contents of \"{}\"", ROOT.as_os_str().to_str().unwrap());
    }

    let mut lock = stdout().lock();
    print_contents(&*ROOT, 0, &mut lock);
    lock.flush().unwrap();
}

fn print_contents<P: AsRef<Path>>(
    path: P,
    depth: usize,
    mut lock: &mut StdoutLock<'static>,
)
{
    if depth > *MAX_DEPTH {
        return;
    }
    let prefix = " ".repeat(depth);
    let dir;
    if let Ok(dir_reader) = read_dir(&path) {
        dir = dir_reader;
    } else {
        write!(lock, "{prefix}[ACCESS DENIED]\n").unwrap();
        return;
    };
    for entry in dir {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    write!(lock, "{prefix}={:?}\n", entry.file_name()).unwrap();
                    print_contents(entry.path(), depth + 1, &mut lock)
                } else if *SHOW_FILES && file_type.is_file() {
                    write!(lock, "{prefix}-{:?}\n", entry.file_name()).unwrap();
                } else if *SHOW_SYMLINKS && file_type.is_symlink() {
                    write!(lock, "{prefix}&{:?}\n", entry.file_name()).unwrap();
                }
            }
        }
    }
}