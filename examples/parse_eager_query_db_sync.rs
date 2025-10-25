//! This example demonstrates eagerly loading and parsing descriptions from pacman's sync databases (repository packages).
//!
//! **What it does:**
//! * Eagerly loading desc files from repository databases (`core.db`, `extra.db`, `multilib.db`) in `/var/lib/pacman/sync/`
//!   using [`arch_pkg_db::TextCollection::extend_from_archive`] to extract archives (with automatic MIME type detection).
//! * Eagerly parsing all loaded desc texts with [`arch_pkg_db::TextCollection::par_parse`].
//! * Asking for a package by name, looking it up using [`arch_pkg_db::QueryDatabase::get`].
//! * Displaying some information of the queried package using methods in [`arch_pkg_db::desc::Query`].

use arch_pkg_db::{
    TextCollection,
    desc::{
        EagerQuerier, Query,
        value::{Base, Description, Name, Url, Version},
    },
};
use pipe_trait::Pipe;
use std::{
    fs::{metadata, read},
    io::{IsTerminal, Write, stdin, stdout},
    path::PathBuf,
    process::ExitCode,
};

fn main() -> ExitCode {
    let db_path = "/var/lib/pacman/sync/";
    let db_path_exists = db_path
        .pipe(metadata)
        .map(|stats| stats.is_dir())
        .unwrap_or(false);

    if !db_path_exists {
        eprintln!(
            "error: The path {db_path:?} either does not exist in your filesystem or is not a directory"
        );
        return ExitCode::FAILURE;
    }

    let db_path = PathBuf::from(db_path);

    let mut texts = TextCollection::new();
    for repository in ["core.db", "extra.db", "multilib.db"] {
        let archive = match read(db_path.join(repository)) {
            Ok(archive) => archive,
            Err(error) => {
                eprintln!("warning: Cannot read {repository:?}: {error}");
                continue;
            }
        };
        if let Err(error) = texts.extend_from_archive(&archive) {
            eprintln!("warning: Cannot extract {repository:?} as an archive: {error}");
            continue;
        }
    }

    if texts.is_empty() {
        eprintln!("error: No desc files were found");
        return ExitCode::FAILURE;
    }

    if stdin().is_terminal() {
        eprintln!("info: Loaded {} desc files", texts.len());
    }

    let db = texts.par_parse::<EagerQuerier>().expect("parse queriers");

    if stdin().is_terminal() {
        eprintln!("info: Parsed {} desc files", db.len());

        eprint!("Enter a package name: ");
        stdout().flush().ok();
    }

    let mut name = String::new();
    if let Err(error) = stdin().read_line(&mut name) {
        eprintln!("error: Can't read from stdin: {error}");
        return ExitCode::FAILURE;
    }
    let name = name.trim().pipe(Name);

    let Some(pkg) = db.get(name) else {
        eprintln!("error: No package with name {name}");
        return ExitCode::FAILURE;
    };

    println!("Package Name: {}", pkg.name().unwrap_or(Name("-")));
    println!("Package Base: {}", pkg.base().unwrap_or(Base("-")));
    println!("Version: {}", pkg.version().unwrap_or(Version("-")));
    println!(
        "Description: {}",
        pkg.description().unwrap_or(Description("-")),
    );
    println!("URL: {}", pkg.url().unwrap_or(Url("-")));

    ExitCode::SUCCESS
}
