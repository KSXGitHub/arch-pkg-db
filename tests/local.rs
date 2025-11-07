use _utils::temp::Temp;
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
fn from_local_db() {
    let local_db = Temp::bash_db();
    let texts = TextCollection::from_local_db(&local_db).unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}

#[test]
fn par_from_local_db() {
    let local_db = Temp::bash_db();
    let texts = TextCollection::par_from_local_db(&local_db).unwrap();
    let queriers: EagerQueryDatabase<'_> = texts.parse().unwrap();
    assert_bash_db(&queriers);
}
