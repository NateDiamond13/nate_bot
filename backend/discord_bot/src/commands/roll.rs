use crate::prelude::{Context, Error, Result};

use poise::{command, CreateReply};
use rand::random_range;
use regex::Regex;
use serenity::all::Mentionable;
use serenity::builder::CreateEmbed;

const MAX_ROLLS: u32 = 50;
const DEFAULT_ROLL: u32 = 100;
const EMBED_COLOR: u32 = 0x00FFBA;

struct DiceRoll {
    pub count: u32,
    pub sides: u32,
}

/// Roll a single dice (default: 100).
#[command(
    slash_command,
    category = "Roll",
    subcommands("number_roll", "dice_roll")
)]
pub async fn roll(ctx: Context<'_>) -> Result<()> {
    let author_mention = ctx.author().mention().to_string();
    let response = format_simple_roll(author_mention, DEFAULT_ROLL);
    ctx.say(response).await?;
    Ok(())
}

/// Roll a single dice (number > 0).
#[command(slash_command, category = "Roll", rename = "num")]
pub async fn number_roll(
    ctx: Context<'_>,
    #[description = "A positive integer"]
    #[min = 1]
    number: u32,
) -> Result<()> {
    let author_mention = ctx.author().mention().to_string();
    let response = format_simple_roll(author_mention, number);
    ctx.say(response).await?;
    Ok(())
}

/// Roll at least one dice (dice format: XdY).
#[command(slash_command, category = "Roll", rename = "dice")]
pub async fn dice_roll(
    ctx: Context<'_>,
    #[description = "A multi-dice roll in form XdY (e.g. 1d6, 2d20, etc.)"] dice: String,
) -> Result<()> {
    let Ok(roll) = parse_dice_string(dice) else {
        ctx.say(format!(
            "Argument must be in form XdY (1d6, 2d20, etc.) where 1 <= X <= {MAX_ROLLS}, Y >= 1"
        ))
        .await?;
        return Ok(());
    };

    let author_mention = ctx.author().mention().to_string();
    let response = format_dice_roll(author_mention, &roll);
    ctx.send(response).await?;
    Ok(())
}

fn format_simple_roll(author: String, max_roll: u32) -> String {
    let roll = if max_roll > 1 {
        random_range(1..=max_roll)
    } else {
        1
    };
    format!("{author} rolls a {roll} (1-{max_roll})")
}

fn format_dice_roll(author: String, roll: &DiceRoll) -> CreateReply {
    let &DiceRoll { count, sides } = roll;

    let mut rolls = Vec::new();
    for _ in 0..count {
        rolls.push(random_range(1..=sides));
    }
    let total: u32 = rolls.iter().sum();

    let embed = CreateEmbed::new()
        .description(format!(
            "**Result**: {count}d{sides} -> {rolls:?}\n**Total**: {total}"
        ))
        .color(EMBED_COLOR);
    CreateReply::default()
        .content(format!("{author} :game_die: **{total}**"))
        .embed(embed)
}

fn parse_dice_string(dice_string: impl Into<String>) -> Result<DiceRoll> {
    let dice = dice_string.into();
    let re = Regex::new(r"^([0-9]+)d([0-9]+)$")?;
    let (_, [count, sides]) = re
        .captures(&dice)
        .ok_or_else(|| Error::CommandArgParse)?
        .extract();

    let count: u32 = count.parse().map_err(|_| Error::CommandArgParse)?;
    let sides: u32 = sides.parse().map_err(|_| Error::CommandArgParse)?;

    if count > MAX_ROLLS {
        return Err(Error::CommandArgParse);
    }
    Ok(DiceRoll { count, sides })
}

#[cfg(test)]
mod tests {
    use crate::commands::roll::{parse_dice_string, DiceRoll};
    use crate::prelude::Error;

    #[test]
    fn parse_good_dice() {
        let v1 = String::from("1d6");
        assert!(matches!(
            parse_dice_string(v1),
            Ok(DiceRoll { count: 1, sides: 6 })
        ));

        let v2 = String::from("2d20");
        assert!(matches!(
            parse_dice_string(v2),
            Ok(DiceRoll {
                count: 2,
                sides: 20
            })
        ));

        let v3 = String::from("50d1000");
        assert!(matches!(
            parse_dice_string(v3),
            Ok(DiceRoll {
                count: 50,
                sides: 1000
            })
        ));
    }

    #[test]
    fn parse_bad_dice() {
        let v1 = String::from("16");
        assert!(matches!(parse_dice_string(v1), Err(Error::CommandArgParse)));

        let v2 = String::from("");
        assert!(matches!(parse_dice_string(v2), Err(Error::CommandArgParse)));

        let v3 = String::from("60d1000");
        assert!(matches!(parse_dice_string(v3), Err(Error::CommandArgParse)));

        let v4 = String::from("1d100000000000");
        assert!(matches!(parse_dice_string(v4), Err(Error::CommandArgParse)));

        let v5 = String::from("-10d40");
        assert!(matches!(parse_dice_string(v5), Err(Error::CommandArgParse)));
    }
}
