pub mod _utils;
pub use _utils::*;

use arch_pkg_db::{
    EagerQueryDatabase, TextCollection,
    desc::{
        Query,
        value::{Description, Name},
    },
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

fn assert_bash_db(queriers: &EagerQueryDatabase<'_>) {
    dbg!(&queriers);

    let querier = queriers.get(Name("bash")).unwrap();
    assert_eq!(querier.name(), Some(Name("bash")));
    assert_eq!(
        querier.description(),
        Some(Description("The GNU Bourne Again shell")),
    );

    let querier = queriers.get(Name("bash-completion")).unwrap();
    assert_eq!(querier.name(), Some(Name("bash-completion")));
    assert_eq!(
        querier.description(),
        Some(Description("Programmable completion for the bash shell")),
    );

    assert!(queriers.get(Name("not-exist")).is_none());
}

#[test]
fn valid_tar() {
    let texts = BASH_TAR.as_slice().pipe(TextCollection::from_tar).unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}

#[test]
fn valid_tgz() {
    let texts = BASH_TGZ.as_slice().pipe(TextCollection::from_gz).unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}

#[test]
fn valid_txz() {
    let texts = BASH_TXZ.as_slice().pipe(TextCollection::from_xz).unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}

#[test]
fn detect_archive_type() {
    eprintln!("CASE: tar");
    let texts = BASH_TAR
        .as_slice()
        .pipe(TextCollection::from_archive)
        .unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);

    eprintln!("CASE: tgz");
    let texts = BASH_TGZ
        .as_slice()
        .pipe(TextCollection::from_archive)
        .unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);

    eprintln!("CASE: txz");
    let texts = BASH_TXZ
        .as_slice()
        .pipe(TextCollection::from_archive)
        .unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}

#[test]
fn valid_local() {
    let texts = TextCollection::from_local_db(&BASH_LOCAL).unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}
