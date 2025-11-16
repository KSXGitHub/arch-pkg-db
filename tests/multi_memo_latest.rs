pub mod _utils;

use _utils::MULTI_TEXTS;
use arch_pkg_db::{
    desc::{MemoQuerier, QueryMut},
    misc::{Attached, AttachedUtils},
    multi::MemoMultiQueryDatabaseLatest,
    value::{Name, ParsedVersion, RepositoryName},
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

type QuerierVersionRepository<'r, 'a> =
    Attached<Attached<&'r mut MemoQuerier<'a>, ParsedVersion<'a>>, RepositoryName<'a>>;

fn assert_db_get_mut(
    db: &mut MemoMultiQueryDatabaseLatest,
    name: Name<'static>,
    assertion: fn(QuerierVersionRepository),
) {
    eprintln!();
    eprintln!("============");
    eprintln!("PACKAGE: {name}");
    db.get_mut(name).unwrap().pipe(assertion);
}

fn assert_bash(mut querier: QuerierVersionRepository) {
    assert_eq!(querier.name_mut(), Some(Name("bash")));
    let version = querier.version_mut().unwrap();
    assert_eq!(version.as_str(), "5.2.026-2");
    assert_eq!(querier.attachment(), &RepositoryName("core"));
    assert_eq!(querier.main().attachment(), &version.parse().unwrap());
}

fn assert_bash_completion(mut querier: QuerierVersionRepository) {
    assert_eq!(querier.name_mut(), Some(Name("bash-completion")));
    let version = querier.version_mut().unwrap();
    assert_eq!(version.as_str(), "2.14.0-2");
    assert_eq!(querier.attachment(), &RepositoryName("extra"));
    assert_eq!(querier.main().attachment(), &version.parse().unwrap());
}

fn assert_parallel_disk_usage(mut querier: QuerierVersionRepository) {
    assert_eq!(querier.name_mut(), Some(Name("parallel-disk-usage")));
    let version = querier.version_mut().unwrap();
    assert_eq!(version.as_str(), "0.21.1-1");
    assert_eq!(querier.attachment(), &RepositoryName("extra"));
    assert_eq!(querier.main().attachment(), &version.parse().unwrap());
}

fn assert_paru(mut querier: QuerierVersionRepository) {
    assert_eq!(querier.name_mut(), Some(Name("paru")));
    let version = querier.version_mut().unwrap();
    assert_eq!(version.as_str(), "2.1.0-1");
    assert_eq!(querier.attachment(), &RepositoryName("derivative"));
    assert_eq!(querier.main().attachment(), &version.parse().unwrap());
}

#[test]
fn db_parse_mut_latest_get_mut() {
    let mut db = MULTI_TEXTS.parse_mut().unwrap();
    let mut db: MemoMultiQueryDatabaseLatest = db.latest_mut();
    assert_db_get_mut(&mut db, Name("bash"), assert_bash);
    assert_db_get_mut(&mut db, Name("bash-completion"), assert_bash_completion);
    assert_db_get_mut(
        &mut db,
        Name("parallel-disk-usage"),
        assert_parallel_disk_usage,
    );
    assert_db_get_mut(&mut db, Name("paru"), assert_paru);
}
