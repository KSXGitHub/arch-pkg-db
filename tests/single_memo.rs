use _utils::fixtures::archives::official::DB_TEXTS;
use arch_pkg_db::{
    MemoQueryDatabase, Text, TextCollection,
    desc::QueryMut,
    single::Entry,
    value::{Description, Name},
};
use itertools::Itertools;
use pretty_assertions::assert_eq;

fn assert_official_db(queriers: &mut MemoQueryDatabase<'_>) {
    dbg!(&queriers);

    let all_names = [
        "bash",
        "bash-completion",
        "glibc",
        "ncurses",
        "pacman",
        "parallel-disk-usage",
        "readline",
        "rust",
        "rustup",
    ]
    .map(Name);

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

    assert!(queriers.get_mut(Name("not-exist")).is_none());
    assert_eq!(queriers.names().sorted().collect::<Vec<_>>(), all_names,);

    assert_eq!(
        queriers
            .queriers_mut()
            .map(QueryMut::name_mut)
            .sorted()
            .collect::<Vec<_>>(),
        all_names.map(Some),
    );

    assert_eq!(
        queriers
            .entries_mut()
            .map(Entry::into_tuple)
            .map(|(name, querier)| (name, querier.name_mut()))
            .sorted()
            .collect::<Vec<_>>(),
        all_names.map(|name| (name, Some(name))),
    )
}

#[test]
fn db_parse_mut_get_mut() {
    let texts: TextCollection = DB_TEXTS.iter().copied().map(Text::from).collect();
    let mut queriers: MemoQueryDatabase = texts.parse_mut().unwrap();
    assert_official_db(&mut queriers);
}

#[test]
fn db_par_parse_mut_get_mut() {
    let texts: TextCollection = DB_TEXTS.iter().copied().map(Text::from).collect();
    let mut queriers: MemoQueryDatabase = texts.par_parse_mut().unwrap();
    assert_official_db(&mut queriers);
}
