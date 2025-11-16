//! Intermediate utility module to contain extra features couldn't be easily expressed in the proper [`::_utils`] crate,
//! such as resources from [`arch_pkg_db`] itself with potentially multitude of compilation flag configurations.

pub use ::_utils::*;

use arch_pkg_db::{MultiTextCollection, Text, TextCollection};
use std::sync::LazyLock;

macro_rules! texts {
    ($repository:ident :: [$($source:ident),* $(,)?]) => {{
        let sources: [&'static str; _] = [$(::_utils::fixtures::$repository::$source),*];
        sources.into_iter().map(Text::from).collect::<TextCollection>()
    }};
}

macro_rules! multi_text {
    ($($repository:ident :: [$($source:ident),*]),* $(,)?) => {{
        let repositories: [(&'static str, TextCollection); _] = [$((
            stringify!($repository),
            texts!($repository :: [$($source),*]),
        )),*];
        let mut multi_text = MultiTextCollection::with_capacity(repositories.len());
        for (repository, texts) in repositories {
            multi_text.insert(repository.into(), texts);
        }
        multi_text
    }};
}

pub static MULTI_TEXTS: LazyLock<MultiTextCollection> = LazyLock::new(|| {
    multi_text!(
        core::[BASH, GLIBC, NCURSES, PACMAN, READLINE],
        extra::[BASH_COMPLETION, PARALLEL_DISK_USAGE, RUST, RUSTUP],
        derivative::[PARU],
        personal::[PARALLEL_DISK_USAGE, PARU],
    )
});
