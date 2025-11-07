use _utils::fixtures;
use arch_pkg_db::{
    EagerQueryDatabase, TextCollection,
    desc::Query,
    single::Entry,
    value::{Description, Name},
};
use itertools::Itertools;
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

    assert_eq!(
        queriers.names().sorted().collect::<Vec<_>>(),
        ["bash", "bash-completion"].map(Name),
    );

    assert_eq!(
        queriers
            .queriers()
            .map(Query::name)
            .sorted()
            .collect::<Vec<_>>(),
        ["bash", "bash-completion"].map(Name).map(Some),
    );

    assert_eq!(
        queriers
            .entries()
            .map(Entry::into_tuple)
            .map(|(name, querier)| (name, querier.name()))
            .sorted()
            .collect::<Vec<_>>(),
        ["bash", "bash-completion"]
            .map(Name)
            .map(|name| (name, Some(name))),
    )
}

#[test]
fn db_parse_get() {
    let texts = TextCollection::new()
        .add_item(fixtures::core::BASH.into())
        .add_item(fixtures::extra::BASH_COMPLETION.into());
    let queriers: EagerQueryDatabase = texts.parse().unwrap();
    assert_bash_db(&queriers);
}

#[test]
fn db_par_parse_get() {
    let texts = TextCollection::new()
        .add_item(fixtures::core::BASH.into())
        .add_item(fixtures::extra::BASH_COMPLETION.into());
    let queriers: EagerQueryDatabase = texts.par_parse().unwrap();
    assert_bash_db(&queriers);
}
