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
    let pref_max_chars = 500;
    let trunc_length = 4096;

    let Ok(markdown_str) = htmd::convert(content) else {
        return "".to_string();
    };

    let lines = markdown_str
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    let mut total_lines = 0;
    let mut char_count = 0;
    for line in lines.iter().take(max_lines) {
        total_lines += 1;
        char_count += line.len();
        if char_count >= pref_max_chars {
            break;
        }
    }

    let mut description = if lines.len() > total_lines {
        [&lines[0..total_lines], &["..."]].concat()
    } else {
        lines
    }
    .join("\n");
    description.truncate(trunc_length);
    description
}
