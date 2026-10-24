/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(stylesheet-parse-options): the options an embedder passes
//! when it parses a style sheet or a declaration block. Upstream has no
//! such carrier: what the parser accepts is fixed at build time or by
//! global preferences, so one process cannot parse two documents at two
//! CSS levels. `ParseOptions::default()` is upstream's behaviour.

use crate::derives::*;
// VIPER-PATCH(selector-names-hook): the types the hook answers with.
use crate::selector_parser::{NonTSPseudoClass, PseudoElement, SelectorImpl};
use cssparser::{Parser, Token};
use selectors::parser::Component;
use style_traits::ParseError;

/// Options for parsing a style sheet or a declaration block, carried by
/// [`ParserContext`](crate::parser::ParserContext) and read by the
/// parsers that need them (the selector parser for selectors).
///
/// The default is upstream's behaviour. New fields may be added, each
/// defaulting to upstream's behaviour, so construct a value with
/// `ParseOptions::default()` and set the fields to change.
// VIPER-PATCH(selector-names-hook): `Debug` and `PartialEq` by hand, as
// the hook is a trait object.
#[derive(Clone, Copy, MallocSizeOf)]
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
    /// VIPER-PATCH(value-grammar-options): values refused only where a
    /// supports condition is evaluated (`@supports` and `CSS.supports`, CSS
    /// Conditional 3 section 6.1: values without a usable level of support),
    /// matched as `refused_values` are; declarations still parse them.
    /// Default: none.
    #[ignore_malloc_size_of = "static data"]
    pub unsupported_values: &'static [(&'static str, &'static str)],
    /// VIPER-PATCH(selector-names-hook): the embedder's answers about the
    /// pseudo-class and pseudo-element names beyond servo's own, the names
    /// it refuses and what may follow a pseudo-element ([`SelectorNames`]).
    /// Default `None`: servo's parser keeps its built-in answers.
    #[ignore_malloc_size_of = "static data"]
    pub selector_names: Option<&'static dyn SelectorNames>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            attribute_s_flag: true,
            value_aliases: &[],
            refused_values: &[],
            unsupported_values: &[],
            selector_names: None,
        }
    }
}

// VIPER-PATCH(selector-names-hook): two options are equal when their hooks
// are the same object.
impl PartialEq for ParseOptions {
    fn eq(&self, other: &Self) -> bool {
        let same_hook = match (self.selector_names, other.selector_names) {
            (Some(a), Some(b)) => std::ptr::addr_eq(a, b),
            (a, b) => a.is_none() && b.is_none(),
        };
        self.attribute_s_flag == other.attribute_s_flag
            && self.value_aliases == other.value_aliases
            && self.refused_values == other.refused_values
            && self.unsupported_values == other.unsupported_values
            && same_hook
    }
}

impl Eq for ParseOptions {}

impl std::fmt::Debug for ParseOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ParseOptions")
            .field("attribute_s_flag", &self.attribute_s_flag)
            .field("value_aliases", &self.value_aliases)
            .field("refused_values", &self.refused_values)
            .field("unsupported_values", &self.unsupported_values)
            .field("selector_names", &self.selector_names.is_some())
            .finish()
    }
}

/// VIPER-PATCH(selector-names-hook): an embedder's selector names. Servo's
/// `SelectorParser` asks it, when [`ParseOptions::selector_names`] holds
/// it, before its own names: which pseudo-classes and pseudo-elements exist
/// beyond servo's (Selectors 4 section 3.9 makes a selector with any other
/// name invalid), which names servo parses that author and user sheets
/// refuse, and what may follow a pseudo-element in its compound (Selectors
/// 4 section 3.6.3, and each pseudo-element's own specification). Names
/// compare ASCII case-insensitively (Selectors 4 sections 3.5 and 3.6.1).
///
/// A name the embedder admits and has no state for parses as one of the
/// generic variants [`NonTSPseudoClass::Unmatched`] (holding its
/// serialization), [`NonTSPseudoClass::Dir`] or
/// [`PseudoElement::Unmatched`] (holding its name), which match nothing
/// unless the embedder matches them.
/// <https://drafts.csswg.org/selectors-4/#pseudo-element-states>
pub trait SelectorNames: Sync {
    /// The pseudo-class `name`, with `args` the arguments of a functional
    /// one: `Some` when the embedder owns `name` (an `Err` when the
    /// arguments are invalid), `None` to leave it to servo's names.
    fn pseudo_class<'i>(
        &self,
        name: &str,
        args: Option<&mut Parser<'i>>,
    ) -> Option<Result<NonTSPseudoClass, ParseError>>;

    /// The same for the pseudo-element `name`.
    fn pseudo_element<'i>(
        &self,
        name: &str,
        args: Option<&mut Parser<'i>>,
    ) -> Option<Result<PseudoElement, ParseError>>;

    /// Whether a sheet other than a user-agent sheet refuses the
    /// pseudo-class or pseudo-element `name`, plain or functional, which
    /// servo parses.
    fn refuses(&self, name: &str) -> bool;

    /// Whether the simple selector `simple` (a pseudo-class, or a
    /// tree-structural one such as `:only-child`) may follow `element`,
    /// which is not element-backed, in its compound; `:is()`, `:where()`
    /// and `:not()` are asked about what they hold instead. `None`:
    /// upstream's rule, a user-action pseudo-class after a pseudo-element
    /// that accepts state pseudo-classes.
    fn valid_after(
        &self,
        element: &PseudoElement,
        simple: &Component<SelectorImpl>,
    ) -> Option<bool>;

    /// Whether the functional pseudo-class `name()` is an alias of `:is()`
    /// over compound selectors, none forgiven (`selectors-compound-is-alias`).
    /// Default: none is.
    fn compound_is_alias(&self, _name: &str) -> bool {
        false
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
        let refused = self.scan(self.refused_values, property, input, &mut edits);
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

    /// VIPER-PATCH(value-grammar-options): whether the value in `input`, a
    /// supports condition's declaration of `property`, holds one of
    /// `unsupported_values`; `input` is left at its start.
    pub(crate) fn unsupported(&self, property: &str, input: &mut Parser) -> bool {
        if !self.unsupported_values.iter().any(|u| u.0.eq_ignore_ascii_case(property)) {
            return false;
        }
        let start = input.state();
        let found = self.scan(self.unsupported_values, property, input, &mut Vec::new());
        input.reset(&start);
        found
    }

    /// Walks every token of `input` and its nested blocks, recording the
    /// aliased identifiers' byte ranges; returns whether a refused name
    /// occurs.
    fn scan(
        &self,
        refusals: &[(&str, &str)],
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
                refused |= refusals.iter().any(|r| is(r.0, r.1));
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
                    Ok::<_, cssparser::ParseError<()>>(self.scan(refusals, property, input, edits))
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

    #[test]
    fn value_grammar_options_refuse_unsupported_values_in_supports_only() {
        let mut options = ParseOptions::default();
        options.unsupported_values = &[("text-indent", "hanging")];
        assert!(!supports(options, "text-indent", "1px Hanging"));
        assert!(supports(options, "text-indent", "1px each-line"));
        assert!(supports(options, "text-transform", "uppercase"));
        assert_eq!(
            declared(options, "text-indent: 1px hanging"),
            "text-indent: 1px hanging;"
        );
    }
}

// VIPER-PATCH(selector-names-hook): the hook decides through a test
// implementation, in sheets and in servo's parser.
#[cfg(all(test, feature = "servo"))]
mod selector_names_tests {
    use super::{ParseOptions, SelectorNames};
    use crate::context::QuirksMode;
    use crate::selector_parser::{NonTSPseudoClass, PseudoElement, SelectorImpl, SelectorParser};
    use crate::shared_lock::SharedRwLock;
    use crate::stylesheets::UrlExtraData;
    use crate::stylesheets::{AllowImportRules, Namespaces, Origin, StylesheetContents};
    use cssparser::{Parser, ToCss};
    use selectors::parser::{Component, NthSelectorData, ParseRelative, SelectorList};
    use style_traits::ParseError;

    /// Names no specification has: `:made-up`, `:made-up-fn(<ident>)`,
    /// `::made-up`, `::made-up-fn(<ident>)` and the alias `:made-up-any()`;
    /// refuses `:focus` and `::slider-thumb`; `:focus` may follow
    /// `::made-up` and `:only-child` `::made-up-fn()`.
    struct TestNames;

    fn ident<'i>(args: &mut Parser<'i>) -> Result<String, ParseError> {
        Ok(args.expect_ident()?.to_string())
    }

    impl SelectorNames for TestNames {
        fn pseudo_class<'i>(
            &self,
            name: &str,
            args: Option<&mut Parser<'i>>,
        ) -> Option<Result<NonTSPseudoClass, ParseError>> {
            let class = |css: String| NonTSPseudoClass::Unmatched(css.into());
            match (name.to_ascii_lowercase().as_str(), args) {
                ("made-up", None) => Some(Ok(class(":made-up".into()))),
                ("made-up-fn", Some(args)) => {
                    Some(ident(args).map(|a| class(format!(":made-up-fn({a})"))))
                },
                ("dir", Some(args)) => Some(ident(args).map(|a| NonTSPseudoClass::Dir(a.into()))),
                _ => None,
            }
        }

        fn pseudo_element<'i>(
            &self,
            name: &str,
            args: Option<&mut Parser<'i>>,
        ) -> Option<Result<PseudoElement, ParseError>> {
            match (name.to_ascii_lowercase().as_str(), args) {
                ("made-up", None) => Some(Ok(PseudoElement::Unmatched("made-up".into()))),
                ("made-up-fn", Some(args)) => {
                    Some(ident(args).map(|_| PseudoElement::Unmatched("made-up-fn()".into())))
                },
                _ => None,
            }
        }

        fn refuses(&self, name: &str) -> bool {
            name.eq_ignore_ascii_case("focus") || name.eq_ignore_ascii_case("slider-thumb")
        }

        fn valid_after(
            &self,
            element: &PseudoElement,
            simple: &Component<SelectorImpl>,
        ) -> Option<bool> {
            let PseudoElement::Unmatched(name) = element else {
                return None;
            };
            match (&**name, simple) {
                ("made-up", Component::NonTSPseudoClass(NonTSPseudoClass::Focus)) => Some(true),
                ("made-up-fn()", Component::Nth(nth)) => Some(*nth == NthSelectorData::only(false)),
                _ => None,
            }
        }

        fn compound_is_alias(&self, name: &str) -> bool {
            name.eq_ignore_ascii_case("made-up-any")
        }
    }

    static TEST_NAMES: TestNames = TestNames;

    fn with_names() -> ParseOptions {
        ParseOptions {
            selector_names: Some(&TEST_NAMES),
            ..Default::default()
        }
    }

    /// The serialization of `selectors` in a sheet of `origin` under
    /// `options`.
    fn parse(origin: Origin, options: ParseOptions, selectors: &str) -> Option<String> {
        let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
        let namespaces = Namespaces::default();
        let parser = SelectorParser {
            stylesheet_origin: origin,
            namespaces: &namespaces,
            url_data: &url,
            for_supports_rule: false,
            parse_options: options,
        };
        let mut input = Parser::new(selectors);
        let list = SelectorList::parse(&parser, &mut input, ParseRelative::No);
        list.ok().map(|list| list.to_css_string())
    }

    fn author(selectors: &str) -> bool {
        parse(Origin::Author, with_names(), selectors).is_some()
    }

    #[test]
    fn selector_names_hook_decides_the_names() {
        assert!(ParseOptions::default().selector_names.is_none());
        for (selectors, serialized) in [
            (":MADE-UP", ":made-up"),
            (":made-up-fn(A)", ":made-up-fn(A)"),
            (":dir(rtl)", ":dir(rtl)"),
            ("::made-up", "::-internal-unmatched"),
            ("::made-up-fn(a)", "::-internal-unmatched"),
            (":made-up-any(p, div.x)", ":is(p, div.x)"),
            ("p:hover", "p:hover"),
        ] {
            let parsed = parse(Origin::Author, with_names(), selectors);
            assert_eq!(parsed.as_deref(), Some(serialized), "{selectors}");
        }
        // Without the hook's answer, servo's names alone: the default
        // tables' names are refused.
        for selectors in [
            ":made-up-fn(\"a\")",
            ":made-up-fn",
            "::made-up-fn",
            ":horizontal",
            "::-webkit-scrollbar",
            ":-webkit-any(p)",
            ":host-context(p)",
        ] {
            assert!(!author(selectors), "{selectors}");
        }
        // Without a hook, the default tables keep their answers.
        let default = |s| parse(Origin::Author, ParseOptions::default(), s).is_some();
        assert!(default(":horizontal") && default(":-webkit-any(p)"));
        assert!(!default(":made-up") && !default("::made-up"));
    }

    #[test]
    fn selector_names_hook_refuses_outside_user_agent_sheets() {
        for selectors in [":focus", ":FOCUS", "::slider-thumb", ":is(:focus, p)"] {
            let user_agent = parse(Origin::UserAgent, with_names(), selectors);
            assert!(user_agent.is_some(), "{selectors}");
        }
        // A plain list drops the whole selector list; a forgiving one
        // keeps the refused item as written, matching nothing.
        assert!(!author(":focus") && !author("::slider-thumb") && !author(":focus, p"));
        assert!(author(":is(:focus, p)"));
        assert!(author(":focus-visible") && author("::slider-track"));
    }

    #[test]
    fn selector_names_hook_decides_what_follows_a_pseudo_element() {
        for selectors in [
            "::made-up:focus",
            "::made-up:hover",
            "::made-up-fn(a):only-child",
            "::made-up:not(:hover)",
        ] {
            assert!(
                parse(Origin::UserAgent, with_names(), selectors).is_some(),
                "{selectors}"
            );
        }
        for selectors in [
            "::made-up:only-child",
            "::made-up-fn(a):first-child",
            "::before:hover",
            "::made-up:made-up",
        ] {
            assert!(!author(selectors), "{selectors}");
        }
    }

    #[test]
    fn selector_names_hook_in_stylesheets() {
        let lock = SharedRwLock::new();
        let rules = |options| {
            let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
            let contents = StylesheetContents::from_str_with_options(
                ":made-up {} ::made-up {} :horizontal {} ::-webkit-scrollbar {} p {}",
                url,
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
            let count = contents.rules.read_with(&guard).0.len();
            drop(guard);
            count
        };
        assert_eq!(rules(with_names()), 3);
        assert_eq!(rules(ParseOptions::default()), 3);
        assert_eq!(with_names(), with_names());
        assert_ne!(with_names(), ParseOptions::default());
        assert!(format!("{:?}", with_names()).contains("selector_names: true"));
    }
}
