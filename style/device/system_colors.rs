/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(system-color-palette): the values of CSS Color 4's system
//! colours (section 6.2) as an embedder-set palette of the servo `Device`,
//! one table per colour scheme. Upstream fixes them in
//! `Device::system_color`, so an embedder cannot give the device's own
//! colours. `SystemColorPalette::default()` is upstream's values, so a
//! device whose embedder sets no palette resolves every system colour as
//! before.

use crate::color::AbsoluteColor;
use crate::values::specified::color::SystemColor;

/// The values of CSS Color 4 section 6.2's nineteen system colours in one
/// colour scheme. A deprecated system colour (section 6.2.1) takes the
/// value of the colour that section maps it to ([`SystemColors::get`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemColors {
    /// `AccentColor`: the background of accented user interface controls.
    pub accent_color: AbsoluteColor,
    /// `AccentColorText`: text on `AccentColor`.
    pub accent_color_text: AbsoluteColor,
    /// `ActiveText`: the text of active links.
    pub active_text: AbsoluteColor,
    /// `ButtonBorder`: the base border colour of controls.
    pub button_border: AbsoluteColor,
    /// `ButtonFace`: the face background of push buttons.
    pub button_face: AbsoluteColor,
    /// `ButtonText`: the text of push buttons.
    pub button_text: AbsoluteColor,
    /// `Canvas`: the background of application content or documents.
    pub canvas: AbsoluteColor,
    /// `CanvasText`: text in application content or documents.
    pub canvas_text: AbsoluteColor,
    /// `Field`: the background of input fields.
    pub field: AbsoluteColor,
    /// `FieldText`: text in input fields.
    pub field_text: AbsoluteColor,
    /// `GrayText`: disabled text.
    pub gray_text: AbsoluteColor,
    /// `Highlight`: the background of selected text.
    pub highlight: AbsoluteColor,
    /// `HighlightText`: selected text.
    pub highlight_text: AbsoluteColor,
    /// `LinkText`: the text of non-active, non-visited links.
    pub link_text: AbsoluteColor,
    /// `Mark`: the background of text marked specially, such as by `mark`.
    pub mark: AbsoluteColor,
    /// `MarkText`: text marked specially.
    pub mark_text: AbsoluteColor,
    /// `SelectedItem`: the background of selected items, such as a checked
    /// checkbox.
    pub selected_item: AbsoluteColor,
    /// `SelectedItemText`: the text of selected items.
    pub selected_item_text: AbsoluteColor,
    /// `VisitedText`: the text of visited links.
    pub visited_text: AbsoluteColor,
}

impl SystemColors {
    /// The value of `color`; a deprecated system colour takes the value of
    /// the colour CSS Color 4 section 6.2.1 maps it to.
    pub fn get(&self, color: SystemColor) -> AbsoluteColor {
        use SystemColor as C;
        match color {
            C::Accentcolor => self.accent_color,
            C::Accentcolortext => self.accent_color_text,
            C::Activetext => self.active_text,
            C::Linktext => self.link_text,
            C::Visitedtext => self.visited_text,
            C::Buttonborder
            | C::Activeborder
            | C::Inactiveborder
            | C::Threeddarkshadow
            | C::Threedshadow
            | C::Windowframe => self.button_border,
            C::Buttonface
            | C::Buttonhighlight
            | C::Buttonshadow
            | C::Threedface
            | C::Threedhighlight
            | C::Threedlightshadow => self.button_face,
            C::Buttontext => self.button_text,
            C::Canvas
            | C::Activecaption
            | C::Appworkspace
            | C::Background
            | C::Inactivecaption
            | C::Infobackground
            | C::Menu
            | C::Scrollbar
            | C::Window => self.canvas,
            C::Canvastext | C::Captiontext | C::Infotext | C::Menutext | C::Windowtext => {
                self.canvas_text
            },
            C::Field => self.field,
            C::Fieldtext => self.field_text,
            C::Graytext | C::Inactivecaptiontext => self.gray_text,
            C::Highlight => self.highlight,
            C::Highlighttext => self.highlight_text,
            C::Mark => self.mark,
            C::Marktext => self.mark_text,
            C::Selecteditem => self.selected_item,
            C::Selecteditemtext => self.selected_item_text,
        }
    }
}

/// The system colours of the light and of the dark colour scheme: the
/// servo `Device` resolves a system colour through the table of the
/// element's used scheme. The default is upstream's values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemColorPalette {
    /// The light scheme's colours.
    pub light: SystemColors,
    /// The dark scheme's colours.
    pub dark: SystemColors,
}

/// An opaque sRGB colour in the legacy syntax.
fn srgb(red: u8, green: u8, blue: u8) -> AbsoluteColor {
    AbsoluteColor::srgb_legacy(red, green, blue, 1.0)
}

impl Default for SystemColorPalette {
    /// Upstream's values, which `Device::system_color` fixed before.
    fn default() -> Self {
        Self {
            light: SystemColors {
                accent_color: srgb(0, 102, 204),
                accent_color_text: srgb(255, 255, 255),
                active_text: srgb(238, 0, 0),
                button_border: srgb(169, 169, 169),
                button_face: srgb(220, 220, 220),
                button_text: srgb(0, 0, 0),
                canvas: srgb(255, 255, 255),
                canvas_text: srgb(0, 0, 0),
                field: srgb(255, 255, 255),
                field_text: srgb(0, 0, 0),
                gray_text: srgb(109, 109, 109),
                highlight: srgb(0, 65, 198),
                highlight_text: srgb(0, 0, 0),
                link_text: srgb(0, 0, 238),
                mark: srgb(255, 235, 59),
                mark_text: srgb(0, 0, 0),
                selected_item: srgb(0, 102, 204),
                selected_item_text: srgb(255, 255, 255),
                visited_text: srgb(85, 26, 139),
            },
            dark: SystemColors {
                accent_color: srgb(10, 132, 255),
                accent_color_text: srgb(255, 255, 255),
                active_text: srgb(255, 0, 0),
                button_border: srgb(255, 255, 255),
                button_face: srgb(107, 107, 107),
                button_text: srgb(245, 245, 245),
                canvas: srgb(30, 30, 30),
                canvas_text: srgb(232, 232, 232),
                field: srgb(45, 45, 45),
                field_text: srgb(240, 240, 240),
                gray_text: srgb(155, 155, 155),
                highlight: srgb(38, 79, 120),
                highlight_text: srgb(255, 255, 255),
                link_text: srgb(158, 158, 255),
                mark: srgb(102, 92, 0),
                mark_text: srgb(255, 255, 255),
                selected_item: srgb(153, 200, 255),
                selected_item_text: srgb(59, 59, 59),
                visited_text: srgb(208, 173, 240),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::QuirksMode;
    use crate::device::servo::FontMetricsProvider;
    use crate::device::Device;
    use crate::font_metrics::FontMetrics;
    use crate::media_queries::MediaType;
    use crate::properties::style_structs::Font;
    use crate::properties::ComputedValues;
    use crate::queries::values::PrefersColorScheme;
    use crate::servo::media_features::PointerCapabilities;
    use crate::values::computed::font::{GenericFontFamily, QueryFontMetricsFlags};
    use crate::values::computed::{CSSPixelLength, Length};
    use crate::values::specified::color::ColorSchemeFlags;
    use euclid::{Scale, Size2D};

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

    fn device() -> Device {
        Device::new(
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
        )
    }

    /// Every table entry a distinct colour: entry `i` of the light table is
    /// `rgb(i, 0, 0)`, of the dark table `rgb(0, i, 0)`.
    fn numbered(scheme: fn(u8) -> AbsoluteColor) -> SystemColors {
        SystemColors {
            accent_color: scheme(1),
            accent_color_text: scheme(2),
            active_text: scheme(3),
            button_border: scheme(4),
            button_face: scheme(5),
            button_text: scheme(6),
            canvas: scheme(7),
            canvas_text: scheme(8),
            field: scheme(9),
            field_text: scheme(10),
            gray_text: scheme(11),
            highlight: scheme(12),
            highlight_text: scheme(13),
            link_text: scheme(14),
            mark: scheme(15),
            mark_text: scheme(16),
            selected_item: scheme(17),
            selected_item_text: scheme(18),
            visited_text: scheme(19),
        }
    }

    #[test]
    fn system_color_palette_defaults_to_upstreams_values() {
        let device = device();
        assert_eq!(device.system_colors(), SystemColorPalette::default());
        let light = ColorSchemeFlags::LIGHT;
        let dark = ColorSchemeFlags::DARK;
        assert_eq!(device.system_color(SystemColor::Canvas, light), srgb(255, 255, 255));
        assert_eq!(device.system_color(SystemColor::Canvastext, dark), srgb(232, 232, 232));
        assert_eq!(device.system_color(SystemColor::Linktext, light), srgb(0, 0, 238));
        assert_eq!(device.system_color(SystemColor::Window, dark), srgb(30, 30, 30));
    }

    #[test]
    fn system_color_palette_is_the_embedders_per_scheme() {
        use SystemColor as C;
        let palette = SystemColorPalette {
            light: numbered(|i| srgb(i, 0, 0)),
            dark: numbered(|i| srgb(0, i, 0)),
        };
        let mut device = device();
        device.set_system_colors(palette);
        assert_eq!(device.system_colors(), palette);
        // Each of the nineteen, then each deprecated colour by the one it
        // maps to (CSS Color 4 section 6.2.1).
        let cases = [
            (C::Accentcolor, 1),
            (C::Accentcolortext, 2),
            (C::Activetext, 3),
            (C::Buttonborder, 4),
            (C::Buttonface, 5),
            (C::Buttontext, 6),
            (C::Canvas, 7),
            (C::Canvastext, 8),
            (C::Field, 9),
            (C::Fieldtext, 10),
            (C::Graytext, 11),
            (C::Highlight, 12),
            (C::Highlighttext, 13),
            (C::Linktext, 14),
            (C::Mark, 15),
            (C::Marktext, 16),
            (C::Selecteditem, 17),
            (C::Selecteditemtext, 18),
            (C::Visitedtext, 19),
            (C::Activeborder, 4),
            (C::Inactiveborder, 4),
            (C::Threeddarkshadow, 4),
            (C::Threedshadow, 4),
            (C::Windowframe, 4),
            (C::Buttonhighlight, 5),
            (C::Buttonshadow, 5),
            (C::Threedface, 5),
            (C::Threedhighlight, 5),
            (C::Threedlightshadow, 5),
            (C::Activecaption, 7),
            (C::Appworkspace, 7),
            (C::Background, 7),
            (C::Inactivecaption, 7),
            (C::Infobackground, 7),
            (C::Menu, 7),
            (C::Scrollbar, 7),
            (C::Window, 7),
            (C::Captiontext, 8),
            (C::Infotext, 8),
            (C::Menutext, 8),
            (C::Windowtext, 8),
            (C::Inactivecaptiontext, 11),
        ];
        for (color, i) in cases {
            let light = device.system_color(color, ColorSchemeFlags::LIGHT);
            let dark = device.system_color(color, ColorSchemeFlags::DARK);
            assert_eq!(light, srgb(i, 0, 0), "{color:?} light");
            assert_eq!(dark, srgb(0, i, 0), "{color:?} dark");
        }
    }
}
