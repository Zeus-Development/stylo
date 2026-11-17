/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(text-spacing-trim-longhand): the value of CSS Text 4's
//! `text-spacing-trim` (section 8.5), which upstream does not have, so a
//! servo build dropped its declarations. Its grammar is the Editor's
//! Draft's:
//!
//! ```text
//! <spacing-trim> | auto
//! <spacing-trim> = [ trim-start | space-start | space-first ] ||
//!                  [ trim-adjacent | space-adjacent ] ||
//!                  [ trim-end | space-end | allow-end ]
//!                  | trim-both | space-all | trim-all
//! ```
//!
//! with `normal`, the initial value, as its own keyword. The computed value
//! is "the keyword auto, the keyword trim-all, or three keywords, one per
//! controlled position": omitted positions take `trim-start trim-adjacent
//! allow-end`, and `normal`, `space-all` and `trim-both` are their three
//! keywords. Spacing the punctuation is the embedder's.
//! <https://drafts.csswg.org/css-text-4/#text-spacing-trim-property>

use crate::derives::*;
use crate::parser::{Parse, ParserContext};
use cssparser::Parser;
use std::fmt::{self, Write};
use style_traits::values::SequenceWriter;
use style_traits::{CssWriter, KeywordsCollectFn, ParseError, SpecifiedValueInfo};
use style_traits::{StyleParseErrorKind, ToCss};

/// How fullwidth opening punctuation at the start of a line is set.
#[allow(missing_docs)]
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    MallocSizeOf,
    Parse,
    PartialEq,
    Serialize,
    ToComputedValue,
    ToCss,
    ToResolvedValue,
    ToShmem,
)]
#[repr(u8)]
pub enum SpacingTrimStart {
    TrimStart,
    SpaceStart,
    SpaceFirst,
}

/// How adjacent pairs of fullwidth punctuation are set.
#[allow(missing_docs)]
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    MallocSizeOf,
    Parse,
    PartialEq,
    Serialize,
    ToComputedValue,
    ToCss,
    ToResolvedValue,
    ToShmem,
)]
#[repr(u8)]
pub enum SpacingTrimAdjacent {
    TrimAdjacent,
    SpaceAdjacent,
}

/// How fullwidth closing punctuation at the end of a line is set.
#[allow(missing_docs)]
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    MallocSizeOf,
    Parse,
    PartialEq,
    Serialize,
    ToComputedValue,
    ToCss,
    ToResolvedValue,
    ToShmem,
)]
#[repr(u8)]
pub enum SpacingTrimEnd {
    AllowEnd,
    TrimEnd,
    SpaceEnd,
}

/// The specified and computed value of `text-spacing-trim`.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    MallocSizeOf,
    PartialEq,
    Serialize,
    ToComputedValue,
    ToResolvedValue,
    ToShmem,
    ToTyped,
)]
#[repr(C, u8)]
#[typed(todo_derive_fields)]
pub enum TextSpacingTrim {
    /// One keyword per controlled position.
    Positions {
        /// At the start of a line.
        start: SpacingTrimStart,
        /// Between adjacent pairs.
        adjacent: SpacingTrimAdjacent,
        /// At the end of a line.
        end: SpacingTrimEnd,
    },
    /// `trim-all`.
    TrimAll,
    /// `auto`.
    Auto,
}

impl TextSpacingTrim {
    /// `normal`, the initial value: `space-start trim-adjacent allow-end`.
    pub const NORMAL: Self = Self::Positions {
        start: SpacingTrimStart::SpaceStart,
        adjacent: SpacingTrimAdjacent::TrimAdjacent,
        end: SpacingTrimEnd::AllowEnd,
    };

    /// `space-all`: `space-start space-adjacent space-end`.
    pub const SPACE_ALL: Self = Self::Positions {
        start: SpacingTrimStart::SpaceStart,
        adjacent: SpacingTrimAdjacent::SpaceAdjacent,
        end: SpacingTrimEnd::SpaceEnd,
    };

    /// `trim-both`: `trim-start trim-adjacent trim-end`.
    pub const TRIM_BOTH: Self = Self::Positions {
        start: SpacingTrimStart::TrimStart,
        adjacent: SpacingTrimAdjacent::TrimAdjacent,
        end: SpacingTrimEnd::TrimEnd,
    };

    /// `[start] || [adjacent] || [end]`, each position at most once and
    /// at least one, the omitted ones taken from `trim-start trim-adjacent
    /// allow-end`.
    fn parse_positions(input: &mut Parser) -> Result<Self, ParseError> {
        let (mut start, mut adjacent, mut end) = (None, None, None);
        loop {
            if start.is_none() {
                start = input.try_parse(SpacingTrimStart::parse).ok();
                if start.is_some() {
                    continue;
                }
            }
            if adjacent.is_none() {
                adjacent = input.try_parse(SpacingTrimAdjacent::parse).ok();
                if adjacent.is_some() {
                    continue;
                }
            }
            if end.is_none() {
                end = input.try_parse(SpacingTrimEnd::parse).ok();
                if end.is_some() {
                    continue;
                }
            }
            break;
        }
        if start.is_none() && adjacent.is_none() && end.is_none() {
            return Err(ParseError::custom(StyleParseErrorKind::UnspecifiedError));
        }
        Ok(Self::Positions {
            start: start.unwrap_or(SpacingTrimStart::TrimStart),
            adjacent: adjacent.unwrap_or(SpacingTrimAdjacent::TrimAdjacent),
            end: end.unwrap_or(SpacingTrimEnd::AllowEnd),
        })
    }
}

impl Parse for TextSpacingTrim {
    fn parse(_: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        let single = input.try_parse(|i| -> Result<Self, ParseError> {
            Ok(try_match_ident_ignore_ascii_case! { i,
                "auto" => Self::Auto,
                "trim-all" => Self::TrimAll,
                "normal" => Self::NORMAL,
                "space-all" => Self::SPACE_ALL,
                "trim-both" => Self::TRIM_BOTH,
            })
        });
        match single {
            Ok(value) => Ok(value),
            Err(_) => Self::parse_positions(input),
        }
    }
}

impl ToCss for TextSpacingTrim {
    /// The shortest form: `normal`, `space-all` and `trim-both` by name,
    /// otherwise the keywords that differ from the omitted positions'
    /// defaults (`trim-start` when none does).
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        let (start, adjacent, end) = match *self {
            Self::Auto => return dest.write_str("auto"),
            Self::TrimAll => return dest.write_str("trim-all"),
            _ if *self == Self::NORMAL => return dest.write_str("normal"),
            _ if *self == Self::SPACE_ALL => return dest.write_str("space-all"),
            _ if *self == Self::TRIM_BOTH => return dest.write_str("trim-both"),
            Self::Positions {
                start,
                adjacent,
                end,
            } => (start, adjacent, end),
        };
        let trimmed_adjacent = adjacent == SpacingTrimAdjacent::TrimAdjacent;
        let allowed_end = end == SpacingTrimEnd::AllowEnd;
        let mut writer = SequenceWriter::new(dest, " ");
        if start != SpacingTrimStart::TrimStart || (trimmed_adjacent && allowed_end) {
            writer.item(&start)?;
        }
        if !trimmed_adjacent {
            writer.item(&adjacent)?;
        }
        if !allowed_end {
            writer.item(&end)?;
        }
        Ok(())
    }
}

impl SpecifiedValueInfo for TextSpacingTrim {
    fn collect_completion_keywords(f: KeywordsCollectFn) {
        f(&[
            "auto",
            "normal",
            "space-all",
            "trim-both",
            "trim-all",
            "trim-start",
            "space-start",
            "space-first",
            "trim-adjacent",
            "space-adjacent",
            "trim-end",
            "space-end",
            "allow-end",
        ]);
    }
}

#[cfg(all(test, feature = "servo"))]
mod tests {
    use super::TextSpacingTrim;
    use crate::context::QuirksMode;
    use crate::parser::{Parse, ParserContext};
    use crate::stylesheets::{CssRuleType, Origin, UrlExtraData};
    use cssparser::Parser;
    use style_traits::{ParsingMode, ToCss};

    fn parsed(text: &str) -> Option<TextSpacingTrim> {
        let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
        let context = ParserContext::new(
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
        let mut parser = Parser::new(text);
        parser
            .parse_entirely(|i| TextSpacingTrim::parse(&context, i))
            .ok()
    }

    #[test]
    fn text_spacing_trim_parses_the_editors_draft_grammar() {
        let css = |text: &str| parsed(text).map(|value| value.to_css_string());
        for (text, serialized) in [
            ("normal", "normal"),
            ("auto", "auto"),
            ("space-all", "space-all"),
            ("trim-both", "trim-both"),
            ("trim-all", "trim-all"),
            ("TRIM-START", "trim-start"),
            ("space-first", "space-first"),
            ("space-start", "normal"),
            ("space-start space-adjacent space-end", "space-all"),
            ("trim-end", "trim-both"),
            ("trim-start trim-end", "trim-both"),
            ("allow-end", "trim-start"),
            ("trim-adjacent", "trim-start"),
            ("space-adjacent", "space-adjacent"),
            ("space-end space-first", "space-first space-end"),
            ("space-adjacent trim-start", "space-adjacent"),
            (
                "trim-end space-adjacent space-first",
                "space-first space-adjacent trim-end",
            ),
        ] {
            assert_eq!(css(text).as_deref(), Some(serialized), "{text}");
            let again = parsed(serialized).unwrap();
            assert_eq!(parsed(text), Some(again), "{text} round-trips");
        }
        for text in [
            "none",
            "normal space-all",
            "auto space-first",
            "trim-start space-start",
            "trim-end allow-end",
            "trim-all trim-start",
            "space-first space-first",
            "trim-auto",
        ] {
            assert_eq!(parsed(text), None, "{text}");
        }
    }
}
