pub mod _utils;
pub use _utils::*;

use arch_pkg_db::{
    MemoQueryDatabase, TextCollection,
    desc::{
        QueryMut,
        value::{Description, Name},
    },
};
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;

fn assert_bash_db(queriers: &mut MemoQueryDatabase<'_>) {
    dbg!(&queriers);

    let querier = queriers.get_mut(Name("bash")).unwrap();
    assert_eq!(querier.name_mut(), Some(Name("bash")));
    assert_eq!(
        querier.description_mut(),
        Some(Description("The GNU Bourne Again shell")),
    );

    let querier = queriers.get_mut(Name("bash-completion")).unwrap();
    assert_eq!(querier.name_mut(), Some(Name("bash-completion")));
    assert_eq!(
        querier.description_mut(),
        Some(Description("Programmable completion for the bash shell")),
    );

    assert!(queriers.get(Name("not-exist")).is_none());
}

#[test]
fn valid_tar() {
    let texts = BASH_TAR.as_slice().pipe(TextCollection::from_tar).unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);
}

#[test]
fn valid_tgz() {
    let texts = BASH_TGZ.as_slice().pipe(TextCollection::from_gz).unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);
}

#[test]
fn valid_txz() {
    let texts = BASH_TXZ.as_slice().pipe(TextCollection::from_xz).unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);
}

#[test]
fn detect_archive_type() {
    eprintln!("CASE: tar");
    let texts = BASH_TAR
        .as_slice()
        .pipe(TextCollection::from_archive)
        .unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);

    eprintln!("CASE: tgz");
    let texts = BASH_TGZ
        .as_slice()
        .pipe(TextCollection::from_archive)
        .unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);

    eprintln!("CASE: txz");
    let texts = BASH_TXZ
        .as_slice()
        .pipe(TextCollection::from_archive)
        .unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);
}

#[test]
fn valid_local() {
    let local_db = Temp::bash_db();
    let texts = TextCollection::from_local_db(&local_db).unwrap();
    let mut queriers: MemoQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&mut queriers);
}
