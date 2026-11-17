/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! VIPER-PATCH(pseudo-element-hooks): an embedder's pseudo-element hooks,
//! kept by the servo [`Device`](super::Device). CSS Pseudo-Elements 4 and
//! CSS Lists 3 name the properties that apply to `::placeholder`,
//! `::first-letter`, `::first-line` and `::marker`, and leave the user
//! agent's own styles of a pseudo-element to it. Upstream fixes each
//! pseudo-element's restriction in the engine (the `APPLIES_TO_*` flags)
//! and adjusts no pseudo-element after the cascade in servo builds, so an
//! embedder can neither widen a restriction nor set what its user agent
//! derives after the cascade (a marker's display from its list item's
//! `list-style-position`). The defaults are upstream's: the engine's sets
//! and no adjustment.

use crate::properties::{ComputedValues, LonghandIdSet, StyleBuilder};
use crate::selector_parser::PseudoElement;
use std::fmt::Debug;
use std::sync::Arc;

/// The element whose pseudo-element a [`PseudoElementAdjuster`] adjusts.
#[derive(Clone, Copy, Debug)]
pub struct OriginatingElement<'a> {
    /// The element's namespace.
    pub namespace: &'a web_atoms::Namespace,
    /// The element's local name.
    pub local_name: &'a web_atoms::LocalName,
}

/// An embedder's adjustment of a pseudo-element's computed style, run after
/// the cascade and the engine's own adjustments.
pub trait PseudoElementAdjuster: Debug + Send + Sync {
    /// Adjusts `style`, the computed style of `pseudo` of `element` (`None`
    /// when the style is computed without one), whose parent style is
    /// `style.inherited_style`; `author_specified` holds the longhands an
    /// author or user rule declared.
    fn adjust(
        &self,
        pseudo: &PseudoElement,
        element: Option<OriginatingElement>,
        style: &mut StyleBuilder,
        author_specified: &LonghandIdSet,
    );
}

/// The servo [`Device`](super::Device)'s pseudo-element hooks: the property
/// restrictions an embedder sets in place of the engine's, and its
/// post-cascade adjustment.
#[derive(Clone, Debug, Default)]
pub struct PseudoElementHooks {
    /// By pseudo-element index: the longhands that apply, or `None` for
    /// every longhand.
    restrictions: Vec<(usize, Option<LonghandIdSet>)>,
    adjuster: Option<Arc<dyn PseudoElementAdjuster>>,
}

impl PseudoElementHooks {
    /// Sets the longhands that apply to `pseudo` (and to every
    /// pseudo-element of its index) in author and user rules, `None` for
    /// every longhand, in place of the engine's set.
    pub fn set_restriction(&mut self, pseudo: &PseudoElement, longhands: Option<LonghandIdSet>) {
        let index = pseudo.index();
        self.restrictions.retain(|(i, _)| *i != index);
        self.restrictions.push((index, longhands));
    }

    /// The longhands that apply to `pseudo` in author and user rules, or
    /// `None` when every longhand does: the embedder's set, else the
    /// engine's (`PseudoElement::property_restriction`).
    pub fn restriction(&self, pseudo: &PseudoElement) -> Option<&LonghandIdSet> {
        let index = pseudo.index();
        match self.restrictions.iter().find(|(i, _)| *i == index) {
            Some((_, set)) => set.as_ref(),
            None => pseudo
                .property_restriction()
                .and_then(LonghandIdSet::with_restriction_flag),
        }
    }

    /// Sets the post-cascade adjustment, `None` for none (the default).
    pub fn set_adjuster(&mut self, adjuster: Option<Arc<dyn PseudoElementAdjuster>>) {
        self.adjuster = adjuster;
    }

    /// The post-cascade adjustment, if any.
    pub fn adjuster(&self) -> Option<&dyn PseudoElementAdjuster> {
        self.adjuster.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::{LonghandId, PropertyFlags};

    /// An adjuster that changes nothing.
    #[derive(Debug)]
    struct Unchanged;

    impl PseudoElementAdjuster for Unchanged {
        fn adjust(
            &self,
            _: &PseudoElement,
            _: Option<OriginatingElement>,
            _: &mut StyleBuilder,
            _: &LonghandIdSet,
        ) {
        }
    }

    #[test]
    fn pseudo_element_hooks_default_to_the_engines_sets() {
        let hooks = PseudoElementHooks::default();
        let marker = hooks.restriction(&PseudoElement::Marker).unwrap();
        assert_eq!(
            Some(marker),
            LonghandIdSet::with_restriction_flag(PropertyFlags::APPLIES_TO_MARKER)
        );
        assert!(marker.contains(LonghandId::Color) && marker.contains(LonghandId::FontSize));
        assert!(!marker.contains(LonghandId::LetterSpacing));
        assert!(!marker.contains(LonghandId::Display));
        let placeholder = hooks.restriction(&PseudoElement::Placeholder).unwrap();
        assert!(placeholder.contains(LonghandId::Color));
        assert!(!placeholder.contains(LonghandId::MarginLeft));
        assert!(hooks.restriction(&PseudoElement::Before).is_none());
        assert!(hooks.restriction(&PseudoElement::Backdrop).is_none());
        assert!(hooks.adjuster().is_none());
    }

    #[test]
    fn pseudo_element_hooks_take_the_embedders_sets() {
        let mut hooks = PseudoElementHooks::default();
        let mut marker = *hooks.restriction(&PseudoElement::Marker).unwrap();
        marker.insert(LonghandId::LetterSpacing);
        hooks.set_restriction(&PseudoElement::Marker, Some(marker));
        hooks.set_restriction(&PseudoElement::Placeholder, None);
        let mut before = LonghandIdSet::new();
        before.insert(LonghandId::Color);
        hooks.set_restriction(&PseudoElement::Before, Some(before));
        assert!(hooks
            .restriction(&PseudoElement::Marker)
            .unwrap()
            .contains(LonghandId::LetterSpacing));
        assert!(hooks.restriction(&PseudoElement::Placeholder).is_none());
        assert_eq!(hooks.restriction(&PseudoElement::Before), Some(&before));
        // A later set replaces the earlier one.
        hooks.set_restriction(&PseudoElement::Before, None);
        assert!(hooks.restriction(&PseudoElement::Before).is_none());
        assert!(hooks.restriction(&PseudoElement::After).is_none());
        hooks.set_adjuster(Some(Arc::new(Unchanged)));
        assert!(hooks.adjuster().is_some());
        hooks.set_adjuster(None);
        assert!(hooks.adjuster().is_none());
    }
}
