use arch_pkg_db::{TextCollection, desc::EagerQuerier};
use pipe_trait::Pipe;

fn main() {
    let texts = "/var/lib/pacman/local/"
        .pipe_as_ref(TextCollection::from_local_db)
        .expect("load text collection");
    let db = texts.par_parse::<EagerQuerier>().expect("parse queriers");
    dbg!(&db); // TODO: do more
}
