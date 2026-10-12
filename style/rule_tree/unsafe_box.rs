/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(unsafe_code)]

use std::ops::Deref;
use std::ptr::NonNull;

/// An unsafe box, derefs to `T`.
//
// VIPER-PATCH(rule-tree-raw-box): the field was `ManuallyDrop<Box<T>>`. The
// rule tree keeps several copies of one box (`clone`) and stores node
// pointers taken from shared references (`&RuleNode as *const _ as *mut _`)
// in its free list, then frees them with `Box::from_raw`. Every copy of a
// `Box` asserts unique access, and a pointer cast from a shared reference
// may not be written or freed through, so both are undefined behaviour under
// Miri's Stacked Borrows and Tree Borrows. A raw pointer from
// `Box::into_raw` makes no uniqueness claim and keeps the provenance of the
// whole allocation; `as_mut_ptr` hands that pointer out for the free list.
// Size, niche and auto traits stay those of `Box<T>`.
pub(super) struct UnsafeBox<T> {
    inner: NonNull<T>,
}

// VIPER-PATCH(rule-tree-raw-box): `Box<T>` is `Send` and `Sync` exactly when
// `T` is; keep that for the raw-pointer field.
unsafe impl<T: Send> Send for UnsafeBox<T> {}
unsafe impl<T: Sync> Sync for UnsafeBox<T> {}

impl<T> UnsafeBox<T> {
    /// Creates a new unsafe box.
    pub(super) fn from_box(value: Box<T>) -> Self {
        // VIPER-PATCH(rule-tree-raw-box): keep the allocation as a raw
        // pointer; `Box::into_raw` never returns null.
        Self {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(value)) },
        }
    }

    /// Creates a new box from a pointer.
    ///
    /// # Safety
    ///
    /// The input should point to a valid `T`.
    pub(super) unsafe fn from_raw(ptr: *mut T) -> Self {
        // VIPER-PATCH(rule-tree-raw-box): no `Box` until the value is dropped.
        unsafe {
            Self {
                inner: NonNull::new_unchecked(ptr),
            }
        }
    }

    /// Creates a new unsafe box from an existing one.
    ///
    /// # Safety
    ///
    /// There is no refcounting or whatever else in an unsafe box, so this
    /// operation can lead to double frees.
    pub(super) unsafe fn clone(this: &Self) -> Self {
        // VIPER-PATCH(rule-tree-raw-box): copying the pointer claims nothing.
        Self { inner: this.inner }
    }

    /// Returns the raw pointer to the value, with the provenance of the
    /// whole allocation, so that `from_raw` may later free it.
    // VIPER-PATCH(rule-tree-raw-box): used for the free list's pointers.
    pub(super) fn as_mut_ptr(this: &Self) -> *mut T {
        this.inner.as_ptr()
    }

    /// Returns a mutable reference to the inner value of this unsafe box.
    ///
    /// # Safety
    ///
    /// Given `Self::clone`, nothing prevents anyone from creating
    /// multiple mutable references to the inner value, which is completely UB.
    pub(crate) unsafe fn deref_mut(this: &mut Self) -> &mut T {
        // VIPER-PATCH(rule-tree-raw-box): the caller guarantees no other
        // reference to the value is live (see above).
        unsafe { this.inner.as_mut() }
    }

    /// Drops the inner value of this unsafe box.
    ///
    /// # Safety
    ///
    /// Given this doesn't consume the unsafe box itself, this has the same
    /// safety caveats as `ManuallyDrop::drop`.
    pub(super) unsafe fn drop(this: &mut Self) {
        // VIPER-PATCH(rule-tree-raw-box): rebuild the `Box` only to drop it.
        unsafe { drop(Box::from_raw(this.inner.as_ptr())) }
    }
}

impl<T> Deref for UnsafeBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // VIPER-PATCH(rule-tree-raw-box): the value lives until `drop`, which
        // the rule tree calls only once no copy of the box is used again.
        unsafe { self.inner.as_ref() }
    }
}
