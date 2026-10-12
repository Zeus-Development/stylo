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
part 3b, `stylo_static_prefs` (`chrome-prefs`) are patched on this branch,
and since part 4 `stylo_traits` (`owned-slice-into-raw`); Viper's V10b
part 1 adds `style-struct-arc-borrow`, `stylesheet-contents-arc`,
`rule-tree-raw-box` and `thread-cache-miri-root` to `stylo` and
`has-subtree-iterative` to `selectors`.
The other six directories equal the base commit's.

## Ledger

| Id | Crates and files | Reason | Viper proof | Upstream status | Added by (Viper PR) | Lines (code / tests) |
| --- | --- | --- | --- | --- | --- | --- |
| `servo-arc-provenance` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: refcount access and thin-arc construction without the allocation's provenance (servo/stylo#299, #210); undefined behaviour under both of Miri's aliasing models | `miri` | Never offered (owner decision) | Viper PR #21 (V5b-SERVO-ARC), tests in #22 | 90 / 124 |
| `servo-arc-header-slice` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: `HeaderSlice` reaches its items through a reference to the sized struct (servo/stylo#210); undefined behaviour under Stacked Borrows | `miri` | Never offered (owner decision) | Viper PR #43 (V10a part 1) | 13 / 137 |
| `selectors-attr-flag-hook` | `selectors`: `parser.rs` | Chrome match: a `Parser` hook, `parse_case_sensitive_attribute_flag`, lets a parser refuse the `s` attribute flag while parsing, so `[a=b s]` fails as a parse error of that selector and a forgiving list (`:is()`, `:where()`) drops just that item; Chrome 154 rejects the flag (Viper's selector goldens `attr-flag-s`, `attr-flag-s-nested`, `attr-flag-s-forgiving`); upstream accepts it and has no hook | `viper-style::api fork_patches::attr_flag_hook_rejects_s_while_parsing`, `viper-style::api fork_patches::attr_flag_hook_lets_forgiving_lists_drop_the_item` | Candidate; draft to the owner first | Viper PR #47 (V10a part 2) | 25 / 57 |
| `element-data-checked` | `stylo`: `data.rs` | Soundness: `ElementDataWrapper::borrow` and `borrow_mut` are safe functions that hand out `&ElementData` and `&mut ElementData` from an `UnsafeCell`, and upstream checks the borrow flag only under `debug_assertions`, so a release build lets safe code alias `&mut`; the flag is now checked in every build, and `try_borrow`/`try_borrow_mut` report a conflict | `viper-style::api fork_patches::element_data_checked_try_borrow_mut_refuses_a_second_borrow` | Candidate; draft to the owner first | Viper V10a part 3a | 47 / 23 |
| `chrome-attr-s-flag` | `stylo`: `servo/selector_parser.rs` | Chrome match: Stylo's servo `SelectorParser` returns `false` from the `selectors-attr-flag-hook` hook, so a stylesheet rule whose selector uses the `s` attribute flag is dropped and a forgiving list drops just that selector, as in Chrome 154 (which refuses the flag in stylesheets as in its DOM APIs); upstream parses the flag | `viper-style::cascade fork_patches::chrome_attr_s_flag_in_stylesheets` (the cascade test binary) | Chrome-specific; not offered | Viper V10a part 3b | 13 / 19 |
| `chrome-prefs` | `stylo_static_prefs`: `preferences.toml`, `src/lib.rs` (its test) | Chrome match: compiled defaults instead of run-time preferences, each justified by a Viper test: `layout.threads` 0 (no style thread pool; Viper styles sequentially), `layout.css.has-selector.enabled`, `layout.css.nth-child-of.enabled` and `layout.grid.enabled` true (Chrome 154 parses `:has()`, `:nth-child(An+B of S)` and `display: grid` in stylesheets; upstream's servo defaults refuse them) | `viper-style::api fork_patches::chrome_prefs_defaults` | Chrome-specific; not offered | Viper V10a part 3b | 17 / 2 |
| `dom-apis-local-name` | `stylo`: `dom_apis.rs` | Chrome match and consistency: `query_selector`'s fast paths for a lone or rightmost type selector compared `TElement::local_name` for equality, while selector matching asks `Element::has_local_name`; both spots now ask `has_local_name`, so Viper's ASCII case-insensitive names of non-HTML elements hold there too (Chrome 154 matches `foreignobject` and `foreignObject` against SVG's `foreignObject`; Viper's selector goldens `svg-lower-case-name`, `svg-upper-case-name`) | `viper-style::api fork_patches::dom_apis_local_name_matches_mixed_case_svg_names`, and the selector golden suite through `dom_apis` | Candidate; draft to the owner first | Viper V10a part 3a | 17 / 0 |
| `chrome-font-size-precision` | `stylo`: `values/specified/font.rs` | Chrome match: computed font sizes stay exact in servo builds (Chrome 154: `1.17em` of 16px is 18.72px, `smaller` of 16px is 13.3333px); upstream quantizes them to 10 bits of precision, as Gecko does (18.7188px, 13.3281px), which also shifts the em lengths and line heights derived from them (Viper's computed-value goldens `ua-structure-font-size`, `cascade-inherit-font-size`); Gecko builds still quantize | `viper-style::cascade fork_patches::chrome_font_size_precision`, and the computed-value golden suite | Chrome-specific; not offered | Viper V10a part 4 | 11 / 0 |
| `chrome-display-ruby` | `stylo`: `values/specified/box.rs` | Chrome match: servo builds parse and report `display: ruby` and `ruby-text` (Chrome 154 gives them to `ruby` and `rt` and reports them; Viper's golden `ua-text-display`), and a blockified ruby serializes as `block ruby`; upstream has them in Gecko builds only. The other internal ruby values, which Chrome refuses, and the Gecko ruby style adjustments stay Gecko-only | `viper-style::cascade fork_patches::chrome_display_ruby`, and the computed-value golden suite | Chrome-specific; not offered | Viper V10a part 4 | 19 / 43 |
| `owned-slice-into-raw` | `stylo_traits`: `owned_slice.rs` | Soundness: `From<Box<[T]>> for OwnedSlice` took the pointer with `as_mut_ptr()` and then moved the box into `mem::forget`, which under Stacked Borrows invalidates the pointer, so every later access was undefined behaviour (Miri reports it for any `calc()` expression Stylo parses, such as the one in Viper's user-agent sheet); `Box::into_raw` hands the allocation over without the move | `miri` (Viper's `viper-style` library tests parse its user-agent sheet under both models) | Candidate; draft to the owner first | Viper V10a part 4 | 12 / 13 |
| `style-struct-arc-borrow` | `stylo`: `properties/properties.mako.rs` | Soundness: `StyleStructRef::Borrowed` held a `&T` to a style struct, and `StyleStructRef::build` rebuilt its `Arc` with `Arc::from_raw_addrefed`; a reference to the struct grants no access to the refcount in front of it, so the first restyle was undefined behaviour under both of Miri's aliasing models. The builder now keeps `ArcBorrow`s from servo's new `ComputedValuesInner::borrow_<struct>()` (`Arc::borrow_arc`) | `miri` (Viper's `viper-style` library tests restyle documents under both models) | Candidate; draft to the owner first (Gecko's `ComputedValues` would need the same accessor) | Viper V10b part 1 | 34 / 0 |
| `stylesheet-contents-arc` | `stylo`: `stylesheets/stylesheet.rs`, `stylist.rs` | Soundness: `CascadeData::collect_applicable_media_query_results_into` rebuilt a stylesheet's contents `Arc` with `Arc::from_raw_addrefed` from the `&StylesheetContents` of `StylesheetInDocument::contents`, undefined behaviour under both models whenever a stylist is flushed. `StylesheetInDocument` gains `contents_arc`, which servo's `Stylesheet` and `DocumentStyleSheet` answer with `Arc::borrow_arc`; the provided default keeps upstream's assumption for other implementors | `miri` | Candidate; draft to the owner first | Viper V10b part 1 | 43 / 0 |
| `rule-tree-raw-box` | `stylo`: `rule_tree/unsafe_box.rs`, `rule_tree/core.rs` | Soundness: the rule tree's `UnsafeBox` wrapped `ManuallyDrop<Box<T>>` and was copied with `ptr::read`, and `RuleNode::push_on_free_list` stored a pointer cast from `&RuleNode` that `swap_free_list_and_gc` later freed with `Box::from_raw`: undefined behaviour under both models. `UnsafeBox` now holds the pointer from `Box::into_raw`, and the free list stores that pointer | `miri` | Candidate; draft to the owner first | Viper V10b part 1 | 46 / 52 |
| `has-subtree-iterative` | `selectors`: `matching.rs` | Robustness and Chrome match: `matches_relative_selector_subtree`, the `:has()` subtree search, called itself once per tree level, so `:has()` on a document tens of thousands of levels deep overflowed the stack and aborted the process (`html:has(span)` at depth 50,000), where Chrome 154 answers; the walk is now iterative with the same order, selector flags and early return | `viper-style::api deep_trees_are_walked_without_recursion` | Candidate; draft to the owner first | Viper V10b part 1 | 31 / 0 |
| `thread-cache-miri-root` | `stylo`: `bloom.rs`, `sharing/mod.rs` | Test tooling: Stylo leaks its bloom-filter and style-sharing thread-local caches by design, once per thread that styles, and Miri's leak check reported them whenever a test thread that restyled exited, so a Miri run that restyles needed `-Zmiri-ignore-leaks`, which hides every other leak too. Under `cfg(miri)` only, each of the two allocations is registered with Miri as a static root (`miri_static_root`) when it is made: this exempts exactly those two allocations from Miri's leak report and nothing else. What is leaked, and when, is unchanged, and builds outside Miri are upstream's code | `miri` (Viper's restyling library tests pass Miri's leak check) | Candidate; draft to the owner first | Viper V10b part 1 | 24 / 0 |

"Lines" counts added lines in the patched file, comments included, above
and inside the test module. The `miri` proof is Viper's gate step `miri`, which runs
`viper-style`'s API tests (selectors over Viper's DOM, through these thin
arcs) and library tests under Stacked Borrows and under Tree Borrows; since
Viper's V10b part 1 the library tests include two that restyle a document,
through the stylist, the rule tree and the style builder.

## Known unpatched sites

Sites with the same fault as a patch above that no Viper path reaches yet.
Each is recorded in Viper's deferred-cases register with an owner, and
must be patched (with a fork test or a Viper proof under Miri) before
anything makes it reachable.

| Site | Fault | Why unreachable now | Viper register entry |
| --- | --- | --- | --- |
| `ComputedValues::to_arc` (`style/properties/properties.mako.rs`), called only from `ContainerCondition`'s container lookup (`style/stylesheets/container_rule.rs`) | Rebuilds an `Arc` with `Arc::from_raw_addrefed` from `&self`, a reference that grants no access to the refcount: the fault `style-struct-arc-borrow` and `stylesheet-contents-arc` fix elsewhere, undefined behaviour under both of Miri's aliasing models | Servo builds parse no `@container` rule (`rule_parser.rs` accepts `container` in Gecko builds only), and `container-type` and `container-name` sit behind `layout.container-queries.enabled`, which is `false`; so no container lookup runs | `adr:style-container-to-arc` |

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

## Patch `chrome-font-size-precision`

- **Problem.** Chrome 154 keeps a computed font size as computed:
  `font-size: 1.17em` of 16px is 18.72px, `smaller` of 16px is 13.3333px
  and `28.8px` is 28.8px. Stylo's `FontSize::to_computed_value_against`
  quantized every computed size to 10 bits of precision
  (`quantize_font_size`, Gecko's behaviour, which limits the number of
  font instances Gecko creates), giving 18.7188px, 13.3281px and 28.8125px,
  and shifting every em length and line height derived from them.
- **Change.** Servo builds keep the exact size; Gecko builds still quantize.
- **Spots.** `style/values/specified/font.rs`: the end of
  `to_computed_value_against`.
- **Fork tests.** None: a computed font size needs a full style context,
  which the fork's tests do not build. All 12 `stylo` library tests pass.
- **Viper proofs.** `chrome_font_size_precision` (the cascade test binary)
  and the computed-value golden suite (headings, `small`, `big`, `sub`,
  `sup` and the em margins of headings).
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `chrome-display-ruby`

- **Problem.** Chrome 154 parses `display: ruby` and `display: ruby-text`,
  which its user-agent sheet gives `ruby` and `rt` inside a `ruby`, and
  reports them as computed values. Stylo parsed them, and had their display
  types, in Gecko builds only, so servo builds dropped the declarations.
- **Change.** In servo builds too: `DisplayOutside::InternalRuby`,
  `DisplayInside::Ruby` and `RubyText`, the constants `Display::Ruby` and
  `Display::RubyText`, ruby's inline default outside type, the keywords
  `ruby` and `ruby-text`, and the `block ruby` serialization. `ruby-base`,
  `ruby-base-container` and `ruby-text-container`, which Chrome refuses,
  and Gecko's ruby style adjustments (`adjust_for_ruby`) stay Gecko-only.
- **Spots.** `style/values/specified/box.rs`: the two display enums,
  `default_display_outside`, two constants, `DisplayKeyword::parse`,
  `Display::to_css` and the test module.
- **Fork tests.** `chrome_display_ruby_parses_ruby_and_ruby_text`; all 12
  `stylo` library tests pass.
- **Viper proofs.** `chrome_display_ruby` (the cascade test binary) and the
  computed-value goldens `ua-text-display` and `ua-line-height-display`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `owned-slice-into-raw`

- **Problem.** `impl From<Box<[T]>> for OwnedSlice<T>` took the items'
  pointer with `b.as_mut_ptr()` and then moved `b` into `mem::forget`.
  Under Stacked Borrows that move retags the box as unique, which
  invalidates the pointer taken before, so every later access through the
  `OwnedSlice` is undefined behaviour. Miri reports it whenever Stylo parses
  a `calc()` expression (`style/values/generics/calc.rs` stores the
  children of a sum in an `OwnedSlice`); Viper's user-agent sheet has one.
  Tree Borrows accepts the code.
- **Change.** `Box::into_raw` hands the allocation over without a move.
- **Spots.** `style_traits/owned_slice.rs`: `From<Box<[T]>>` and a test
  module.
- **Fork tests.** `owned_slice_from_box_reads_and_writes_its_items`, under
  Miri on both models (it fails under Stacked Borrows without the fix), and
  `cargo test -p stylo_traits`.
- **Viper proof.** `miri`: Viper's `viper-style` library tests parse its
  user-agent sheet under both models.
- **Commands.** `cargo test -p stylo_traits`;
  `cargo +nightly miri test -p stylo_traits owned_slice` with and without
  `MIRIFLAGS=-Zmiri-tree-borrows`.
- **Upstream.** A generic soundness fix: candidate, draft to the owner
  first.

## Patch `style-struct-arc-borrow`

- **Problem.** `StyleBuilder` starts from the parent's (or the default)
  style structs without cloning them: `StyleStructRef::Borrowed(&'a T)`.
  `StyleStructRef::build` turned such a reference back into an `Arc` with
  `Arc::from_raw_addrefed`, which writes the refcount in front of the
  struct. A pointer derived from a reference to the struct may access the
  struct only, so Miri reports the first restyle: Stacked Borrows when
  `Arc::clone` retags outside the reference's range, Tree Borrows when the
  refcount is written through a frozen tag.
- **Change.** `Borrowed` holds an `ArcBorrow<'a, T>`, which keeps the
  provenance of the whole `ArcInner` allocation (`servo-arc-provenance`).
  Servo's `ComputedValuesInner` gains `borrow_<struct>()`, `Arc::borrow_arc`
  of the field, and every `Borrowed` is built from it; `build` calls
  `clone_arc`, so no unsafe code remains there. `mutate`, `take`, `ptr_eq`
  and `Deref` read the struct through `ArcBorrow::get`.
- **Spots.** `style/properties/properties.mako.rs`: the import,
  `borrow_<struct>()` in servo's `ComputedValuesInner`, `StyleStructRef`
  and its methods, and the five places that build `Borrowed`
  (`StyleBuilder::new`, `for_derived_style`, `copy_reset_from`,
  `reset_<struct>_struct`).
- **Fork tests.** None: a style builder needs a device and computed values,
  which the fork's tests do not build. All 14 `stylo` library tests pass.
- **Viper proof.** `miri`: Viper's restyling library tests run under both
  models in its gate; before this patch (and the two below) Stacked Borrows
  and Tree Borrows report undefined behaviour there.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** A generic soundness fix: candidate, draft to the owner
  first. Gecko builds would need `borrow_<struct>()` on Gecko's
  `ComputedValues`, which the fork does not build.

## Patch `stylesheet-contents-arc`

- **Problem.** `CascadeData::collect_applicable_media_query_results_into`
  keeps each effective stylesheet's contents alive in the cascade data
  cache's key: it rebuilt the contents' `Arc` with
  `Arc::from_raw_addrefed(stylesheet.contents(guard))`. That reference
  covers the contents only, so the refcount write is undefined behaviour
  under both models; Miri reports it when Viper's stylist is flushed with
  its user-agent sheets.
- **Change.** A provided method `StylesheetInDocument::contents_arc`
  returns an `ArcBorrow` of the contents. Servo's `Stylesheet` (whose
  contents are a `Locked<Arc<StylesheetContents>>`) answers with
  `Arc::borrow_arc`, and `DocumentStyleSheet` forwards to it; the stylist
  clones that borrow. The default keeps upstream's assumption
  (`ArcBorrow::from_ref` of `contents`) for implementors whose contents
  come from elsewhere, such as Gecko's sheets.
- **Spots.** `style/stylesheets/stylesheet.rs`: the import, the trait
  method, the two servo implementations; `style/stylist.rs`:
  `collect_applicable_media_query_results_into`.
- **Fork tests.** None: the cascade data cache needs a device and a
  stylist. All 14 `stylo` library tests pass.
- **Viper proof.** `miri`, as above.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** A generic soundness fix: candidate, draft to the owner
  first.

## Patch `rule-tree-raw-box`

- **Problem.** The rule tree's nodes live in `UnsafeBox<RuleNode>`, which
  wrapped a `ManuallyDrop<Box<RuleNode>>` and was copied with `ptr::read`
  for every strong and weak reference: several live `Box`es of one
  allocation, each asserting unique access. `RuleNode::push_on_free_list`
  stored `RuleNode::as_mut_ptr()`, a pointer cast from `&RuleNode`, in the
  free list, and `RuleTree::swap_free_list_and_gc` later freed those nodes
  with `Box::from_raw`. Freeing (or writing) through a pointer derived from
  a shared reference is undefined behaviour: Stacked Borrows reports a
  `Unique` retag through a `SharedReadOnly` tag when the tree collects or
  drops its free list, Tree Borrows a write through such a pointer in
  `RuleNode::drop_without_free_list`.
- **Change.** `UnsafeBox` holds a `NonNull<T>` from `Box::into_raw`; `clone`
  copies it, `deref` and `deref_mut` read through it, and `drop` rebuilds
  the `Box` only to drop it. The new `UnsafeBox::as_mut_ptr` returns that
  pointer, with the provenance of the whole allocation, and the free list
  stores it. Size, niche (`Option<StrongRuleNode>` stays one word) and
  auto traits stay those of `Box<T>` (explicit `Send` and `Sync`).
- **Spots.** `style/rule_tree/unsafe_box.rs`: the struct, `Send`/`Sync`,
  `from_box`, `from_raw`, `clone`, the new `as_mut_ptr`, `deref_mut`,
  `drop`, `Deref`; `style/rule_tree/core.rs`: `push_on_free_list` (two
  spots) and the test module.
- **Fork tests.** `rule_tree_raw_box_frees_nodes_through_the_free_list`
  (nodes on the free list, collected and dropped with the tree; under Miri
  it fails on both models without the fix and passes both with it) and
  `rule_tree_raw_box_frees_nodes_after_the_tree` (the path without a free
  list; sound before and after). All 14 `stylo` library tests pass.
- **Viper proof.** `miri`, as above.
- **Commands.** `cargo test -p stylo --lib`;
  `cargo +nightly miri test -p stylo --lib rule_tree_raw_box` with and
  without `MIRIFLAGS=-Zmiri-tree-borrows`.
- **Upstream.** A generic soundness fix: candidate, draft to the owner
  first.

## Patch `has-subtree-iterative`

- **Problem.** For `:has()` with a descendant (or sibling-subtree)
  argument, `matches_relative_selector_subtree` searched the subtree below
  each candidate by calling itself once per tree level. On a document tens
  of thousands of levels deep, `html:has(span)` overflowed the thread's
  stack and aborted the process; Chrome 154 answers such queries. The DOM
  selector APIs' own walks are iterative.
- **Change.** The same pre-order walk (each element, then its subtree, then
  its next sibling), the same selector flags and the same early return,
  with the ancestors whose later siblings remain kept in a
  `SmallVec<[E; 16]>` instead of call frames. The bloom filter that
  `RelativeSelectorFilterMap` builds for a second search of one anchor
  (`add_to_filter`) also recurses per level, but only when the element adds
  hashes to it: Viper's elements add none (`add_element_unique_hashes`
  returns `false` at the first child), so it is not patched.
- **Spots.** `selectors/matching.rs`: `matches_relative_selector_subtree`.
- **Fork tests.** None: the crate's tests have no `Element` implementation.
  All 16 `selectors` tests pass.
- **Viper proof.** `deep_trees_are_walked_without_recursion` (the `api`
  test binary): `:has()` found and not found on a 50,000-deep tree; it
  aborts with a stack overflow without the patch.
- **Commands.** `cargo test -p selectors`.
- **Upstream.** Generic, so a candidate; a draft goes to the Viper owner
  first.

## Patch `thread-cache-miri-root`

- **Problem.** `BLOOM_KEY` (`style/bloom.rs`) and `SHARING_CACHE_KEY`
  (`style/sharing/mod.rs`) are thread-local `&'static` caches made with
  `Box::leak`: leaked on purpose, once per thread that styles, because
  thread-local destructors may not run on worker threads and the caches
  are borrowed for `'static`. Miri's leak check reports the two
  allocations of every test thread that restyled once that thread exits,
  so the run fails unless `-Zmiri-ignore-leaks` turns the whole check off.
- **Change.** Both caches are made by `leak_thread_cache`, which leaks as
  before and, under `cfg(miri)` only, passes the allocation to Miri's
  `miri_static_root`, so Miri treats it like memory a static holds. The
  exemption covers exactly these two allocations (one of each per thread
  that styles): every other leak, in Stylo or in Viper, is still reported,
  and the leak itself is not changed. Outside Miri the code is the same as
  upstream's.
- **Spots.** `style/bloom.rs`: `BLOOM_KEY` and the new
  `leak_thread_cache`; `style/sharing/mod.rs`: `SHARING_CACHE_KEY`.
- **Fork tests.** None: the caches are filled by a traversal. All 14
  `stylo` library tests pass.
- **Viper proof.** `miri`: Viper's restyling library tests run under both
  models with Miri's leak check on; without the patch each run fails with
  4 leaked allocations (two per restyling test thread).
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** A candidate for test tooling; draft to the owner first.

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
| 2026-10-12 | dry run: `viper/stylo-0.22.0` (`839e3e03`) → upstream `main` `c81cd9c7` (183 commits after `v0.22.0`; no 0.23.0 published), scratch branch `dry-run/stylo-main-2026-10-12` at `c15f00e5`, not pinned | all 10 patches carried; `chrome-prefs` loses its `layout.grid.enabled` line (upstream removed the preference, grid is always on); none retired | 4, resolved by hand: `servo_arc/lib.rs` (`ArcBorrow::ptr_eq`, upstream's clippy change), `selectors/parser.rs` (the test parser: upstream's `parse_non_ts_pseudo_class` now takes `&str`, which Viper's `selectors::Parser` implementation must follow), `style/dom_apis.rs` (`select_name`'s call and result changed), `stylo_static_prefs/preferences.toml` (the grid preference); `style/values/specified/box.rs` merged again by hand after a wrong end-of-file merge. Fork tests: `servo_arc` 17, `selectors` 16, `stylo` 12, `stylo_traits` 1 and `stylo_static_prefs` 1 pass; Miri on `servo_arc` (17 tests) passes both models; Viper was not built against it | 0.5 | the review of Viper V10a part 4 | V10a part 4 |
