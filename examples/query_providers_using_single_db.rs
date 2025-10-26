//! This example demonstrates eagerly loading and parsing descriptions from pacman's sync databases (repository packages).
//!
//! **What it does:**
//! * Eagerly loading desc files from repository databases (`core.db`, `extra.db`, `multilib.db`) in `/var/lib/pacman/sync/`
//!   using [`arch_pkg_db::TextCollection::extend_from_archive`] to extract archives (with automatic MIME type detection).
//! * Eagerly parsing all loaded desc texts with [`arch_pkg_db::TextCollection::par_parse`] into [`arch_pkg_db::desc::EagerQuerier`] objects.
//! * Asking for a package by name or provide, looking it up using [`arch_pkg_db::QueryDatabase::get`] and
//!   [`arch_pkg_db::QueryDatabase::alternative_providers`].
//! * Displaying some information of the queried packages using methods in [`arch_pkg_db::desc::Query`].

use arch_pkg_db::{
    TextCollection,
    desc::{
        EagerQuerier, Query,
        value::{
            Base, Dependency, DependencyList, DependencyName, Description, Name, Url, Version,
        },
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
        eprintln!("---");
    }

    let mut failures = 0;

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

        let provide_target = DependencyName(&name);
        let providers = db
            .alternative_providers(provide_target)
            .filter(|pkg| pkg.name() != Some(name));
        let pkgs: Vec<_> = db.get(name).into_iter().chain(providers).collect();

        if pkgs.is_empty() {
            eprintln!("error: No package that provides {name}");
            failures += 1;
            continue;
        }

        println!("Found {} packages", pkgs.len());

        for pkg in pkgs {
            println!(
                "{name} {version} (base {base})",
                name = pkg.name().unwrap_or(Name("-")),
                version = pkg.version().unwrap_or(Version("-")),
                base = pkg.base().unwrap_or(Base("-")),
            );
            println!(
                "\tDescription: {}",
                pkg.description().unwrap_or(Description("-")),
            );
            println!("\tURL: {}", pkg.url().unwrap_or(Url("-")));

            let is_other = |item: &Dependency| -> bool {
                let (name, _) = item.components();
                name != provide_target
            };
            let others = pkg.provides();
            let others: Vec<Dependency> = others
                .iter()
                .flat_map(DependencyList::iter)
                .filter(is_other)
                .collect();
            println!(
                "\tAlso provide features for: {} other package(s)",
                others.len(),
            );
            for name in others {
                println!("\t\t{name}");
            }
        }

        println!("---");
    }

    if failures != 0 {
        eprintln!("info: {failures} queries failed");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
