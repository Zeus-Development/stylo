/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(servo-remaining-media-features): the media features of
//! Media Queries 4 and 5 that upstream's servo list lacks (upstream's
//! gecko build reads most of them from its document), each evaluated from
//! the embedder's [`RemainingMediaFeatures`], which the servo `Device`
//! keeps. `device-aspect-ratio` reads the device's size, and
//! `ua-color-scheme` the device's color scheme unless the embedder sets
//! one. Since `ParseOptions::default()` leaves every one of them unknown
//! ([`REMAINING_MEDIA_FEATURES`]), the default options parse media queries
//! as upstream does; an embedder that evaluates them sets
//! `ParseOptions::unknown_media_features` to the features it leaves
//! unknown.
//!
//! In a boolean context each matches unless its value is `0`, `none` or a
//! value its specification makes false
//! (<https://drafts.csswg.org/mediaqueries-5/#mq-boolean-context>).

use crate::derives::*;
use crate::queries::values::PrefersColorScheme;
use crate::servo::media_features::{ColorGamut, DynamicRange};
use crate::values::computed::{Context, Ratio};

/// The features this module evaluates, which `ParseOptions::default()`
/// leaves unknown.
pub const REMAINING_MEDIA_FEATURES: &[&str] = &[
    "color",
    "color-index",
    "monochrome",
    "grid",
    "update",
    "overflow-block",
    "overflow-inline",
    "device-aspect-ratio",
    "horizontal-viewport-segments",
    "vertical-viewport-segments",
    "display-mode",
    "environment-blending",
    "inverted-colors",
    "nav-controls",
    "video-color-gamut",
    "video-dynamic-range",
    "prefers-reduced-transparency",
    "prefers-reduced-data",
    "prefers-bold-text",
    "ua-color-scheme",
];

/// An `update` value (Media Queries 5 section 5.4).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum Update {
    /// The layout cannot change once rendered, as on paper.
    None,
    /// The layout changes, but not fast enough for usual animation.
    Slow,
    /// The layout changes as fast as usual animation needs.
    Fast,
}

/// An `overflow-block` value (Media Queries 5 section 4.5).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum OverflowBlock {
    /// Content overflowing the block axis is not displayed.
    None,
    /// Content overflowing the block axis is reached by scrolling.
    Scroll,
    /// Content is broken into pages.
    Paged,
}

/// An `overflow-inline` value (Media Queries 5 section 4.6).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum OverflowInline {
    /// Content overflowing the inline axis is not displayed.
    None,
    /// Content overflowing the inline axis is reached by scrolling.
    Scroll,
}

/// A `display-mode` value (Media Queries 5 section 4.9).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum DisplayMode {
    /// A conventional browser tab or window.
    Browser,
    /// Like `standalone`, with a minimal set of navigation controls.
    MinimalUi,
    /// A window of its own, like a native application.
    Standalone,
    /// The whole display, with no user agent interface.
    Fullscreen,
    /// A floating, always-on-top window.
    PictureInPicture,
    /// An application context of the operating system's.
    Application,
}

/// An `environment-blending` value (Media Queries 5 section 5.5).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum EnvironmentBlending {
    /// An opaque medium, such as a monitor or paper.
    Opaque,
    /// Blended with the real world by additive mixing.
    Additive,
    /// Blended with the real world by subtractive mixing.
    Subtractive,
}

/// An `inverted-colors` value (Media Queries 5 section 6.6).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum InvertedColors {
    /// Colors are displayed normally.
    None,
    /// The display inverts every color.
    Inverted,
}

/// A `nav-controls` value (Media Queries 5 section 7.4).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum NavControls {
    /// The user agent shows no navigation controls.
    None,
    /// The user agent shows navigation controls, a back control among them.
    Back,
}

/// A `prefers-reduced-transparency` or `prefers-reduced-data` value (Media
/// Queries 5 sections 12.2 and 12.7).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum PrefersReduced {
    /// The user has expressed no preference; false in a boolean context.
    NoPreference,
    /// The user prefers less.
    Reduce,
}

/// A `prefers-bold-text` value (Media Queries 5 section 12.6).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum PrefersBoldText {
    /// The user has expressed no preference; false in a boolean context.
    NoPreference,
    /// The user prefers heavier text.
    Bold,
}

/// The display's, the user agent's and the user's settings behind the
/// features of [`REMAINING_MEDIA_FEATURES`] but `device-aspect-ratio`. The
/// default is a conventional browser on a color screen: 8 bits per color
/// component, no color lookup table, not monochrome, a bitmap that updates
/// fast and scrolls, one viewport segment each way, an opaque display, no
/// inverted colors, a back control, the standard video range in sRGB, no
/// preference, and the device's color scheme.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct RemainingMediaFeatures {
    /// `color`: bits per color component, 0 when not a color device.
    pub color: u32,
    /// `color-index`: entries of the color lookup table, 0 for none.
    pub color_index: u32,
    /// `monochrome`: bits per pixel of a monochrome device, else 0.
    pub monochrome: u32,
    /// `grid`: whether the device is a grid (character cell) device.
    pub grid: bool,
    /// `update`.
    pub update: Update,
    /// `overflow-block`.
    pub overflow_block: OverflowBlock,
    /// `overflow-inline`.
    pub overflow_inline: OverflowInline,
    /// `horizontal-viewport-segments`.
    pub horizontal_viewport_segments: u32,
    /// `vertical-viewport-segments`.
    pub vertical_viewport_segments: u32,
    /// `display-mode`.
    pub display_mode: DisplayMode,
    /// `environment-blending`.
    pub environment_blending: EnvironmentBlending,
    /// `inverted-colors`.
    pub inverted_colors: InvertedColors,
    /// `nav-controls`.
    pub nav_controls: NavControls,
    /// `video-color-gamut`; a plane matches every gamut up to its own.
    pub video_color_gamut: ColorGamut,
    /// `video-dynamic-range`; a plane matches every range up to its own.
    pub video_dynamic_range: DynamicRange,
    /// `prefers-reduced-transparency`.
    pub prefers_reduced_transparency: PrefersReduced,
    /// `prefers-reduced-data`.
    pub prefers_reduced_data: PrefersReduced,
    /// `prefers-bold-text`.
    pub prefers_bold_text: PrefersBoldText,
    /// `ua-color-scheme`, the user's own scheme; `None` for the device's
    /// (`prefers-color-scheme`'s).
    pub ua_color_scheme: Option<PrefersColorScheme>,
}

impl Default for RemainingMediaFeatures {
    fn default() -> Self {
        Self {
            color: 8,
            color_index: 0,
            monochrome: 0,
            grid: false,
            update: Update::Fast,
            overflow_block: OverflowBlock::Scroll,
            overflow_inline: OverflowInline::Scroll,
            horizontal_viewport_segments: 1,
            vertical_viewport_segments: 1,
            display_mode: DisplayMode::Browser,
            environment_blending: EnvironmentBlending::Opaque,
            inverted_colors: InvertedColors::None,
            nav_controls: NavControls::Back,
            video_color_gamut: ColorGamut::Srgb,
            video_dynamic_range: DynamicRange::Standard,
            prefers_reduced_transparency: PrefersReduced::NoPreference,
            prefers_reduced_data: PrefersReduced::NoPreference,
            prefers_bold_text: PrefersBoldText::NoPreference,
            ua_color_scheme: None,
        }
    }
}

/// The device's settings.
fn settings(context: &Context) -> RemainingMediaFeatures {
    context.device().remaining_media_features()
}

/// A keyword feature whose value is `value`: `query` matches that value;
/// in a boolean context the feature matches unless `value` is `falsy`.
fn keyword<T: PartialEq>(value: T, query: Option<T>, falsy: Option<T>) -> bool {
    match query {
        Some(query) => query == value,
        None => falsy != Some(value),
    }
}

/// An integer feature's value, at most `i32::MAX`.
fn integer(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

pub(super) fn eval_color(context: &Context) -> i32 {
    integer(settings(context).color)
}

pub(super) fn eval_color_index(context: &Context) -> i32 {
    integer(settings(context).color_index)
}

pub(super) fn eval_monochrome(context: &Context) -> i32 {
    integer(settings(context).monochrome)
}

pub(super) fn eval_grid(context: &Context) -> bool {
    settings(context).grid
}

pub(super) fn eval_update(context: &Context, query: Option<Update>) -> bool {
    keyword(settings(context).update, query, Some(Update::None))
}

pub(super) fn eval_overflow_block(context: &Context, query: Option<OverflowBlock>) -> bool {
    let value = settings(context).overflow_block;
    keyword(value, query, Some(OverflowBlock::None))
}

pub(super) fn eval_overflow_inline(context: &Context, query: Option<OverflowInline>) -> bool {
    let value = settings(context).overflow_inline;
    keyword(value, query, Some(OverflowInline::None))
}

/// `device-aspect-ratio` (Media Queries 5 appendix A): the device's width
/// to its height.
pub(super) fn eval_device_aspect_ratio(context: &Context) -> Ratio {
    let size = context.device().device_size();
    Ratio::new(size.width, size.height)
}

pub(super) fn eval_horizontal_viewport_segments(context: &Context) -> i32 {
    integer(settings(context).horizontal_viewport_segments)
}

pub(super) fn eval_vertical_viewport_segments(context: &Context) -> i32 {
    integer(settings(context).vertical_viewport_segments)
}

pub(super) fn eval_display_mode(context: &Context, query: Option<DisplayMode>) -> bool {
    keyword(settings(context).display_mode, query, None)
}

pub(super) fn eval_environment_blending(
    context: &Context,
    query: Option<EnvironmentBlending>,
) -> bool {
    keyword(settings(context).environment_blending, query, None)
}

pub(super) fn eval_inverted_colors(context: &Context, query: Option<InvertedColors>) -> bool {
    let value = settings(context).inverted_colors;
    keyword(value, query, Some(InvertedColors::None))
}

pub(super) fn eval_nav_controls(context: &Context, query: Option<NavControls>) -> bool {
    keyword(settings(context).nav_controls, query, Some(NavControls::None))
}

pub(super) fn eval_video_color_gamut(context: &Context, query: Option<ColorGamut>) -> bool {
    let gamut = settings(context).video_color_gamut;
    query.map_or(true, |v| v <= gamut)
}

pub(super) fn eval_video_dynamic_range(context: &Context, query: Option<DynamicRange>) -> bool {
    let range = settings(context).video_dynamic_range;
    query.map_or(true, |v| v <= range)
}

pub(super) fn eval_prefers_reduced_transparency(
    context: &Context,
    query: Option<PrefersReduced>,
) -> bool {
    let value = settings(context).prefers_reduced_transparency;
    keyword(value, query, Some(PrefersReduced::NoPreference))
}

pub(super) fn eval_prefers_reduced_data(context: &Context, query: Option<PrefersReduced>) -> bool {
    let value = settings(context).prefers_reduced_data;
    keyword(value, query, Some(PrefersReduced::NoPreference))
}

pub(super) fn eval_prefers_bold_text(context: &Context, query: Option<PrefersBoldText>) -> bool {
    let value = settings(context).prefers_bold_text;
    keyword(value, query, Some(PrefersBoldText::NoPreference))
}

pub(super) fn eval_ua_color_scheme(context: &Context, query: Option<PrefersColorScheme>) -> bool {
    let device = context.device();
    let value = settings(context)
        .ua_color_scheme
        .unwrap_or_else(|| device.color_scheme());
    keyword(value, query, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::QuirksMode;
    use crate::device::Device;
    use crate::media_queries::MediaList;
    use crate::parser::ParserContext;
    use crate::queries::condition::QueryCondition;
    use crate::servo::media_features::device_media_features_tests::device;
    use crate::servo::media_features::DeviceMediaFeatures;
    use crate::stylesheets::{CssRuleType, CustomMediaEvaluator, Origin, ParseOptions};
    use crate::stylesheets::UrlExtraData;
    use cssparser::Parser;
    use style_traits::ParsingMode;

    /// `query` parsed with `unknown` left unknown: whether it matches
    /// `device`, and whether its condition is a feature rather than a
    /// `<general-enclosed>`.
    fn eval(device: &Device, unknown: &'static [&'static str], query: &str) -> (bool, bool) {
        let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
        let mut context = ParserContext::new(
            Origin::Author,
            &url,
            Some(CssRuleType::Media),
            ParsingMode::DEFAULT,
            QuirksMode::NoQuirks,
            Default::default(),
            None,
            None,
            Default::default(),
        );
        let mut options = ParseOptions::default();
        options.unknown_media_features = unknown;
        context.parse_options = options;
        let list = MediaList::parse(&mut context, &mut Parser::new(query));
        let condition = &list.media_queries[0].condition;
        let feature = matches!(condition, Some(QueryCondition::Feature(_)));
        let mut custom = CustomMediaEvaluator::none();
        (list.evaluate(device, QuirksMode::NoQuirks, &mut custom), feature)
    }

    #[test]
    fn servo_remaining_media_features_evaluate() {
        let defaults = device(DeviceMediaFeatures::default());
        let mut set = device(DeviceMediaFeatures::default());
        set.set_remaining_media_features(RemainingMediaFeatures {
            color: 0,
            color_index: 256,
            monochrome: 8,
            grid: true,
            update: Update::Slow,
            overflow_block: OverflowBlock::Paged,
            overflow_inline: OverflowInline::None,
            horizontal_viewport_segments: 2,
            vertical_viewport_segments: 2,
            display_mode: DisplayMode::Standalone,
            environment_blending: EnvironmentBlending::Additive,
            inverted_colors: InvertedColors::Inverted,
            nav_controls: NavControls::None,
            video_color_gamut: ColorGamut::P3,
            video_dynamic_range: DynamicRange::High,
            prefers_reduced_transparency: PrefersReduced::Reduce,
            prefers_reduced_data: PrefersReduced::Reduce,
            prefers_bold_text: PrefersBoldText::Bold,
            ua_color_scheme: Some(PrefersColorScheme::Dark),
        });
        let cases = [
            // (query, under the defaults, under `set`)
            ("(color)", true, false),
            ("(color: 8)", true, false),
            ("(min-color: 1)", true, false),
            ("(color-index)", false, true),
            ("(color-index >= 256)", false, true),
            ("(monochrome)", false, true),
            ("(monochrome: 8)", false, true),
            ("(grid)", false, true),
            ("(grid: 0)", true, false),
            ("(update)", true, true),
            ("(update: fast)", true, false),
            ("(update: slow)", false, true),
            ("(overflow-block)", true, true),
            ("(overflow-block: scroll)", true, false),
            ("(overflow-block: paged)", false, true),
            ("(overflow-inline)", true, false),
            ("(overflow-inline: none)", false, true),
            ("(device-aspect-ratio: 4/3)", true, true),
            ("(min-device-aspect-ratio: 2/1)", false, false),
            ("(horizontal-viewport-segments: 1)", true, false),
            ("(horizontal-viewport-segments > 1)", false, true),
            ("(vertical-viewport-segments)", true, true),
            ("(vertical-viewport-segments: 2)", false, true),
            ("(display-mode)", true, true),
            ("(display-mode: browser)", true, false),
            ("(display-mode: standalone)", false, true),
            ("(environment-blending)", true, true),
            ("(environment-blending: opaque)", true, false),
            ("(environment-blending: additive)", false, true),
            ("(inverted-colors)", false, true),
            ("(inverted-colors: none)", true, false),
            ("(nav-controls)", true, false),
            ("(nav-controls: back)", true, false),
            ("(nav-controls: none)", false, true),
            ("(video-color-gamut)", true, true),
            ("(video-color-gamut: srgb)", true, true),
            ("(video-color-gamut: p3)", false, true),
            ("(video-dynamic-range: high)", false, true),
            ("(prefers-reduced-transparency)", false, true),
            ("(prefers-reduced-transparency: no-preference)", true, false),
            ("(prefers-reduced-data)", false, true),
            ("(prefers-reduced-data: reduce)", false, true),
            ("(prefers-bold-text)", false, true),
            ("(prefers-bold-text: bold)", false, true),
            ("(ua-color-scheme)", true, true),
            ("(ua-color-scheme: light)", true, false),
            ("(ua-color-scheme: dark)", false, true),
        ];
        for (query, by_default, when_set) in cases {
            assert_eq!(eval(&defaults, &[], query), (by_default, true), "{query}");
            assert_eq!(eval(&set, &[], query).0, when_set, "{query}");
        }
        // Not a value of the feature: a `<general-enclosed>`.
        for query in ["(display-mode: window)", "(grid: 2)", "(nav-controls: forward)"] {
            assert_eq!(eval(&defaults, &[], query), (false, false), "{query}");
        }
    }

    #[test]
    fn servo_remaining_media_features_default_to_unknown() {
        let defaults = device(DeviceMediaFeatures::default());
        assert_eq!(
            ParseOptions::default().unknown_media_features,
            REMAINING_MEDIA_FEATURES
        );
        for name in REMAINING_MEDIA_FEATURES {
            for query in [format!("({name})"), format!("not ({name})")] {
                let (matches, _) = eval(&defaults, REMAINING_MEDIA_FEATURES, &query);
                assert!(!matches, "{query}");
            }
            let (_, feature) = eval(&defaults, REMAINING_MEDIA_FEATURES, &format!("({name})"));
            assert!(!feature, "{name}");
            let (_, feature) = eval(&defaults, &[], &format!("({name})"));
            assert!(feature, "{name}");
        }
        // Any feature may be left unknown, compared ASCII case-insensitively,
        // with its prefixes; an unknown condition is unknown, not false.
        const WIDTH: &[&str] = &["WIDTH"];
        assert_eq!(eval(&defaults, WIDTH, "(min-width: 1px)"), (false, false));
        assert_eq!(eval(&defaults, &[], "(min-width: 1px)"), (true, true));
        assert!(eval(&defaults, WIDTH, "(scripting) or (width > 1px)").0);
        assert!(!eval(&defaults, WIDTH, "(scripting) and (width > 1px)").0);
    }
}
