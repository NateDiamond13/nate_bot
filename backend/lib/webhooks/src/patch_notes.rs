use crate::prelude::Result;

use database::{patch_notes::PatchNotes, patch_notes_subscriptions::PatchNotesSub};
use serenity::all::{CreateEmbed, CreateEmbedAuthor, ExecuteWebhook, Http, Webhook};

pub async fn send_all_alerts(patch_notes: &PatchNotes, subs: &[PatchNotesSub]) -> Result<()> {
    let embed = create_patch_embed(patch_notes);
    let http = Http::new("");

    for sub in subs {
        if let Ok(hook) =
            Webhook::from_id_with_token(&http, sub.webhook_id.into(), &sub.webhook_token).await
        {
            let builder = ExecuteWebhook::new().embed(embed.clone());
            hook.execute(&http, false, builder).await?;
        }
    }

    Ok(())
}

pub fn create_patch_embed(patch_notes: &PatchNotes) -> CreateEmbed<'_> {
    let mut embed = CreateEmbed::new()
        .title(&patch_notes.title)
        .url(&patch_notes.link)
        .author(CreateEmbedAuthor::new(&patch_notes.game_title))
        .description(format_embed_description(&patch_notes.content));
    if let Some(thumbnail) = &patch_notes.thumbnail_url {
        embed = embed.thumbnail(thumbnail);
    }
    embed
}

fn format_embed_description(content: &str) -> String {
    let max_lines = 8;
    let max_length = 4096;

    let parsed = content.replace("<br>", "");
    let lines = parsed
        .lines()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut description = if lines.len() > max_lines {
        [&lines[0..max_lines - 1], &["..."]].concat()
    } else {
        lines
    }
    .join("\n");
    description.truncate(max_length);
    description
}
