pub mod _utils;

use _utils::MULTI_TEXTS;
use arch_pkg_db::{
    desc::{EagerQuerier, Query},
    multi::{
        EagerMultiQueryDatabaseLatest, WithParsedVersion, WithParsedVersionUtils,
        WithRepositoryName, WithRepositoryNameUtils,
    },
    value::{Name, RepositoryName},
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

type QuerierVersionRepository<'a> =
    WithRepositoryName<'a, WithParsedVersion<'a, &'a EagerQuerier<'a>>>;

fn assert_db_get(
    db: &EagerMultiQueryDatabaseLatest,
    name: Name<'static>,
    assertion: fn(QuerierVersionRepository),
) {
    eprintln!();
    eprintln!("============");
    eprintln!("PACKAGE: {name}");
    db.get(name).unwrap().pipe(assertion);
}

fn assert_bash(querier: QuerierVersionRepository) {
    assert_eq!(querier.name(), Some(Name("bash")));
    let version = querier.version().unwrap();
    assert_eq!(version.as_str(), "5.2.026-2");
    assert_eq!(querier.repository_name(), RepositoryName("core"));
    assert_eq!(querier.parsed_version(), version.parse().unwrap());
}
fn assert_bash_completion(querier: QuerierVersionRepository) {
    assert_eq!(querier.name(), Some(Name("bash-completion")));
    let version = querier.version().unwrap();
    assert_eq!(version.as_str(), "2.14.0-2");
    assert_eq!(querier.repository_name(), RepositoryName("extra"));
    assert_eq!(querier.parsed_version(), version.parse().unwrap());
}

fn assert_parallel_disk_usage(querier: QuerierVersionRepository) {
    assert_eq!(querier.name(), Some(Name("parallel-disk-usage")));
    let version = querier.version().unwrap();
    assert_eq!(querier.repository_name(), RepositoryName("extra"));
    assert_eq!(querier.parsed_version(), version.parse().unwrap());
}

fn assert_paru(querier: QuerierVersionRepository) {
    assert_eq!(querier.name(), Some(Name("paru")));
    let version = querier.version().unwrap();
    assert_eq!(version.as_str(), "2.1.0-1");
    assert_eq!(querier.repository_name(), RepositoryName("derivative"));
    assert_eq!(querier.parsed_version(), version.parse().unwrap());
}

#[test]
fn db_parse_latest_get() {
    let db = MULTI_TEXTS.parse().unwrap();
    let db: EagerMultiQueryDatabaseLatest = db.latest();
    assert_db_get(&db, Name("bash"), assert_bash);
    assert_db_get(&db, Name("bash-completion"), assert_bash_completion);
    assert_db_get(&db, Name("parallel-disk-usage"), assert_parallel_disk_usage);
    assert_db_get(&db, Name("paru"), assert_paru);
}

#[test]
fn db_par_parse_latest_get() {
    let db = MULTI_TEXTS.par_parse().unwrap();
    let db: EagerMultiQueryDatabaseLatest = db.latest();
    assert_db_get(&db, Name("bash"), assert_bash);
    assert_db_get(&db, Name("bash-completion"), assert_bash_completion);
    assert_db_get(&db, Name("parallel-disk-usage"), assert_parallel_disk_usage);
    assert_db_get(&db, Name("paru"), assert_paru);
}
