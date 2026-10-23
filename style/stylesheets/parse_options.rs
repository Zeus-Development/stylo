/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(stylesheet-parse-options): the options an embedder passes
//! when it parses a style sheet or a declaration block. Upstream has no
//! such carrier: what the parser accepts is fixed at build time or by
//! global preferences, so one process cannot parse two documents at two
//! CSS levels. `ParseOptions::default()` is upstream's behaviour.

use crate::derives::*;
use cssparser::{Parser, Token};

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
    /// VIPER-PATCH(value-grammar-options): spellings outside the grammars
    /// that parse as a keyword the grammar accepts, in every origin:
    /// `(property, spelling, keyword)`. Before a declaration of `property`
    /// (its declared name, shorthand or longhand, compared ASCII
    /// case-insensitively; no expansion to sub-properties) is parsed, every
    /// whole identifier token of its value, at any nesting depth, that
    /// equals `spelling` ASCII case-insensitively is replaced by `keyword`.
    /// Default: none.
    #[ignore_malloc_size_of = "static data"]
    pub value_aliases: &'static [(&'static str, &'static str, &'static str)],
    /// VIPER-PATCH(value-grammar-options): values the grammars accept that
    /// are refused, in every origin: `(property, keyword or function)`. A
    /// declaration of `property` (as for `value_aliases`) whose value holds
    /// an identifier or a function name equal to the second item, ASCII
    /// case-insensitively, at any nesting depth, is invalid. Refusals are
    /// matched before aliases are applied. Default: none.
    #[ignore_malloc_size_of = "static data"]
    pub refused_values: &'static [(&'static str, &'static str)],
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            attribute_s_flag: true,
            value_aliases: &[],
            refused_values: &[],
        }
    }
}

/// VIPER-PATCH(value-grammar-options): what the options make of one
/// declared value.
pub(crate) enum ValueRewrite {
    /// Neither list applies: the value parses as written.
    Unchanged,
    /// The value holds a refused keyword or function name.
    Refused,
    /// The value's text with the aliases replaced.
    Rewritten(String),
}

impl ParseOptions {
    /// VIPER-PATCH(value-grammar-options): reads the value of a declaration
    /// of `property` from `input`, which is left at its start unless the
    /// value is refused or rewritten.
    pub(crate) fn rewrite_value(&self, property: &str, input: &mut Parser) -> ValueRewrite {
        let applies = |name: &str| name.eq_ignore_ascii_case(property);
        if !self.value_aliases.iter().any(|a| applies(a.0))
            && !self.refused_values.iter().any(|r| applies(r.0))
        {
            return ValueRewrite::Unchanged;
        }
        let start = input.state();
        let mut edits = Vec::new();
        let refused = self.scan(property, input, &mut edits);
        if refused {
            return ValueRewrite::Refused;
        }
        if edits.is_empty() {
            input.reset(&start);
            return ValueRewrite::Unchanged;
        }
        let text = input.slice_from(start.position());
        let base = start.position().byte_index();
        let mut rewritten = String::with_capacity(text.len());
        let mut copied = 0;
        for (from, to, keyword) in edits {
            rewritten.push_str(&text[copied..from - base]);
            rewritten.push_str(keyword);
            copied = to - base;
        }
        rewritten.push_str(&text[copied..]);
        ValueRewrite::Rewritten(rewritten)
    }

    /// Walks every token of `input` and its nested blocks, recording the
    /// aliased identifiers' byte ranges; returns whether a refused name
    /// occurs.
    fn scan(
        &self,
        property: &str,
        input: &mut Parser,
        edits: &mut Vec<(usize, usize, &'static str)>,
    ) -> bool {
        let mut refused = false;
        loop {
            let before = input.position().byte_index();
            let token = match input.next() {
                Ok(token) => token.clone(),
                Err(_) => return refused,
            };
            let name = match token {
                Token::Ident(ref name) | Token::Function(ref name) => Some(name),
                _ => None,
            };
            if let Some(name) = name {
                let is = |listed: &str, value: &str| {
                    listed.eq_ignore_ascii_case(property) && value.eq_ignore_ascii_case(name)
                };
                refused |= self.refused_values.iter().any(|r| is(r.0, r.1));
                let alias = self.value_aliases.iter().find(|a| is(a.0, a.1));
                if let (Token::Ident(_), Some(alias)) = (&token, alias) {
                    edits.push((before, input.position().byte_index(), alias.2));
                }
            }
            if matches!(
                token,
                Token::Function(_)
                    | Token::ParenthesisBlock
                    | Token::SquareBracketBlock
                    | Token::CurlyBracketBlock
            ) {
                let nested = input.parse_nested_block(|input| {
                    Ok::<_, cssparser::ParseError<()>>(self.scan(property, input, edits))
                });
                refused |= nested.unwrap_or(false);
            }
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

// VIPER-PATCH(value-grammar-options): the grammars and the value options.
#[cfg(all(test, feature = "servo"))]
pub(crate) mod value_grammar_tests {
    use super::ParseOptions;
    use crate::context::QuirksMode;
    use crate::parser::ParserContext;
    use crate::properties::parse_style_attribute_with_options;
    use crate::stylesheets::supports_rule::Declaration;
    use crate::stylesheets::{CssRuleType, Origin, UrlExtraData};
    use style_traits::ParsingMode;

    fn url() -> UrlExtraData {
        UrlExtraData::from(::url::Url::parse("about:blank").unwrap())
    }

    /// `CSS.supports(property, value)` under `options`.
    fn supports(options: ParseOptions, property: &str, value: &str) -> bool {
        let url = url();
        let mut context = ParserContext::new(
            Origin::Author,
            &url,
            Some(CssRuleType::Style),
            ParsingMode::DEFAULT,
            QuirksMode::NoQuirks,
            Default::default(),
            None,
            None,
            Default::default(),
        );
        context.parse_options = options;
        Declaration(format!("{property}:{value}")).eval(&context)
    }

    /// `CSS.supports(property, value)` under the default options (also
    /// the test of `chrome-layout-properties`).
    pub(crate) fn supports_by_default(property: &str, value: &str) -> bool {
        supports(ParseOptions::default(), property, value)
    }

    /// A `style` attribute parsed under `options`, serialized.
    fn declared(options: ParseOptions, css: &str) -> String {
        let block = parse_style_attribute_with_options(
            css,
            &url(),
            None,
            QuirksMode::NoQuirks,
            CssRuleType::Style,
            options,
        );
        let mut text = String::new();
        block.to_css(&mut text).unwrap();
        text
    }

    #[test]
    fn value_grammar_options_default_parses_the_grammars() {
        let options = ParseOptions::default();
        assert!(options.value_aliases.is_empty() && options.refused_values.is_empty());
        for (property, value) in [
            ("text-align", "match-parent"),
            ("text-align", "-moz-center"),
            ("text-transform", "uppercase full-width full-size-kana"),
            ("text-decoration-line", "grammar-error"),
            ("text-indent", "1px hanging each-line"),
            ("text-wrap", "nowrap pretty"),
            ("text-wrap-style", "pretty"),
            ("counter-reset", "reversed(list-item) 3"),
            ("counter-increment", "a 2 b"),
            ("counter-set", "list-item 3"),
            ("list-style-type", "symbols(cyclic \"*\")"),
            ("cursor", "-moz-grab"),
        ] {
            assert!(supports(options, property, value), "{property}: {value}");
        }
        for (property, value) in [
            ("text-align", "-webkit-auto"),
            ("text-align", "-webkit-match-parent"),
            ("cursor", "-webkit-grab"),
            ("unicode-bidi", "-webkit-isolate"),
            ("font-size", "-webkit-xxx-large"),
            ("text-wrap-style", "avoid-orphans"),
        ] {
            assert!(!supports(options, property, value), "{property}: {value}");
        }
    }

    #[test]
    fn value_grammar_options_alias_whole_identifiers() {
        let mut options = ParseOptions::default();
        options.value_aliases = &[
            ("text-align", "-webkit-auto", "start"),
            ("cursor", "-webkit-grab", "grab"),
            ("Unicode-Bidi", "-webkit-isolate", "isolate"),
        ];
        assert!(supports(options, "text-align", "-WEBKIT-AUTO"));
        assert!(supports(options, "cursor", "url(x.png) 1 2, -webkit-grab"));
        assert!(supports(options, "unicode-bidi", "-webkit-isolate"));
        // Only whole identifiers of the listed property.
        assert!(!supports(options, "text-align-last", "-webkit-auto"));
        assert!(!supports(options, "text-align", "\"-webkit-auto\""));
        assert!(!supports(options, "text-align", "-webkit-autox"));
        assert_eq!(
            declared(options, "text-align: -webkit-auto; cursor: -webkit-grab"),
            "text-align: start; cursor: grab;"
        );
    }

    #[test]
    fn value_grammar_options_refuse_names_at_any_depth() {
        let mut options = ParseOptions::default();
        options.refused_values = &[
            ("text-transform", "full-width"),
            ("counter-reset", "reversed"),
            ("list-style", "symbols"),
            ("content", "SYMBOLS"),
            ("text-align", "-moz-center"),
        ];
        for (property, value) in [
            ("text-transform", "full-width"),
            ("text-transform", "uppercase Full-Width"),
            ("counter-reset", "a 1 reversed(b)"),
            ("list-style", "inside symbols(cyclic \"*\")"),
            ("content", "counter(a, symbols(cyclic \"*\"))"),
            ("text-align", "-moz-center"),
        ] {
            assert!(!supports(options, property, value), "{property}: {value}");
        }
        for (property, value) in [
            ("text-transform", "uppercase full-size-kana"),
            ("list-style-type", "symbols(cyclic \"*\")"),
            ("text-align", "-moz-center-or-inherit"),
            ("text-align", "-webkit-center"),
        ] {
            let expected = property != "text-align" || value == "-webkit-center";
            assert_eq!(
                supports(options, property, value),
                expected,
                "{property}: {value}"
            );
        }
        assert_eq!(
            declared(options, "text-transform: full-width; text-align: center"),
            "text-align: center;"
        );
    }
}
