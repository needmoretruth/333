//! Putting live values into the HTML pages.
//!
//! Two forms and nothing else, so a page author can see at a glance what is escaped how:
//!
//! - `{{name}}` — the value as HTML text, with `& < > " '` escaped, safe in text and in
//!   a quoted attribute.
//! - `{{json:name}}` — the value as JSON, safe inside `<script type="application/json">`:
//!   `<`, `>`, `&`, U+2028 and U+2029 are written as `\uXXXX`, which every JSON reader
//!   reads back as the same character, so nothing in it can end the script element.
//!
//! A token this server does not know is left as it is, and named in the log once, so a
//! typo shows on the page instead of disappearing. Only `.html` files are templated.

use std::collections::{BTreeMap, HashSet};
use std::sync::{Mutex, OnceLock, PoisonError};

/// The values one page is rendered with.
#[derive(Debug, Default)]
pub(crate) struct Values {
    /// Text values by name.
    text: BTreeMap<&'static str, String>,
    /// Values already written as JSON, by name.
    json: BTreeMap<&'static str, String>,
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
}

/// The page with every known token replaced.
pub(crate) fn render(page: &str, values: &Values) -> String {
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
        match replacement(token, values) {
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

/// What a token becomes, if it names a value.
fn replacement(token: &str, values: &Values) -> Option<String> {
    if let Some(name) = token.strip_prefix("json:") {
        return values.json.get(name).map(|json| script_safe(json));
    }
    values.text.get(token).map(|text| html_escaped(text))
}

/// Could this be a token somebody meant? Letters, digits, `_` and one `json:` prefix.
fn is_name(token: &str) -> bool {
    let name = token.strip_prefix("json:").unwrap_or(token);
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
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

        let out = render(page, &values);
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
}
