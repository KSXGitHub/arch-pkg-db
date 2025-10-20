use arch_pkg_db::{
    TextCollection,
    desc::{
        EagerQuerier, Query,
        value::{Base, Description, Name, Url, Version},
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
        .pipe_as_ref(TextCollection::from_local_db)
        .expect("load text collection");
    let db = texts.par_parse::<EagerQuerier>().expect("parse queriers");

    if stdin().is_terminal() {
        // TODO: implement the ability to get the number of texts
        // TODO: implement the ability to get the number of parsed entries
        // TODO: display them here

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
