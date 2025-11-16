# arch-pkg-db

Pure Rust library to read Arch Linux's package database archives.

## Description

This is a collection of APIs that allow for loading a database of pacman packages and query them.

The database could be a local database of installed packages or a sync database of all installable packages.

The sync database may or may not contain non-official repositories, with duplicated package names.

## Documentation

See [docs.rs](https://docs.rs/arch-pkg-db/).

## Quick Start

### Querying installed packages

```rust no_run
use arch_pkg_db::{TextCollection, EagerQueryDatabase, desc::Query, value::Name};

let texts = TextCollection::par_from_local_db("/var/lib/pacman/local/".as_ref()).unwrap();
let db: EagerQueryDatabase = texts.par_parse().unwrap();

let pkg = db.get(Name("bash")).unwrap();
println!("Name: {}", pkg.name().unwrap());
println!("Version: {}", pkg.version().unwrap());
println!("Description: {}", pkg.description().unwrap());
```

### Querying installable packages without caring about repository names

```rust no_run
use std::fs::read;
use arch_pkg_db::{TextCollection, EagerQueryDatabase, desc::Query, value::Name};

let texts = TextCollection::new()
    .add_archive(&read("/var/lib/pacman/sync/core.db").unwrap())
    .unwrap()
    .add_archive(&read("/var/lib/pacman/sync/extra.db").unwrap())
    .unwrap()
    .add_archive(&read("/var/lib/pacman/sync/multilib.db").unwrap())
    .unwrap();

let db: EagerQueryDatabase = texts.par_parse().unwrap();

let pkg = db.get(Name("bash")).unwrap();
println!("Name: {}", pkg.name().unwrap());
println!("Version: {}", pkg.version().unwrap());
println!("Description: {}", pkg.description().unwrap());
```

### Querying installable packages and their repository names

```rust no_run
use std::fs::read;
use arch_pkg_db::{
    MultiTextCollection,
    EagerMultiQueryDatabase,
    desc::Query,
    value::{Name, RepositoryName},
};

let multi_texts = MultiTextCollection::new()
    .add_archive(RepositoryName("core"), &read("/var/lib/pacman/sync/core.db").unwrap())
    .unwrap()
    .add_archive(RepositoryName("extra"), &read("/var/lib/pacman/sync/extra.db").unwrap())
    .unwrap()
    .add_archive(RepositoryName("multilib"), &read("/var/lib/pacman/sync/multilib.db").unwrap())
    .unwrap()
    .add_archive(RepositoryName("chaotic-aur"), &read("/var/lib/pacman/sync/chaotic-aur.db").unwrap())
    .unwrap()
    .add_archive(RepositoryName("arch-derivative-distro"), &read("/var/lib/pacman/sync/arch-derivative-distro.db").unwrap())
    .unwrap()
    .add_archive(RepositoryName("my-personal-repo"), &read("/var/lib/pacman/sync/my-personal-repo.db").unwrap())
    .unwrap();

let db: EagerMultiQueryDatabase = multi_texts.par_parse().unwrap();

let Some(pkgs) = db.get(Name("paru")) else {
    println!("No repositories contain paru");
    return;
};

for (repository, pkg) in pkgs.entries() {
    let name = pkg.name().unwrap();
    let version = pkg.version().unwrap();
    println!("{repository}/{name} {version}");
}
```

### More examples

See the [examples](https://github.com/pacman-repo-builder/arch-pkg-db/tree/master/examples) directory.

## License

[MIT](https://github.com/pacman-repo-builder/arch-pkg-db/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub).
