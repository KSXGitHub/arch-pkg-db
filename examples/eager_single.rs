//! This example demonstrates eagerly loading and parsing descriptions from pacman packages that have been installed into the current system.
//!
//! **What it does:**
//! * Eagerly loading all desc files from `/var/lib/pacman/local/` with [`arch_pkg_db::TextCollection::par_from_local_db`].
//! * Eagerly parsing all loaded desc texts with [`arch_pkg_db::TextCollection::par_parse`] into [`arch_pkg_db::desc::EagerQuerier`] objects.
//! * Asking for a package by name, looking it up using [`arch_pkg_db::QueryDatabase::get`].
//! * Displaying some information of the queried package using methods in [`arch_pkg_db::desc::Query`].

use arch_pkg_db::{
    TextCollection,
    desc::{
        EagerQuerier, Query,
        value::{Base, Dependency, DependencyList, Description, Name, Url, Version},
    },
};
use pipe_trait::Pipe;
use std::{
    fs::metadata,
    io::{IsTerminal, Write, stdin, stdout},
    process::ExitCode,
};

fn main() -> ExitCode {
    let db_path = "/var/lib/pacman/local/";
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

    let texts = db_path
        .pipe_as_ref(TextCollection::par_from_local_db)
        .expect("load text collection");

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
        eprintln!("---");
    }

    let mut failures: u64 = 0;

    loop {
        if stdin().is_terminal() {
            eprint!("Enter a package name: ");
            stdout().flush().ok();
        }

        let mut name = String::new();
        if let Err(error) = stdin().read_line(&mut name) {
            eprintln!("error: Can't read from stdin: {error}");
            return ExitCode::FAILURE;
        }
        let name = name.trim().pipe(Name);

        if name.is_empty() {
            if stdin().is_terminal() {
                eprintln!("Exiting...");
            }
            break;
        }

        let Some(pkg) = db.get(name) else {
            eprintln!("error: No package with name {name}");
            failures += 1;
            continue;
        };

        println!("Package Name: {}", pkg.name().unwrap_or(Name("-")));
        println!("Package Base: {}", pkg.base().unwrap_or(Base("-")));
        println!("Version: {}", pkg.version().unwrap_or(Version("-")));
        println!(
            "Description: {}",
            pkg.description().unwrap_or(Description("-")),
        );
        println!("URL: {}", pkg.url().unwrap_or(Url("-")));

        let provides = pkg.provides();
        let provides: Vec<Dependency> = provides.iter().flat_map(DependencyList::iter).collect();
        println!(
            "Also provide features for: {} additional package(s)",
            provides.len(),
        );
        for provide in provides {
            println!("\t{provide}");
        }

        println!("---");
    }

    if failures != 0 {
        eprintln!("info: {failures} queries failed");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
