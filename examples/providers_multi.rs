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
use derive_more::Display;
use pipe_trait::Pipe;
use std::{
    env::args,
    ffi::OsStr,
    fs::{metadata, read},
    io::{IsTerminal, Write, stdin, stdout},
    path::{Path, PathBuf},
    process::ExitCode,
    sync::LazyLock,
};
use text_block_macros::text_block;

static DB_PATH_STR: &str = "/var/lib/pacman/sync/";
static DB_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    let db_path = PathBuf::from(DB_PATH_STR);
    let db_path_exists = db_path
        .pipe_ref(metadata)
        .map(|stats| stats.is_dir())
        .unwrap_or(false);
    db_path_exists.then_some(db_path)
});

static HELP: &str = text_block! {
    "Usage:"
    "  cargo run [--release] --example=providers_multi -- [REPOSITORIES]..."
    ""
    "Syntax to specify a repository:"
    "  * <REPOSITORY_NAME>:<ARCHIVE_PATH>"
    "  * <REPOSITORY_NAME>"
    "  * <ARCHIVE_PATH>"
    ""
    "Examples:"
    "  cargo run --release --example=providers_multi -- core multilib extra endeavouros chaotic-aur"
    "  cargo run --release --example=providers_multi -- core:/mnt/ARCH/var/lib/pacman/sync/core.db"
    "  cargo run --release --example=providers_multi -- /mnt/ARCH/var/lib/pacman/sync/core.db"
};

#[derive(Debug)]
struct Arg<'a>(RepositoryName<'a>, PathBuf);

#[derive(Debug, Display)]
enum ParseArgExit<'a> {
    #[display("{HELP}")]
    Help,
    #[display("Unknown flag: {_0}")]
    UnsupportedFlag(&'a str),
    #[display("Invalid repository name: {_0}")]
    InvalidRepositoryName(&'a str),
    #[display("Repository {_0} requires {DB_PATH_STR} which doesn't exist as a directory")]
    RequiredDatabaseNotFound(RepositoryName<'a>),
    #[display("Path does not contain a filename: {_0:?}")]
    NoFileName(&'a str),
}

impl ParseArgExit<'_> {
    fn display(&self) {
        match self {
            ParseArgExit::Help => println!("{self}"),
            _ => eprintln!("{self}"),
        }
    }

    fn code(&self) -> ExitCode {
        match self {
            ParseArgExit::Help => ExitCode::SUCCESS,
            _ => ExitCode::FAILURE,
        }
    }
}

fn parse_arg(arg: &str) -> Result<Arg<'_>, ParseArgExit<'_>> {
    if matches!(arg, "-h" | "--help") {
        return Err(ParseArgExit::Help);
    }

    if arg.starts_with('-') {
        return Err(ParseArgExit::UnsupportedFlag(arg));
    }

    fn validate_repository_name(repository: &str) -> Result<RepositoryName<'_>, &str> {
        if repository.is_empty() {
            return Err(repository);
        }
        let valid_char = |char| matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.');
        repository
            .chars()
            .all(valid_char)
            .then_some(RepositoryName(repository))
            .ok_or(repository)
    }

    if let Some((repository, archive_path)) = arg.split_once(':') {
        let repository = repository
            .pipe(validate_repository_name)
            .map_err(ParseArgExit::InvalidRepositoryName)?;
        let archive_path = PathBuf::from(archive_path);
        return Ok(Arg(repository, archive_path));
    }

    if validate_repository_name(arg).is_ok() {
        let repository = RepositoryName(arg);
        let archive_path = DB_PATH
            .as_ref()
            .ok_or(ParseArgExit::RequiredDatabaseNotFound(repository))?
            .join(format!("{repository}.db"));
        return Ok(Arg(repository, archive_path));
    }

    let archive_path: &Path = arg.as_ref();
    let file_name = archive_path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or(ParseArgExit::NoFileName(arg))?;
    let repository = file_name
        .strip_suffix(".db")
        .or_else(|| file_name.strip_suffix(".tar.gz"))
        .or_else(|| file_name.strip_suffix(".tar.xz"))
        .or_else(|| file_name.strip_suffix(".tgz"))
        .or_else(|| file_name.strip_suffix(".txz"))
        .or_else(|| file_name.strip_suffix(".tar"))
        .or_else(|| file_name.strip_suffix(".gz"))
        .or_else(|| file_name.strip_suffix(".xz"))
        .unwrap_or(file_name)
        .pipe(validate_repository_name)
        .map_err(ParseArgExit::InvalidRepositoryName)?;
    Ok(Arg(repository, archive_path.to_path_buf()))
}

fn main() -> ExitCode {
    let args: Vec<_> = args().skip(1).collect();
    let parse_args_result: Result<Vec<Arg>, ParseArgExit> =
        args.iter().map(String::as_str).map(parse_arg).collect();

    let repositories = match parse_args_result {
        Ok(repositories) if repositories.is_empty() => {
            eprintln!("error: No repository specified");
            eprintln!("hint: Run with --help to see usage");
            return ExitCode::FAILURE;
        }
        Ok(repositories) => repositories,
        Err(exit) => {
            exit.display();
            return exit.code();
        }
    };

    if cfg!(debug_assertions) {
        eprintln!("warning: The archive extraction processes may be slow on debug build");
    }

    let mut multi_collection = MultiTextCollection::new();
    for Arg(repository, archive_path) in &repositories {
        if stdin().is_terminal() {
            eprintln!(
                "info: Loading {repository} from {}...",
                archive_path.to_string_lossy(),
            );
        }
        let archive = match read(archive_path) {
            Ok(archive) => archive,
            Err(error) => {
                eprintln!("warning: Cannot read {archive_path:?}: {error}");
                continue;
            }
        };
        if let Err(error) = multi_collection.extend_from_archive(*repository, &archive) {
            eprintln!("warning: Cannot extract {archive_path:?} as an archive: {error}");
            continue;
        }
    }

    if multi_collection.is_empty() {
        eprintln!("error: No desc files were found");
        return ExitCode::FAILURE;
    }

    if stdin().is_terminal() {
        let archives = repositories.len();
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
