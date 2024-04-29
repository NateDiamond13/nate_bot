use crate::prelude::{Context, Result};

use poise::{builtins, command};

/// Show help menu for all commands or a specific command.
#[command(prefix_command, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<()> {
    let prefix = ctx.prefix();

    builtins::help(
        ctx,
        command.as_deref(),
        builtins::HelpConfiguration {
            extra_text_at_bottom: format!("Type {prefix}help command for more info on a command.")
                .as_str(),
            show_subcommands: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
