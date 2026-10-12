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
against its published crate: `servo_arc` and `selectors` on 2026-10-11,
the other nine on 2026-10-12, when Stylo itself entered Viper's graph:

| Crate | Published `.crate` SHA-256 (equals the crates.io index `cksum`) | `.cargo_vcs_info.json` `path_in_vcs` | Sources compared with the commit's directory |
| --- | --- | --- | --- |
| `servo_arc` 0.5.0 | `b4eceb519ec34d8290aaf6e46f435102f2bfa63f22b05a144fb16ab266d3b198` | `servo_arc` | identical: `lib.rs`, the licence files, `Cargo.toml.orig` = `servo_arc/Cargo.toml`, `README.md` = the root `README.md` (`readme = "../README.md"`) |
| `selectors` 0.41.0 | `8322aebea94a6cc288ff884e8a49bdbceef60206c400c6c46d22e62fc4fd91a6` | `selectors` | identical: every source file, `README.md`, `CHANGES.md`, `Cargo.toml.orig` = `selectors/Cargo.toml` |
| `stylo` 0.22.0 | `7a255e46af2478628c48daab3774090caabd7812b1dd3ca98801c2b9c53491ed` | `style` | identical: all 313 other files (sources, the mako templates, the vendored Python wheels), `Cargo.toml.orig` = `style/Cargo.toml` |
| `stylo_traits` 0.22.0 | `645d6bfe40eb6e86ebe9f2d2adadcbced41bfa96175de7cb13d4d178d27bc83d` | `style_traits` | identical, `Cargo.toml.orig` = `style_traits/Cargo.toml`, `README.md` = the root `README.md` |
| `stylo_atoms` 0.22.0 | `ee1771588fd747ef16f8b1990061b6f37c7d3251a0f9b74f405a44c0c5fb8966` | `stylo_atoms` | identical, `Cargo.toml.orig` = `stylo_atoms/Cargo.toml`, `README.md` = the root `README.md` |
| `stylo_dom` 0.22.0 | `9febcb03f2b669ef06b3e8a5209e6280971ed55cecf83fbf3c0562fcf2aa2ec6` | `stylo_dom` | identical, `Cargo.toml.orig` = `stylo_dom/Cargo.toml`, `README.md` = the root `README.md` |
| `stylo_static_prefs` 0.22.0 | `6767c48a1590461db79f8b434ccc5a2d65600a3b716751f7999b35ccead1e024` | `stylo_static_prefs` | identical, `Cargo.toml.orig` = `stylo_static_prefs/Cargo.toml`, `README.md` = the root `README.md` |
| `stylo_malloc_size_of` 0.22.0 | `83ed69d55ee907599f093221004a28371925236f09f20961963020abf38bab6a` | `malloc_size_of` | identical, `Cargo.toml.orig` = `malloc_size_of/Cargo.toml` |
| `stylo_derive` 0.22.0 | `4ee5c550c2e00a0b3c046bef84a9ee217e04e7c014142eeb96f9e48781c5cfd3` | `style_derive` | identical, `Cargo.toml.orig` = `style_derive/Cargo.toml`, `README.md` = the root `README.md` |
| `to_shmem` 0.6.0 | `bbf077da8dd01cc18453bd7d9bc3a945c262f5f344562e5f6deb3914b4dd6f14` | `to_shmem` | identical, `Cargo.toml.orig` = `to_shmem/Cargo.toml` |
| `to_shmem_derive` 0.2.0 | `0f8e91a023810fd2019648410d00eea13a71f21d849e71d6d9ed936fb8061174` | `to_shmem_derive` | identical, `Cargo.toml.orig` = `to_shmem_derive/Cargo.toml` |

Every `.cargo_vcs_info.json` names commit `34d25f741…`. In every crate the
only other files are generated at packaging time and have no counterpart in
git: the normalized `Cargo.toml`, `Cargo.lock` and `.cargo_vcs_info.json`.
`servo_arc` (`servo-arc-*`), `selectors` (`selectors-attr-flag-hook`),
since Viper's V10a part 3a `stylo` (`element-data-checked`,
`dom-apis-local-name`, and since part 3b `chrome-attr-s-flag`) and, since
part 3b, `stylo_static_prefs` (`chrome-prefs`) are patched on this branch;
the other seven directories equal the base commit's.

## Ledger

| Id | Crates and files | Reason | Viper proof | Upstream status | Added by (Viper PR) | Lines (code / tests) |
| --- | --- | --- | --- | --- | --- | --- |
| `servo-arc-provenance` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: refcount access and thin-arc construction without the allocation's provenance (servo/stylo#299, #210); undefined behaviour under both of Miri's aliasing models | `miri` | Never offered (owner decision) | Viper PR #21 (V5b-SERVO-ARC), tests in #22 | 90 / 124 |
| `servo-arc-header-slice` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: `HeaderSlice` reaches its items through a reference to the sized struct (servo/stylo#210); undefined behaviour under Stacked Borrows | `miri` | Never offered (owner decision) | Viper PR #43 (V10a part 1) | 13 / 137 |
| `selectors-attr-flag-hook` | `selectors`: `parser.rs` | Chrome match: a `Parser` hook, `parse_case_sensitive_attribute_flag`, lets a parser refuse the `s` attribute flag while parsing, so `[a=b s]` fails as a parse error of that selector and a forgiving list (`:is()`, `:where()`) drops just that item; Chrome 154 rejects the flag (Viper's selector goldens `attr-flag-s`, `attr-flag-s-nested`, `attr-flag-s-forgiving`); upstream accepts it and has no hook | `viper-style::api fork_patches::attr_flag_hook_rejects_s_while_parsing`, `viper-style::api fork_patches::attr_flag_hook_lets_forgiving_lists_drop_the_item` | Candidate; draft to the owner first | Viper PR #47 (V10a part 2) | 25 / 57 |
| `element-data-checked` | `stylo`: `data.rs` | Soundness: `ElementDataWrapper::borrow` and `borrow_mut` are safe functions that hand out `&ElementData` and `&mut ElementData` from an `UnsafeCell`, and upstream checks the borrow flag only under `debug_assertions`, so a release build lets safe code alias `&mut`; the flag is now checked in every build, and `try_borrow`/`try_borrow_mut` report a conflict | `viper-style::api fork_patches::element_data_checked_try_borrow_mut_refuses_a_second_borrow` | Candidate; draft to the owner first | Viper V10a part 3a | 47 / 23 |
| `chrome-attr-s-flag` | `stylo`: `servo/selector_parser.rs` | Chrome match: Stylo's servo `SelectorParser` returns `false` from the `selectors-attr-flag-hook` hook, so a stylesheet rule whose selector uses the `s` attribute flag is dropped and a forgiving list drops just that selector, as in Chrome 154 (which refuses the flag in stylesheets as in its DOM APIs); upstream parses the flag | `viper-style::api fork_patches::chrome_attr_s_flag_in_stylesheets` | Chrome-specific; not offered | Viper V10a part 3b | 13 / 19 |
| `chrome-prefs` | `stylo_static_prefs`: `preferences.toml`, `src/lib.rs` (its test) | Chrome match: compiled defaults instead of run-time preferences, each justified by a Viper test: `layout.threads` 0 (no style thread pool; Viper styles sequentially), `layout.css.has-selector.enabled`, `layout.css.nth-child-of.enabled` and `layout.grid.enabled` true (Chrome 154 parses `:has()`, `:nth-child(An+B of S)` and `display: grid` in stylesheets; upstream's servo defaults refuse them) | `viper-style::api fork_patches::chrome_prefs_defaults` | Chrome-specific; not offered | Viper V10a part 3b | 17 / 2 |
| `dom-apis-local-name` | `stylo`: `dom_apis.rs` | Chrome match and consistency: `query_selector`'s fast paths for a lone or rightmost type selector compared `TElement::local_name` for equality, while selector matching asks `Element::has_local_name`; both spots now ask `has_local_name`, so Viper's ASCII case-insensitive names of non-HTML elements hold there too (Chrome 154 matches `foreignobject` and `foreignObject` against SVG's `foreignObject`; Viper's selector goldens `svg-lower-case-name`, `svg-upper-case-name`) | `viper-style::api fork_patches::dom_apis_local_name_matches_mixed_case_svg_names`, and the selector golden suite through `dom_apis` | Candidate; draft to the owner first | Viper V10a part 3a | 17 / 0 |

"Lines" counts added lines in the patched file, comments included, above
and inside the test module. The `miri` proof is Viper's gate step `miri`, which runs
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
  `HeaderSlice`'s `Drop` reached the items through a pointer derived from a
  reference to the sized `HeaderSlice`, which ends where the items begin.
  Stacked Borrows rejects every such access for a thin arc with items; Tree
  Borrows accepts it.
- **Change.** The only constructor exposes the provenance of the whole
  allocation; `HeaderSlice::data` and `data_mut` take the address of `data`
  with that exposed provenance. The struct and its layout stay upstream's
  (`len` is stored), so public signatures, auto traits and swap behaviour
  are unchanged. Limit: Miri checks item accesses through exposed
  (wildcard) provenance, so aliasing among item accesses is checked less
  strictly; bounds, liveness and initialization are still checked.
- **History.** The first design (commit `e9c48256`) stored an `end` pointer
  instead of `len`; a safe `swap` of two values moved it, which broke even
  equal lengths. It was replaced before any Viper merge pinned it.
- **Spots.** `servo_arc/PATCHES.md`, section "Patch `servo-arc-header-slice`".
- **References.** [servo/stylo#210](https://github.com/servo/stylo/issues/210).
- **Fork tests.** `header_slice_layout_unchanged`,
  `header_slice_len_for_item_sizes`,
  `header_slice_slice_mut_and_data_mut_round_trip`,
  `header_slice_drops_every_item_once`,
  `header_slice_swap_of_equal_lengths_stays_sound`,
  `header_slice_refuses_zero_sized_items` and
  `header_slice_auto_traits_unchanged`, plus every existing test. All 17
  pass `cargo test` and Miri under both models.
- **Commands.** `cargo test -p servo_arc`, then
  `cargo +nightly-2026-10-08 miri test -p servo_arc`, with and without
  `MIRIFLAGS=-Zmiri-tree-borrows`.

## Patch `selectors-attr-flag-hook`

- **Problem.** Selectors 4 defines the `s` attribute flag (explicitly
  case-sensitive values); `selectors` parses it unconditionally. Chrome 154
  rejects it, as measured on Viper's selector goldens (`attr-flag-s`,
  `attr-flag-s-nested`, `attr-flag-s-forgiving`, captured headful): in
  `querySelectorAll`, `[a=b s]` throws a `SyntaxError`, and inside a
  forgiving list, `:is([a=b s], p)`, only that selector is dropped. Viper
  had refused the flag after parsing, which turned the whole forgiving list
  into an error (its golden case `attr-flag-s-forgiving`).
- **Change.** A new provided method of `selectors::Parser`,
  `parse_case_sensitive_attribute_flag(&self) -> bool`, default `true`, so
  every existing parser (Stylo's own included) behaves as upstream.
  `parse_attribute_selector` passes it to `parse_attribute_flags`, which
  treats `s` as an unexpected token when it is `false`, exactly like an
  unknown flag; the forgiving-list machinery then handles the error as for
  any invalid selector. Viper's DOM selector parser returns `false`.
- **Spots.** `selectors/parser.rs`: the trait method, the call in
  `parse_attribute_selector`, the guard in `parse_attribute_flags`, and the
  test parser's switch.
- **References.** [Selectors 4, attribute case-sensitivity](https://drafts.csswg.org/selectors-4/#attribute-case).
- **Fork tests.** `attr_flag_hook_default_accepts_s`,
  `attr_flag_hook_refuses_s_while_parsing` and
  `attr_flag_hook_lets_forgiving_lists_drop_the_item`, plus every existing
  test: all 16 `selectors` tests pass.
- **Commands.** `cargo test -p selectors`.
- **Upstream.** Generic (it changes no default), so a candidate; a draft goes
  to the Viper owner first, and nothing is sent without the owner's
  go-ahead.

## Patch `element-data-checked`

- **Problem.** `ElementDataWrapper` holds Stylo's per-element data in an
  `UnsafeCell`. Its `borrow` and `borrow_mut` are safe functions returning
  `&ElementData` and `&mut ElementData`; the `AtomicRefCell` flag that
  guards them exists only under `debug_assertions`. In a release build two
  `borrow_mut` calls on one wrapper alias `&mut` from safe code, which is
  undefined behaviour.
- **Change.** The flag and the guards in `ElementDataRef` and
  `ElementDataMut` exist in every build: a conflicting borrow panics, as
  `AtomicRefCell` does, and the new `try_borrow` and `try_borrow_mut`
  return `None` instead. One word per wrapper and an atomic operation per
  borrow in release builds. Viper's side tables use `try_borrow` and
  `try_borrow_mut` only, so a conflict never panics there.
- **Spots.** `style/data.rs`: the import, the `refcell` field, the guards
  of `ElementDataRef` and `ElementDataMut`, `borrow`, `borrow_mut`, the new
  `try_borrow` and `try_borrow_mut`, and the test module.
- **Fork tests.** `element_data_checked_try_borrow_mut_refuses_a_second_borrow`
  and `element_data_checked_borrow_mut_panics_on_a_second_borrow`, in debug
  and release builds; all 10 `stylo` library tests pass.
- **Commands.** `cargo test -p stylo --lib` and
  `cargo test --release -p stylo --lib element_data`.
- **Upstream.** Generic, so a candidate; a draft goes to the Viper owner
  first.

## Patch `dom-apis-local-name`

- **Problem.** `style::dom_apis::query_selector` has fast paths for a
  selector that is a lone type selector and for a type selector in the
  rightmost compound. Both compared `TElement::local_name()` with the
  selector's name for equality, while selector matching itself
  (`matches_local_name`) asks `Element::has_local_name`. An embedder whose
  `has_local_name` is not plain equality got different results from the
  fast and the slow paths. Viper's compares the names of non-HTML elements
  ASCII case-insensitively, as Chrome 154 does in HTML documents
  (`foreignobject` and `foreignObject` both match SVG's `foreignObject`),
  so its selector goldens `svg-lower-case-name` and `svg-upper-case-name`
  failed through the fast path.
- **Change.** Both spots call `has_local_name` with the name
  `select_name` picks, as `matches_local_name` does. For an embedder whose
  `has_local_name` is equality (Gecko, Servo) nothing changes.
- **Spots.** `style/dom_apis.rs`: `query_selector_single_query`
  (`Component::LocalName`) and `query_selector_fast`
  (`SimpleFilter::LocalName`).
- **Fork tests.** None in the fork: exercising `dom_apis` needs a
  `TElement` implementation, which the fork does not have. The Viper proof
  covers it: the named test above and the whole selector golden suite run
  through `dom_apis`.
- **Upstream.** Generic, so a candidate; a draft goes to the Viper owner
  first.

## Patch `chrome-attr-s-flag`

- **Problem.** Chrome 154 refuses the Selectors 4 `s` attribute flag
  (explicitly case-sensitive values) in stylesheets, as in its DOM selector
  APIs: a rule whose selector list uses it is dropped, and inside a
  forgiving `:is()` or `:where()` only that selector is. Stylo's servo
  `SelectorParser` accepted the flag.
- **Change.** `SelectorParser` implements the hook of
  `selectors-attr-flag-hook`, `parse_case_sensitive_attribute_flag`, as
  `false`. Viper's DOM selector parser already did.
- **Spots.** `style/servo/selector_parser.rs`: the hook and the test module.
- **Fork tests.** `chrome_attr_s_flag_refused_in_author_selectors`; all 11
  `stylo` library tests pass.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific (the specification defines the flag), so
  not offered.

## Patch `chrome-prefs`

- **Problem.** Stylo reads its preferences from process-wide atomics with
  compiled defaults that `set_pref!` can change at run time. Viper never
  changes a preference at run time (its gate step `style-policy` refuses
  `set_pref!` in Viper's sources), so the defaults must be Chrome's where
  Viper relies on them. Upstream's servo defaults leave `:has()`,
  `:nth-child(An+B of S)` and grid layout off, and `layout.threads = -1`
  starts a style thread pool of up to six threads on first use.
- **Change.** In `preferences.toml`: `layout.threads` 0 (no pool even if
  one were forced; Viper styles sequentially on its engine thread),
  `layout.css.has-selector.enabled`, `layout.css.nth-child-of.enabled` and
  `layout.grid.enabled` true. Each line carries the marker and names its
  Viper proof. `test_basic_preferences` now expects `layout.threads` 0.
- **Spots.** `stylo_static_prefs/preferences.toml` (four values) and
  `stylo_static_prefs/src/lib.rs` (the test's expectation).
- **Fork tests.** `test_basic_preferences`.
- **Viper proofs.** `chrome_prefs_defaults` (the four values) in the gate's
  patch proofs, and Viper's cascade tests `has_in_a_stylesheet`,
  `nth_child_of_in_a_stylesheet` and `display_grid_is_accepted`, and the
  thread test `restyle_spawns_no_thread`.
- **Commands.** `cargo test -p stylo_static_prefs`.
- **Upstream.** Chrome-specific defaults, so not offered.

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
| 2026-10-11 | `viper/servo-arc-provenance` (`7a13cd06`) → `viper/stylo-0.22.0` | `servo-arc-provenance` carried by branching at `7a13cd06`, hashes unchanged; none retired | none | 0 | the review of Viper PR #43 | #43 |
