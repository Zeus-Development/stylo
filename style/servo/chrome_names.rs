/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(chrome-selector-names): Chrome match. The pseudo-class and
//! pseudo-element names Chrome 154 accepts and servo's `SelectorParser`
//! does not, with their argument grammars, and the names servo accepts
//! that Chrome refuses outside user-agent sheets. Chrome's stylesheets and
//! its DOM selector APIs (`querySelector`, `matches`, ...) parse selectors
//! with one parser, so both accept the same names; Viper's ADR 0017 ("The
//! selector names Chrome accepts") records the probe of Chrome 154 these
//! tables come from. Upstream has none of these: Servo refuses the names.
//!
//! Each name maps to a variant of the patch `chrome-dom-api-pseudos`
//! (`NonTSPseudoClass::Dir`, `NonTSPseudoClass::Unmatched` and
//! `PseudoElement::Unmatched`) or to an alias of a servo pseudo-class;
//! `:-webkit-any()` is `selectors-compound-is-alias`' `:is()`. Names
//! compare ASCII case-insensitively, as Chrome compares them. Every
//! function returns `None` for a name it does not own, so the caller falls
//! through to servo's own names.
//! <https://drafts.csswg.org/selectors-4/>

// VIPER-PATCH(chrome-after-pseudo-element): each name's kind.
use super::chrome_after::{functional_kind, plain_kind};
use super::selector_parser::{NonTSPseudoClass, PseudoElement, SelectorImpl};
use crate::selector_parser::SelectorParser;
use cssparser::{Parser as CssParser, ToCss, Token};
use selectors::parser::{Component, ParseRelative, Selector, SelectorList, SelectorParseErrorKind};
use style_traits::ParseError;

/// The plain pseudo-classes Chrome 154 accepts that servo does not parse
/// (Viper's probe: its 40 names, less the three aliases of [`alias`] and
/// the legacy one-colon `:first-line`, which is a pseudo-element). Each is
/// `NonTSPseudoClass::Unmatched` with its serialization; the embedder
/// decides what matches (Chrome matches nearly none on a static page).
const CHROME_UNMATCHED_PSEUDO_CLASSES: [&str; 36] = [
    "-internal-autofill-previewed",
    "-internal-autofill-selected",
    "-internal-dialog-in-top-layer",
    "-internal-popover-in-top-layer",
    "-internal-relative-anchor",
    "-internal-select-has-slotted-button",
    "-internal-text-field",
    "-webkit-drag",
    "-webkit-full-page-media",
    "-webkit-full-screen-ancestor",
    "active-view-transition",
    "corner-present",
    "current",
    "decrement",
    "double-button",
    "end",
    "future",
    "granted",
    "horizontal",
    "increment",
    "interest-source",
    "interest-target",
    "no-button",
    "past",
    "picture-in-picture",
    "single-button",
    "start",
    "target-after",
    "target-before",
    "target-current",
    "tool-form-active",
    "tool-submit-active",
    "unbounded",
    "vertical",
    "window-inactive",
    "xr-overlay",
];

/// The plain pseudo-elements Chrome 154 accepts that servo does not parse.
/// `::first-line` is not one: the patch `servo-first-line` parses it.
const CHROME_UNMATCHED_PSEUDO_ELEMENTS: [&str; 14] = [
    "checkmark",
    "column",
    "cue",
    "grammar-error",
    "interest-button",
    "permission-icon",
    "picker-icon",
    "scroll-marker",
    "scroll-marker-group",
    "search-text",
    "select-listbox",
    "spelling-error",
    "target-text",
    "view-transition",
];

/// The `::view-transition-*()` pseudo-elements, which take a
/// `<pt-name-and-class-selector>` ([`view_transition_argument`]).
const VIEW_TRANSITION_PARTS: [&str; 5] = [
    "view-transition-group",
    "view-transition-group-children",
    "view-transition-image-pair",
    "view-transition-new",
    "view-transition-old",
];

/// The `::scroll-button()` directions.
/// <https://drafts.csswg.org/css-overflow-5/#scroll-buttons>
const SCROLL_BUTTON_DIRECTIONS: [&str; 8] = [
    "up",
    "down",
    "left",
    "right",
    "block-start",
    "block-end",
    "inline-start",
    "inline-end",
];

/// The pseudo-elements servo parses that Chrome 154 refuses (the color
/// input's swatch and the range input's parts have other names there).
const REFUSED_PSEUDO_ELEMENTS: [&str; 4] = [
    "color-swatch",
    "slider-fill",
    "slider-thumb",
    "slider-track",
];

fn listed(names: &[&str], name: &str) -> bool {
    names.iter().any(|n| n.eq_ignore_ascii_case(name))
}

fn unsupported() -> ParseError {
    ParseError::custom(SelectorParseErrorKind::UnsupportedPseudoClassOrElement)
}

/// Whether a sheet that is not a user-agent sheet refuses the pseudo-class
/// or pseudo-element `name`, which servo parses and Chrome 154 refuses:
/// Gecko's and Servo's own `-moz-*` and `-servo-*` names, and with
/// `pseudo_element` also [`REFUSED_PSEUDO_ELEMENTS`].
pub(super) fn refused_outside_user_agent_sheets(name: &str, pseudo_element: bool) -> bool {
    let prefixed = |prefix: &str| {
        name.get(..prefix.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(prefix))
    };
    prefixed("-moz-")
        || prefixed("-servo-")
        || (pseudo_element && listed(&REFUSED_PSEUDO_ELEMENTS, name))
}

/// Chrome's prefixed aliases of pseudo-classes servo parses.
fn alias(name: &str) -> Option<NonTSPseudoClass> {
    let is = |n: &str| name.eq_ignore_ascii_case(n);
    if is("-webkit-any-link") {
        Some(NonTSPseudoClass::AnyLink)
    } else if is("-webkit-autofill") {
        Some(NonTSPseudoClass::Autofill)
    } else if is("-webkit-full-screen") {
        Some(NonTSPseudoClass::Fullscreen)
    } else {
        None
    }
}

/// The plain pseudo-class `name`, if Chrome accepts it and servo does not.
pub(super) fn pseudo_class(name: &str) -> Option<NonTSPseudoClass> {
    alias(name).or_else(|| {
        listed(&CHROME_UNMATCHED_PSEUDO_CLASSES, name)
            .then(|| NonTSPseudoClass::Unmatched(format!(":{}", name.to_ascii_lowercase()).into()))
    })
}

/// The functional pseudo-class `name(arguments)`, if Chrome accepts the
/// name and servo does not; `Some(Err)` when its argument is invalid.
pub(super) fn functional_pseudo_class<'i>(
    parser: &SelectorParser,
    name: &str,
    arguments: &mut CssParser<'i>,
) -> Option<Result<NonTSPseudoClass, ParseError>> {
    let is = |n: &str| name.eq_ignore_ascii_case(n);
    Some(if is("dir") {
        arguments
            .expect_ident()
            .map(|dir| NonTSPseudoClass::Dir(dir.as_ref().into()))
            .map_err(Into::into)
    } else if is("host-context") {
        compound_list(parser, arguments, false)
            .map(|inner| NonTSPseudoClass::Unmatched(format!(":host-context({inner})").into()))
    } else if is("active-view-transition-type") {
        // CSS View Transitions 2 says `<custom-ident>#`; Chrome 154 accepts
        // any identifiers.
        arguments
            .parse_comma_separated(|a| Ok(a.expect_ident()?.to_string()))
            .map(|types| {
                let types = types.join(", ");
                NonTSPseudoClass::Unmatched(format!(":active-view-transition-type({types})").into())
            })
    } else {
        return None;
    })
}

/// The plain pseudo-element `name`, if Chrome accepts it and servo does
/// not: one of [`CHROME_UNMATCHED_PSEUDO_ELEMENTS`] or any `-webkit-` name
/// (Chrome accepts a made-up one too).
pub(super) fn pseudo_element(name: &str) -> Option<PseudoElement> {
    let webkit = name
        .get(.."-webkit-".len())
        .is_some_and(|start| start.eq_ignore_ascii_case("-webkit-"));
    // VIPER-PATCH(chrome-after-pseudo-element): with its kind.
    (webkit || listed(&CHROME_UNMATCHED_PSEUDO_ELEMENTS, name))
        .then(|| PseudoElement::Unmatched(plain_kind(name)))
}

/// The functional pseudo-element `name(arguments)`, if Chrome accepts the
/// name; `Some(Err)` when its argument is invalid.
pub(super) fn functional_pseudo_element<'i>(
    parser: &SelectorParser,
    name: &str,
    arguments: &mut CssParser<'i>,
) -> Option<Result<PseudoElement, ParseError>> {
    let is = |n: &str| name.eq_ignore_ascii_case(n);
    let parsed = if is("highlight") {
        // CSS Highlight API: `::highlight(<custom-ident>)`; Chrome 154
        // accepts any identifier.
        arguments.expect_ident().map(drop).map_err(Into::into)
    } else if is("cue") {
        // WebVTT: `::cue(<compound-selector-list>)`.
        compound_list(parser, arguments, true).map(drop)
    } else if is("scroll-button") {
        scroll_button_argument(arguments)
    } else if listed(&VIEW_TRANSITION_PARTS, name) {
        view_transition_argument(arguments)
    } else {
        return None;
    };
    // VIPER-PATCH(chrome-after-pseudo-element): with its kind.
    let kind = functional_kind(name, listed(&VIEW_TRANSITION_PARTS, name));
    Some(parsed.map(|()| PseudoElement::Unmatched(kind)))
}

/// A `<compound-selector>`, or with `list` a `<compound-selector-list>`: no
/// combinator and no pseudo-element. Returns the serialized list.
fn compound_list<'i>(
    parser: &SelectorParser,
    arguments: &mut CssParser<'i>,
    list: bool,
) -> Result<String, ParseError> {
    let selectors = SelectorList::<SelectorImpl>::parse(parser, arguments, ParseRelative::No)?;
    let compound = |s: &Selector<SelectorImpl>| {
        !s.has_pseudo_element()
            && !s
                .iter_raw_match_order()
                .any(|c| matches!(c, Component::Combinator(_)))
    };
    let slice = selectors.slice();
    if (list || slice.len() == 1) && slice.iter().all(compound) {
        Ok(selectors.to_css_string())
    } else {
        Err(unsupported())
    }
}

/// The identifiers a `<custom-ident>` excludes: the CSS-wide keywords and
/// `default`. <https://drafts.csswg.org/css-values-4/#custom-idents>
const NOT_CUSTOM_IDENTS: [&str; 6] = [
    "initial",
    "inherit",
    "unset",
    "revert",
    "revert-layer",
    "default",
];

/// A `<custom-ident>`; with `adjacent`, no whitespace may come before it.
fn custom_ident<'i>(arguments: &mut CssParser<'i>, adjacent: bool) -> Result<(), ParseError> {
    let token = if adjacent {
        arguments.next_including_whitespace()?
    } else {
        arguments.next()?
    };
    match token {
        Token::Ident(ident) if !listed(&NOT_CUSTOM_IDENTS, ident) => Ok(()),
        _ => Err(unsupported()),
    }
}

/// `* | <scroll-button-direction>`.
fn scroll_button_argument<'i>(arguments: &mut CssParser<'i>) -> Result<(), ParseError> {
    if arguments.try_parse(|a| a.expect_delim('*')).is_ok() {
        return Ok(());
    }
    let direction = arguments.expect_ident()?;
    if listed(&SCROLL_BUTTON_DIRECTIONS, direction) {
        Ok(())
    } else {
        Err(unsupported())
    }
}

/// `<pt-name-and-class-selector>`: `*` or a `<custom-ident>`, then
/// `.<custom-ident>` classes; or the classes alone. As in Chrome 154,
/// whitespace may come before a `.` but not after it.
/// <https://drafts.csswg.org/css-view-transitions-2/#pseudo-element-class-additions>
fn view_transition_argument<'i>(arguments: &mut CssParser<'i>) -> Result<(), ParseError> {
    let named = arguments.try_parse(|a| a.expect_delim('*')).is_ok()
        || arguments.try_parse(|a| custom_ident(a, false)).is_ok();
    let mut classes = 0;
    while arguments.try_parse(|a| a.expect_delim('.')).is_ok() {
        custom_ident(arguments, true)?;
        classes += 1;
    }
    if named || classes > 0 {
        Ok(())
    } else {
        Err(unsupported())
    }
}

// VIPER-PATCH(chrome-selector-names): the names' tests, in author and
// user-agent sheets.
#[cfg(test)]
mod tests {
    use crate::selector_parser::SelectorParser;
    use crate::stylesheets::{Namespaces, Origin, UrlExtraData};
    use cssparser::{Parser, ToCss};
    use selectors::parser::{ParseRelative, SelectorList};

    /// The serialization of `selectors` parsed in a sheet of `origin`.
    fn parse(origin: Origin, selectors: &str) -> Option<String> {
        let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
        let namespaces = Namespaces::default();
        let parser = SelectorParser {
            stylesheet_origin: origin,
            namespaces: &namespaces,
            url_data: &url,
            for_supports_rule: false,
        };
        let mut input = Parser::new(selectors);
        let list = SelectorList::parse(&parser, &mut input, ParseRelative::No);
        list.ok().map(|list| list.to_css_string())
    }

    fn author(selectors: &str) -> bool {
        parse(Origin::Author, selectors).is_some()
    }

    #[test]
    fn chrome_selector_names_parse_in_every_origin() {
        for selectors in [
            ":dir(rtl)",
            ":horizontal",
            ":-internal-text-field",
            ":host-context(div.a)",
            ":active-view-transition-type(a, inherit)",
            ":-webkit-any(p, div.x)",
            ":-webkit-autofill",
            ":-webkit-full-screen",
            "::cue",
            "::cue(b, i.x)",
            "::-WEBKIT-made-up",
            "::-webkit-scrollbar:hover",
            "::highlight(x)",
            "::scroll-button(*)",
            "::scroll-button(inline-end)",
            "::view-transition-group(a.b.c)",
            "::view-transition-new(.a)",
            "::view-transition-image-pair(a .b)",
            "::spelling-error",
        ] {
            assert!(author(selectors), "{selectors}");
            assert!(parse(Origin::UserAgent, selectors).is_some(), "{selectors}");
        }
        for (selectors, serialized) in [
            (":HORIZONTAL", ":horizontal"),
            (":DIR(Rtl)", ":dir(Rtl)"),
            (":host-context(div.A)", ":host-context(div.A)"),
            (":-webkit-any-link", ":any-link"),
            ("::view-transition-old(*)", "::-internal-unmatched"),
        ] {
            assert_eq!(
                parse(Origin::Author, selectors).as_deref(),
                Some(serialized)
            );
        }
    }

    #[test]
    fn chrome_selector_names_refuse_bad_arguments() {
        for selectors in [
            ":-webkit-made-up",
            ":dir()",
            ":dir(\"rtl\")",
            ":host-context(div p)",
            ":host-context(a, b)",
            ":host-context(::before)",
            ":-webkit-any()",
            ":-webkit-any(div p)",
            ":-webkit-any(p, :made-up)",
            "::highlight(a b)",
            "::cue(a > b)",
            "::scroll-button(sideways)",
            "::view-transition-new(inherit)",
            "::view-transition-old(a. b)",
            "::view-transition-old()",
        ] {
            assert!(!author(selectors), "{selectors}");
        }
    }

    #[test]
    fn chrome_selector_names_refused_outside_user_agent_sheets() {
        for selectors in [
            ":-moz-meter-optimum",
            ":-MOZ-meter-sub-optimum",
            ":lang(\"en\")",
            "::color-swatch",
            "::slider-fill",
            "::slider-thumb",
            "::slider-track",
            "::-moz-progress-bar",
        ] {
            assert!(!author(selectors), "{selectors}");
            assert!(parse(Origin::UserAgent, selectors).is_some(), "{selectors}");
        }
        assert!(author(":lang(en)") && !author(":-servo-nonzero-border"));
    }
}
