use crate::prelude::{Context, Error, Result};

use poise::command;
use rand::{thread_rng, Rng};
use regex::Regex;
use serenity::all::Mentionable;
use serenity::builder::{CreateEmbed, CreateMessage};

const MAX_ROLLS: u32 = 50;
const DEFAULT_ROLL: u32 = 100;

#[derive(Debug)]
enum ParsedRoll {
    Default,
    Number(u32),
    Dice(u32, u32),
}

/// Roll some dice.
#[command(prefix_command, slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "A positive number or multi-dice roll (XdY => 1d6, 2d20, etc.)"]
    custom_roll: Vec<String>,
) -> Result<()> {
    let prefix = ctx.prefix();
    let parse_result = parse_roll_args(custom_roll);
    if parse_result.is_err() {
        ctx.say(format!("Argument for {prefix}roll command must be either a number (6, 8, etc.) or in XdY format (1d6, 2d20, etc.) where X <= {MAX_ROLLS}")).await?;
        return Ok(());
    }

    let author_mention = ctx.author().mention().to_string();
    match parse_result.unwrap() {
        ParsedRoll::Default => {
            let response = format_simple_roll(author_mention, DEFAULT_ROLL);
            ctx.say(response).await?;
        }
        ParsedRoll::Number(num) => {
            let response = format_simple_roll(author_mention, num);
            ctx.say(response).await?;
        }
        ParsedRoll::Dice(count, sides) => {
            let response = format_dice_roll(author_mention, count, sides);
            ctx.channel_id().send_message(ctx, response).await?;
        }
    }
    Ok(())
}

fn parse_roll_args(args: Vec<String>) -> Result<ParsedRoll> {
    if args.is_empty() {
        return Ok(ParsedRoll::Default);
    } else if args.len() > 1 {
        return Err(Error::CommandArgParse);
    }

    if let Ok(val) = args[0].parse() {
        return Ok(ParsedRoll::Number(val));
    }

    let re = Regex::new(r"^([0-9]+)d([0-9]+)$")?;
    let (_, [count, sides]) = re
        .captures(&args[0])
        .ok_or_else(|| Error::CommandArgParse)?
        .extract();

    let count: u32 = count.parse().map_err(|_| Error::CommandArgParse)?;
    let sides: u32 = sides.parse().map_err(|_| Error::CommandArgParse)?;

    if count <= MAX_ROLLS {
        return Ok(ParsedRoll::Dice(count, sides));
    }

    Err(Error::CommandArgParse)
}

fn format_simple_roll(author: String, max_roll: u32) -> String {
    let roll = thread_rng().gen_range(1..max_roll);
    format!("{author} rolls a {roll} (1-{max_roll})")
}

fn format_dice_roll(author: String, count: u32, sides: u32) -> CreateMessage {
    let mut rolls = Vec::new();
    for _ in 0..count {
        rolls.push(thread_rng().gen_range(1..sides));
    }
    let total: u32 = rolls.iter().sum();

    let embed = CreateEmbed::new().description(format!(
        "**Result**: {count}d{sides} -> {rolls:?}\n**Total**: {total}"
    ));
    CreateMessage::new()
        .content(format!("{author} :game_die: **{total}**"))
        .embed(embed)
}

#[cfg(test)]
mod tests {
    use crate::commands::roll::{parse_roll_args, ParsedRoll};
    use crate::prelude::Error;

    #[test]
    fn parse_good_roll() {
        let v1 = vec![];
        assert!(matches!(parse_roll_args(v1), Ok(ParsedRoll::Default)));

        let v2 = vec!["50".to_string()];
        assert!(matches!(parse_roll_args(v2), Ok(ParsedRoll::Number(50))));

        let v3 = vec!["2d6".to_string()];
        assert!(matches!(parse_roll_args(v3), Ok(ParsedRoll::Dice(2, 6))));
    }

    #[test]
    fn parse_bad_roll() {
        let v1 = vec!["50".to_string(), "100".to_string()];
        assert!(matches!(parse_roll_args(v1), Err(Error::CommandArgParse)));

        let v2 = vec!["50x".to_string()];
        assert!(matches!(parse_roll_args(v2), Err(Error::CommandArgParse)));

        let v3 = vec!["-100".to_string()];
        assert!(matches!(parse_roll_args(v3), Err(Error::CommandArgParse)));

        let v4 = vec!["100000000000000000".to_string()];
        assert!(matches!(parse_roll_args(v4), Err(Error::CommandArgParse)));

        let v5 = vec!["70d100".to_string()];
        assert!(matches!(parse_roll_args(v5), Err(Error::CommandArgParse)));
    }
}
