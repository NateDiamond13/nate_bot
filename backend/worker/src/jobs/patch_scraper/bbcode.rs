use std::sync::LazyLock;

use regex::{Captures, Regex};

static RE_CODE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)\[(code|preformatted)\](.*?)\[/(code|preformatted)\]").unwrap()
});

static RE_PATTERNS: LazyLock<[(Regex, &'static str); 28]> = LazyLock::new(|| {
    [
        (Regex::new(r"(?s)\[p\](.*?)\[/p\]").unwrap(), "<p>$1</p>"),
        // Font changes
        (
            Regex::new(r"(?s)\[b\](.*?)\[/b\]").unwrap(),
            "<strong>$1</strong>",
        ),
        (Regex::new(r"(?s)\[i\](.*?)\[/i\]").unwrap(), "<em>$1</em>"),
        (Regex::new(r"(?s)\[u\](.*?)\[/u\]").unwrap(), "<u>$1</u>"),
        (
            Regex::new(r"(?s)\[s\](.*?)\[/s\]").unwrap(),
            "<strike>$1</strike>",
        ),
        (
            Regex::new(r"(?s)\[size=(\d+)](.*?)\[/size\]").unwrap(),
            "<span style=\"font-size: ${1}px;\">$2</span>",
        ),
        (
            Regex::new(r"(?s)\[color=(.+)](.*?)\[/color\]").unwrap(),
            "<span style=\"color: $1;\">$2</span>",
        ),
        // Alignment
        (
            Regex::new(r"(?s)\[center\](.*?)\[/center\]").unwrap(),
            "<div style=\"text-align: center;\">$1</div>",
        ),
        (
            Regex::new(r"(?s)\[left\](.*?)\[/left\]").unwrap(),
            "<div style=\"text-align: left;\">$1</div>",
        ),
        (
            Regex::new(r"(?s)\[right\](.*?)\[/right\]").unwrap(),
            "<div style=\"text-align: right;\">$1</div>",
        ),
        // Tables
        (
            Regex::new(r"(?s)\[table\](.*?)\[/table\]").unwrap(),
            "<table>$1</table>",
        ),
        (
            Regex::new(r"(?s)\[td\](.*?)\[/td\]").unwrap(),
            "<td>$1</td>",
        ),
        (
            Regex::new(r"(?s)\[tr\](.*?)\[/tr\]").unwrap(),
            "<tr>$1</tr>",
        ),
        (
            Regex::new(r"(?s)\[th\](.*?)\[/th\]").unwrap(),
            "<th>$1</th>",
        ),
        // Links
        (
            Regex::new(r"(?s)\[url\](.*?)\[/url\]").unwrap(),
            "<a href=\"$1\" rel=\"nofollow\" target=\"_new\">$1</a>",
        ),
        (
            Regex::new(r"(?s)\[url=(.+)\](.*?)\[/url\]").unwrap(),
            "<a href=\"$2\" rel=\"nofollow\" target=\"_new\">$1</a>",
        ),
        // Quotes
        (
            Regex::new(r"(?s)\[quote\](.*?)\[/quote\]").unwrap(),
            "<div class=\"quote\">$1</div>",
        ),
        (
            Regex::new(r"(?s)\[quote=(.+)\](.*?)\[/quote\]").unwrap(),
            "<div class=\"quote\"><strong>$1 wrote:</strong>\n$2</div>",
        ),
        // Images
        (
            Regex::new(r"(?s)\[img=(\d+)x(\d+)(\b.*)?\](.*?)\[/img\]").unwrap(),
            "<img src=\"$4\" width=\"$1\" height=\"$2\"$3 />",
        ),
        (
            Regex::new(r"(?s)\[img=(.+)(\b.*)?\](.*?)\[/img\]").unwrap(),
            "<img src=\"$3\" alt=\"$1\"$2 />",
        ),
        (
            Regex::new(r"(?s)\[img(\b.*)?\](.*?)\[/img\]").unwrap(),
            "<img src=\"$2\"$1 />",
        ),
        // Lists
        (
            Regex::new(r"(?s)\[ol\](.*?)\[/ol\]").unwrap(),
            "<ol>$1</ol>",
        ),
        (
            Regex::new(r"(?s)\[ul\](.*?)\[/ul\]").unwrap(),
            "<ul>$1</ul>",
        ),
        (
            Regex::new(r"(?s)\[list\](.*?)\[/list\]").unwrap(),
            "<ul>$1</ul>",
        ),
        // Youtube
        (
            Regex::new(r"(?s)\[youtube\](.*?)\[/youtube\]").unwrap(),
            "<object data=\"http://www.youtube.com/embed/$1\"></object>",
        ),
        (
            Regex::new(r"(?s)\[youtube=(\d+)x(\d+)\](.*?)\[/youtube\]").unwrap(),
            "<object width=\"$1\" height=\"$2\" data=\"http://www.youtube.com/embed/$3\"></object>",
        ),
        // List Items
        (
            Regex::new(r"(?s)\[li\](.*?)\[/li\]").unwrap(),
            "<li>$1</li>",
        ),
        (
            Regex::new(r"(?s)\[\*\](.*?)\[/\*\]").unwrap(),
            "<li>$1</li>",
        ),
    ]
});

fn code_replacer(captures: &Captures) -> String {
    let mut replaced = captures.get(2).map_or("", |m| m.as_str()).to_string();
    for &(input, output) in [("[", "&#91;"), ("]", "&#93;"), ("<br />", "\n")].iter() {
        replaced = replaced.replace(input, output);
    }

    format!("<pre><code>{}</code></pre>", replaced)
}

pub fn convert_to_html(bbcode_input: impl Into<String>) -> String {
    let mut output = bbcode_input.into();
    output = RE_CODE.replace_all(&output, code_replacer).to_string();

    for &(ref pattern, replace) in RE_PATTERNS.iter() {
        output = pattern.replace_all(&output, replace).into_owned();
    }
    output
}
