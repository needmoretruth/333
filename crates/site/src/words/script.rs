//! The script messages, handed to the pages' scripts as JSON they can fill in themselves.
//!
//! A script cannot run Fluent, so each `js-` message is written out in a shape a few
//! lines of JavaScript can say (`say` in `site/assets/live.js`):
//!
//! - a message of text and `{ $name }` placeables is a string, with each placeable
//!   written `{$name}`;
//! - a message with one selector on a variable is an object: `"$"` names the variable,
//!   `"type"` is `"cardinal"` or `"ordinal"`, `"*"` names the default variant, and each
//!   variant is the whole message as a string, `"one"` or `"=0"` by its key.
//!
//! Anything else (a second selector, a message reference, a function other than
//! `NUMBER`) cannot be written this way; such a message is left out, and the tests
//! refuse it in a catalog.

use std::collections::BTreeSet;

use fluent_syntax::ast::{Expression, InlineExpression, Pattern, PatternElement, VariantKey};
use serde_json::{Map, Value};

use super::{Catalog, SCRIPT_PREFIX};

/// Every script message `spoken` has, English's where it has none, as one JSON object.
pub(super) fn json(spoken: &Catalog, english: Option<&Catalog>) -> String {
    let mut out = Map::new();
    let keys: BTreeSet<&String> = spoken
        .keys
        .iter()
        .chain(english.map(|catalog| &catalog.keys).into_iter().flatten())
        .filter(|key| key.starts_with(SCRIPT_PREFIX))
        .collect();
    for key in keys {
        let shaped = [Some(spoken), english]
            .into_iter()
            .flatten()
            .find_map(|catalog| catalog.bundle.get_message(key)?.value().and_then(shape));
        if let Some(shaped) = shaped {
            out.insert(key.clone(), shaped);
        }
    }
    Value::Object(out).to_string()
}

/// One message in the shape a script reads, if it has one.
pub(super) fn shape(pattern: &Pattern<&str>) -> Option<Value> {
    let mut before = String::new();
    let mut select = None;
    let mut after = String::new();
    for element in &pattern.elements {
        let into = if select.is_some() {
            &mut after
        } else {
            &mut before
        };
        match element {
            PatternElement::TextElement { value } => into.push_str(value),
            PatternElement::Placeable {
                expression: Expression::Inline(inline),
            } => into.push_str(&flat(inline)?),
            PatternElement::Placeable {
                expression: Expression::Select { selector, variants },
            } => {
                if select.is_some() {
                    return None;
                }
                select = Some((selector, variants));
            }
        }
    }
    let Some((selector, variants)) = select else {
        return Some(Value::String(before));
    };
    let (variable, kind) = selected(selector)?;
    let mut object = Map::new();
    object.insert("$".to_owned(), Value::String(variable.to_owned()));
    object.insert("type".to_owned(), Value::String(kind.to_owned()));
    for variant in variants {
        let name = match &variant.key {
            VariantKey::Identifier { name } => (*name).to_owned(),
            VariantKey::NumberLiteral { value } => format!("={value}"),
        };
        let mut text = before.clone();
        text.push_str(&flat_pattern(&variant.value)?);
        text.push_str(&after);
        if variant.default {
            object.insert("*".to_owned(), Value::String(name.clone()));
        }
        object.insert(name, Value::String(text));
    }
    Some(Value::Object(object))
}

/// A pattern with no selector in it, as a script fills it.
fn flat_pattern(pattern: &Pattern<&str>) -> Option<String> {
    let mut text = String::new();
    for element in &pattern.elements {
        match element {
            PatternElement::TextElement { value } => text.push_str(value),
            PatternElement::Placeable {
                expression: Expression::Inline(inline),
            } => text.push_str(&flat(inline)?),
            PatternElement::Placeable { .. } => return None,
        }
    }
    Some(text)
}

/// A placeable a script can fill: a variable, or a literal.
fn flat(inline: &InlineExpression<&str>) -> Option<String> {
    match inline {
        InlineExpression::VariableReference { id } => Some(format!("{{${}}}", id.name)),
        InlineExpression::StringLiteral { value } => Some((*value).to_owned()),
        InlineExpression::NumberLiteral { value } => Some((*value).to_owned()),
        _ => None,
    }
}

/// What a selector chooses by: a variable, as a cardinal or (through `NUMBER`) an ordinal.
fn selected<'a>(selector: &'a InlineExpression<&'a str>) -> Option<(&'a str, &'static str)> {
    match selector {
        InlineExpression::VariableReference { id } => Some((id.name, "cardinal")),
        InlineExpression::FunctionReference { id, arguments } if id.name == "NUMBER" => {
            let Some(InlineExpression::VariableReference { id: variable }) =
                arguments.positional.first()
            else {
                return None;
            };
            let ordinal = arguments.named.iter().any(|named| {
                named.name.name == "type"
                    && matches!(named.value, InlineExpression::StringLiteral { value } if value == "ordinal")
            });
            Some((variable.name, if ordinal { "ordinal" } else { "cardinal" }))
        }
        _ => None,
    }
}

/// Every variable a message names, wherever in it.
#[cfg(test)]
pub(super) fn variables(pattern: &Pattern<&str>) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    walk(pattern, &mut found);
    found
}

/// Collect the variables in a pattern.
#[cfg(test)]
fn walk(pattern: &Pattern<&str>, found: &mut BTreeSet<String>) {
    for element in &pattern.elements {
        if let PatternElement::Placeable { expression } = element {
            match expression {
                Expression::Inline(inline) => inline_variables(inline, found),
                Expression::Select { selector, variants } => {
                    inline_variables(selector, found);
                    for variant in variants {
                        walk(&variant.value, found);
                    }
                }
            }
        }
    }
}

/// Collect the variables in one inline expression.
#[cfg(test)]
fn inline_variables(inline: &InlineExpression<&str>, found: &mut BTreeSet<String>) {
    match inline {
        InlineExpression::VariableReference { id } => {
            found.insert(id.name.to_owned());
        }
        InlineExpression::FunctionReference { arguments, .. } => {
            for argument in &arguments.positional {
                inline_variables(argument, found);
            }
        }
        InlineExpression::Placeable { expression } => {
            let pattern = Pattern {
                elements: vec![PatternElement::Placeable {
                    expression: (**expression).clone(),
                }],
            };
            walk(&pattern, found);
        }
        _ => {}
    }
}
