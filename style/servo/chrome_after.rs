/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(chrome-after-pseudo-element): Chrome match. The
//! pseudo-classes Chrome 154 accepts after each pseudo-element in its
//! compound, in stylesheets and in its DOM selector APIs alike: Chromium's
//! `IsPseudoClassValidAfterPseudoElement` (BSD, `css_selector_parser.cc`),
//! as Viper's selector goldens confirm it name by name (Viper's ADR 0017,
//! "The patch of V10b-4c2", lists where they decide). Upstream accepts only
//! a user-action pseudo-class, and only after a pseudo-element that accepts
//! state pseudo-classes. The pseudo-elements servo has no variant for are
//! `PseudoElement::Unmatched`, whose [`UnmatchedKind`] says which row of
//! Chrome's table applies.
//! <https://chromium.googlesource.com/chromium/src/+/refs/tags/154.0.8037.57/third_party/blink/renderer/core/css/parser/css_selector_parser.cc>

use super::selector_parser::{NonTSPseudoClass, PseudoElement, SelectorImpl};
use crate::derives::*;
use selectors::parser::{Component, NthSelectorData};

/// Which of Chrome's pseudo-elements an `Unmatched` one is, for what may
/// follow it in its compound. Every kind serializes, cascades and is
/// indexed as the one `Unmatched` pseudo-element.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, MallocSizeOf, PartialEq, Serialize, ToShmem,
)]
pub enum UnmatchedKind {
    /// `::-webkit-scrollbar` and its parts, and `::-webkit-resizer`: the
    /// scrollbar pseudo-classes.
    ScrollbarPart,
    /// Any other `::-webkit-` name: the user-action pseudo-classes.
    WebkitCustom,
    /// `::view-transition-group()` and the other parts in the tree of
    /// `::view-transition`: `:only-child`.
    ViewTransitionPart,
    /// `::search-text`: `:current`.
    SearchText,
    /// `::scroll-marker`: the user actions and `:target-current`,
    /// `:target-before`, `:target-after`.
    ScrollMarker,
    /// `::scroll-marker-group`: `:focus-within` and `:hover`.
    ScrollMarkerGroup,
    /// `::scroll-button()`: the user actions, `:enabled` and `:disabled`.
    ScrollButton,
    /// The plain `::cue`: the user actions, as for a `-webkit-` name
    /// (Viper's golden `names-after-cue-hover`; `::cue()` is `Other`).
    Cue,
    /// `::column`: no pseudo-class.
    Column,
    /// Every other name (`::view-transition`, `::cue()`, `::highlight()`,
    /// `::target-text`, ...): no pseudo-class.
    Other,
}

/// The scrollbar pseudo-elements.
const SCROLLBAR_PARTS: [&str; 7] = [
    "-webkit-resizer",
    "-webkit-scrollbar",
    "-webkit-scrollbar-button",
    "-webkit-scrollbar-corner",
    "-webkit-scrollbar-thumb",
    "-webkit-scrollbar-track",
    "-webkit-scrollbar-track-piece",
];

/// The scrollbar pseudo-classes servo has no variant for, as their
/// `NonTSPseudoClass::Unmatched` serializations.
const SCROLLBAR_PSEUDO_CLASSES: [&str; 11] = [
    ":corner-present",
    ":decrement",
    ":double-button",
    ":end",
    ":horizontal",
    ":increment",
    ":no-button",
    ":single-button",
    ":start",
    ":vertical",
    ":window-inactive",
];

/// The kind of the plain pseudo-element `name`, one Chrome accepts and
/// servo has no variant for.
pub(super) fn plain_kind(name: &str) -> UnmatchedKind {
    let is = |n: &str| name.eq_ignore_ascii_case(n);
    if SCROLLBAR_PARTS.iter().any(|part| is(part)) {
        UnmatchedKind::ScrollbarPart
    } else if name
        .get(..8)
        .is_some_and(|s| s.eq_ignore_ascii_case("-webkit-"))
    {
        UnmatchedKind::WebkitCustom
    } else if is("cue") {
        UnmatchedKind::Cue
    } else if is("search-text") {
        UnmatchedKind::SearchText
    } else if is("scroll-marker") {
        UnmatchedKind::ScrollMarker
    } else if is("scroll-marker-group") {
        UnmatchedKind::ScrollMarkerGroup
    } else if is("column") {
        UnmatchedKind::Column
    } else {
        UnmatchedKind::Other
    }
}

/// The kind of a functional pseudo-element: `::scroll-button()`, one of
/// `view_transition_part`'s, or another (`::cue()`, `::highlight()`).
pub(super) fn functional_kind(name: &str, view_transition_part: bool) -> UnmatchedKind {
    if name.eq_ignore_ascii_case("scroll-button") {
        UnmatchedKind::ScrollButton
    } else if view_transition_part {
        UnmatchedKind::ViewTransitionPart
    } else {
        UnmatchedKind::Other
    }
}

/// Whether `simple` may follow `pseudo_element` (not element-backed) in
/// its compound: `selectors::Parser::valid_after_pseudo_element` for servo.
pub(super) fn valid_after(
    pseudo_element: &PseudoElement,
    simple: &Component<SelectorImpl>,
) -> bool {
    let class = match simple {
        Component::NonTSPseudoClass(class) => class,
        Component::Nth(nth) => {
            return *nth == NthSelectorData::only(false)
                && *pseudo_element == PseudoElement::Unmatched(UnmatchedKind::ViewTransitionPart)
        },
        _ => return false,
    };
    match pseudo_element {
        PseudoElement::Unmatched(kind) => valid_after_unmatched(*kind, class),
        PseudoElement::Selection => named(class, &[":window-inactive"]),
        PseudoElement::FileSelectorButton => user_action(class),
        // `::before`, `::after` and `::marker` take `:hover` only behind a
        // runtime flag that Chrome 154 leaves off (Viper's goldens
        // `names-after-before-hover`, `names-after-marker-hover`).
        _ => false,
    }
}

fn valid_after_unmatched(kind: UnmatchedKind, class: &NonTSPseudoClass) -> bool {
    use self::NonTSPseudoClass::{Active, Disabled, Enabled, FocusWithin, Hover};
    match kind {
        UnmatchedKind::ScrollbarPart => {
            matches!(class, Active | Disabled | Enabled | Hover)
                || named(class, &SCROLLBAR_PSEUDO_CLASSES)
        },
        UnmatchedKind::WebkitCustom | UnmatchedKind::Cue => user_action(class),
        UnmatchedKind::SearchText => named(class, &[":current"]),
        UnmatchedKind::ScrollMarker => {
            user_action(class)
                || named(
                    class,
                    &[":target-after", ":target-before", ":target-current"],
                )
        },
        UnmatchedKind::ScrollMarkerGroup => matches!(class, FocusWithin | Hover),
        UnmatchedKind::ScrollButton => user_action(class) || matches!(class, Disabled | Enabled),
        UnmatchedKind::ViewTransitionPart | UnmatchedKind::Column | UnmatchedKind::Other => false,
    }
}

/// Chrome's user-action pseudo-classes (`IsUserActionPseudoClass`).
fn user_action(class: &NonTSPseudoClass) -> bool {
    use self::NonTSPseudoClass::{Active, Focus, FocusVisible, FocusWithin, Hover};
    matches!(class, Active | Focus | FocusVisible | FocusWithin | Hover)
}

/// Whether `class` is one of the `Unmatched` pseudo-classes `names`.
fn named(class: &NonTSPseudoClass, names: &[&str]) -> bool {
    matches!(class, NonTSPseudoClass::Unmatched(css) if names.contains(&&**css))
}

// VIPER-PATCH(chrome-after-pseudo-element): Chrome's table in author
// sheets, through servo's parser.
#[cfg(test)]
mod tests {
    use crate::selector_parser::SelectorParser;
    use crate::stylesheets::{Namespaces, Origin, UrlExtraData};
    use cssparser::{Parser, ToCss};
    use selectors::parser::{ParseRelative, SelectorList};

    fn parse(selectors: &str) -> Option<String> {
        let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
        let namespaces = Namespaces::default();
        let parser = SelectorParser {
            stylesheet_origin: Origin::Author,
            namespaces: &namespaces,
            url_data: &url,
            for_supports_rule: false,
            // VIPER-PATCH(stylesheet-parse-options): upstream's behaviour.
            parse_options: Default::default(),
        };
        let mut input = Parser::new(selectors);
        let list = SelectorList::parse(&parser, &mut input, ParseRelative::No);
        list.ok().map(|list| list.to_css_string())
    }

    #[test]
    fn chrome_pseudo_classes_after_pseudo_elements() {
        for selectors in [
            "::-webkit-scrollbar:horizontal",
            "::-WEBKIT-SCROLLBAR-BUTTON:decrement:hover",
            "::-webkit-resizer:disabled",
            "::-webkit-scrollbar-corner:window-inactive",
            "::-webkit-foo:focus-visible",
            "::cue:focus-within",
            "::file-selector-button:active",
            "::selection:window-inactive",
            "::view-transition-old(*):only-child",
            "::search-text:current",
            "::scroll-marker:target-current",
            "::scroll-marker:focus",
            "::scroll-marker-group:focus-within",
            "::scroll-button(up):enabled",
            "::scroll-button(*):hover",
            "::-webkit-scrollbar:not(:vertical)",
            "::selection:not(:window-inactive)",
            "::-webkit-scrollbar:where(:focus)",
        ] {
            assert!(parse(selectors).is_some(), "{selectors}");
        }
        for selectors in [
            "::-webkit-scrollbar:focus",
            "::-webkit-scrollbar:first-child",
            "::-webkit-scrollbar:nth-child(1)",
            "::-webkit-scrollbar:dir(rtl)",
            "::-webkit-scrollbar:has(:hover)",
            "::-webkit-scrollbar:-webkit-any(:hover)",
            "::-webkit-scrollbar:not(:focus)",
            "::-webkit-foo:horizontal",
            "::selection:hover",
            "::placeholder:hover",
            "::before:hover",
            "::marker:hover",
            "::view-transition:only-child",
            "::view-transition-group(*):hover",
            "::cue(b):hover",
            "::highlight(x):hover",
            "::column:hover",
            "::scroll-marker-group:focus",
        ] {
            assert!(parse(selectors).is_none(), "{selectors}");
        }
        // A forgiving list keeps an item invalid after the pseudo-element,
        // as written, and matches nothing through it.
        assert_eq!(
            parse("::-webkit-scrollbar:is(:horizontal, :focus)").as_deref(),
            Some("::-internal-unmatched:is(:horizontal, :focus)")
        );
    }
}
