//! "Everything this node reports": the site node's own `status --json`, as `network.js`
//! lays it out, written into the page.
//!
//! It is read again from the observation's bytes, keeping the order the node wrote its
//! keys in; a `serde_json::Value` sorts them, and the page would then rearrange itself
//! the moment its script ran. The labels are the keys themselves with `_` as spaces:
//! they are the names of fields a program reads, and the same in every language.

use std::fmt::{self, Write as _};

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

use crate::serve::template::html_escaped;
use crate::words::Speaker;

/// A JSON value with its keys in the order they were written.
enum Ordered {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A number, as it was written.
    Number(String),
    /// A string.
    Text(String),
    /// An array.
    List(Vec<Ordered>),
    /// An object, in order.
    Object(Vec<(String, Ordered)>),
}

impl<'de> Deserialize<'de> for Ordered {
    fn deserialize<D: Deserializer<'de>>(from: D) -> Result<Self, D::Error> {
        from.deserialize_any(OrderedVisitor)
    }
}

/// Reads any JSON value into an [`Ordered`].
struct OrderedVisitor;

impl<'de> Visitor<'de> for OrderedVisitor {
    type Value = Ordered;

    fn expecting(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("any JSON value")
    }
    fn visit_unit<E>(self) -> Result<Ordered, E> {
        Ok(Ordered::Null)
    }
    fn visit_none<E>(self) -> Result<Ordered, E> {
        Ok(Ordered::Null)
    }
    fn visit_bool<E>(self, value: bool) -> Result<Ordered, E> {
        Ok(Ordered::Bool(value))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Ordered, E> {
        Ok(Ordered::Number(value.to_string()))
    }
    fn visit_i64<E>(self, value: i64) -> Result<Ordered, E> {
        Ok(Ordered::Number(value.to_string()))
    }
    fn visit_f64<E>(self, value: f64) -> Result<Ordered, E> {
        Ok(Ordered::Number(value.to_string()))
    }
    fn visit_str<E>(self, value: &str) -> Result<Ordered, E> {
        Ok(Ordered::Text(value.to_owned()))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut items: A) -> Result<Ordered, A::Error> {
        let mut list = Vec::new();
        while let Some(item) = items.next_element()? {
            list.push(item);
        }
        Ok(Ordered::List(list))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut entries: A) -> Result<Ordered, A::Error> {
        let mut object = Vec::new();
        while let Some(entry) = entries.next_entry()? {
            object.push(entry);
        }
        Ok(Ordered::Object(object))
    }
}

/// The report's sections, from the observation as written; nothing if it cannot be read.
pub(super) fn html(written: &[u8], words: &Speaker<'_>) -> String {
    let Ok(Ordered::Object(top)) = serde_json::from_slice::<Ordered>(written) else {
        return String::new();
    };
    let Some((_, Ordered::Object(status))) = top.into_iter().find(|(key, _)| key == "status")
    else {
        return String::new();
    };
    let mut plain = Vec::new();
    let mut groups = Vec::new();
    for (key, value) in status {
        match value {
            Ordered::List(_) | Ordered::Object(_) => groups.push((key, value)),
            other => plain.push((key, other)),
        }
    }
    let mut out = format!(
        "<section><h3>{}</h3><dl>{}</dl></section>",
        html_escaped(&words.word("js-network-this-node")),
        rows(&plain, words)
    );
    for (key, value) in groups {
        let inner = match value {
            Ordered::Object(entries) => rows(&entries, words),
            list => rows(&[("entries".to_owned(), list)], words),
        };
        let _ = write!(
            out,
            "<section><h3>{}</h3><dl>{inner}</dl></section>",
            html_escaped(&label(&key))
        );
    }
    out
}

/// A key as a label: `_` as spaces, the first letter a capital.
fn label(key: &str) -> String {
    let spaced = key.replace('_', " ");
    let mut letters = spaced.chars();
    letters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(letters).collect()
    })
}

/// A value as the report shows it.
fn value_of(value: &Ordered, words: &Speaker<'_>) -> String {
    match value {
        Ordered::Null => "\u{2014}".to_owned(),
        Ordered::Bool(true) => html_escaped(&words.word("js-network-yes")),
        Ordered::Bool(false) => html_escaped(&words.word("js-network-no")),
        Ordered::Number(number) => html_escaped(number),
        Ordered::Text(text) => html_escaped(text),
        Ordered::List(_) | Ordered::Object(_) => String::new(),
    }
}

/// The rows of one object, nested objects and lists inside.
fn rows(entries: &[(String, Ordered)], words: &Speaker<'_>) -> String {
    let mut out = String::new();
    for (key, value) in entries {
        let key = html_escaped(&label(key));
        match value {
            Ordered::Object(inner) => {
                let _ = write!(
                    out,
                    r#"<dt>{key}</dt><dd></dd><div class="nested"><dl>{}</dl></div>"#,
                    rows(inner, words)
                );
            }
            Ordered::List(items) if items.is_empty() => {
                let none = html_escaped(&words.word("js-network-none"));
                let _ = write!(out, "<dt>{key}</dt><dd>{none}</dd>");
            }
            Ordered::List(items) => {
                let _ = write!(
                    out,
                    r#"<dt>{key}</dt><dd>{}</dd><div class="nested">"#,
                    items.len()
                );
                for item in items {
                    match item {
                        Ordered::Object(inner) => {
                            let _ = write!(out, "<dl>{}</dl>", rows(inner, words));
                        }
                        other => {
                            let _ = write!(
                                out,
                                "<dl><dt></dt><dd>{}</dd></dl>",
                                value_of(other, words)
                            );
                        }
                    }
                }
                out.push_str("</div>");
            }
            other => {
                let _ = write!(out, "<dt>{key}</dt><dd>{}</dd>", value_of(other, words));
            }
        }
    }
    out
}
