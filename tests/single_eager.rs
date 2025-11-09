use _utils::fixtures::archives::official::DB_TEXTS;
use arch_pkg_db::{
    EagerQueryDatabase, Text, TextCollection,
    desc::Query,
    single::Entry,
    value::{Description, Name},
};
use itertools::Itertools;
use pretty_assertions::assert_eq;

fn assert_official_db(queriers: &EagerQueryDatabase<'_>) {
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

    assert_eq!(queriers.names().sorted().collect::<Vec<_>>(), all_names);

    assert_eq!(
        queriers
            .queriers()
            .map(Query::name)
            .sorted()
            .collect::<Vec<_>>(),
        all_names.map(Some),
    );

    assert_eq!(
        queriers
            .entries()
            .map(Entry::into_tuple)
            .map(|(name, querier)| (name, querier.name()))
            .sorted()
            .collect::<Vec<_>>(),
        all_names.map(|name| (name, Some(name))),
    )
}

#[test]
fn db_parse_get() {
    let texts: TextCollection = DB_TEXTS.iter().copied().map(Text::from).collect();
    let queriers: EagerQueryDatabase = texts.parse().unwrap();
    assert_official_db(&queriers);
}

#[test]
fn db_par_parse_get() {
    let texts: TextCollection = DB_TEXTS.iter().copied().map(Text::from).collect();
    let queriers: EagerQueryDatabase = texts.par_parse().unwrap();
    assert_official_db(&queriers);
}
