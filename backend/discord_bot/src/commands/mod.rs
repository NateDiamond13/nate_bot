mod music;
mod patch_notes;
mod pictures;
mod ping;
mod purge;
mod roles;
mod roll;
mod utils;

// use crate::lavalink;
use crate::prelude::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        // music::play(),
        // music::skip(),
        // music::stop(),
        // lavalink::commands::play(),
        // lavalink::commands::stop(),
        patch_notes::patch_notes(),
        pictures::pictures(),
        ping::ping(),
        purge::purge(),
        roles::roles(),
        roll::roll(),
    ]
}
