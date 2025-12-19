mod patch_notes;
mod pictures;
mod ping;
mod purge;
mod roles;
mod roll;

use crate::prelude::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        patch_notes::patch_notes(),
        pictures::pictures(),
        ping::ping(),
        purge::purge(),
        roles::roles(),
        roll::roll(),
    ]
}
