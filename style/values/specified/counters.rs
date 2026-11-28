/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Specified types for counter properties.

use crate::counter_style::CounterStyle;
use crate::parser::{Parse, ParserContext};
use crate::values::generics::counters as generics;
use crate::values::generics::counters::CounterPair;
use crate::values::specified::image::Image;
use crate::values::specified::Integer;
use crate::values::CustomIdent;
use cssparser::{match_ignore_ascii_case, Parser, Token};
use selectors::parser::SelectorParseErrorKind;
use style_traits::{ParseError, StyleParseErrorKind};

#[derive(PartialEq)]
enum CounterType {
    Increment,
    Set,
    Reset,
}

impl CounterType {
    fn default_value(&self) -> i32 {
        match *self {
            Self::Increment => 1,
            Self::Reset | Self::Set => 0,
        }
    }
}

/// A specified value for the `counter-increment` property.
pub type CounterIncrement = generics::GenericCounterIncrement<Integer>;

impl Parse for CounterIncrement {
    fn parse(context: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        Ok(Self::new(parse_counters(
            context,
            input,
            CounterType::Increment,
        )?))
    }
}

/// A specified value for the `counter-set` property.
pub type CounterSet = generics::GenericCounterSet<Integer>;

impl Parse for CounterSet {
    fn parse(context: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        Ok(Self::new(parse_counters(context, input, CounterType::Set)?))
    }
}

/// A specified value for the `counter-reset` property.
pub type CounterReset = generics::GenericCounterReset<Integer>;

impl Parse for CounterReset {
    fn parse(context: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        Ok(Self::new(parse_counters(
            context,
            input,
            CounterType::Reset,
        )?))
    }
}

fn parse_counters(
    context: &ParserContext,
    input: &mut Parser,
    counter_type: CounterType,
) -> Result<Vec<CounterPair<Integer>>, ParseError> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(vec![]);
    }

    let mut counters = Vec::new();
    loop {
        let (name, is_reversed) = match input.next() {
            Ok(Token::Ident(ident)) => (CustomIdent::from_ident(ident, &["none"])?, false),
            Ok(Token::Function(name))
                if counter_type == CounterType::Reset && name.eq_ignore_ascii_case("reversed") =>
            {
                input
                    .parse_nested_block(|input| Ok((CustomIdent::parse(input, &["none"])?, true)))?
            },
            Ok(..) => {
                return Err(ParseError::unexpected_token());
            },
            Err(_) => break,
        };

        let value = match input.try_parse(|input| Integer::parse(context, input)) {
            Ok(start) => {
                if start.get() == Some(i32::MIN) {
                    // The spec says that values must be clamped to the valid range,
                    // and we reserve i32::MIN as an internal magic value.
                    // https://drafts.csswg.org/css-lists/#auto-numbering
                    Integer::new(i32::MIN + 1)
                } else {
                    start
                }
            },
            _ => Integer::new(if is_reversed {
                i32::MIN
            } else {
                counter_type.default_value()
            }),
        };
        counters.push(CounterPair {
            name,
            value,
            is_reversed,
        });
    }

    if !counters.is_empty() {
        Ok(counters)
    } else {
        Err(ParseError::custom(StyleParseErrorKind::UnspecifiedError))
    }
}

/// The specified value for the `content` property.
pub type Content = generics::GenericContent<Image>;

/// The specified value for a content item in the `content` property.
pub type ContentItem = generics::GenericContentItem<Image>;

impl Content {
    fn parse_counter_style(context: &ParserContext, input: &mut Parser) -> CounterStyle {
        use crate::counter_style::CounterStyleParsingFlags;
        input
            .try_parse(|input| {
                input.expect_comma()?;
                CounterStyle::parse(context, input, CounterStyleParsingFlags::empty())
            })
            .unwrap_or_else(|_| CounterStyle::decimal())
    }
}

impl Parse for Content {
    // normal | none | [ <string> | <counter> | open-quote | close-quote | no-open-quote |
    // no-close-quote ]+
    #[cfg_attr(feature = "servo", allow(unused_mut))]
    fn parse(context: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        if input
            .try_parse(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            return Ok(generics::Content::Normal);
        }
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            return Ok(generics::Content::None);
        }

        let mut items = thin_vec::ThinVec::new();
        let mut alt_start = None;
        loop {
            if alt_start.is_none() {
                if let Ok(image) = input.try_parse(|i| Image::parse_forbid_none(context, i)) {
                    items.push(generics::ContentItem::Image(image));
                    continue;
                }
            }
            let Ok(t) = input.next() else { break };
            match *t {
                Token::QuotedString(ref value) => {
                    items.push(generics::ContentItem::String(
                        value.as_ref().to_owned().into(),
                    ));
                },
                Token::Function(ref name) => {
                    // VIPER-PATCH(content-alt-counters): CSS Generated Content 3
                    // section 1.1 lets `<counter>` follow the `/` in the alternative
                    // text (https://drafts.csswg.org/css-content/#alt), which an
                    // accessible name reads; upstream refused it past the alt marker
                    // (its FIXME: no other engine took it then), so the whole
                    // declaration was invalid. Images and quotes stay refused there.
                    let result = match_ignore_ascii_case! { &name,
                        "counter" => input.parse_nested_block(|input| {
                            let name = CustomIdent::parse(input, &[])?;
                            let style = Content::parse_counter_style(context, input);
                            Ok(generics::ContentItem::Counter(name, style))
                        }),
                        "counters" => input.parse_nested_block(|input| {
                            let name = CustomIdent::parse(input, &[])?;
                            input.expect_comma()?;
                            let separator = input.expect_string()?.as_ref().to_owned().into();
                            let style = Content::parse_counter_style(context, input);
                            Ok(generics::ContentItem::Counters(name, separator, style))
                        }),
                        _ => {
                            use style_traits::StyleParseErrorKind;
                            return Err(ParseError::custom(
                                StyleParseErrorKind::UnexpectedFunction,
                            ))
                        }
                    }?;
                    items.push(result);
                },
                Token::Ident(ref ident) if alt_start.is_none() => {
                    items.push(match_ignore_ascii_case! { &ident,
                        "open-quote" => generics::ContentItem::OpenQuote,
                        "close-quote" => generics::ContentItem::CloseQuote,
                        "no-open-quote" => generics::ContentItem::NoOpenQuote,
                        "no-close-quote" => generics::ContentItem::NoCloseQuote,
                        #[cfg(feature = "gecko")]
                        "-moz-alt-content" if context.in_ua_sheet() => {
                            generics::ContentItem::MozAltContent
                        },
                        #[cfg(feature = "gecko")]
                        "-moz-label-content" if context.chrome_rules_enabled() => {
                            generics::ContentItem::MozLabelContent
                        },
                        _ =>{
                            return Err(ParseError::custom(
                                SelectorParseErrorKind::UnexpectedIdent
                            ));
                        }
                    });
                },
                Token::Delim('/') if alt_start.is_none() && !items.is_empty() => {
                    alt_start = Some(items.len());
                },
                _ => return Err(ParseError::unexpected_token()),
            }
        }
        if items.is_empty() {
            return Err(ParseError::custom(StyleParseErrorKind::UnspecifiedError));
        }
        let alt_start = alt_start.unwrap_or(items.len());
        Ok(generics::Content::Items(generics::GenericContentItems {
            items,
            alt_start,
        }))
    }
}

// VIPER-PATCH(content-alt-counters): the alternative text's grammar.
#[cfg(test)]
mod tests {
    use super::Content;
    use crate::context::QuirksMode;
    use crate::parser::{Parse, ParserContext};
    use crate::stylesheets::{CssRuleType, Origin, UrlExtraData};
    use crate::values::generics::counters::{GenericContent, GenericContentItem};
    use cssparser::Parser;
    use style_traits::{ParsingMode, ToCss};

    fn parsed(text: &str) -> Option<Content> {
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
        parser.parse_entirely(|i| Content::parse(&context, i)).ok()
    }

    #[test]
    fn content_alt_counters_parse_after_the_slash() {
        let css = |text: &str| parsed(text).map(|value| value.to_css_string());
        for (text, serialized) in [
            (r#""" / counter(cnt)"#, r#""" / counter(cnt)"#),
            (r#""a" / counter(cnt, upper-roman) " ""#, r#""a" / counter(cnt, upper-roman) " ""#),
            (r#""a" / counters(c, ".")"#, r#""a" / counters(c, ".")"#),
            (r#""a" / counters(c, ".", lower-alpha)"#, r#""a" / counters(c, ".", lower-alpha)"#),
            (r#""a" / COUNTER(cnt, decimal)"#, r#""a" / counter(cnt)"#),
        ] {
            assert_eq!(css(text).as_deref(), Some(serialized), "{text}");
        }
        let Some(GenericContent::Items(items)) = parsed(r#""a" / "b" counter(n)"#) else {
            panic!("an alternative with a counter parses");
        };
        assert_eq!(items.alt_start, 1);
        assert!(matches!(items.items[2], GenericContentItem::Counter(..)));
    }

    #[test]
    fn content_alt_counters_keep_the_other_refusals() {
        for text in [
            r#"/ counter(cnt)"#,
            r#""a" / counter()"#,
            r#""a" / counters(c)"#,
            r#""a" / counter(cnt) / "b""#,
            r#""a" / open-quote"#,
            r#""a" / url(x.png)"#,
            r#""a" / linear-gradient(red, blue)"#,
            r#""a" / foo(cnt)"#,
        ] {
            assert!(parsed(text).is_none(), "{text}");
        }
    }
}
