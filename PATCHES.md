# Viper's Stylo fork: patch ledger

This repository is a fork of [Stylo](https://github.com/servo/stylo) kept by
the Viper project. Its `viper/*` branches carry a small set of patches on top
of exact Stylo release commits; `main` and every other branch follow upstream
unchanged. Viper builds the Stylo crates (servo mode only) from one exact
commit of a `viper/*` branch, through Cargo `[patch.crates-io]` entries.

This file is the canonical ledger of those patches. It holds the verified
base of every crate Viper takes from the fork, one row and one section per
patch, the fork-side rebase procedure and the rebase log. Viper's own record
of the fork (how the pin moves, the gate that checks it, the cargo-deny
exception and the licence obligations) is its ADR 0017, "The Stylo fork".

## Rules

- **Branches.** One long-lived branch per Stylo release, `viper/stylo-<version>`,
  created at the release's tag commit, or, for `viper/stylo-0.22.0`, at the
  last commit of `viper/servo-arc-provenance`, so that its reviewed commits
  keep their hashes. A `viper/*` branch is never force-pushed, and never
  deleted while any Viper revision pins a commit on it.
- **Markers.** Every changed hunk carries `VIPER-PATCH(<id>)` in a comment
  (`//` in Rust, `#` in TOML and Python), with `<id>` matching `[a-z0-9-]+`
  and naming a row of the ledger below. The comment states the soundness
  problem, or the Chrome behaviour matched, and why upstream differs, with a
  specification or Chromium link where one exists. Viper's gate step
  `stylo-fork` fails when the set of marker ids in the pinned checkout
  differs from the set of ledger ids.
- **Size.** Each patch is small (target at most 100 changed code lines) and
  self-contained, one commit per patch where possible.
- **Tests.** Every patch has tests in the fork (listed in its section) and a
  proof in Viper's gate: Miri on Viper's `viper-style` under both aliasing
  models, or named tests that must pass in the gate's run.
- **Upstream.** The `servo-arc-*` patches are never offered upstream. A
  generic patch may be offered later; nothing goes to any Servo repository
  without the Viper owner's go-ahead for that specific draft.
- **Licences.** Modified MPL-2.0 files keep their MPL headers; `servo_arc`
  stays `MIT OR Apache-2.0`. The licence files are unchanged.
- **Commits.** Patch commits name their id in the subject (`(VIPER-PATCH)`)
  and use the Viper owner's GitHub noreply identity.

## Base

`viper/stylo-0.22.0` is based on Stylo commit
`34d25f741765a0bdb894f8271852e633299352d8` ("Bump stylo to 0.22.0, and more
(#473)", tag `v0.22.0`). Each crate Viper takes from the fork was verified
against its published crate on 2026-10-11:

| Crate | Published `.crate` SHA-256 (equals the crates.io index `cksum`) | `.cargo_vcs_info.json` | Sources compared with the commit's directory |
| --- | --- | --- | --- |
| `servo_arc` 0.5.0 | `b4eceb519ec34d8290aaf6e46f435102f2bfa63f22b05a144fb16ab266d3b198` | `34d25f741…`, `path_in_vcs = "servo_arc"` | identical: `lib.rs`, the licence files, `Cargo.toml.orig` = `servo_arc/Cargo.toml`, `README.md` = the root `README.md` (`readme = "../README.md"`) |
| `selectors` 0.41.0 | `8322aebea94a6cc288ff884e8a49bdbceef60206c400c6c46d22e62fc4fd91a6` | `34d25f741…`, `path_in_vcs = "selectors"` | identical: every source file, `README.md`, `CHANGES.md`, `Cargo.toml.orig` = `selectors/Cargo.toml` |

In both crates the only other files are generated at packaging time and have
no counterpart in git: the normalized `Cargo.toml`, `Cargo.lock` and
`.cargo_vcs_info.json`. `selectors` is unpatched on this branch: its
directory equals the base commit's.

## Ledger

| Id | Crates and files | Reason | Viper proof | Upstream status | Added by (Viper PR) | Lines (code / tests) |
| --- | --- | --- | --- | --- | --- | --- |
| `servo-arc-provenance` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: refcount access and thin-arc construction without the allocation's provenance (servo/stylo#299, #210); undefined behaviour under both of Miri's aliasing models | `miri` | Never offered (owner decision) | V5b-SERVO-ARC (#21), tests in #22 | 90 / 124 |
| `servo-arc-header-slice` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: `HeaderSlice` reaches its items through a reference to the sized struct (servo/stylo#210); undefined behaviour under Stacked Borrows | `miri` | Never offered (owner decision) | V10a part 1 | 40 / 116 |

"Lines" counts added lines in `lib.rs`, comments included, above and inside
the test module. The `miri` proof is Viper's gate step `miri`, which runs
`viper-style`'s API tests (selectors over Viper's DOM, through these thin
arcs) under Stacked Borrows and under Tree Borrows.

## Patch `servo-arc-provenance`

Problem, change, every spot, the Miri commands and the before/after results:
[`servo_arc/PATCHES.md`](servo_arc/PATCHES.md). Fork tests:
`arc_borrow_clone_arc_and_with_arc`, `arc_into_raw_from_raw`,
`arc_union_clone_and_drop`, `thin_arc_union_clone_and_drop`,
`arc_borrow_traits_unchanged`, `unique_thin_arc_mutation` and
`unique_arc_new_uninit`. Commits on `viper/servo-arc-provenance`:
`93f27d32` (tests), `34518d1e` (fix), `b231290d` (ledger), `7a13cd06`
(coverage tests).

## Patch `servo-arc-header-slice`

- **Problem.** `HeaderSlice::slice`, `slice_mut`, `UniqueArc::data_mut` and
  `HeaderSlice`'s `Drop` reached the items through a reference to the sized
  `HeaderSlice`, which ends where the items begin. Stacked Borrows rejects
  every such access for a thin arc with items; Tree Borrows accepts it.
- **Change.** The private `len: usize` becomes `end: NonNull<u8>`, one past
  the last item, with the allocation's provenance; the items are reached
  with that provenance and the address of `data`, and the length is derived
  from the two addresses. Size, alignment, public signatures and auto traits
  are unchanged; `Send` and `Sync` are restored by hand with the former
  conditions. Servo mode only (Gecko's C++ reads `len`).
- **Spots.** `servo_arc/PATCHES.md`, section "Patch `servo-arc-header-slice`".
- **References.** [servo/stylo#210](https://github.com/servo/stylo/issues/210).
- **Fork tests.** `header_slice_layout_unchanged`,
  `header_slice_len_for_item_sizes`,
  `header_slice_slice_mut_and_data_mut_round_trip`,
  `header_slice_drops_every_item_once`,
  `header_slice_refuses_zero_sized_items` and
  `header_slice_auto_traits_unchanged`, plus every existing test. All 16
  pass `cargo test` and Miri under both models.
- **Commands.** `cargo test -p servo_arc`, then
  `cargo +nightly-2026-10-08 miri test -p servo_arc`, with and without
  `MIRIFLAGS=-Zmiri-tree-borrows`.

## Rebase procedure (fork side)

Viper's ADR 0017 holds the whole procedure; these are its fork-side steps.

1. **Trigger.** A new Stylo release tag; a security or soundness fix may
   trigger one earlier.
2. **Verify the base** of every crate Viper takes from the fork: the
   published `.crate` SHA-256 against the crates.io index, its
   `.cargo_vcs_info.json` naming the tag commit and the right
   `path_in_vcs`, and its unpacked sources identical to that directory of
   the commit, apart from the files generated at packaging time. Record the
   result in the "Base" section.
3. **Create the branch** `viper/stylo-<version>` at the tag commit and
   cherry-pick the patch commits in ledger order, keeping the markers. A
   patch that upstream made unnecessary is retired: its row moves to a
   "Retired" table with the upstream commit, and its Viper proof must still
   pass without it. A patch that no longer applies is reworked as a new
   commit with the same id.
4. **Test.** `cargo test` for every patched crate (servo feature), Miri on
   `servo_arc` under both models, and each patch's fork tests. Update this
   file: base, ledger and rebase log, with conflicts and effort in hours.
5. **Review.** An independent review of
   `git range-diff <old base>..<old pin> <new base>..<new pin>`.
6. **Push the new branch.** Never force-push; never delete a branch that
   holds a commit any Viper revision pins.

## Rebase log

| Date | From → to | Patches carried / retired | Conflicts | Effort (hours) | Reviewer | Viper PR |
| --- | --- | --- | --- | --- | --- | --- |
| 2026-10-11 | `viper/servo-arc-provenance` (`7a13cd06`) → `viper/stylo-0.22.0` | `servo-arc-provenance` carried by branching at `7a13cd06`, hashes unchanged; none retired | none | 0 | the review of V10a part 1 | V10a part 1 |
