#[macro_use]
extern crate clap;
extern crate exec;

use std::error::Error as ErrorTrait;
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Write};
use std::path::PathBuf;
use std::process;
use std::time;

use clap::ArgMatches;


fn main() {
    let matches: ArgMatches = clap_app!(myapp =>
        (version: "0.1")
        (author: "Stephen Sugden <me@stephensugden.com>")
        (about: "Remember things in directories")
        (@arg recursive: -r --recursive "Whether to search recursively for a .mmr file")
        (@subcommand edit =>
            (about: "Edit (or create) an .mmr file in this directory")
        )
        (@subcommand remind =>
            (about: "Check for a .mmr file and print the contents if it's old enough")
            (@arg age: -a --age +takes_value "Minimum age of the .mmr file (in seconds). If the file is newer than this it won't be printed")
            (@arg subject: -s --subject "Print only the first line of the reminder file")
        )
        (@subcommand print =>
            (about: "Print the contents of the .mmr file regardless of it's age")
            (@arg subject: -s --subject "Print only the first line of the reminder file")
        )
    ).get_matches();

    let path_result = find_mmr_file(matches.is_present("recursive")).map_err(Error::IO);

    let result = path_result.and_then(|path| match matches.subcommand() {
        ("edit", _) => edit(path),
        ("print", Some(print_args)) => print_file(path, print_args.is_present("subject")),
        ("remind", Some(remind_args)) => maybe_remind(path, remind_args),
        _ => edit(path),
    });

    if let Err(err) = result {
        eprintln!("{}", err);
        process::exit(7);
    }
}

#[derive(Debug)]
pub enum Error {
    Exec(exec::Error),
    IO(io::Error),
    Clock(time::SystemTimeError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Exec(ref e) => e.description(),
            &Error::IO(ref e) => e.description(),
            &Error::Clock(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&ErrorTrait> {
        match self {
            &Error::Exec(ref e) => e.cause(),
            &Error::IO(ref e) => e.cause(),
            &Error::Clock(ref e) => e.cause(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Exec(ref e) => write!(f, "{}", e),
            &Error::IO(ref e) => write!(f, "{}", e),
            &Error::Clock(ref e) => write!(f, "{}", e),
        }
    }
}

fn edit(path: PathBuf) -> Result<(), Error> {
    let editor = find_editor();
    let arg1 = editor.clone();
    Err(Error::Exec(exec::execvp(
        editor,
        &[arg1, path.into_os_string()],
    )))
}

fn find_mmr_file(recursive: bool) -> Result<PathBuf, io::Error> {
    let mut start = std::env::current_dir()?.clone();
    start.push(".mmr");
    let mut path = start.clone();
    loop {
        if path.is_file() || !recursive {
            return Ok(path);
        }
        path.pop();
        if path.parent().is_none() {
            return Ok(start);
        }
        path.pop();
        path.push(".mmr");
    }
}

fn find_editor() -> OsString {
    "vim".into()
}

fn maybe_remind(path: PathBuf, args: &ArgMatches) -> Result<(), Error> {
    let min_age: u64 = args.value_of("age")
        .and_then(|s| s.parse().ok())
        .unwrap_or(2700);
    if !path.is_file() {
        return Ok(());
    }
    let age = path.metadata()?
        .modified()?
        .elapsed()
        .map_err(Error::Clock)?;
    // todo - check --age CLI arg
    if age > time::Duration::new(min_age, 0) {
        print_file(path, args.is_present("subject"))?;
    }
    Ok(())
}

fn print_file(path: PathBuf, only_subject: bool) -> Result<(), Error> {
    let mut reader = io::BufReader::new(File::open(path)?);
    let mut buf = Vec::with_capacity(2048);
    if only_subject {
        // hope you like unix line endings
        reader.read_until('\n' as u8, &mut buf)?;
    } else {
        reader.read_to_end(&mut buf)?;
    }
    io::stdout().write(&buf)?;
    Ok(())
}
