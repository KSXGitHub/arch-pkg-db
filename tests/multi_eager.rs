pub mod _utils;

use _utils::{MULTI_TEXTS, fixtures};
use arch_pkg_db::{
    EagerMultiQueryDatabase,
    desc::{EagerQuerier, Query},
    misc::{Attached, AttachedUtils},
    multi::MultiQuerier,
    value::{DependencyName, Name, ParsedVersion, RepositoryName, Version},
};
use itertools::Itertools;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

type ProviderPair<'a> = (
    RepositoryName<'a>,
    &'a Attached<EagerQuerier<'a>, ParsedVersion<'a>>,
);

fn assert_db_get(
    db: &EagerMultiQueryDatabase,
    name: Name<'static>,
    assertion: fn(&MultiQuerier<EagerQuerier>),
) {
    eprintln!();
    eprintln!("============");
    eprintln!("PACKAGE: {name}");
    db.get(name).unwrap().pipe(assertion);
}

fn assert_alternative_providers(
    db: &EagerMultiQueryDatabase,
    target: DependencyName<'static>,
    assertion: fn(&[ProviderPair]),
) {
    eprintln!();
    eprintln!("============");
    eprintln!("PROVIDES: {target}");
    let providers: Vec<_> = db
        .alternative_providers(target)
        .sorted_by_key(|(repository, pkg)| {
            (repository.as_str(), pkg.name().map(|pkg| pkg.as_str()))
        })
        .collect();
    assertion(&providers);
}

fn assert_repositories<const LEN: usize>(
    multi_querier: &MultiQuerier<EagerQuerier>,
    expected: [&'static str; LEN],
) {
    let repositories: Vec<_> = multi_querier
        .repositories()
        .map(|x| x.as_str())
        .sorted()
        .collect();
    dbg!(&repositories);
    assert_eq!(repositories, expected);
    assert_eq!(multi_querier.len(), LEN);
    assert_eq!(multi_querier.len(), repositories.len());
}

fn assert_repositories_packages<const LEN: usize>(
    pairs: &[ProviderPair],
    expected: [(&'static str, &'static str); LEN],
) {
    let repositories_packages: Vec<_> = pairs
        .iter()
        .map(|(repository, pkg)| (*repository, pkg.name()))
        .sorted_by_key(|(repository, pkg)| (repository.as_str(), pkg.map(|pkg| pkg.as_str())))
        .collect();
    let expected = expected.map(|(repository, pkg)| (RepositoryName(repository), Some(Name(pkg))));
    dbg!(&repositories_packages);
    assert_eq!(repositories_packages, expected);
}

fn assert_package<'a, Querier: Query<'a>>(
    actual_pkg: &Attached<Querier, ParsedVersion>,
    expected_desc: &'static str,
    expected_name: Name<'static>,
    expected_version: Version<'static>,
) {
    let expected_pkg = expected_desc.pipe(EagerQuerier::parse).unwrap();
    assert_eq!(actual_pkg.name(), Some(expected_name));
    assert_eq!(actual_pkg.name(), expected_pkg.name());
    let actual_version = actual_pkg.version().unwrap();
    assert_eq!(actual_version.as_str(), expected_version.as_str());
    assert_eq!(
        actual_version.as_str(),
        expected_pkg.version().unwrap().as_str(),
    );
    assert_eq!(*actual_pkg.attachment(), actual_version.parse().unwrap());
}

fn assert_multi_querier_get(
    multi_querier: &MultiQuerier<EagerQuerier>,
    repository: RepositoryName,
    expected_desc: &'static str,
    expected_name: Name<'static>,
    expected_version: Version<'static>,
) {
    eprintln!("REPOSITORY/PACKAGE: {repository}/{expected_name}");
    let actual_pkg = multi_querier.get(repository).unwrap();
    assert_package(&actual_pkg, expected_desc, expected_name, expected_version);
}

fn assert_repository_package(
    (repository, actual_pkg): &ProviderPair,
    expected_desc: &'static str,
    expected_name: Name<'static>,
    expected_version: Version<'static>,
) {
    eprintln!("REPOSITORY/PACKAGE: {repository}/{expected_name}");
    assert_package(actual_pkg, expected_desc, expected_name, expected_version);
}

fn assert_not_provide_itself(pairs: &[ProviderPair], name: Name<'static>) {
    assert!(
        pairs.iter().all(|(_, pkg)| pkg.name() != Some(name)),
        "it is expected for alternative_providers of {name} to not include itself when its own %PROVIDES% doesn't",
    );
}

fn assert_bash(multi_querier: &MultiQuerier<EagerQuerier>) {
    assert_repositories(multi_querier, ["core"]);
    assert_multi_querier_get(
        multi_querier,
        RepositoryName("core"),
        fixtures::core::BASH,
        Name("bash"),
        Version("5.2.026-2"),
    );
}

fn assert_bash_completion(multi_querier: &MultiQuerier<EagerQuerier>) {
    assert_repositories(multi_querier, ["extra"]);
    assert_multi_querier_get(
        multi_querier,
        RepositoryName("extra"),
        fixtures::extra::BASH_COMPLETION,
        Name("bash-completion"),
        Version("2.14.0-2"),
    );
}

fn assert_parallel_disk_usage(multi_querier: &MultiQuerier<EagerQuerier>) {
    assert_repositories(multi_querier, ["extra", "personal"]);
    assert_multi_querier_get(
        multi_querier,
        RepositoryName("extra"),
        fixtures::extra::PARALLEL_DISK_USAGE,
        Name("parallel-disk-usage"),
        Version("0.21.1-1"),
    );
    assert_multi_querier_get(
        multi_querier,
        RepositoryName("personal"),
        fixtures::personal::PARALLEL_DISK_USAGE,
        Name("parallel-disk-usage"),
        Version("0.9.2-1"),
    );
}

fn assert_paru(multi_querier: &MultiQuerier<EagerQuerier>) {
    assert_repositories(multi_querier, ["derivative", "personal"]);
    assert_multi_querier_get(
        multi_querier,
        RepositoryName("derivative"),
        fixtures::derivative::PARU,
        Name("paru"),
        Version("2.1.0-1"),
    );
    assert_multi_querier_get(
        multi_querier,
        RepositoryName("personal"),
        fixtures::personal::PARU,
        Name("paru"),
        Version("2.0.3-1"),
    );
}

fn assert_empty_without_so(pairs: &[ProviderPair]) {
    assert!(
        pairs.is_empty(),
        ".so is always required for a library file name as a provider target",
    );
}

fn assert_provides_sh(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("core", "bash")]);
    assert_repository_package(
        &pairs[0],
        fixtures::core::BASH,
        Name("bash"),
        Version("5.2.026-2"),
    );
    assert_not_provide_itself(pairs, Name("sh"));
}

fn assert_provides_libalpm(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("core", "pacman")]);
    assert_repository_package(
        &pairs[0],
        fixtures::core::PACMAN,
        Name("pacman"),
        Version("7.0.0.r6.gc685ae6-6"),
    );
    assert_not_provide_itself(pairs, Name("libalpm.so"));
}

fn assert_provides_libhistory(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("core", "readline")]);
    assert_repository_package(
        &pairs[0],
        fixtures::core::READLINE,
        Name("readline"),
        Version("8.3.001-1"),
    );
    assert_not_provide_itself(pairs, Name("libhistory.so"));
}

fn assert_provides_libreadline(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("core", "readline")]);
    assert_repository_package(
        &pairs[0],
        fixtures::core::READLINE,
        Name("readline"),
        Version("8.3.001-1"),
    );
    assert_not_provide_itself(pairs, Name("libreadline.so"));
}

fn assert_provides_rust(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("extra", "rustup")]);
    assert_repository_package(
        &pairs[0],
        fixtures::extra::RUSTUP,
        Name("rustup"),
        Version("1.28.2-3"),
    );
    assert_not_provide_itself(pairs, Name("rust"));
}

fn assert_provides_cargo(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("extra", "rust"), ("extra", "rustup")]);
    assert_repository_package(
        &pairs[0],
        fixtures::extra::RUST,
        Name("rust"),
        Version("1:1.90.0-3"),
    );
    assert_repository_package(
        &pairs[1],
        fixtures::extra::RUSTUP,
        Name("rustup"),
        Version("1.28.2-3"),
    );
    assert_not_provide_itself(pairs, Name("cargo"));
}

fn assert_provides_rustfmt(pairs: &[ProviderPair]) {
    assert_repositories_packages(pairs, [("extra", "rust"), ("extra", "rustup")]);
    assert_repository_package(
        &pairs[0],
        fixtures::extra::RUST,
        Name("rust"),
        Version("1:1.90.0-3"),
    );
    assert_repository_package(
        &pairs[1],
        fixtures::extra::RUSTUP,
        Name("rustup"),
        Version("1.28.2-3"),
    );
    assert_not_provide_itself(pairs, Name("rustfmt"));
}

#[test]
fn db_parse_get() {
    let db: EagerMultiQueryDatabase = MULTI_TEXTS.parse().unwrap();
    assert_db_get(&db, Name("bash"), assert_bash);
    assert_db_get(&db, Name("bash-completion"), assert_bash_completion);
    assert_db_get(&db, Name("parallel-disk-usage"), assert_parallel_disk_usage);
    assert_db_get(&db, Name("paru"), assert_paru);
}

#[test]
fn db_par_parse_get() {
    let db: EagerMultiQueryDatabase = MULTI_TEXTS.par_parse().unwrap();
    assert_db_get(&db, Name("bash"), assert_bash);
    assert_db_get(&db, Name("bash-completion"), assert_bash_completion);
    assert_db_get(&db, Name("parallel-disk-usage"), assert_parallel_disk_usage);
    assert_db_get(&db, Name("paru"), assert_paru);
}

#[test]
fn alternative_providers() {
    let db: EagerMultiQueryDatabase = MULTI_TEXTS.parse().unwrap();
    assert_alternative_providers(&db, DependencyName("sh"), assert_provides_sh);
    assert_alternative_providers(&db, DependencyName("libalpm.so"), assert_provides_libalpm);
    assert_alternative_providers(&db, DependencyName("libalpm"), assert_empty_without_so);
    assert_alternative_providers(
        &db,
        DependencyName("libhistory.so"),
        assert_provides_libhistory,
    );
    assert_alternative_providers(&db, DependencyName("libhistory"), assert_empty_without_so);
    assert_alternative_providers(
        &db,
        DependencyName("libreadline.so"),
        assert_provides_libreadline,
    );
    assert_alternative_providers(&db, DependencyName("libreadline"), assert_empty_without_so);
    assert_alternative_providers(&db, DependencyName("rust"), assert_provides_rust);
    assert_alternative_providers(&db, DependencyName("cargo"), assert_provides_cargo);
    assert_alternative_providers(&db, DependencyName("rustfmt"), assert_provides_rustfmt);
}
