use std::env::current_dir;
use std::fs::DirEntry;
use std::io::Write;
use std::io::stdout;
use std::println;

fn main() {
    if ROOT.is_relative() {
        let mut abs = current_dir().unwrap();
        abs.extend(&*ROOT);
        println!("Displaying contents of \"{}\"", abs.into_os_string().into_string().unwrap());
    } else {
        println!("Displaying contents of \"{}\"", ROOT.as_os_str().to_str().unwrap());
    }

    let mut lock = stdout().lock();
    if let Some(_) = *PATTERN {
        for entry in fs::read_dir(&*ROOT).unwrap_or_else(generate_exit("Failed to read root directory: ")) {
            if let Ok(entry) = entry {
                unsafe { search_and_print(&entry, 0, &mut lock, true) };
            }
        }
    } else {
        print_contents(&*ROOT, 0, &mut lock);
    }
    lock.flush().unwrap();
}

use std::writeln;
use std::{path::{Path, PathBuf}, io::StdoutLock, env, fs};

use clap::{arg, command};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MATCHES: clap::ArgMatches = {
        command!()
            .arg(arg!(-p --path <PATH> "The path at which to start. If unspecified, the current directory is used."))
            .arg(arg!(-r --recursive "Enables recursion"))
            .arg(
                arg!(-d --depth <DEPTH> "The depth to which to recurse. If unspecified, the depth is unlimited.")
                    .requires("recursive")
                    .value_parser(clap::value_parser!(usize))
            )
            .arg(arg!(show_files: -f --"show-files" "Displays files along with directories"))
            .arg(arg!(show_symlinks: -s --"show-symlinks" "Displays symlinks along with directories"))
            .arg(arg!(-m --matches <REGEX> "Only display directories, files and symlinks which match the given regex"))
            .get_matches()
    };
    static ref RECURSIVE: bool = *MATCHES.get_one::<bool>("recursive").unwrap();
    static ref MAX_DEPTH: usize = *MATCHES.get_one::<usize>("depth").unwrap_or_else(
        || if *RECURSIVE {&usize::MAX} else {&0}
    );
    static ref SHOW_FILES: bool = *MATCHES.get_one::<bool>("show_files").unwrap();
    static ref SHOW_SYMLINKS: bool = *MATCHES.get_one::<bool>("show_symlinks").unwrap();
    static ref ROOT: PathBuf = PathBuf::from(MATCHES.get_one::<String>("path").unwrap_or(
        &env::current_dir().unwrap().into_os_string().into_string().unwrap())
    );
    static ref PATTERN: Option<Regex> = MATCHES
        .get_one::<String>("matches")
        .map(|pat| Regex::new(&pat).unwrap_or_else(generate_exit("")));
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
    if let Ok(dir_reader) = fs::read_dir(&path) {
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

fn generate_exit<E: std::fmt::Display, O>(message: &str) -> impl FnOnce(E) -> O + '_ {
    move |e| -> O {
        eprintln!("{message}{e}");
        std::process::exit(1)
    }
}


/// Directory tree explorer.
/// # SAFETY
///
/// Can only be called if PATTERN is Some.
unsafe fn search_and_print(entry: &fs::DirEntry, depth: usize, lock: &mut StdoutLock, parent_printed: bool) -> Option<String> {
    if depth > *MAX_DEPTH {
        return None;
    }
    let prefix = " ".repeat(depth);
    let file_type = entry.file_type().ok()?;
    let pattern_matches = PATTERN
                            .as_ref()
                            .unwrap_unchecked()
                            .is_match(
                                entry
                                    .file_name()
                                    .as_os_str()
                                    .to_str()
                                    .unwrap()
                                );
    // Whether the entry will get displayed
    let mut displayed = pattern_matches;
    let mut text;
    if file_type.is_dir() {
        // Allocate here. Change if I want
        text = format!("{prefix}={:?}\n", entry.file_name().as_os_str());
        for child in fs::read_dir(entry.path()).ok()? {
            // If the entry is valid
            if let Ok(entry) = child {
                // If the entry matches regex or has a child that does
                if let Some(s) = search_and_print(&entry, depth + 1, lock, displayed && parent_printed) {
                    match (displayed, parent_printed) {
                        (false, true) => {
                            write!{
                                lock,
                                "{text}"
                            }.unwrap_or_else(generate_exit("Error writing to stdout: "));
                            write!{
                                lock,
                                "{s}"
                            }.unwrap_or_else(generate_exit("Error writing to stdout: "));
                            displayed = true;
                        },
                        (true, true) => {
                            write!{
                                lock,
                                "{s}"
                            }.unwrap_or_else(generate_exit("Error writing to stdout: "));
                        },
                        (_, false) => {
                            displayed = true;
                            text.push_str(&s);
                        }
                    } 
                    
                }
            }
        }
        if !displayed {
            return None;
        }
    } else if file_type.is_file() {
        if pattern_matches {
            text = format!("{prefix}-{:?}\n", entry.file_name().as_os_str());
        } else {
            return None;
        }
    } else if file_type.is_symlink() {
        if PATTERN.as_ref().unwrap_unchecked().is_match(entry.file_name().as_os_str().to_str().unwrap()) {
            text = format!("{prefix}&{:?}\n", entry.file_name().as_os_str());  
        } else {
            return None
        }
    } else {
        unreachable!()
    }

    Some(text)
}