//! Putting live values and the page's words into the HTML pages.
//!
//! Few forms, so a page author can see at a glance what is escaped how:
//!
//! - `{{name}}` — the value as HTML text, with `& < > " '` escaped, safe in text and in
//!   a quoted attribute.
//! - `{{json:name}}` — the value as JSON, safe inside `<script type="application/json">`:
//!   `<`, `>`, `&`, U+2028 and U+2029 are written as `\uXXXX`, which every JSON reader
//!   reads back as the same character, so nothing in it can end the script element.
//! - `{{html:name}}` — HTML the server made itself (a table's rows, the head's links),
//!   put in as it is.
//! - `{{t:key}}` — the message `key` in the page's language, as HTML, put in as it is:
//!   the catalogs are ours, and their markup (`<a>`, `<code>`, `<b>`) is meant.
//! - `{{a:key}}` — the same message as plain text, escaped, for an attribute or
//!   `<title>`.
//!
//! A message can be handed values: `{{t:key|answering|epoch}}` gives it `$answering`
//! and `$epoch`, each the value of that name, escaped for HTML first unless it is an
//! `html:` value. Every message is also handed `$base`, the language's path prefix, so
//! a link in a catalog stays in the reader's language.
//!
//! A token this server does not know, or a key no catalog has, is left as it is and
//! named in the log once, so a typo shows on the page instead of disappearing. Only
//! `.html` files are templated.

use std::collections::{BTreeMap, HashSet};
use std::sync::{Mutex, OnceLock, PoisonError};

use crate::words::{Arg, Speaker};

/// The values one page is rendered with.
#[derive(Debug, Default)]
pub(crate) struct Values {
    /// Text values by name.
    text: BTreeMap<&'static str, String>,
    /// Values already written as JSON, by name.
    json: BTreeMap<&'static str, String>,
    /// HTML the server made, by name.
    html: BTreeMap<&'static str, String>,
}

impl Values {
    /// Offer `value` as `{{name}}`.
    pub(crate) fn text(&mut self, name: &'static str, value: impl Into<String>) {
        self.text.insert(name, value.into());
    }

    /// Offer `json`, which must already be JSON, as `{{json:name}}`.
    pub(crate) fn json(&mut self, name: &'static str, json: String) {
        self.json.insert(name, json);
    }

    /// Offer `html`, which this server made and is trusted, as `{{html:name}}`.
    pub(crate) fn html(&mut self, name: &'static str, html: String) {
        self.html.insert(name, html);
    }

    /// A value as a message is handed it: escaped text, or HTML as it is. For an
    /// attribute (`plain`), text as it is; the whole message is escaped afterwards.
    fn arg(&self, name: &str, plain: bool) -> Option<String> {
        if let Some(text) = self.text.get(name) {
            return Some(if plain {
                text.clone()
            } else {
                html_escaped(text)
            });
        }
        self.html.get(name).cloned()
    }
}

/// The page with every known token replaced.
pub(crate) fn render(page: &str, values: &Values, words: &Speaker<'_>) -> String {
    let mut out = String::with_capacity(page.len());
    let mut rest = page;
    while let Some(start) = rest.find("{{") {
        let (before, from) = rest.split_at(start);
        out.push_str(before);
        let inner = from.get(2..).unwrap_or_default();
        let Some(end) = inner.find("}}") else {
            out.push_str(from);
            return out;
        };
        let token = inner.get(..end).unwrap_or_default();
        match replacement(token, values, words) {
            Some(value) => {
                out.push_str(&value);
                rest = inner.get(end + 2..).unwrap_or_default();
            }
            None => {
                // Not ours: keep the braces and look again just after them, so a token
                // that starts inside this one is still found.
                if is_name(token) {
                    unknown(token);
                }
                out.push_str("{{");
                rest = inner;
            }
        }
    }
    out.push_str(rest);
    out
}

/// What a token becomes, if it names a value or a message.
fn replacement(token: &str, values: &Values, words: &Speaker<'_>) -> Option<String> {
    if let Some(name) = token.strip_prefix("json:") {
        return values.json.get(name).map(|json| script_safe(json));
    }
    if let Some(name) = token.strip_prefix("html:") {
        return values.html.get(name).cloned();
    }
    if let Some(message) = token.strip_prefix("t:") {
        return said(message, values, words, false);
    }
    if let Some(message) = token.strip_prefix("a:") {
        return said(message, values, words, true).map(|text| html_escaped(&text));
    }
    values.text.get(token).map(|text| html_escaped(text))
}

/// A message token, `key|name|name`, said with the values it names.
fn said(message: &str, values: &Values, words: &Speaker<'_>, plain: bool) -> Option<String> {
    let mut parts = message.split('|');
    let key = parts.next()?;
    let mut args = Vec::new();
    for name in parts {
        args.push((name, Arg::Text(values.arg(name, plain)?)));
    }
    words.say(key, &args)
}

/// Could this be a token somebody meant? A value's name, maybe after `json:` or `html:`,
/// or a message's key after `t:` or `a:`.
fn is_name(token: &str) -> bool {
    let (name, extra) = match token.split_once(':') {
        Some(("json" | "html", name)) => (name, ""),
        Some(("t" | "a", key)) => (key, "-|"),
        Some(_) => return false,
        None => (token, ""),
    };
    !name.is_empty()
        && name.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || extra.as_bytes().contains(&byte)
        })
}

/// Say once that a page asks for a value nobody gives.
fn unknown(token: &str) {
    static SEEN: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    let mut seen = SEEN
        .get_or_init(Mutex::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    if seen.insert(token.to_owned()) {
        tracing::warn!("a page asks for {{{{{token}}}}}, which nothing fills; left as it is");
    }
}

/// Text made safe to stand in HTML text or a quoted attribute.
pub(crate) fn html_escaped(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

/// JSON made safe to stand inside a script element.
pub(crate) fn script_safe(json: &str) -> String {
    let mut out = String::with_capacity(json.len());
    for character in json.chars() {
        match character {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site::ENGLISH;
    use crate::words::Words;

    #[test]
    fn text_is_html_escaped_and_json_cannot_end_its_script() {
        let mut values = Values::default();
        values.text("site_node", "<b>\"333\" & 'x'</b>");
        values.json(
            "board",
            serde_json::json!(["</script>&\u{2028}"]).to_string(),
        );
        let page = "<p title=\"{{site_node}}\">{{site_node}}</p>\
                    <script type=\"application/json\">{{json:board}}</script>{{nobody}}";

        let words = Words::repository();
        let out = render(page, &values, &words.speaker(&ENGLISH));
        assert_eq!(
            out,
            "<p title=\"&lt;b&gt;&quot;333&quot; &amp; &#39;x&#39;&lt;/b&gt;\">\
             &lt;b&gt;&quot;333&quot; &amp; &#39;x&#39;&lt;/b&gt;</p>\
             <script type=\"application/json\">[\"\\u003c/script\\u003e\\u0026\\u2028\"]</script>{{nobody}}"
        );
        let inside = out
            .split("json\">")
            .nth(1)
            .unwrap()
            .split("</script>")
            .next()
            .unwrap();
        let back: Vec<String> = serde_json::from_str(inside).unwrap();
        assert_eq!(back, vec!["</script>&\u{2028}".to_owned()]);
    }

    #[test]
    fn a_message_is_html_in_text_and_escaped_in_an_attribute() {
        let words = Words::repository();
        let english = words.speaker(&ENGLISH);
        let mut values = Values::default();
        values.text("answering", "<3>");
        values.text("epoch", "9");
        let page = "<b title=\"{{a:js-state-awake}}\">{{t:home-join-others}}</b>\
                    {{t:home-hero-foot|answering|epoch}}{{t:no-such-key}}";
        assert_eq!(
            render(page, &values, &english),
            "<b title=\"This site&#39;s node is awake\">Windows, Light, and installing by \
             hand are on <a href=\"/start\">Take the program</a>.</b>\
             <span data-figure=\"answering\">&lt;3&gt;</span> answering in epoch \
             <span data-figure=\"epoch\">9</span>{{t:no-such-key}}"
        );
    }
}
