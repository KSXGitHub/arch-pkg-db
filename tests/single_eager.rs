use _utils::fixtures::archives::official::DB_TEXTS;
use arch_pkg_db::{
    EagerQueryDatabase, Text, TextCollection,
    desc::Query,
    single::Entry,
    value::{DependencyName, Description, Name},
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

fn assert_provider_names<const LEN: usize>(
    queriers: &EagerQueryDatabase,
    target: &'static str,
    expected_names: [&'static str; LEN],
) {
    eprintln!("PROVIDES: {target}");
    let actual_names: Vec<_> = queriers
        .alternative_providers(DependencyName(target))
        .flat_map(|provider| provider.name())
        .map(|name| name.as_str())
        .sorted()
        .collect();
    assert_eq!(actual_names, expected_names);
    assert!(!actual_names.contains(&target));
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

#[test]
fn alternative_providers() {
    let texts: TextCollection = DB_TEXTS.iter().copied().map(Text::from).collect();
    let queriers: EagerQueryDatabase = texts.parse().unwrap();
    assert_provider_names(&queriers, "sh", ["bash"]);
    assert_provider_names(&queriers, "bash", []);
    assert_provider_names(&queriers, "libalpm.so", ["pacman"]);
    assert_provider_names(&queriers, "pacman", []);
    assert_provider_names(&queriers, "rust", ["rustup"]);
    assert_provider_names(&queriers, "cargo", ["rust", "rustup"]);
}
