# Viper patches to servo_arc

This file is the detailed record of Viper's two patches to `servo_arc`,
`servo-arc-provenance` and `servo-arc-header-slice`. The fork's patch ledger,
with every patch of every crate, is [`PATCHES.md`](../PATCHES.md) at the
repository root. Neither patch is **ever submitted upstream**: no issue, pull
request or comment based on them goes to `servo/stylo` or any other Servo
repository. Viper consumes this crate through a Cargo `[patch.crates-io]`
entry pinned to an exact commit of a `viper/*` branch.

Every changed spot in `lib.rs` carries a `// VIPER-PATCH(<id>)` comment with
one of the two ids.

## Base

The patch applies to the exact sources of the published crate
`servo_arc 0.5.0`, which Viper uses through `selectors 0.41.0` (the
selectors release of Stylo 0.22).

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

The branch `viper/servo-arc-provenance` starts at that commit. Its first
commit adds the regression tests only; the second commit holds the
`servo-arc-provenance` fix and this file; the third clarifies this file; the
fourth adds the `UniqueArc` coverage tests `unique_thin_arc_mutation` and
`unique_arc_new_uninit`. The branch `viper/stylo-0.22.0` starts at that
fourth commit (`7a13cd06`) and adds the `servo-arc-header-slice` commit, with
its tests, on top.

## Patch `servo-arc-provenance`: the problem

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

## Patch `servo-arc-provenance`: spots

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

## Patch `servo-arc-header-slice`: Stacked Borrows and `HeaderSlice`

`HeaderSlice<H, T>` is a sized struct whose items follow a zero-length array.
Upstream, `HeaderSlice::slice(&self)`, `slice_mut(&mut self)`,
`UniqueArc::data_mut` and `impl Drop for HeaderSlice` reach the items through
a reference to that sized struct. Stacked Borrows limits a reference to its
type's size, so after `servo-arc-provenance` it still reported these
accesses for every thin arc with at least one item: a zero-size retag
(`[0x20..0x20]`) created by `addr_of!(self.data)` in `HeaderSlice::data` or
`addr_of_mut!(self.data)` in `HeaderSlice::data_mut`, after which the item
access has no valid tag. Tree Borrows accepts them by design. The `&self`
receiver carries no other provenance, so the fix stores one.

| Spot | Change |
| --- | --- |
| `HeaderSlice` (struct) | The private field `len: usize` becomes `end: NonNull<u8>`, one past the last item, with the provenance of the whole allocation. Same size and alignment as `usize`, so the struct's size and alignment are unchanged (test `header_slice_layout_unchanged`). A `u8` pointer, so the auto traits that depend on `T` stay those of `[T; 0]`. |
| `Send`, `Sync` for `HeaderSlice` | `unsafe impl` with exactly the former auto-trait conditions (`H: Send, T: Send`; `H: Sync, T: Sync`), because `NonNull` has neither. |
| `HeaderSlice::data`, `data_mut` | The address of `data` with the provenance of `end` (`with_addr`), not that of `self`. `slice`, `slice_mut`, `UniqueArc::data_mut` and `Drop` use them. |
| `HeaderSlice::len` | `(end - data) / size_of::<T>()`, from the two addresses. Zero-sized items are refused by the constructor, as upstream. |
| `Arc::from_header_and_iter_alloc` | Writes `end`, derived from the raw allocation pointer, and starts the item writes at the same pointer. It stays the only constructor (`to_shmem` goes through it too). |

Public signatures, `header`, `Eq`, `PartialEq` and `Debug` are unchanged.
Gecko's C++ code reads the `len` field, so this layout is servo-mode only;
one more reason the patch is never offered upstream.

## Reproducing with Miri

Toolchain: `nightly-2026-10-08` with the `miri` and `rust-src` components.
From the repository root:

```text
# Stacked Borrows (Miri's default model)
cargo +nightly-2026-10-08 miri test -p servo_arc -- --test-threads=1 --exact tests::<name>
# Tree Borrows
MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly-2026-10-08 miri test -p servo_arc -- --test-threads=1 --exact tests::<name>
# All tests, under each model
cargo +nightly-2026-10-08 miri test -p servo_arc
MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly-2026-10-08 miri test -p servo_arc
```

Run one test per invocation for the before/after comparison, because Miri
stops at the first undefined behaviour. To see the failures, check out the
first commit of `viper/servo-arc-provenance` (tests only, unpatched code), or
`7a13cd06` for the state before `servo-arc-header-slice`. Tests added by a
later commit were measured by adding them to the earlier code. Results on
`nightly-2026-10-08` (aarch64 macOS); "provenance" is `7a13cd06`, "both" is
the `servo-arc-header-slice` commit:

| Test | Unpatched SB | Unpatched TB | Provenance SB | Provenance TB | Both SB | Both TB |
| --- | --- | --- | --- | --- | --- | --- |
| `arc_borrow_clone_arc_and_with_arc` | UB (#299) | UB (#299) | pass | pass | pass | pass |
| `arc_into_raw_from_raw` | UB (#299) | UB (#299) | pass | pass | pass | pass |
| `arc_union_clone_and_drop` | UB (#299) | UB (#299) | pass | pass | pass | pass |
| `thin_arc_union_clone_and_drop` | UB (#210) | UB (#299) | UB (`HeaderSlice`) | pass | pass | pass |
| `arc_borrow_traits_unchanged` | pass | pass | pass | pass | pass | pass |
| `unique_thin_arc_mutation` | UB (#210) | pass | UB (`HeaderSlice`) | pass | pass | pass |
| `unique_arc_new_uninit` | pass | pass | pass | pass | pass | pass |
| `header_slice_layout_unchanged` | not run | not run | pass | pass | pass | pass |
| `header_slice_len_for_item_sizes` | not run | not run | UB (`HeaderSlice`) | pass | pass | pass |
| `header_slice_slice_mut_and_data_mut_round_trip` | not run | not run | UB (`HeaderSlice`) | pass | pass | pass |
| `header_slice_drops_every_item_once` | not run | not run | UB (`HeaderSlice`) | pass | pass | pass |
| `header_slice_refuses_zero_sized_items` | not run | not run | pass | pass | pass | pass |
| `header_slice_auto_traits_unchanged` | not run | not run | pass | pass | pass | pass |
| `thin_assert_padding` (upstream) | UB (#210) | pass | UB (`HeaderSlice`) | pass | pass | pass |
| `slices_and_thin` (upstream) | UB (#210) | pass | UB (`HeaderSlice`) | pass | pass | pass |
| `empty_thin` (upstream) | pass | pass | pass | pass | pass | pass |

"UB (`HeaderSlice`)" is the Stacked Borrows report at `HeaderSlice::slice`,
`HeaderSlice::slice_mut` (through `UniqueArc::data_mut`) or in
`HeaderSlice`'s `Drop` described above. With both patches all 16 tests pass
`cargo test` and Miri under Stacked Borrows and under Tree Borrows. `cargo
test -p servo_arc` passes at every commit.

## Upstream

The upstream issues stay open. When upstream fixes them, retire the patch
that the fix replaces (the root ledger's rebase procedure); until then rebase
the patches only onto the exact commit of the crate version Viper locks,
verified as above.
