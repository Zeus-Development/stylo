# Viper patches to servo_arc

This branch carries a private patch to `servo_arc` for the Viper project. It
is **never submitted upstream**: no issue, pull request or comment based on it
goes to `servo/stylo` or any other Servo repository. It lives only in the
private fork and is consumed by Viper through a Cargo `[patch]` entry.

Every changed spot in `lib.rs` carries a `// VIPER-PATCH(servo-arc-provenance)`
comment.

## Base

The patch applies to the exact sources of the published crate
`servo_arc 0.5.0`, which Viper uses through `selectors 0.41.0` and Stylo 0.22.

- Crate: `https://static.crates.io/crates/servo_arc/servo_arc-0.5.0.crate`,
  SHA-256 `b4eceb519ec34d8290aaf6e46f435102f2bfa63f22b05a144fb16ab266d3b198`
  (the same checksum as the `servo_arc` entry in Viper's `Cargo.lock`).
- Its `.cargo_vcs_info.json` names git commit
  `34d25f741765a0bdb894f8271852e633299352d8` ("Bump stylo to 0.22.0, and
  more (#473)", tag `v0.22.0`) with `path_in_vcs = "servo_arc"`.
- Comparison of the unpacked crate with that commit:
  - `lib.rs` is byte-identical (SHA-256
    `a5ac5a0f7ba85747f9898ace506413fb41de23270951fb738fae44755931935d`);
  - `Cargo.toml.orig` is identical to `servo_arc/Cargo.toml`;
  - `LICENSE-APACHE` and `LICENSE-MIT` are identical;
  - `README.md` is the repository root `README.md` (the manifest says
    `readme = "../README.md"`);
  - `Cargo.toml` (normalized by `cargo package`) and `Cargo.lock` are
    generated at packaging time and have no counterpart in git.

The branch starts at that commit. Its first commit adds the regression tests
only; the second commit holds the fix and this file.

## Problem

`servo_arc` turns pointers to the data of an `Arc` back into the `Arc` and
then reads or writes the refcount in front of the data, and it writes the
trailing items of a thin arc through a pointer taken from a reference that
ends before them. Miri reports these as undefined behaviour:

- [servo/stylo#210](https://github.com/servo/stylo/issues/210): building a
  `ThinArc` (`Arc::from_header_and_iter_alloc`) writes the items through a
  pointer derived from `p.as_mut()`, a reference to the sized `ArcInner`
  whose zero-length `data` array ends where the items begin. Stacked Borrows
  rejects the write; Tree Borrows accepts it.
- [servo/stylo#299](https://github.com/servo/stylo/issues/299): `ArcBorrow`
  stored a `&T`, `ArcUnion::borrow` built it from `&(*ptr).data`, and
  `Arc::into_raw` went through `&data`. A reference only grants access to the
  `T` it points to, so the `Arc` rebuilt by `ArcBorrow::clone_arc`,
  `ArcBorrow::with_arc`, `ArcUnion`'s `Clone` and `Drop` or `Arc::from_raw`
  updates the refcount without permission. Both models reject it (Tree
  Borrows: "write access through a Frozen tag"). `ArcUnion` also set and
  cleared its tag bit through an integer round trip, which drops provenance;
  Miri warns that Tree Borrows does not support it.

## Patch spots

The fix follows `std::sync::Arc`: pointers into the allocation are projected
from the raw allocation pointer (`ptr::addr_of!((*p).field)`, `map_addr`)
instead of passing through a reference to a field, and the refcount is
reached only through pointers that keep the provenance of the whole
allocation. No public signature or behaviour changes.

| Spot | Change |
| --- | --- |
| `Arc::into_raw` | `ptr::addr_of!((*this.ptr()).data)` instead of `&data as *const _`. |
| `Arc::borrow_arc` | Builds the `ArcBorrow` from the raw allocation pointer instead of `&**self`. |
| `Arc::from_header_and_iter_alloc` | Item pointer `addr_of_mut!((*p.as_ptr()).data.data)` instead of going through `p.as_mut()`. |
| `ArcBorrow` (struct) | Private field `NonNull<T>` plus `PhantomData<&'a T>` instead of `&'a T`. |
| `ArcBorrow` traits | Hand-written `Debug`, `PartialEq`, `Eq` with the former derives' output and semantics (they print and compare the value); `Send`/`Sync` exactly when `T: Sync`, as for `&'a T`. |
| `ArcBorrow::from_ref`, `get`, `Deref`, `ptr_eq` | Adapted to the pointer field; `from_ref` keeps the caller's provenance. |
| `ArcBorrow::from_ptr` | New private constructor from a raw pointer with whole-allocation provenance. |
| `ArcBorrow::clone_arc`, `with_arc` | `Arc::from_raw` from the stored pointer. |
| `ArcUnion::borrow` | Projects `data` through the raw pointer and clears the tag bit with `map_addr`. |
| `ArcUnion::from_second` | Sets the tag bit with `map_addr`. |
| `ArcUnion` `Drop` | `Arc::from_raw` from the borrow's pointer instead of `&*x`. |

`ArcBorrow` keeps its size and niche (`Option<ArcBorrow<_>>` is one word),
its variance and its auto traits; the test `arc_borrow_traits_unchanged`
checks the visible parts.

## Known limit: Stacked Borrows and `HeaderSlice`

`HeaderSlice<H, T>` is a sized struct whose items follow a zero-length array.
`HeaderSlice::slice(&self)`, `slice_mut(&mut self)`, `UniqueArc::data_mut`
and `impl Drop for HeaderSlice` reach the items through a reference to that
sized struct. Stacked Borrows limits a reference to its type's size, so it
still reports these accesses, for every thin arc with at least one item, after
the patch. Tree Borrows accepts them by design. Fixing them for Stacked
Borrows would need a different representation or public signatures
(the `&self` receiver has no other provenance), so it is out of scope. Run
code that uses thin arcs under Miri with Tree Borrows.

## Reproducing with Miri

Toolchain: `nightly-2026-10-08` with the `miri` and `rust-src` components.
From the repository root:

```text
# Stacked Borrows (Miri's default model)
cargo +nightly-2026-10-08 miri test -p servo_arc -- --test-threads=1 --exact tests::<name>
# Tree Borrows
MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly-2026-10-08 miri test -p servo_arc -- --test-threads=1 --exact tests::<name>
# All tests under Tree Borrows
MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly-2026-10-08 miri test -p servo_arc
```

Run one test per invocation for the before/after comparison, because Miri
stops at the first undefined behaviour. To see the failures, check out the
first commit of this branch (tests only, unpatched code). Results on
`nightly-2026-10-08` (aarch64 macOS):

| Test | Unpatched SB | Unpatched TB | Patched SB | Patched TB |
| --- | --- | --- | --- | --- |
| `arc_borrow_clone_arc_and_with_arc` | UB (#299) | UB (#299) | pass | pass |
| `arc_into_raw_from_raw` | UB (#299) | UB (#299) | pass | pass |
| `arc_union_clone_and_drop` | UB (#299) | UB (#299) | pass | pass |
| `thin_arc_union_clone_and_drop` | UB (#210) | UB (#299) | UB (known limit) | pass |
| `arc_borrow_traits_unchanged` | pass | pass | pass | pass |
| `thin_assert_padding` (upstream) | UB (#210) | pass | UB (known limit) | pass |
| `slices_and_thin` (upstream) | UB (#210) | pass | UB (known limit) | pass |
| `empty_thin` (upstream) | pass | pass | pass | pass |

"UB (known limit)" is the Stacked Borrows report at `HeaderSlice::slice` or in
`HeaderSlice`'s `Drop` described above. `cargo test -p servo_arc` passes
before and after the patch.

## Upstream

The upstream issues stay open. When upstream fixes them, drop this patch and
move Viper back to the published crate; until then rebase it only onto the
exact commit of the crate version Viper locks, verified as above.
