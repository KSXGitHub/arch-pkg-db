//! This example demonstrates eagerly loading and parsing descriptions from pacman's sync databases (repository packages) and querying them
//! by `provides`.
//!
//! **What it does:**
//! * Eagerly loading desc files from repository databases (`core.db`, `extra.db`, `multilib.db`) in `/var/lib/pacman/sync/` using
//!   [`arch_pkg_db::MultiTextCollection::extend_from_archive`] to extract archives (with automatic MIME type detection) into a repository-aware
//!   database object of type [`arch_pkg_db::MultiQueryDatabase`].
//! * Eagerly parsing all loaded desc texts with [`arch_pkg_db::MultiTextCollection::par_parse`] into [`arch_pkg_db::desc::EagerQuerier`] objects.
//! * Asking for a package by name or provide, looking it up using [`arch_pkg_db::MultiQueryDatabase::get`] and
//!   [`arch_pkg_db::MultiQueryDatabase::alternative_providers`].
//! * Listing [repository names](arch_pkg_db::value::RepositoryName) and packages using [`arch_pkg_db::multi::MultiQuerier::entries`].
//! * Displaying some information of the queried packages using methods in [`arch_pkg_db::desc::Query`].

use arch_pkg_db::{
    MultiTextCollection,
    desc::{EagerQuerier, Query},
    multi::MultiQuerier,
    value::{
        Base, Dependency, DependencyList, DependencyName, Description, Name, RepositoryName, Url,
        Version,
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
    let repository_path = |name: RepositoryName| -> PathBuf {
        debug_assert!(
            !name.ends_with(".db"),
            "Repository name shouldn't have an extension"
        );
        let name = format!("{name}.db");
        db_path.join(name)
    };

    let mut multi_collection = MultiTextCollection::new();
    for repository in ["core", "extra", "multilib"].map(RepositoryName) {
        let archive_path = repository_path(repository);
        if stdin().is_terminal() {
            eprintln!("info: Loading {}...", archive_path.to_string_lossy());
        }
        let archive = match read(&archive_path) {
            Ok(archive) => archive,
            Err(error) => {
                eprintln!("warning: Cannot read {archive_path:?}: {error}");
                continue;
            }
        };
        if let Err(error) = multi_collection.extend_from_archive(repository, &archive) {
            eprintln!("warning: Cannot extract {archive_path:?} as an archive: {error}");
            continue;
        }
    }

    if multi_collection.is_empty() {
        eprintln!("error: No desc files were found");
        return ExitCode::FAILURE;
    }

    if stdin().is_terminal() {
        // NOTE: I see now that `len` and `iter().count()` are different
        // TODO: Rename `len` to something else
        let archives = multi_collection.len();
        let texts = multi_collection.iter().count();
        eprintln!("info: Loaded {texts} desc files from {archives} archives");
    }

    let db = multi_collection
        .par_parse::<EagerQuerier>()
        .expect("parse queriers");

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
            .filter(|(_, pkg)| pkg.name() != Some(name));
        let pkgs: Vec<_> = db
            .get(name)
            .into_iter()
            .flat_map(MultiQuerier::entries)
            .chain(providers)
            .collect();

        if pkgs.is_empty() {
            eprintln!("error: No package that provides {name}");
            failures += 1;
            continue;
        }

        println!("Found {} packages", pkgs.len());

        for (repository, pkg) in pkgs {
            println!(
                "{repository}/{name} {version} (base {base})",
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
