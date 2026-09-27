//! What a person is told on the run that makes their node's name.
//!
//! Beside the commands rather than inside one, because every command that can make a
//! name says it: `id`, `serve`, `join`, `bootstrap`, `status`, `say` and `ping`.

/// Say, on the run that made this node's name, that it was made and where to keep it.
///
/// Every command that can make a name says this, and only on that run: it is the one
/// run on which the warning can still be acted on, and a person who began with `serve`
/// or `join` instead of `id` is owed it just the same. One function, so that nobody is
/// told less because of which command they typed first.
pub(crate) fn report(origin: crate::identity_file::Origin, home: &std::path::Path) {
    let crate::identity_file::Origin::Created { not_called } = origin else {
        return;
    };
    aloud!("{}", crate::commands::naming(not_called));
    aloud!("home     {}", home.display());
    aloud!(
        "keep     that directory. lose it and you lose this name, every hour\n\
         \x20        anyone ever witnessed for you, and any way of proving you\n\
         \x20        were here. There is no recovery and there is no appeal.\n\
         \x20        To move it to another machine: `333 pack <FILE>` here, then\n\
         \x20        `333 unpack <FILE>` there."
    );
}
