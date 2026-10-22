/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(stylesheet-parse-options): the options an embedder passes
//! when it parses a style sheet or a declaration block. Upstream has no
//! such carrier: what the parser accepts is fixed at build time or by
//! global preferences, so one process cannot parse two documents at two
//! CSS levels. `ParseOptions::default()` is upstream's behaviour.

use crate::derives::*;

/// Options for parsing a style sheet or a declaration block, carried by
/// [`ParserContext`](crate::parser::ParserContext) and read by the
/// parsers that need them (the selector parser for selectors).
///
/// The default is upstream's behaviour. New fields may be added, each
/// defaulting to upstream's behaviour, so construct a value with
/// `ParseOptions::default()` and set the fields to change.
#[derive(Clone, Copy, Debug, Eq, MallocSizeOf, PartialEq)]
#[non_exhaustive]
pub struct ParseOptions {
    /// Whether an attribute selector accepts the `s` flag, which compares
    /// the attribute's value case-sensitively ([Selectors 4 section 6.3]).
    /// Default `true`, as the specification defines it. When `false`, a
    /// selector using the flag is invalid: a style rule whose selector
    /// list holds it is dropped, and a forgiving list (`:is()`, `:where()`)
    /// drops just that selector.
    ///
    /// [Selectors 4 section 6.3]: https://drafts.csswg.org/selectors-4/#attribute-case
    pub attribute_s_flag: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            attribute_s_flag: true,
        }
    }
}

// VIPER-PATCH(stylesheet-parse-options): the options reach the parsers.
#[cfg(test)]
mod tests {
    use super::ParseOptions;
    use crate::context::QuirksMode;
    use crate::properties::parse_style_attribute_with_options;
    use crate::selector_parser::SelectorParser;
    use crate::shared_lock::SharedRwLock;
    use crate::stylesheets::{AllowImportRules, CssRule, CssRuleType, Origin};
    use crate::stylesheets::{StylesheetContents, UrlExtraData};

    fn url() -> UrlExtraData {
        UrlExtraData::from(::url::Url::parse("about:blank").unwrap())
    }

    /// The top-level rules of `css` parsed with `options`, and whether
    /// each `@supports` rule among them is enabled.
    fn parsed(css: &str, options: ParseOptions) -> (usize, Vec<bool>) {
        let lock = SharedRwLock::new();
        let contents = StylesheetContents::from_str_with_options(
            css,
            url(),
            Origin::Author,
            &lock,
            None,
            None,
            QuirksMode::NoQuirks,
            AllowImportRules::Yes,
            None,
            options,
        );
        let guard = lock.read();
        let rules = &contents.rules.read_with(&guard).0;
        let supports = rules.iter().filter_map(|rule| match rule {
            CssRule::Supports(rule) => Some(rule.enabled),
            _ => None,
        });
        (rules.len(), supports.collect())
    }

    fn rules(css: &str, options: ParseOptions) -> usize {
        parsed(css, options).0
    }

    fn without_s_flag() -> ParseOptions {
        let mut options = ParseOptions::default();
        options.attribute_s_flag = false;
        options
    }

    #[test]
    fn stylesheet_parse_options_default_accepts_the_s_flag() {
        assert!(ParseOptions::default().attribute_s_flag);
        assert_eq!(
            rules("[a=b s] {} [a=b S] {} [a=b i] {}", ParseOptions::default()),
            3
        );
        assert!(SelectorParser::parse_author_origin_no_namespace("[a=b s]", &url()).is_ok());
    }

    #[test]
    fn stylesheet_parse_options_refuse_the_s_flag() {
        let options = without_s_flag();
        assert_eq!(rules("[a=b s] {} [a=b S] {}", options), 0);
        assert_eq!(rules("[a=b i] {} [a=b] {}", options), 2);
        // A forgiving list drops just the refused selector; a plain list
        // drops the whole rule.
        assert_eq!(rules(":is([a=b s], p) {}", options), 1);
        assert_eq!(rules(":where([a=b s]) {}", options), 1);
        assert_eq!(rules("[a=b s], p {}", options), 0);
        // `@supports selector()` reads the same options.
        let supports = "@supports selector([a=b s]) {} @supports selector([a=b i]) {}";
        assert_eq!(parsed(supports, options).1, [false, true]);
        assert_eq!(parsed(supports, ParseOptions::default()).1, [true, true]);
    }

    #[test]
    fn stylesheet_parse_options_style_attribute() {
        let url = url();
        let parse = |options| {
            parse_style_attribute_with_options(
                "color: red; margin: 0",
                &url,
                None,
                QuirksMode::NoQuirks,
                CssRuleType::Style,
                options,
            )
            .len()
        };
        assert_eq!(parse(ParseOptions::default()), 5);
        assert_eq!(parse(without_s_flag()), 5);
    }
}
