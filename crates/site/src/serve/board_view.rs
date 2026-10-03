//! The board as a list for people, written into the page the way `board.js` draws it,
//! so the page says everything before any script runs.

use std::fmt::Write as _;

use serde_json::Value;

use super::template::{Values, html_escaped};
use crate::board::{Board, Said};
use crate::place::Place;
use crate::words::{Arg, Speaker};

/// How many digits of a name the list shows.
const SHORT: usize = 12;

/// `{{html:board_list}}` and `{{html:board_empty_hidden}}`: the live statements that
/// verify, newest epoch first.
pub(crate) fn fill(
    values: &mut Values,
    board: &Board,
    observed: Option<&Value>,
    now: u64,
    words: &Speaker<'_>,
) {
    let site = observed
        .and_then(|observed| observed.get("site_node"))
        .and_then(Value::as_str);
    let mut lines: Vec<_> = board
        .alive(now)
        .filter_map(|line| Some((line.said()?, line.place())))
        .collect();
    lines.sort_by_key(|line| std::cmp::Reverse(line.0.epoch));
    let list: String = lines
        .iter()
        .map(|(said, place)| item(said, *place, site, words))
        .collect();
    let hidden = if lines.is_empty() { "" } else { " hidden" };
    values.html("board_list", list);
    values.html("board_empty_hidden", hidden.to_owned());
}

/// One statement as an invitation, and who said it when.
fn item(said: &Said, place: Option<&Place>, site: Option<&str>, words: &Speaker<'_>) -> String {
    let short = said.node.get(..SHORT).unwrap_or(&said.node);
    let link = format!(
        r#"<a href="{}/network?node={short}">{short}</a>"#,
        words.language.base()
    );
    let epoch = Arg::Text(said.epoch.to_string());
    let mut line = words
        .say(
            "js-board-said",
            &[("epoch", epoch), ("node", Arg::Text(link))],
        )
        .unwrap_or_default();
    if site == Some(said.node.as_str()) {
        let _ = write!(line, " · {}", html_escaped(&words.word("js-board-site")));
    }
    match place {
        Some(Place::Tor) => {
            let _ = write!(line, " · {}", html_escaped(&words.word("js-board-tor")));
        }
        Some(Place::At { country, .. }) => {
            let _ = write!(line, " · {}", html_escaped(country));
        }
        None => {}
    }
    format!(
        "\n    <li>\n      <div class=\"command\"><code>333 join 333:{}</code>\
         <button type=\"button\" data-copy>{}</button></div>\n      \
         <p class=\"dim\">{line}</p>\n    </li>",
        html_escaped(&said.address),
        html_escaped(&words.word("js-copy")),
    )
}
