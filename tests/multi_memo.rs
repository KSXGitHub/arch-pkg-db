pub mod _utils;

use _utils::{MULTI_TEXTS, fixtures};
use arch_pkg_db::{
    MemoMultiQueryDatabase,
    desc::{MemoQuerier, QueryMut},
    misc::{Attached, AttachedUtils},
    multi::MultiQuerier,
    value::{Name, ParsedVersion, RepositoryName, Version},
};
use itertools::Itertools;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

fn assert_db_get_mut(
    db: &mut MemoMultiQueryDatabase,
    name: Name<'static>,
    assertion: fn(&mut MultiQuerier<MemoQuerier>),
) {
    eprintln!();
    eprintln!("============");
    eprintln!("PACKAGE: {name}");
    db.get_mut(name).unwrap().pipe(assertion);
}

fn assert_repositories<const LEN: usize>(
    multi_querier: &MultiQuerier<MemoQuerier>,
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

fn assert_package<'a, Querier: QueryMut<'a>>(
    actual_pkg: &mut Attached<Querier, ParsedVersion>,
    expected_desc: &'static str,
    expected_name: Name<'static>,
    expected_version: Version<'static>,
) {
    use arch_pkg_db::desc::{EagerQuerier, Query};
    let expected_pkg = expected_desc.pipe(EagerQuerier::parse).unwrap();
    assert_eq!(actual_pkg.name_mut(), Some(expected_name));
    assert_eq!(actual_pkg.name_mut(), expected_pkg.name());
    let actual_version = actual_pkg.version_mut().unwrap();
    assert_eq!(actual_version.as_str(), expected_version.as_str());
    assert_eq!(
        actual_version.as_str(),
        expected_pkg.version().unwrap().as_str(),
    );
    assert_eq!(*actual_pkg.attachment(), actual_version.parse().unwrap());
}

fn assert_multi_querier_get_mut(
    multi_querier: &mut MultiQuerier<MemoQuerier>,
    repository: RepositoryName,
    expected_desc: &'static str,
    expected_name: Name<'static>,
    expected_version: Version<'static>,
) {
    eprintln!("REPOSITORY/PACKAGE: {repository}/{expected_name}");
    let mut actual_pkg = multi_querier.get_mut(repository).unwrap();
    assert_package(
        &mut actual_pkg,
        expected_desc,
        expected_name,
        expected_version,
    );
}

fn assert_bash(multi_querier: &mut MultiQuerier<MemoQuerier>) {
    assert_repositories(multi_querier, ["core"]);
    assert_multi_querier_get_mut(
        multi_querier,
        RepositoryName("core"),
        fixtures::core::BASH,
        Name("bash"),
        Version("5.2.026-2"),
    );
}

fn assert_bash_completion(multi_querier: &mut MultiQuerier<MemoQuerier>) {
    assert_repositories(multi_querier, ["extra"]);
    assert_multi_querier_get_mut(
        multi_querier,
        RepositoryName("extra"),
        fixtures::extra::BASH_COMPLETION,
        Name("bash-completion"),
        Version("2.14.0-2"),
    );
}

fn assert_parallel_disk_usage(multi_querier: &mut MultiQuerier<MemoQuerier>) {
    assert_repositories(multi_querier, ["extra", "personal"]);
    assert_multi_querier_get_mut(
        multi_querier,
        RepositoryName("extra"),
        fixtures::extra::PARALLEL_DISK_USAGE,
        Name("parallel-disk-usage"),
        Version("0.21.1-1"),
    );
    assert_multi_querier_get_mut(
        multi_querier,
        RepositoryName("personal"),
        fixtures::personal::PARALLEL_DISK_USAGE,
        Name("parallel-disk-usage"),
        Version("0.9.2-1"),
    );
}

fn assert_paru(multi_querier: &mut MultiQuerier<MemoQuerier>) {
    assert_repositories(multi_querier, ["derivative", "personal"]);
    assert_multi_querier_get_mut(
        multi_querier,
        RepositoryName("derivative"),
        fixtures::derivative::PARU,
        Name("paru"),
        Version("2.1.0-1"),
    );
    assert_multi_querier_get_mut(
        multi_querier,
        RepositoryName("personal"),
        fixtures::personal::PARU,
        Name("paru"),
        Version("2.0.3-1"),
    );
}

#[test]
fn db_parse_mut_get_mut() {
    let mut db: MemoMultiQueryDatabase = MULTI_TEXTS.parse_mut().unwrap();
    assert_db_get_mut(&mut db, Name("bash"), assert_bash);
    assert_db_get_mut(&mut db, Name("bash-completion"), assert_bash_completion);
    assert_db_get_mut(
        &mut db,
        Name("parallel-disk-usage"),
        assert_parallel_disk_usage,
    );
    assert_db_get_mut(&mut db, Name("paru"), assert_paru);
}

#[test]
fn db_par_parse_mut_get_mut() {
    let mut db: MemoMultiQueryDatabase = MULTI_TEXTS.par_parse_mut().unwrap();
    assert_db_get_mut(&mut db, Name("bash"), assert_bash);
    assert_db_get_mut(&mut db, Name("bash-completion"), assert_bash_completion);
    assert_db_get_mut(
        &mut db,
        Name("parallel-disk-usage"),
        assert_parallel_disk_usage,
    );
    assert_db_get_mut(&mut db, Name("paru"), assert_paru);
}
