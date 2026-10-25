/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! Specified percentages.

use crate::derives::*;
use crate::parser::{Parse, ParserContext};
use crate::typed_om::{ToTyped, TypedValue};
use crate::values::computed::percentage::Percentage as ComputedPercentage;
use crate::values::computed::{Context, ToComputedValue};
use crate::values::generics::{NonNegative, Optional};
use crate::values::specified::calc::{CalcNode, CalcNumeric, CalcPercentageLeaf, Leaf};
use crate::values::specified::{CalcLengthPercentage, LengthPercentage, NoCalcNumber, Number};
use crate::values::tagged_numeric::{Extracted, NumericUnion, Unpacked, UnpackedMut};
use crate::values::{normalize, reify_percentage, serialize_percentage, CSSFloat};
use cssparser::{Parser, Token};
use std::fmt::{self, Write};
use style_traits::values::specified::AllowedNumericType;
use style_traits::{CssWriter, ParseError, SpecifiedValueInfo, ToCss};
use thin_vec::ThinVec;

/// A percentage value, where [0 .. 100%] maps to [0.0 .. 1.0]
// VIPER-PATCH(percentage-number-f64): the fraction is a `PercentageFraction`,
// `f64` in servo builds, so a percentage token keeps the number written.
#[derive(Clone, Copy, Debug, Default, MallocSizeOf, PartialEq, ToShmem)]
#[repr(C)]
pub struct NoCalcPercentage(PercentageFraction);

/// VIPER-PATCH(percentage-number-f64): the fraction a percentage keeps.
///
/// Upstream keeps `f32`, the percentage token's number divided by 100 and
/// rounded, so a number with more significant digits than `f32` holds
/// (`66.666667%`) cannot be read back from it. CSS Values 4 section 5.1.3
/// defines a `<percentage>` by the number written, and section 5 leaves a
/// number's precision to the implementation. Servo builds keep the
/// fraction in `f64` where a token's text gives it, always one that
/// rounds to upstream's `f32`, which every computation still uses
/// ([`NoCalcPercentage::get`]); the other builds keep upstream's `f32`,
/// which their C layout of the value needs.
#[cfg(feature = "servo")]
pub type PercentageFraction = f64;

/// The fraction a percentage keeps: upstream's `f32` outside servo builds.
#[cfg(not(feature = "servo"))]
pub type PercentageFraction = CSSFloat;

impl SpecifiedValueInfo for NoCalcPercentage {}

impl ToCss for NoCalcPercentage {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        // VIPER-PATCH(percentage-number-f64): the `f32` fraction.
        serialize_percentage(self.get(), dest)
    }
}

impl ToTyped for NoCalcPercentage {
    fn to_typed(&self, dest: &mut ThinVec<TypedValue>) -> Result<(), ()> {
        // VIPER-PATCH(percentage-number-f64): the `f32` fraction.
        reify_percentage(self.get(), dest)
    }
}

impl NoCalcPercentage {
    /// Creates a percentage from a numeric value.
    pub fn new(value: CSSFloat) -> Self {
        // VIPER-PATCH(percentage-number-f64): the `f32` fraction, widened.
        Self(PercentageFraction::from(value))
    }

    /// VIPER-PATCH(percentage-number-f64): the percentage token whose
    /// fraction is `value` (its `unit_value`) and whose source is `text`:
    /// in servo builds the fraction is `text`'s number divided by 100 in
    /// `f64` when that rounds to `value`, else `value`.
    pub fn from_token(value: CSSFloat, text: &str) -> Self {
        let written = text
            .trim()
            .strip_suffix('%')
            .and_then(|n| n.parse::<f64>().ok());
        match written.map(|number| number / 100.) {
            Some(fraction) if fraction as CSSFloat == value => Self(fraction as PercentageFraction),
            _ => Self::new(value),
        }
    }

    /// VIPER-PATCH(percentage-number-f64): the fraction as kept, which
    /// rounds to [`get`](Self::get).
    #[inline]
    pub fn fraction(&self) -> PercentageFraction {
        self.0
    }

    /// `0%`
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.)
    }

    /// `100%`
    #[inline]
    pub fn hundred() -> Self {
        Self::new(1.)
    }

    /// Gets the underlying value for this float.
    #[inline]
    pub fn get(&self) -> CSSFloat {
        // VIPER-PATCH(percentage-number-f64): the fraction rounded to `f32`.
        self.0 as CSSFloat
    }

    /// Return the unit, as a string.
    pub fn unit(&self) -> &'static str {
        "percent"
    }

    /// Return no canonical unit (percent values do not have one).
    pub fn canonical_unit(&self) -> Option<&'static str> {
        None
    }

    /// Convert only if the unit is the same (conversion to other units does
    /// not make sense).
    pub fn to(&self, unit: &str) -> Result<Self, ()> {
        if !unit.eq_ignore_ascii_case("percent") {
            return Err(());
        }
        Ok(*self)
    }
}

impl ToComputedValue for NoCalcPercentage {
    type ComputedValue = ComputedPercentage;

    fn to_computed_value(&self, _: &Context) -> Self::ComputedValue {
        ComputedPercentage(normalize(self.get()))
    }

    fn from_computed_value(computed: &Self::ComputedValue) -> Self {
        Self::new(computed.0)
    }
}

impl From<f32> for NoCalcPercentage {
    fn from(value: f32) -> Self {
        // VIPER-PATCH(percentage-number-f64): `new` widens the fraction.
        Self::new(value)
    }
}

impl From<NoCalcPercentage> for f32 {
    fn from(percentage: NoCalcPercentage) -> f32 {
        // VIPER-PATCH(percentage-number-f64): the `f32` fraction.
        percentage.get()
    }
}

/// A specified percentage value, either a plain value or a `calc()` expression.
#[derive(Clone, Debug, MallocSizeOf, PartialEq, ToShmem)]
pub struct Percentage(NumericUnion<(), f32, CalcNumeric>);

impl ToCss for Percentage {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        match self.0.unpack() {
            // VIPER-PATCH(percentage-number-f64): `new` widens the fraction.
            Unpacked::Inline((), p) => NoCalcPercentage::new(p).to_css(dest),
            Unpacked::Boxed(calc) => calc.to_css(dest),
        }
    }
}

impl ToTyped for Percentage {
    fn to_typed(&self, dest: &mut ThinVec<TypedValue>) -> Result<(), ()> {
        match self.0.unpack() {
            // VIPER-PATCH(percentage-number-f64): `new` widens the fraction.
            Unpacked::Inline((), p) => NoCalcPercentage::new(p).to_typed(dest),
            Unpacked::Boxed(calc) => calc.to_typed(dest),
        }
    }
}

impl Percentage {
    /// Creates a percentage from a numeric value.
    pub fn new(value: CSSFloat) -> Self {
        Self(NumericUnion::inline((), value))
    }

    /// Returns a new percentage calc value with the value `val`.
    #[inline]
    pub fn new_calc(val: Box<CalcNumeric>) -> Self {
        Self(NumericUnion::boxed(val))
    }

    /// `0%`
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.)
    }

    /// `100%`
    #[inline]
    pub fn hundred() -> Self {
        Self::new(1.)
    }

    /// Returns true if it is a calc percentage.
    #[inline]
    pub fn is_calc(&self) -> bool {
        self.0.is_boxed()
    }

    /// Returns the value if this is a plain (non-calc) percentage, or None otherwise.
    /// Use `resolve()` to also handle resolvable calc expressions, or `to_computed_value()`
    /// when computed context is available.
    #[inline]
    pub fn get(&self) -> Option<f32> {
        match self.0.unpack() {
            Unpacked::Inline((), f) => Some(f),
            Unpacked::Boxed(..) => None,
        }
    }

    /// Returns the value if it can be resolved at parse time, including resolvable calc
    /// expressions. Returns None for calc expressions that require computed context
    /// (e.g. those using relative lengths or sibling-index()).
    #[inline]
    pub fn resolve(&self) -> Option<CSSFloat> {
        match self.0.unpack() {
            Unpacked::Inline((), f) => Some(f),
            Unpacked::Boxed(calc) => calc.as_percentage().map(|p| p.get()),
        }
    }

    /// Returns this percentage as a number.
    pub fn to_number(&self) -> Option<Number> {
        Some(match self.0.unpack() {
            Unpacked::Inline((), p) => Number::new(p),
            Unpacked::Boxed(calc) => {
                let p = calc.as_percentage()?.get();
                Number::new_calc(Box::new(
                    calc.with_leaf_node(Leaf::Number(NoCalcNumber::new(p))),
                ))
            },
        })
    }

    /// Returns this percentage as a LengthPercentage.
    pub fn to_length_percentage(self) -> LengthPercentage {
        match self.0.extract() {
            // VIPER-PATCH(percentage-number-f64): `new` widens the fraction.
            Extracted::Inline((), p) => LengthPercentage::Percentage(NoCalcPercentage::new(p)),
            Extracted::Boxed(calc) => LengthPercentage::Calc(Box::new(CalcLengthPercentage(*calc))),
        }
    }

    /// Reverses this percentage, preserving calc-ness.
    ///
    /// For example: If it was 20%, convert it into 80%.
    pub fn reverse(&mut self) {
        match self.0.unpack_mut() {
            UnpackedMut::Inline(_, p) => {
                *p = 1. - *p;
            },
            UnpackedMut::Boxed(calc) => {
                let mut sum = smallvec::SmallVec::<[CalcNode; 2]>::new();
                sum.push(CalcNode::Leaf(Leaf::Percentage(CalcPercentageLeaf::new(
                    1.,
                    Optional::None,
                ))));
                let mut node = calc.node.clone();
                node.negate();
                sum.push(node);
                let mut diff = CalcNode::Sum(sum.into_boxed_slice().into());
                diff.simplify_and_sort();
                calc.node = diff;
            },
        }
    }

    /// Parses a specific kind of percentage.
    pub fn parse_with_clamping_mode(
        context: &ParserContext,
        input: &mut Parser,
        num_context: AllowedNumericType,
    ) -> Result<Self, ParseError> {
        Ok(Self(match *input.next()? {
            Token::Percentage { unit_value, .. }
                if num_context.is_ok(context.parsing_mode, unit_value) =>
            {
                NumericUnion::inline((), unit_value)
            },
            Token::Function(ref name) => {
                let function = CalcNode::math_function(context, name)?;
                let calc = CalcNode::parse_percentage(context, input, num_context, function)?;
                NumericUnion::boxed(Box::new(calc))
            },
            _ => return Err(ParseError::unexpected_token()),
        }))
    }

    /// Parses a percentage token, but rejects it if it's negative.
    pub fn parse_non_negative(
        context: &ParserContext,
        input: &mut Parser,
    ) -> Result<Self, ParseError> {
        Self::parse_with_clamping_mode(context, input, AllowedNumericType::NonNegative)
    }

    /// Parses a percentage token, but rejects it if it's negative or more than
    /// 100%.
    pub fn parse_zero_to_a_hundred(
        context: &ParserContext,
        input: &mut Parser,
    ) -> Result<Self, ParseError> {
        Self::parse_with_clamping_mode(context, input, AllowedNumericType::ZeroToOne)
    }

    /// Clamp to 100% if the value is over 100%.
    #[inline]
    pub fn clamp_to_hundred(&mut self) {
        match self.0.unpack_mut() {
            UnpackedMut::Inline((), p) => *p = p.min(1.),
            UnpackedMut::Boxed(calc) => {
                calc.clamping_mode = AllowedNumericType::ZeroToOne;
            },
        }
    }
}

impl Parse for Percentage {
    #[inline]
    fn parse(context: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        Self::parse_with_clamping_mode(context, input, AllowedNumericType::All)
    }
}

impl ToComputedValue for Percentage {
    type ComputedValue = ComputedPercentage;

    #[inline]
    fn to_computed_value(&self, context: &Context) -> Self::ComputedValue {
        match self.0.unpack() {
            // VIPER-PATCH(percentage-number-f64): `new` widens the fraction.
            Unpacked::Inline((), p) => NoCalcPercentage::new(p).to_computed_value(context),
            Unpacked::Boxed(calc) => {
                let value = calc.resolve(context, |result| match result {
                    Ok(Leaf::Percentage(p)) => p.get(),
                    _ => {
                        debug_assert!(
                            false,
                            "Unexpected Percentage::Calc without resolved percentage"
                        );
                        f32::NAN
                    },
                });
                ComputedPercentage(crate::values::normalize(value).min(f32::MAX).max(f32::MIN))
            },
        }
    }

    #[inline]
    fn from_computed_value(computed: &Self::ComputedValue) -> Self {
        Percentage::new(computed.0)
    }
}

impl SpecifiedValueInfo for Percentage {}

/// Turns the percentage into a plain float.
pub trait ToPercentage {
    /// Returns whether this percentage used to be a calc().
    fn is_calc(&self) -> bool {
        false
    }
    /// Returns the percentage as a plain float, or None for calc expressions that require
    /// computed context. Will always return Some if `is_calc` is false.
    fn to_percentage(&self) -> Option<CSSFloat>;
}

impl ToPercentage for Percentage {
    fn is_calc(&self) -> bool {
        self.0.is_boxed()
    }

    fn to_percentage(&self) -> Option<CSSFloat> {
        self.resolve()
    }
}

/// A wrapper of Percentage, whose value must be >= 0.
pub type NonNegativePercentage = NonNegative<Percentage>;

impl Parse for NonNegativePercentage {
    #[inline]
    fn parse(context: &ParserContext, input: &mut Parser) -> Result<Self, ParseError> {
        Ok(NonNegative(Percentage::parse_non_negative(context, input)?))
    }
}

impl NonNegativePercentage {
    /// Convert to ComputedPercentage, for FontFaceRule size-adjust getter.
    /// Returns None if the value is a calc expression that cannot be resolved at parse time.
    #[inline]
    pub fn compute(&self) -> Option<ComputedPercentage> {
        self.0
            .resolve()
            .map(|f| AllowedNumericType::NonNegative.clamp(f))
            .map(ComputedPercentage)
    }
}

// VIPER-PATCH(percentage-number-f64): the fork's tests of the patch.
#[cfg(all(test, feature = "servo"))]
mod percentage_number_f64_tests {
    use super::NoCalcPercentage;
    use crate::values::computed::{LengthPercentage, Percentage};
    use crate::values::generics::ClampToNonNegative;

    #[test]
    fn percentage_number_f64_keeps_the_number_written() {
        let token = |text: &str| {
            let number: f64 = text.trim().trim_end_matches('%').parse().unwrap();
            NoCalcPercentage::from_token((number / 100.) as f32, text)
        };
        let long = token("66.666667%");
        assert_eq!(long.fraction(), 66.666667 / 100.);
        assert_eq!(long.get(), 0.666_666_7);
        assert_eq!(token("33.333333%").fraction(), 33.333333 / 100.);
        assert_eq!(token(" +12.5e0% ").fraction(), 0.125);
        // A text that does not round to the token's value keeps the value.
        let other = NoCalcPercentage::from_token(0.5, "51%");
        assert_eq!(other.fraction(), 0.5);
        assert_eq!(NoCalcPercentage::from_token(0.5, "50px").fraction(), 0.5);
        // Other constructors widen the `f32` fraction.
        assert_eq!(NoCalcPercentage::new(0.15).fraction(), f64::from(0.15_f32));
    }

    #[test]
    fn percentage_number_f64_computes_the_number_written() {
        let fraction = 66.666667 / 100.;
        let value = LengthPercentage::new_percent_fraction(Percentage(0.666_666_7), fraction);
        assert_eq!(value.to_percentage_fraction(), Some(fraction));
        assert_eq!(value.to_percentage(), Some(Percentage(0.666_666_7)));
        // Copies, equality and a non-negative clamp keep the fraction.
        assert_eq!(value.clone().to_percentage_fraction(), Some(fraction));
        let narrow = LengthPercentage::new_percent(Percentage(0.666_666_7));
        assert_ne!(value, narrow);
        assert_eq!(value, value.clone());
        let clamped = value.clone().clamp_to_non_negative();
        assert_eq!(clamped.to_percentage_fraction(), Some(fraction));
        // A fraction that does not round to the percentage is not kept.
        let other = LengthPercentage::new_percent_fraction(Percentage(0.5), 0.51);
        assert_eq!(other.to_percentage_fraction(), Some(0.5));
        // Lengths and the `f32` percentages are upstream's.
        assert_eq!(
            narrow.to_percentage_fraction(),
            Some(f64::from(0.666_666_7_f32))
        );
        let length = LengthPercentage::new_length(crate::values::computed::Length::new(1.5));
        assert_eq!(length.to_length().map(|l| l.px()), Some(1.5));
        assert_eq!(length.to_percentage_fraction(), None);
    }
}
