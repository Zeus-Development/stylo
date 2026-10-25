/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Servo's media feature list and evaluator.

use crate::derives::*;
use crate::queries::feature::{AllowsRanges, Evaluator, FeatureFlags, QueryFeatureDescription};
use crate::queries::values::{Orientation, PrefersColorScheme};
use crate::values::computed::{CSSPixelLength, Context, Ratio, Resolution};
// VIPER-PATCH(servo-device-media-features): `forced-colors`' keyword.
use crate::values::specified::color::ForcedColors;
use std::fmt::Debug;

/// https://drafts.csswg.org/mediaqueries-4/#width
fn eval_width(context: &Context) -> CSSPixelLength {
    CSSPixelLength::new(context.device().au_viewport_size().width.to_f32_px())
}

/// https://drafts.csswg.org/mediaqueries-4/#height
fn eval_height(context: &Context) -> CSSPixelLength {
    CSSPixelLength::new(context.device().au_viewport_size().height.to_f32_px())
}

/// https://drafts.csswg.org/mediaqueries-4/#device-width
fn eval_device_width(context: &Context) -> CSSPixelLength {
    let device = context.device();
    let scaled = device.device_size() / device.device_pixel_ratio();
    CSSPixelLength::new(scaled.width)
}

/// https://drafts.csswg.org/mediaqueries-4/#device-height
fn eval_device_height(context: &Context) -> CSSPixelLength {
    let device = context.device();
    let scaled = device.device_size() / device.device_pixel_ratio();
    CSSPixelLength::new(scaled.height)
}

/// https://drafts.csswg.org/mediaqueries-4/#orientation
fn eval_orientation(context: &Context, value: Option<Orientation>) -> bool {
    Orientation::eval(context.device().au_viewport_size(), value)
}

#[derive(Clone, Copy, Debug, FromPrimitive, Parse, ToCss)]
#[repr(u8)]
enum Scan {
    Progressive,
    Interlace,
}

/// https://drafts.csswg.org/mediaqueries-4/#scan
fn eval_scan(_: &Context, _: Option<Scan>) -> bool {
    // Since we doesn't support the 'tv' media type, the 'scan' feature never
    // matches.
    false
}

/// https://drafts.csswg.org/mediaqueries-4/#resolution
fn eval_resolution(context: &Context) -> Resolution {
    Resolution::from_dppx(context.device().device_pixel_ratio().0)
}

/// https://compat.spec.whatwg.org/#css-media-queries-webkit-device-pixel-ratio
fn eval_device_pixel_ratio(context: &Context) -> f32 {
    eval_resolution(context).dppx()
}

fn eval_prefers_color_scheme(context: &Context, query_value: Option<PrefersColorScheme>) -> bool {
    match query_value {
        Some(v) => context.device().color_scheme() == v,
        None => true,
    }
}

bitflags! {
    /// https://drafts.csswg.org/mediaqueries-4/#mf-interaction
    #[derive(Debug, Clone, Copy)]
    pub struct PointerCapabilities: u8 {
        /// The input mechanism includes a pointing device of limited accuracy, such as a finger on a touchscreen.
        const COARSE = 0b001;
        /// The input mechanism includes an accurate pointing device, such as a mouse.
        const FINE = 0b010;
        /// The input mechanism can conveniently hover over elements.
        const HOVER = 0b100;
    }
}

impl Default for PointerCapabilities {
    #[cfg(any(target_os = "ios", target_os = "android", target_env = "ohos"))]
    fn default() -> Self {
        PointerCapabilities::COARSE
    }
    #[cfg(not(any(target_os = "ios", target_os = "android", target_env = "ohos")))]
    fn default() -> Self {
        PointerCapabilities::FINE | PointerCapabilities::HOVER
    }
}

#[derive(Clone, Copy, Debug, FromPrimitive, Parse, ToCss)]
#[repr(u8)]
enum Pointer {
    None,
    Coarse,
    Fine,
}

fn eval_pointer_capabilities(
    query_value: Option<Pointer>,
    pointer_capabilities: PointerCapabilities,
) -> bool {
    match query_value {
        None => !pointer_capabilities.is_empty(),
        Some(Pointer::None) => pointer_capabilities.is_empty(),
        Some(Pointer::Coarse) => pointer_capabilities.intersects(PointerCapabilities::COARSE),
        Some(Pointer::Fine) => pointer_capabilities.intersects(PointerCapabilities::FINE),
    }
}

/// https://drafts.csswg.org/mediaqueries-4/#pointer
fn eval_pointer(context: &Context, query_value: Option<Pointer>) -> bool {
    eval_pointer_capabilities(query_value, context.device().primary_pointer_capabilities())
}

/// https://drafts.csswg.org/mediaqueries-4/#descdef-media-any-pointer
fn eval_any_pointer(context: &Context, query_value: Option<Pointer>) -> bool {
    eval_pointer_capabilities(query_value, context.device().all_pointer_capabilities())
}

#[derive(Clone, Copy, Debug, FromPrimitive, Parse, ToCss)]
#[repr(u8)]
enum Hover {
    None,
    Hover,
}

fn eval_hover_capabilities(
    query_value: Option<Hover>,
    pointer_capabilities: PointerCapabilities,
) -> bool {
    let can_hover = pointer_capabilities.intersects(PointerCapabilities::HOVER);
    match query_value {
        Some(Hover::None) => !can_hover,
        Some(Hover::Hover) => can_hover,
        None => return can_hover,
    }
}

/// https://drafts.csswg.org/mediaqueries-4/#hover
fn eval_hover(context: &Context, query_value: Option<Hover>) -> bool {
    eval_hover_capabilities(query_value, context.device().primary_pointer_capabilities())
}

/// https://drafts.csswg.org/mediaqueries-4/#descdef-media-any-hover
fn eval_any_hover(context: &Context, query_value: Option<Hover>) -> bool {
    eval_hover_capabilities(query_value, context.device().all_pointer_capabilities())
}

/// <https://drafts.csswg.org/mediaqueries-4/#aspect-ratio>
fn eval_aspect_ratio(context: &Context) -> Ratio {
    let size = context.device().au_viewport_size();
    Ratio::new(size.width.0 as f32, size.height.0 as f32)
}

// VIPER-PATCH(servo-scripting-media-feature): the `scripting` media feature
// of Media Queries 5 section 9.1, which upstream's servo list lacks (Gecko's
// evaluator asks its document). It always answers `enabled`, because the
// embedder runs scripts; a device flag for documents whose scripts are off
// can follow (Viper's S3-SCRIPTING-OFF).
/// https://drafts.csswg.org/mediaqueries-5/#scripting
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
enum Scripting {
    None,
    InitialOnly,
    Enabled,
}

fn eval_scripting(_: &Context, query_value: Option<Scripting>) -> bool {
    query_value.map_or(true, |v| v == Scripting::Enabled)
}

// VIPER-PATCH(servo-device-media-features): the color and user-preference
// features of Media Queries 5 sections 6.4 (`color-gamut`), 6.5
// (`dynamic-range`), 11.1 (`prefers-reduced-motion`), 11.3
// (`prefers-contrast`) and 11.4 (`forced-colors`), which upstream's servo
// list lacks (Gecko's evaluators ask its document). They read the
// embedder's values from the `Device` (`DeviceMediaFeatures`). In a
// boolean context (https://drafts.csswg.org/mediaqueries-5/#mq-boolean-context)
// each matches unless its value is one the specification makes false:
// every gamut and range matches, and `no-preference` and `none` do not.
/// A `color-gamut` value (Media Queries 5 section 6.4), ordered from the
/// narrowest gamut: a display matches every gamut up to its own.
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, PartialOrd, ToCss)]
#[repr(u8)]
pub enum ColorGamut {
    /// About the sRGB gamut or more.
    Srgb,
    /// About the gamut of Display P3 or more.
    P3,
    /// About the gamut of ITU-R BT.2020 or more.
    Rec2020,
}

/// A `dynamic-range` value (Media Queries 5 section 6.5), ordered from the
/// lower range: a display matches every range up to its own.
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, PartialOrd, ToCss)]
#[repr(u8)]
pub enum DynamicRange {
    /// Any visual output device.
    Standard,
    /// High peak brightness, contrast ratio and color depth.
    High,
}

/// A `prefers-reduced-motion` value (Media Queries 5 section 11.1).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum PrefersReducedMotion {
    /// The user has expressed no preference.
    NoPreference,
    /// The user prefers less motion.
    Reduce,
}

/// A `prefers-contrast` value (Media Queries 5 section 11.3).
#[derive(Clone, Copy, Debug, FromPrimitive, Parse, PartialEq, ToCss)]
#[repr(u8)]
pub enum PrefersContrast {
    /// The user has expressed no preference.
    NoPreference,
    /// The user prefers more contrast.
    More,
    /// The user prefers less contrast.
    Less,
    /// A palette of the user's that is neither more nor less contrast.
    Custom,
}

/// The display's and the user's settings behind `color-gamut`,
/// `dynamic-range`, `prefers-reduced-motion`, `prefers-contrast` and
/// `forced-colors`. The default is the narrowest gamut, the standard range
/// and no preference or forced colors; an embedder sets its device's.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceMediaFeatures {
    /// `color-gamut`.
    pub color_gamut: ColorGamut,
    /// `dynamic-range`.
    pub dynamic_range: DynamicRange,
    /// `prefers-reduced-motion`.
    pub prefers_reduced_motion: PrefersReducedMotion,
    /// `prefers-contrast`.
    pub prefers_contrast: PrefersContrast,
    /// `forced-colors`; `active` also ignores author colors in the cascade
    /// (CSS Color Adjustment 1 section 3), as `Device::forced_colors` says.
    pub forced_colors: ForcedColors,
}

impl Default for DeviceMediaFeatures {
    fn default() -> Self {
        Self {
            color_gamut: ColorGamut::Srgb,
            dynamic_range: DynamicRange::Standard,
            prefers_reduced_motion: PrefersReducedMotion::NoPreference,
            prefers_contrast: PrefersContrast::NoPreference,
            forced_colors: ForcedColors::None,
        }
    }
}

fn eval_color_gamut(context: &Context, query_value: Option<ColorGamut>) -> bool {
    let gamut = context.device().device_media_features().color_gamut;
    query_value.map_or(true, |v| v <= gamut)
}

fn eval_dynamic_range(context: &Context, query_value: Option<DynamicRange>) -> bool {
    let range = context.device().device_media_features().dynamic_range;
    query_value.map_or(true, |v| v <= range)
}

fn eval_prefers_reduced_motion(context: &Context, query: Option<PrefersReducedMotion>) -> bool {
    let value = context
        .device()
        .device_media_features()
        .prefers_reduced_motion;
    query.map_or(value != PrefersReducedMotion::NoPreference, |v| v == value)
}

fn eval_prefers_contrast(context: &Context, query: Option<PrefersContrast>) -> bool {
    let value = context.device().device_media_features().prefers_contrast;
    query.map_or(value != PrefersContrast::NoPreference, |v| v == value)
}

fn eval_forced_colors(context: &Context, query: Option<ForcedColors>) -> bool {
    let value = context.device().forced_colors();
    query.map_or(value != ForcedColors::None, |v| v == value)
}

// VIPER-PATCH(servo-device-media-features): five more entries (below).
/// A list with all the media features that Servo supports.
pub static MEDIA_FEATURES: [QueryFeatureDescription; 21] = [
    feature!(
        atom!("width"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_width),
        FeatureFlags::VIEWPORT_DEPENDENT,
    ),
    feature!(
        atom!("height"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_height),
        FeatureFlags::VIEWPORT_DEPENDENT,
    ),
    feature!(
        atom!("orientation"),
        AllowsRanges::No,
        keyword_evaluator!(eval_orientation, Orientation),
        FeatureFlags::VIEWPORT_DEPENDENT,
    ),
    feature!(
        atom!("pointer"),
        AllowsRanges::No,
        keyword_evaluator!(eval_pointer, Pointer),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("any-pointer"),
        AllowsRanges::No,
        keyword_evaluator!(eval_any_pointer, Pointer),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("hover"),
        AllowsRanges::No,
        keyword_evaluator!(eval_hover, Hover),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("any-hover"),
        AllowsRanges::No,
        keyword_evaluator!(eval_any_hover, Hover),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("aspect-ratio"),
        AllowsRanges::Yes,
        Evaluator::NumberRatio(eval_aspect_ratio),
        FeatureFlags::VIEWPORT_DEPENDENT,
    ),
    feature!(
        atom!("device-width"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_device_width),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("device-height"),
        AllowsRanges::Yes,
        Evaluator::Length(eval_device_height),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("scan"),
        AllowsRanges::No,
        keyword_evaluator!(eval_scan, Scan),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("resolution"),
        AllowsRanges::Yes,
        Evaluator::Resolution(eval_resolution),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("device-pixel-ratio"),
        AllowsRanges::Yes,
        Evaluator::Float(eval_device_pixel_ratio),
        FeatureFlags::WEBKIT_PREFIX,
    ),
    feature!(
        atom!("-moz-device-pixel-ratio"),
        AllowsRanges::Yes,
        Evaluator::Float(eval_device_pixel_ratio),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-color-scheme"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_color_scheme, PrefersColorScheme),
        FeatureFlags::empty(),
    ),
    // VIPER-PATCH(servo-scripting-media-feature): `scripting` (above).
    feature!(
        atom!("scripting"),
        AllowsRanges::No,
        keyword_evaluator!(eval_scripting, Scripting),
        FeatureFlags::empty(),
    ),
    // VIPER-PATCH(servo-device-media-features): the five features (above),
    // with their atoms.
    feature!(
        atom!("color-gamut"),
        AllowsRanges::No,
        keyword_evaluator!(eval_color_gamut, ColorGamut),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("dynamic-range"),
        AllowsRanges::No,
        keyword_evaluator!(eval_dynamic_range, DynamicRange),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-reduced-motion"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_reduced_motion, PrefersReducedMotion),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("prefers-contrast"),
        AllowsRanges::No,
        keyword_evaluator!(eval_prefers_contrast, PrefersContrast),
        FeatureFlags::empty(),
    ),
    feature!(
        atom!("forced-colors"),
        AllowsRanges::No,
        keyword_evaluator!(eval_forced_colors, ForcedColors),
        FeatureFlags::empty(),
    ),
];

// VIPER-PATCH(servo-scripting-media-feature): the feature parses.
#[cfg(test)]
mod scripting_tests {
    use crate::context::QuirksMode;
    use crate::media_queries::MediaList;
    use crate::parser::ParserContext;
    use crate::queries::condition::QueryCondition;
    use crate::stylesheets::{CssRuleType, Origin, UrlExtraData};
    use cssparser::Parser;
    use style_traits::ParsingMode;

    #[test]
    fn servo_scripting_media_feature_parses() {
        let url = UrlExtraData::from(::url::Url::parse("about:blank").unwrap());
        let feature = |query: &str| {
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
            let list = MediaList::parse(&mut context, &mut Parser::new(query));
            matches!(
                list.media_queries[0].condition,
                Some(QueryCondition::Feature(_))
            )
        };
        for query in [
            "(scripting)",
            "(scripting: none)",
            "(scripting: initial-only)",
            "(scripting: enabled)",
        ] {
            assert!(feature(query), "{query}");
        }
        assert!(!feature("(scripting: off)"));
    }
}

// VIPER-PATCH(servo-device-media-features): each feature evaluates the
// device's setting by Media Queries 5's rules, in a boolean context too.
#[cfg(test)]
mod device_media_features_tests {
    use super::*;
    use crate::context::QuirksMode;
    use crate::device::servo::FontMetricsProvider;
    use crate::device::Device;
    use crate::font_metrics::FontMetrics;
    use crate::media_queries::{MediaList, MediaType};
    use crate::parser::ParserContext;
    use crate::properties::style_structs::Font;
    use crate::properties::ComputedValues;
    use crate::queries::values::PrefersColorScheme;
    use crate::stylesheets::{CssRuleType, CustomMediaEvaluator, Origin, UrlExtraData};
    use crate::values::computed::font::GenericFontFamily;
    use crate::values::computed::Length;
    use crate::values::specified::font::QueryFontMetricsFlags;
    use cssparser::Parser;
    use euclid::{Scale, Size2D};
    use style_traits::ParsingMode;

    #[derive(Debug)]
    struct NoMetrics;

    impl FontMetricsProvider for NoMetrics {
        fn query_font_metrics(
            &self,
            _: bool,
            _: &Font,
            _: CSSPixelLength,
            _: QueryFontMetricsFlags,
        ) -> FontMetrics {
            FontMetrics::default()
        }

        fn base_size_for_generic(&self, _: GenericFontFamily) -> Length {
            Length::new(16.)
        }
    }

    fn device(features: DeviceMediaFeatures) -> Device {
        let mut device = Device::new(
            MediaType::screen(),
            QuirksMode::NoQuirks,
            Size2D::new(800., 600.),
            Size2D::new(800., 600.),
            Scale::new(1.),
            Box::new(NoMetrics),
            ComputedValues::initial_values_with_font_override(Font::initial_values()),
            PrefersColorScheme::Light,
            PointerCapabilities::default(),
            PointerCapabilities::default(),
        );
        device.set_device_media_features(features);
        device
    }

    fn matches(device: &Device, query: &str) -> bool {
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
        let list = MediaList::parse(&mut context, &mut Parser::new(query));
        let mut custom = CustomMediaEvaluator::none();
        list.evaluate(device, QuirksMode::NoQuirks, &mut custom)
    }

    #[test]
    fn servo_device_media_features_evaluate() {
        let defaults = device(DeviceMediaFeatures::default());
        let set = device(DeviceMediaFeatures {
            color_gamut: ColorGamut::P3,
            dynamic_range: DynamicRange::High,
            prefers_reduced_motion: PrefersReducedMotion::Reduce,
            prefers_contrast: PrefersContrast::Custom,
            forced_colors: ForcedColors::Active,
        });
        let cases = [
            // (query, under the defaults, under `set`)
            ("(color-gamut)", true, true),
            ("(color-gamut: srgb)", true, true),
            ("(color-gamut: p3)", false, true),
            ("(color-gamut: rec2020)", false, false),
            ("(dynamic-range)", true, true),
            ("(dynamic-range: standard)", true, true),
            ("(dynamic-range: high)", false, true),
            ("(prefers-reduced-motion)", false, true),
            ("(prefers-reduced-motion: no-preference)", true, false),
            ("(prefers-reduced-motion: reduce)", false, true),
            ("(prefers-contrast)", false, true),
            ("(prefers-contrast: no-preference)", true, false),
            ("(prefers-contrast: more)", false, false),
            ("(prefers-contrast: less)", false, false),
            ("(prefers-contrast: custom)", false, true),
            ("(forced-colors)", false, true),
            ("(forced-colors: none)", true, false),
            ("(forced-colors: active)", false, true),
            // Not a keyword of the feature, or one for privileged sheets alone.
            ("(prefers-contrast: high)", false, false),
            ("(forced-colors: requested)", false, false),
        ];
        for (query, by_default, when_set) in cases {
            assert_eq!(matches(&defaults, query), by_default, "{query}");
            assert_eq!(matches(&set, query), when_set, "{query}");
        }
        assert!(!defaults.forced_colors().is_active());
        assert!(set.forced_colors().is_active());
    }
}
