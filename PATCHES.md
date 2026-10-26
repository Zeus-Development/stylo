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
`dom-apis-local-name`, and from part 3b to STY-FORK-1 `chrome-attr-s-flag`) and, since
part 3b, `stylo_static_prefs` (`chrome-prefs`) are patched on this branch,
and since part 4 `stylo_traits` (`owned-slice-into-raw`); Viper's V10b
part 1 adds `style-struct-arc-borrow`, `stylesheet-contents-arc`,
`rule-tree-raw-box` and `thread-cache-miri-root` to `stylo` and
`has-subtree-iterative` to `selectors`; Viper's V10b part 3 adds
`matches-simple-selector-inline` to `selectors`; Viper's V10b part 4a adds
`chrome-dom-api-pseudos` to `stylo` and `selectors-compound-is-alias` to
`selectors`; Viper's V10b-6 adds `chrome-value-parsing` to `stylo` and
`stylo_atoms` (its `static_atoms.txt`, which has no comment syntax, so the
patch's marker sits beside the `atom!("scripting")` that uses the new atom)
and `chrome-layout-properties` to `stylo`; Viper's V10b-4c1 adds
`chrome-selector-names` to `stylo`; Viper's V10b-4c2 adds
`chrome-after-pseudo-element` to `selectors` and `stylo`; Viper's
STY-FORK-1 adds `stylesheet-parse-options` to `stylo` and retires
`chrome-attr-s-flag` (section "Retired patches"); Viper's STY-FORK-2
retires `chrome-value-parsing` for `value-grammar-options` (`stylo`) and
`servo-scripting-media-feature` (`stylo` and `stylo_atoms`, whose marker
now sits beside the `atom!("scripting")`); Viper's STY-FORK-3a adds
`selector-names-hook` to `stylo`, and Viper's STY-FORK-3a2 passes its name
methods the enclosing selector parser; Viper's STY-FORK-DEVICE-1 adds
`servo-device-media-features` to `stylo` and `stylo_atoms` (its five atoms,
whose marker sits beside the `atom!(...)` entries that use them); Viper's
STY-FORK-VA-1 adds `vertical-align-longhand` to `stylo`.
The other five directories equal the base commit's. V10b part 4a pins no new
crate; on 2026-10-15 it checked the two crates it patches again, as above:
the `.crate` SHA-256 of `selectors` 0.41.0 and `stylo` 0.22.0 still equals
the index `cksum`, both `.cargo_vcs_info.json` name `34d25f741…`, and the
unpacked sources equal the commit's `selectors` and `style` directories
apart from the packaging files.

## Ledger

| Id | Crates and files | Reason | Viper proof | Upstream status | Added by (Viper PR) | Lines (code / tests) |
| --- | --- | --- | --- | --- | --- | --- |
| `servo-arc-provenance` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: refcount access and thin-arc construction without the allocation's provenance (servo/stylo#299, #210); undefined behaviour under both of Miri's aliasing models | `miri` | Never offered (owner decision) | Viper PR #21 (V5b-SERVO-ARC), tests in #22 | 90 / 124 |
| `servo-arc-header-slice` | `servo_arc`: `lib.rs`; detail in [`servo_arc/PATCHES.md`](servo_arc/PATCHES.md) | Soundness: `HeaderSlice` reaches its items through a reference to the sized struct (servo/stylo#210); undefined behaviour under Stacked Borrows | `miri` | Never offered (owner decision) | Viper PR #43 (V10a part 1) | 13 / 137 |
| `selectors-attr-flag-hook` | `selectors`: `parser.rs` | Chrome match: a `Parser` hook, `parse_case_sensitive_attribute_flag`, lets a parser refuse the `s` attribute flag while parsing, so `[a=b s]` fails as a parse error of that selector and a forgiving list (`:is()`, `:where()`) drops just that item; Chrome 154 rejects the flag (Viper's selector goldens `attr-flag-s`, `attr-flag-s-nested`, `attr-flag-s-forgiving`); upstream accepts it and has no hook | `viper-style::api fork_patches::attr_flag_hook_rejects_s_while_parsing`, `viper-style::api fork_patches::attr_flag_hook_lets_forgiving_lists_drop_the_item` | Candidate; draft to the owner first | Viper PR #47 (V10a part 2) | 25 / 57 |
| `element-data-checked` | `stylo`: `data.rs` | Soundness: `ElementDataWrapper::borrow` and `borrow_mut` are safe functions that hand out `&ElementData` and `&mut ElementData` from an `UnsafeCell`, and upstream checks the borrow flag only under `debug_assertions`, so a release build lets safe code alias `&mut`; the flag is now checked in every build, and `try_borrow`/`try_borrow_mut` report a conflict | `viper-style::api fork_patches::element_data_checked_try_borrow_mut_refuses_a_second_borrow` | Candidate; draft to the owner first | Viper V10a part 3a | 47 / 23 |
| `chrome-prefs` | `stylo_static_prefs`: `preferences.toml`, `src/lib.rs` (its test) | Chrome match: compiled defaults instead of run-time preferences, each justified by a Viper test: `layout.threads` 0 (no style thread pool; Viper styles sequentially), `layout.css.has-selector.enabled`, `layout.css.nth-child-of.enabled` and `layout.grid.enabled` true (Chrome 154 parses `:has()`, `:nth-child(An+B of S)` and `display: grid` in stylesheets; upstream's servo defaults refuse them); since Viper V13b part 2f1 also `layout.writing-mode.enabled` and `layout.css.grid-template-subgrid-value.enabled` true (Chrome 154 computes `writing-mode` and the `subgrid` value of `grid-template-columns` and `-rows`; upstream's servo defaults drop both declarations); Viper V10b-6's `layout.container-queries.enabled` moved to `servo-container-queries` in Viper V10b-10a | `viper-style::api fork_patches::chrome_prefs_defaults`, and `fork_patches::chrome_prefs_writing_mode_and_subgrid_compute` | Chrome-specific; not offered | Viper V10a part 3b; extended by Viper V13b part 2f1 and V10b-6 | 27 / 5 |
| `dom-apis-local-name` | `stylo`: `dom_apis.rs` | Chrome match and consistency: `query_selector`'s fast paths for a lone or rightmost type selector compared `TElement::local_name` for equality, while selector matching asks `Element::has_local_name`; both spots now ask `has_local_name`, so Viper's ASCII case-insensitive names of non-HTML elements hold there too (Chrome 154 matches `foreignobject` and `foreignObject` against SVG's `foreignObject`; Viper's selector goldens `svg-lower-case-name`, `svg-upper-case-name`) | `viper-style::api fork_patches::dom_apis_local_name_matches_mixed_case_svg_names`, and the selector golden suite through `dom_apis` | Candidate; draft to the owner first | Viper V10a part 3a | 17 / 0 |
| `chrome-font-size-precision` | `stylo`: `values/specified/font.rs` | Chrome match: computed font sizes stay exact in servo builds (Chrome 154: `1.17em` of 16px is 18.72px, `smaller` of 16px is 13.3333px); upstream quantizes them to 10 bits of precision, as Gecko does (18.7188px, 13.3281px), which also shifts the em lengths and line heights derived from them (Viper's computed-value goldens `ua-structure-font-size`, `cascade-inherit-font-size`); Gecko builds still quantize | `viper-style::cascade fork_patches::chrome_font_size_precision`, and the computed-value golden suite | Chrome-specific; not offered | Viper V10a part 4 | 11 / 0 |
| `chrome-display-ruby` | `stylo`: `values/specified/box.rs` | Chrome match: servo builds parse and report `display: ruby` and `ruby-text` (Chrome 154 gives them to `ruby` and `rt` and reports them; Viper's golden `ua-text-display`), and a blockified ruby serializes as `block ruby`; upstream has them in Gecko builds only. The other internal ruby values, which Chrome refuses, and the Gecko ruby style adjustments stay Gecko-only | `viper-style::cascade fork_patches::chrome_display_ruby`, and the computed-value golden suite | Chrome-specific; not offered | Viper V10a part 4 | 19 / 43 |
| `owned-slice-into-raw` | `stylo_traits`: `owned_slice.rs` | Soundness: `From<Box<[T]>> for OwnedSlice` took the pointer with `as_mut_ptr()` and then moved the box into `mem::forget`, which under Stacked Borrows invalidates the pointer, so every later access was undefined behaviour (Miri reports it for any `calc()` expression Stylo parses, such as the one in Viper's user-agent sheet); `Box::into_raw` hands the allocation over without the move | `miri` (Viper's `viper-style` library tests parse its user-agent sheet under both models) | Candidate; draft to the owner first | Viper V10a part 4 | 12 / 13 |
| `style-struct-arc-borrow` | `stylo`: `properties/properties.mako.rs` | Soundness: `StyleStructRef::Borrowed` held a `&T` to a style struct, and `StyleStructRef::build` rebuilt its `Arc` with `Arc::from_raw_addrefed`; a reference to the struct grants no access to the refcount in front of it, so the first restyle was undefined behaviour under both of Miri's aliasing models. The builder now keeps `ArcBorrow`s from servo's new `ComputedValuesInner::borrow_<struct>()` (`Arc::borrow_arc`) | `miri` (Viper's `viper-style` library tests restyle documents under both models) | Candidate; draft to the owner first (Gecko's `ComputedValues` would need the same accessor) | Viper V10b part 1 | 34 / 0 |
| `stylesheet-contents-arc` | `stylo`: `stylesheets/stylesheet.rs`, `stylist.rs` | Soundness: `CascadeData::collect_applicable_media_query_results_into` rebuilt a stylesheet's contents `Arc` with `Arc::from_raw_addrefed` from the `&StylesheetContents` of `StylesheetInDocument::contents`, undefined behaviour under both models whenever a stylist is flushed. `StylesheetInDocument` gains `contents_arc`, which servo's `Stylesheet` and `DocumentStyleSheet` answer with `Arc::borrow_arc`; the provided default keeps upstream's assumption for other implementors | `miri` | Candidate; draft to the owner first | Viper V10b part 1 | 43 / 0 |
| `rule-tree-raw-box` | `stylo`: `rule_tree/unsafe_box.rs`, `rule_tree/core.rs` | Soundness: the rule tree's `UnsafeBox` wrapped `ManuallyDrop<Box<T>>` and was copied with `ptr::read`, and `RuleNode::push_on_free_list` stored a pointer cast from `&RuleNode` that `swap_free_list_and_gc` later freed with `Box::from_raw`: undefined behaviour under both models. `UnsafeBox` now holds the pointer from `Box::into_raw`, and the free list stores that pointer | `miri` | Candidate; draft to the owner first | Viper V10b part 1 | 46 / 52 |
| `has-subtree-iterative` | `selectors`: `matching.rs` | Robustness and Chrome match: `matches_relative_selector_subtree`, the `:has()` subtree search, called itself once per tree level, so `:has()` on a document tens of thousands of levels deep overflowed the stack and aborted the process (`html:has(span)` at depth 50,000), where Chrome 154 answers; the walk is now iterative with the same order, selector flags and early return | `viper-style::api deep_trees_are_walked_without_recursion` | Candidate; draft to the owner first | Viper V10b part 1 | 31 / 0 |
| `thread-cache-miri-root` | `stylo`: `bloom.rs`, `sharing/mod.rs` | Test tooling: Stylo leaks its bloom-filter and style-sharing thread-local caches by design, once per thread that styles, and Miri's leak check reported them whenever a test thread that restyled exited, so a Miri run that restyles needed `-Zmiri-ignore-leaks`, which hides every other leak too. Under `cfg(miri)` only, each of the two allocations is registered with Miri as a static root (`miri_static_root`) when it is made: this exempts exactly those two allocations from Miri's leak report and nothing else. What is leaked, and when, is unchanged, and builds outside Miri are upstream's code | `miri` (Viper's restyling library tests pass Miri's leak check) | Candidate; draft to the owner first | Viper V10b part 1 | 24 / 0 |
| `matches-simple-selector-inline` | `selectors`: `matching.rs` | Performance: `matches_simple_selector` is `#[inline(always)]`. It has two callers, the loops over a compound's simple selectors in `matches_compound_selector` (selector matching) and in `matches_compound_selector_from` (Stylo's invalidation). The optimizer merges it into its caller only while one of the two is reachable for an element type; once an embedder uses both (Viper's incremental restyle makes the invalidation reachable for its element), the function stays out of line and every simple selector costs a call: measured against Viper's build before on a Linux bench host, `querySelectorAll` of 20 selectors over 10,000 elements ran 3.86% more instructions and took 1.034 to 1.037 times as long, and a restyle from scratch of 10,000 elements with 50 rules ran 2.85% more and took 1.033. With the attribute the function is part of both callers again: the queries run 1.58% fewer instructions than before the invalidation was reachable and take 0.990 to 0.994, the restyle runs the same instructions within 0.02% and takes 1.007 to 1.022 (40 pairs each, two builds). An `#[inline]` hint is not enough (the build is unchanged). No behaviour changes | `viper-style::selector_goldens selector_suite_matches_chrome_goldens` (behaviour kept); the paired measurement in Viper's `viper-style` README (the effect) | Candidate; draft to the owner first | Viper V10b part 3 | 8 / 0 |
| `selectors-compound-is-alias` | `selectors`: `parser.rs` | Chrome match: a `Parser` hook, `parse_compound_is_alias`, names an alias of `:is()` whose argument is a non-empty, non-forgiving, comma-separated list of compound selectors, as Chrome 154's legacy `:-webkit-any()` takes (Viper's selector goldens `names-webkit-any-list`, `names-webkit-any-complex`, `names-webkit-any-empty`, `names-webkit-any-invalid-item`, `names-webkit-any-in-not`); the existing `is_is_alias` parses the forgiving complex-selector list of `:is()` instead, and upstream has no `:-webkit-any()` | `viper-style::api fork_patches::compound_is_alias_parses_webkit_any` | Candidate; draft to the owner first | Viper V10b part 4a | 45 / 39 |
| `chrome-dom-api-pseudos` | `stylo`: `servo/selector_parser.rs` | Chrome match: three variants for the names Chrome 154's DOM selector APIs accept and servo's parser refuses, which only an embedder's DOM-API parser built until `chrome-selector-names` made servo's parser build them in stylesheets too: `NonTSPseudoClass::Dir` (the `:dir()` identifier as written), `NonTSPseudoClass::Unmatched` (any other such pseudo-class, holding its serialization) and `PseudoElement::Unmatched`, appended last (`PSEUDO_COUNT` follows it; it serializes as `::-internal-unmatched`, cascades lazily and accepts user-action pseudo-classes after it; since `selector-names-hook` it holds its name, from which what may follow it is decided). Neither pseudo-class has element state | `viper-style::api fork_patches::chrome_dom_api_pseudos_parse_and_match_nothing` | Chrome-specific; not offered | Viper V10b part 4a | 35 / 42 |
| `servo-first-line` | `stylo`: `servo/selector_parser.rs`, `data.rs`, `properties/data.py` | Chrome match: servo mode had no ::first-line; Chrome 154 styles it. `PseudoElement::FirstLine`, the fifth eager pseudo-element (after `FirstLetter`; `EAGER_PSEUDO_COUNT` 5, and `data.rs`'s empty eager array is sized by the count instead of four `None`s), parsed as `::first-line` and the CSS 2 `:first-line`, serialized as `::first-line`, with `is_first_line()` true and the `APPLIES_TO_FIRST_LINE` property restriction. Servo builds also apply `visibility` to the first line, as Blink 154 does (`valid_for_first_line` in `css_properties.json5`) and Gecko does not. The cascade's first-line reparenting was already shared code. Servo has no first-line layout, which is why upstream lacks it (register entry `adr:layout-first-line`) | `viper-style::api fork_patches::servo_first_line_parses_and_restricts`, and the computed-value golden suite (`first-line.html`) | Candidate; draft to the owner first | Viper V10b part 4b2 | 32 / 43 |
| `stack-limit-miri` | `stylo`: `context.rs` | Test tooling: Stylo's stack-depth sanity check assumes a contiguous stack, which Miri does not model. `StackLimitChecker` records the address of a local variable when a thread-local style context is made, and `limit_exceeded`, which Stylo's invalidation calls (`TreeStyleInvalidator`), checks in debug builds that a later local lies within the style thread's stack size of it; under Miri every local is its own allocation at an address of Miri's allocator, so the check failed Viper's incremental-restyle tests under Miri (`distance_to_stack_bottom <= max_allowable_distance`), and `new`'s subtraction could overflow. Under `cfg(miri)` only, `limit_exceeded` returns `false` before its checks and `new` saturates; builds outside Miri are upstream's code | `miri` (Viper's slow Miri run requires its two incremental-restyle tests by name, under both models) | Candidate; draft to the owner first | Viper V13b part 2f1 | 22 / 0 |
| `chrome-layout-properties` | `stylo`: `properties/longhands.toml`, `properties/helpers/animated_properties.mako.rs`, `values/specified/transform.rs`, `values/specified/box.rs` (its test) | Chrome match: servo builds parse `contain`, `backdrop-filter` and `offset-path` (upstream: behind `layout.unimplemented`), `content-visibility` and `transform-box` (upstream: Gecko only; with `content-visibility`'s animation impls, and all five boxes), as Chrome 154 does; their names in `will-change` follow. Viper's layout corpus loses no value with them | `viper-style::api fork_patches::chrome_layout_properties_parse_and_compute`, and viper-layout's `contained_floats_stay_inside_their_box` | Chrome-specific; not offered | Viper V10b-6 | 16 / 24 |
| `chrome-selector-names` | `stylo`: `servo/chrome_names.rs` (new), `servo/mod.rs`, `servo/selector_parser.rs` | Chrome match: servo's `SelectorParser` accepts the pseudo-class and pseudo-element names Chrome 154 accepts, with their argument grammars, in every origin, mapped to `chrome-dom-api-pseudos`' variants (`:dir()`, `:horizontal`, the `-internal-` names, `:host-context()`, `::cue`, `::highlight()`, every `::-webkit-*` name and the others of Viper's probe) or to aliases (`:-webkit-any-link`, `:-webkit-autofill`, `:-webkit-full-screen`), and `:-webkit-any()` through `selectors-compound-is-alias`; outside user-agent sheets it refuses what Chrome refuses and upstream parses: `-moz-*` and `-servo-*` names, `:lang()` with a string, `::color-swatch` and the `::slider-*` parts. Chrome's stylesheets and DOM selector APIs use one parser, so the tables moved here from Viper's DOM-API parser, one owner for both (Viper's computed goldens `sheet-names-*`: Chrome 154 keeps every rule naming one of these, `-internal-` names included); upstream refuses the names | `viper-style::api fork_patches::chrome_selector_names_in_stylesheets`, the computed-value golden suite (`sheet-names.html`) and the selector golden suite (`names-*`) | Chrome-specific; not offered | Viper V10b-4c1 | 370 / 115 |
| `chrome-after-pseudo-element` | `selectors`: `parser.rs`; `stylo`: `servo/chrome_after.rs` (new), `servo/chrome_names.rs`, `servo/mod.rs`, `servo/selector_parser.rs` | Chrome match: Chrome 154 decides per pseudo-element which pseudo-classes may follow it in its compound (`IsPseudoClassValidAfterPseudoElement`: the scrollbar pseudo-classes after the scrollbar parts, `:window-inactive` after `::selection`, the user actions after a `-webkit-` name and `::file-selector-button`, `:only-child` after the view-transition parts, ...), and `:is()`, `:where()` and `:not()` there contain only what may follow it; upstream accepts only a user-action pseudo-class after a pseudo-element that accepts state pseudo-classes. Two `Parser` hooks, `valid_after_pseudo_element` (its default is upstream's rule) and `parse_non_ts_pseudo_class_after_pseudo_element`, and servo's implementation of the first from Chrome's table, which reads from the name `PseudoElement::Unmatched` holds which of Chrome's pseudo-elements it is (since `selector-names-hook`; Viper's selector goldens `names-after-*`, its computed goldens `sheet-after-pseudo-*`) | `viper-style::cascade fork_patches::chrome_pseudo_classes_after_pseudo_elements`, the selector golden suite (`names-after-*`) and the computed-value golden suite (`sheet-after-pseudo.html`) | Chrome-specific; not offered | Viper V10b-4c2 | 426 / 201 |
| `servo-display-contents-unboxing` | `stylo`: `style_adjuster.rs` | Chrome match: servo builds apply CSS Display 3 Appendix B, by which `display: contents` computes to `none` on br, wbr, meter, progress, canvas, embed, object, audio, iframe, img, video, frame, frameset, input, textarea and select, on an `svg` whose parent is not an SVG element (or is a `foreignObject`), on SVG elements other than `svg`, `a`, `g`, `use`, `tspan` and `textPath`, and on MathML elements, as Chrome 154 reports (Viper's computed-value golden `display-contents-unboxing-display`); upstream applies the rule in Gecko builds only. The element names come from the local names, which servo's atom set lacks; pseudo-elements keep `contents` | `viper-style::cascade fork_patches::servo_display_contents_unboxing` | Candidate; draft to the owner first | Viper V10b-8 | 67 / 0 |
| `chrome-frames-and-textarea` | `stylo`: `style_adjuster.rs` | Chrome match: servo builds compute `display: block`, `position: static` and `float: none` for every HTML `frame` and `frameset` whatever the author asks, and a `textarea`'s `visible` overflow as `auto` on each axis, as Chrome 154 reports (Viper's computed-value goldens `frameset-display-*`, `overflow-controls-*`); upstream has neither rule | `viper-style::cascade fork_patches::chrome_frames_and_textarea` | Chrome-specific; not offered | Viper V10b-8 | 46 / 0 |
| `line-height-whole-percent` | `stylo`: `values/specified/font.rs` | Chrome match: servo builds compute a percentage `line-height` as the font size times the percentage truncated toward zero to a whole percent, divided by 100 once (`130.5%` of 20px is 26px, `0.5%` is 0px; Viper's computed-value golden `line-height-percent-line-height`); upstream computes the exact percentage | `viper-style::cascade fork_patches::line_height_whole_percent` | Chrome-specific; not offered | Viper V10b-8 | 38 / 23 |
| `line-height-calc-layout-unit` | `stylo`: `values/specified/font.rs` | Chrome match: servo builds resolve a line-height `calc()` that mixes a percentage and a length in 1/64 px, the percentage of the font size truncated to 1/64 px and the result truncated to 1/64 px (`calc(130.5% + 0px)` of 10px is 13.046875px); a `calc()` of lengths only stays exact and one of percentages only is that percentage, as Chrome 154 reports (Viper's computed-value goldens `line-height-calc-line-height`, `line-height-percent-line-height`); upstream keeps the exact value | `viper-style::cascade fork_patches::line_height_calc_layout_unit` | Chrome-specific; not offered | Viper V10b-8 | 55 / 16 |
| `stylesheet-parse-options` | `stylo`: `stylesheets/parse_options.rs` (new), `stylesheets/mod.rs`, `parser.rs`, `selector_parser.rs`, `servo/selector_parser.rs`, `stylesheets/stylesheet.rs`, `properties/declaration_block.rs`, `stylesheets/rule_parser.rs`, `stylesheets/supports_rule.rs`, `stylesheets/scope_rule.rs`; the struct literals of the tests in `servo/chrome_names.rs` and `servo/chrome_after.rs` | Embedder control: `ParseOptions`, a non-exhaustive set of options whose default is upstream's behaviour, is carried by `ParserContext` and by `SelectorParser` (from the context wherever one exists), and set through new `_with_options` forms of `StylesheetContents::from_str`, `Stylesheet::from_str` and `parse_style_attribute`. Its first field, `attribute_s_flag` (default `true`, Selectors 4 section 6.3), is what servo's `SelectorParser` answers to `selectors-attr-flag-hook`'s hook, so an embedder decides per sheet whether the `s` attribute flag parses; upstream always accepts it and has no per-parse options | `viper-style::cascade fork_patches::stylesheet_parse_options_decide_the_s_flag` | Candidate; draft to the owner first | Viper STY-FORK-1 | 167 / 97 |
| `value-grammar-options` | `stylo`: `stylesheets/parse_options.rs`, `stylesheets/supports_rule.rs`, `stylesheets/mod.rs`, `properties/mod.rs`, `values/specified/text.rs`, `properties/longhands.toml`, `properties/shorthands.toml`, `properties/shorthands.rs`; the test import of `values/specified/box.rs` | Embedder control and standard grammars: `ParseOptions` gains `value_aliases` (`(property, spelling, keyword)`: every whole identifier of a declared value equal to a listed spelling is replaced by its keyword before the grammar parses) and `refused_values` (`(property, keyword or function)`: a declared value holding a listed identifier or function name at any depth is invalid), both ASCII case-insensitive, in every origin, empty by default, applied in `PropertyDeclaration::parse_into` (style rules, `style` attributes, `@supports` and `CSS.supports`), and `unsupported_values` (the same shape), which only a supports condition reads (`@supports` and `CSS.supports`). Servo builds also parse the standard grammars upstream builds for Gecko or behind `layout.unimplemented`: `text-decoration-line`'s `spelling-error` and `grammar-error` (CSS Text Decoration 4 section 2.1), `text-indent`'s `hanging` and `each-line` (CSS Text 3 and 4 section 9.1), `text-wrap-style` with `pretty` and the `text-wrap` shorthand (CSS Text 4 sections 5.4 and 5.5), and `counter-reset`, `counter-increment` and `counter-set` (CSS Lists 3 section 4) | `viper-style::api value_grammar::value_grammar_options_take_a_test_list` (and `value_grammar_basic_answers`, Viper's Basic lists) | Options: candidate, draft to the owner first; grammars: candidate | Viper STY-FORK-2 | 191 / 153 |
| `servo-scripting-media-feature` | `stylo`: `servo/media_features.rs`; `stylo_atoms`: `static_atoms.txt` | Standard: Media Queries 5 section 9.1's `scripting` feature (`none`, `initial-only` or `enabled`), which upstream's servo list lacks; it evaluates to `enabled`, because the embedder runs scripts | `viper-style::api value_grammar::servo_scripting_media_feature_is_enabled` | Candidate; draft to the owner first | Viper STY-FORK-2 (from `chrome-value-parsing`, V10b-6) | 22 / 44 |
| `to-arc-provenance` | `stylo`: `stylesheets/container_rule.rs`, `values/computed/mod.rs`, `properties/properties.mako.rs` | Soundness: the container lookup rebuilt an `Arc` from `&ComputedValues` with `ComputedValues::to_arc` (`Arc::from_raw_addrefed`), a reference that grants no access to the reference count, undefined behaviour under both of Miri's aliasing models; the lookup now keeps the style it has (the caller's reference, or a clone of the element data's `Arc`), and servo builds drop `to_arc` | `viper-style::api containers::a_container_query_matches_under_miri` (the gate, named; Miri under both models in the PR's pool Miri job and the nightly slow run) | Candidate; draft to the owner first | Viper V10b-10a | 58 / 0 |
| `servo-container-queries` | `stylo`: `stylesheets/rule_parser.rs`, `values/specified/length.rs`, `stylesheets/container_rule.rs` (its tests), `properties/computed_value_flags.rs`, `properties/shorthands.toml`, `properties/shorthands.rs`; `stylo_static_prefs`: `preferences.toml`, `src/lib.rs` (its test) | Standard: servo builds parse CSS Conditional 5's `@container` rules (section 5.4), container relative lengths (section 7), `style()` queries (section 6.2) and the `container` shorthand (section 5.3), which upstream leaves out of servo builds (behind its other build feature) or turns off, and a new computed-value flag `DEPENDS_ON_CONTAINER_QUERY` marks every style whose matching evaluated a container condition | `viper-style::api containers::servo_container_queries_evaluate` | Candidate; draft to the owner first | Viper V10b-10a | 40 / 106 |
| `container-size-px` | `stylo`: `dom.rs`, `stylesheets/container_rule.rs`, `values/specified/length.rs`, `queries/values.rs`, `gecko/wrapper.rs` | Precision: `TElement::query_container_size` answers in CSS px instead of app units (1/60 px), so an embedder's sizes in finer units (1/64 px) compare and compute exactly in size queries and container relative lengths | `viper-style::api containers::container_size_px_compares_exactly` | Candidate; draft to the owner first | Viper V10b-10a | 52 / 27 |
| `line-height-number-f64` | `stylo`: `values/specified/font.rs`, `values/computed/font.rs`, `device/servo.rs`, `properties/longhands.toml`, `properties/shorthands.rs` | Precision: a unitless `line-height` keeps its number as written: in servo builds the computed number is `f64`, read from the number token's text or from a math function of numbers evaluated in `f64`, so the used line height (CSS Inline 3 section 5.1) is the exact number times the font size (`calc(20 / 12)` of 12px is 20px, where the `f32` number makes it a little less); the specified number keeps the parsed one for serialization, and the declaration is boxed for the extra `f64` | `viper-style::cascade fork_patches::line_height_number_f64` | Candidate; draft to the owner first | Viper V10b-8b | 252 / 51 |
| `selector-names-hook` | `stylo`: `stylesheets/parse_options.rs`, `stylesheets/mod.rs`, `servo/selector_parser.rs`, `servo/chrome_names.rs`, `servo/chrome_after.rs`, `matching.rs`, `properties/properties.mako.rs` | Embedder control: `ParseOptions` gains `selector_names`, an embedder's `SelectorNames` (default `None`), which servo's `SelectorParser` asks before its own names: the pseudo-classes and pseudo-elements beyond servo's (Selectors 4 section 3.9 makes any other name invalid), the names servo parses that sheets other than user-agent sheets refuse, what may follow a pseudo-element in its compound (Selectors 4 section 3.6.3 and each pseudo-element's own specification), and the aliases of `:is()` over compound selectors; its name methods receive the enclosing `SelectorParser`, so the selectors an embedder parses in a functional name's arguments keep the sheet's declared namespaces, origin, supports-condition mode and options as the enclosing selector does; without a hook the parser keeps this branch's default name tables (recorded in their own rows), and with one it answers upstream's names and rule where the hook does not. `PseudoElement::Unmatched` holds its name in ASCII lowercase (with `()` when functional) instead of a kind, so `PseudoElement` is `Clone` and no longer `Copy`, and servo's `ComputedValues` grows from 232 to 248 bytes; upstream has no such hook | `viper-style::api fork_patches::selector_names_hook_in_stylesheets_and_queries` | Candidate; draft to the owner first | Viper STY-FORK-3a; extended by Viper STY-FORK-3a2 | 281 / 335 |
| `percentage-number-f64` | `stylo`: `values/specified/percentage.rs`, `values/specified/length.rs`, `values/computed/length_percentage.rs` | Precision: a `<length-percentage>` written as a percentage keeps the number written: in servo builds the specified `NoCalcPercentage` and the computed `LengthPercentage` keep the fraction in `f64` (the percentage token's text over 100, when that rounds to the token's `f32` fraction), so a layout can resolve the percentage from the number written for any number of significant digits (CSS Values 4 section 5.1.3; `66.666667%` is not the `f32` fraction's `66.66667`); every value servo computes and serializes still comes from the `f32` fraction, and the computed `LengthPercentage` is 16 bytes instead of 8 | `viper-style::cascade fork_patches::percentage_number_f64` | Candidate; draft to the owner first | Viper V10b-8c | 142 / 52 |
| `servo-device-media-features` | `stylo`: `servo/media_features.rs`, `device/servo.rs`; `stylo_atoms`: `static_atoms.txt` | Standard: Media Queries 5's `color-gamut` (section 6.4), `dynamic-range` (6.5), `prefers-reduced-motion` (12.1), `prefers-contrast` (12.3) and `forced-colors` (12.4), which upstream's servo list lacks, so a query on one was unknown; each reads the embedder's setting from the `Device` (`DeviceMediaFeatures`, defaulting to `srgb`, `standard`, `no-preference`, `no-preference`, `none`), and `Device::forced_colors` returns its setting instead of a fixed `none` | `viper-style::cascade device::servo_device_media_features_take_the_config` | Candidate; draft to the owner first | Viper STY-FORK-DEVICE-1 | 190 / 118 |
| `vertical-align-longhand` | `stylo`: `stylesheets/parse_options.rs`, `properties/mod.rs`, `properties/shorthands.rs`, `properties/longhands.toml` | Embedder control: `ParseOptions` gains `vertical_align_longhand` (default `false`, upstream's CSS Inline 3 shorthand). When set, `vertical-align` takes CSS 2.1 section 10.8.1's grammar (one value) and still declares the box's three parts, which then hold its value alone, while declarations of `alignment-baseline`, `baseline-shift` and `baseline-source` go to the internal longhands `-x-alignment-baseline` (SVG 1.1 section 10.9.2's initial `auto`), `-x-baseline-shift` and `-x-baseline-source`, so they cascade on their own; the box's `baseline_source` is then always `auto`, and a layout reads `-x-baseline-source` where it is | `viper-style::cascade resolved::vertical_align_longhand_parts_cascade_apart` | Candidate; draft to the owner first | Viper STY-FORK-VA-1 | 121 / 91 |
| `engine-gates` | `stylo`: `properties/longhands.toml`, `properties/shorthands.toml`, `properties/shorthands.rs`, `properties/declaration_block.rs` (tests) | Standard: 46 longhands and 13 shorthands of CSS Scroll Snap 1, CSS Fragmentation 3, CSS Multi-column 1, CSS Box Sizing 4, CSS Motion Path 1, CSS Scroll Anchoring 1, CSS Anchor Positioning 1, CSS UI 4, CSS Overflow 3 and 4 and CSS Shapes 1 are built only for upstream's other engine, so servo builds dropped their declarations; their engine gates go, and they parse and compute in servo builds | `viper-style::cascade fork_patches::engine_gates` | Candidate; draft to the owner first | Viper V10b-7f | 66 / 96 |
| `normal-color-scheme-option` | `stylo`: `device/servo.rs` | Embedder control: CSS Color Adjustment 1 section 2.1 renders an element whose `color-scheme` is `normal` (it lists no scheme) in the user agent's default scheme, and upstream's servo `Device` always uses the user's preferred one for its system colours and `light-dark()`; the `Device` gains `NormalColorScheme` (`Preferred`, upstream's answer and the default, `Light` or `Dark`), read only for `normal`, so a listed scheme (`light`, `dark`, `light dark`) still follows the author and the preference | `viper-style::cascade device::normal_color_scheme_takes_the_config` | Candidate; draft to the owner first | Viper STY-FORK-SCHEME-1 | 53 / 82 |
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
| (none) | | | |

Since Viper V10b-10a (2026-10-24) no site is listed: its last one,
`ComputedValues::to_arc`, reached by the container lookup once servo builds
parse `@container`, was removed by `to-arc-provenance`, and the register
entry `adr:style-container-to-arc` closed.

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
- **Extension (Viper V13b part 2f1).** `layout.writing-mode.enabled` and
  `layout.css.grid-template-subgrid-value.enabled` true: upstream's servo
  defaults leave the `writing-mode` property out and refuse `subgrid` in
  `grid-template-columns` and `-rows`, so both declarations were dropped,
  every writing mode computed to `horizontal-tb` and a subgrid had no
  explicit tracks; Chrome 154 computes both (owner decision of 2026-10-14
  in Viper's ADR 0023). `test_basic_preferences` also expects the two
  values. In Viper's layout corpus (development split) the change loses no
  value: `block-18-vertical-rl` gains 24 values at each of its two
  viewports (156 to 180 of 263) and `grid-19-subgrid` is unchanged, since
  Viper's conversion already gave `subgrid` no explicit tracks. Laying out
  vertical modes and subgrids is still Viper's work, not the patch's.
- **Extension (Viper V10b-6), moved.** `layout.container-queries.enabled`
  true, so `container-type` and `container-name` parse. Since Viper
  V10b-10a the line is `servo-container-queries`' (standard container
  queries), with its marker, test line and proof.
- **Spots.** `stylo_static_prefs/preferences.toml` (six values) and
  `stylo_static_prefs/src/lib.rs` (the test's expectations).
- **Fork tests.** `test_basic_preferences`.
- **Viper proofs.** `chrome_prefs_defaults` (the six values) in the gate's
  patch proofs, and Viper's cascade tests `has_in_a_stylesheet`,
  `nth_child_of_in_a_stylesheet` and `display_grid_is_accepted`, the
  thread test `restyle_spawns_no_thread`, and since V13b part 2f1
  `chrome_prefs_writing_mode_and_subgrid_compute` (a stylesheet's
  `writing-mode: vertical-rl` and `grid-template-columns: subgrid` are
  computed); since V10b-6 `chrome_layout_properties_parse_and_compute`
  computes `container-type` (the preference's assertion moved to
  `servo_container_queries_evaluate` with the line).
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

## Patch `matches-simple-selector-inline`

- **Problem.** `matches_simple_selector`, the match of one simple selector
  against an element, is called from two loops: `matches_compound_selector`
  (inlined into `matches_complex_selector_internal`, the selector matcher)
  and `matches_compound_selector_from` (called by `TreeStyleInvalidator`,
  Stylo's invalidation). It carries no inlining attribute. While an
  embedder's build reaches only the matcher for an element type, LLVM
  merges the function into that one caller; when the invalidation is
  reachable for the same element type too, the function has two callers,
  is too large for the inliner's threshold and stays out of line, so every
  simple selector of every compound costs a call. In Viper this happened
  when the incremental restyle (V10b part 3) made `TElement::has_snapshot`
  able to return `true`: on a Linux bench host, against Viper's build
  before, `querySelectorAll` of 20 selectors over 10,000 elements ran 3.86%
  more instructions (49,461,589 per round against 47,625,054) and took
  1.034 to 1.037 times as long (80 pairs), and a restyle from scratch of
  10,000 elements with 50 author rules ran 2.85% more (190,421,428 against
  185,139,250) and took 1.033 (1.031–1.035). The control confirms the
  cause: that earlier build, with only this function kept out of line
  (`#[inline(never)]`), runs 4.36% and 2.37% more instructions.
  One codegen unit for the embedding crate does not change it (the two
  callers remain), and neither does `#[inline]`: with the hint the build
  has the same two call sites and runs the same instructions.
- **Change.** `#[inline(always)]` on `matches_simple_selector`. The function
  does not call itself directly (`:is()`, `:where()`, `:not()` and `:has()`
  re-enter the matcher through `matches_complex_selector_list` and
  `match_relative_selectors`, which this patch leaves as they are), so the
  attribute is honoured.
  With it Viper's build has no call site of the function, runs
  46,871,488 instructions per round of the queries (1.58% fewer than the
  earlier build) and 185,170,135 per restyle (0.02% more), and measures
  0.990 to 0.994 for the queries and 1.007 to 1.022 for the restyle over
  40 pairs each in two builds (Stylo taken from a local path, and from a
  local git source as a pinned commit is).
- **Spots.** `selectors/matching.rs`: `matches_simple_selector`.
- **Fork tests.** None: an attribute, with no behaviour to test. All 16
  `selectors` tests pass.
- **Viper proof.** `selector_suite_matches_chrome_goldens` (the
  `selector_goldens` test binary) keeps every result; the effect is a
  paired measurement, recorded in Viper's `viper-style` README, and the
  marker and ledger check fails if a rebase drops the patch.
- **Commands.** `cargo test -p selectors`.
- **Upstream.** Generic, so a candidate; a draft goes to the Viper owner
  first.

## Patch `selectors-compound-is-alias`

- **Problem.** Chrome 154's DOM selector APIs accept the legacy
  `:-webkit-any()`: an alias of `:is()` that takes a list of compound
  selectors, where a complex selector, an empty argument or one invalid
  item make the whole selector invalid. `selectors` has `is_is_alias`,
  which parses such an alias as `:is()` itself: a forgiving list of complex
  selectors. Neither Gecko nor Servo has `:-webkit-any()`.
- **Change.** A provided `Parser::parse_compound_is_alias(&self, name)`,
  `false` by default. When it returns `true`, `parse_functional_pseudo_class`
  parses the argument as comma-separated compound selectors
  (`parse_inner_compound_selector`, with the default namespace skipped as
  in `:is()`), none forgiven, and returns one `Component::Is` over them,
  which serializes as `:is(...)`.
- **Spots.** `selectors/parser.rs`: the `Parser` trait,
  `parse_functional_pseudo_class` and the new `parse_compound_is_alias`
  function; the test parser's hook.
- **Fork tests.** `compound_is_alias_takes_a_compound_list` (three
  compounds give one `:is()` of three; inside `:not()`; the name in upper
  case) and `compound_is_alias_refuses_what_is_accepts` (a descendant and a
  child combinator, an empty argument, a trailing comma, an unknown
  pseudo-class, a broken item and a pseudo-element each fail the whole
  selector, while `:is()` still forgives). All 18 `selectors` tests pass.
- **Viper proof.** `compound_is_alias_parses_webkit_any` (`viper-style`'s
  `api` test binary) and the selector goldens `names-webkit-any-*`.
- **Commands.** `cargo test -p selectors`.
- **Upstream.** Generic, so a candidate; a draft goes to the Viper owner
  first.

## Patch `chrome-dom-api-pseudos`

- **Problem.** Chrome 154's DOM selector APIs (`querySelector`,
  `querySelectorAll`, `matches`, `closest`) accept pseudo-classes and
  pseudo-elements servo's `SelectorParser` refuses: `:dir()`,
  `:host-context()`, `:horizontal` and 35 more, `::cue`, `::highlight()`,
  `::view-transition-*()`, every `::-webkit-*` name and others (Viper's ADR
  0017, "The selector names Chrome accepts"). Viper's own DOM-API parser
  needs values of servo's `SelectorImpl` for them; servo's enums have no
  variants to hold them.
- **Change.** `NonTSPseudoClass::Dir(Box<str>)` and
  `NonTSPseudoClass::Unmatched(Box<str>)`, appended to the enum: `Dir`
  serializes as `:dir(<ident>)`, `Unmatched` as the serialization it holds,
  and both have empty `state_flag`. The unit `PseudoElement::Unmatched`,
  appended last: `PSEUDO_COUNT` is `Unmatched as usize + 1`, it serializes
  as `::-internal-unmatched`, its `cascade_type` is `Lazy`, and it is the
  one pseudo-element whose `accepts_state_pseudo_classes` is `true` (Chrome
  accepts `::-webkit-scrollbar:hover` and `::cue:hover`). Servo's parser
  built none of the three until `chrome-selector-names`, which moved the
  names into it, so stylesheets keep those rules since then; what the
  embedder matches is its own decision (Viper: `:dir()` by directionality, Chrome's
  `:-internal-text-field` on text fields, everything else on nothing).
  Since `chrome-after-pseudo-element` the pseudo-element carried an
  `UnmatchedKind`; since `selector-names-hook` it holds its name, and
  `PSEUDO_COUNT` is its `index()` plus one, written out.
  The payloads are boxed strings, as `Lang`'s, so the enum keeps its size
  (24 bytes on 64-bit targets).
- **Spots.** `style/servo/selector_parser.rs`: the two enums, `PSEUDO_COUNT`,
  both `ToCss` implementations, `impl selectors::parser::PseudoElement`,
  `PseudoElement::cascade_type` and `NonTSPseudoClass::state_flag`. Every
  other match on the two enums in servo code has a wildcard arm.
- **Fork tests.** `chrome_dom_api_pseudos_serialize_with_no_state` (the
  three serializations, empty state, lazy cascade, the state pseudo-class
  rule and the last index) and
  `chrome_dom_api_pseudos_stay_out_of_servo_parsing` (servo's author parser
  still refuses `:dir(ltr)`, `:horizontal`, `::cue`, `::-webkit-scrollbar`
  and `::-internal-unmatched`). All 16 `stylo` library tests pass. Since
  `chrome-selector-names` the second is
  `chrome_dom_api_pseudos_internal_name_stays_unparsed`: only
  `::-internal-unmatched` stays refused.
- **Viper proof.** `chrome_dom_api_pseudos_parse_and_match_nothing`
  (`viper-style`'s `api` test binary) and the selector goldens `names-*`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific; not offered.

## Patch `servo-first-line`

- **Problem.** Chrome 154 styles `::first-line`: `getComputedStyle(element,
  '::first-line')` reports its values, and the first formatted line of a
  block takes them. Stylo's servo mode has no such pseudo-element (Servo
  lays out no first line), so its parser refuses `::first-line` and
  `:first-line`, and a stylesheet rule naming one is dropped. Gecko's
  cascade already computes first-line styles and reparents the elements on
  the first line (`FirstLineReparenting`, `Stylist::cascade_style_and_visited`,
  the `APPLIES_TO_FIRST_LINE` flags and the adjuster's
  `IS_IN_FIRST_LINE_SUBTREE`), all in shared code.
- **Change.** `PseudoElement::FirstLine`, an eager pseudo-element placed
  right after `FirstLetter`, so that the eager ones stay first and
  `eager_index()` and `from_eager_index()` keep working; `EAGER_PSEUDO_COUNT`
  is 5, and `cascade_type` returns `Eager` for it, as Gecko cascades it.
  `parse_pseudo_element` accepts `first-line` (the CSS 2 `:first-line`
  reaches it through `selectors::parser::is_css2_pseudo_element`); it
  serializes as `::first-line`; `is_first_line()` is true for it; and
  `property_restriction()` returns `APPLIES_TO_FIRST_LINE`.
  `may_generate_pseudo` keeps its default. In `data.rs` the empty eager
  array is `[const { None }; EAGER_PSEUDO_COUNT]` instead of a literal of
  four `None`s, which no longer has the array's length in servo builds
  (Gecko's count stays four). In `properties/data.py`, servo builds add
  `visibility` to the first-line properties: Blink 154 marks it
  `valid_for_first_line` (`third_party/blink/renderer/core/css/css_properties.json5`),
  and it is the one difference between that list and Stylo's among the 51
  properties Viper reports. Other differences outside those 51 (Chrome
  also lists `box-shadow` and `text-justify`; Stylo also lists SVG text
  properties and some font and text properties under other names) are
  left as they are.
- **Spots.** `style/servo/selector_parser.rs`: the enum, `ToCss`,
  `EAGER_PSEUDO_COUNT`, `is_first_line`, `cascade_type`,
  `property_restriction` and `parse_pseudo_element`; `style/data.rs`:
  `EMPTY_PSEUDO_ARRAY`; `style/properties/data.py`:
  `PropertyRestrictions.first_line`.
- **Fork tests.** `servo_first_line_parses_serializes_and_restricts`: the
  two spellings in any case parse in author selectors and
  `p::first-line::before` does not; the serialization; `is_first_line`,
  eagerness and the eager index round trip, the last eager pseudo-element;
  the restriction, which `color` and `visibility` pass and `margin-top` and
  `display` do not. All 17 `stylo` library tests pass.
- **Viper proof.** `servo_first_line_parses_and_restricts`
  (`viper-style`'s `api` test binary), and the computed-value golden
  fixture `first-line.html`: all 51 cases equal Chrome 154, after a full
  restyle and after incremental ones.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Candidate; draft to the owner first. Nothing sent.

## Patch `stack-limit-miri`

- **Problem.** `StackLimitChecker` (`style/context.rs`) is made with every
  `ThreadLocalStyleContext` and records the address of a local variable
  less the allowed stack size. `limit_exceeded`, which the invalidation
  calls before it descends (`TreeStyleInvalidator`,
  `style/invalidation/element/invalidator.rs`), compares a later local's
  address with it, and in debug builds first checks that the distance to
  the stack's bottom stays within `STYLE_THREAD_STACK_SIZE_KB` plus 10 KiB.
  That assumes one contiguous stack that grows downwards. Miri does not
  model one: each local is a separate allocation at an address of Miri's
  allocator, so the distance means nothing and grows with what was
  allocated since the checker was made. Viper's slow Miri run of
  2026-10-16 failed two incremental-restyle tests on it (`assertion failed:
  distance_to_stack_bottom <= max_allowable_distance` at
  `style/context.rs:582`), with no undefined behaviour and no leak; `new`'s
  subtraction can also overflow when Miri's address is below the limit.
  Real builds are unaffected: their stacks are contiguous.
- **Change.** Under `cfg(miri)` only (`cfg!(miri)`, which is `false`
  outside Miri), `limit_exceeded` returns `false` before its checks, so the
  invalidation recurses as it does on a real stack well within its limit,
  and `new` takes `saturating_sub`. Builds outside Miri run upstream's
  code; nothing else in either function changes. Neither of the other ways
  works: `SERVO_STYLE_THREAD_STACK_SIZE_KB` moves the threshold, and `new`
  then subtracts a larger limit from Miri's address; turning Stylo's debug
  assertions off under Miri hides every other assertion.
- **Spots.** `style/context.rs`: `StackLimitChecker::new` and
  `StackLimitChecker::limit_exceeded`.
- **Fork tests.** None: the checker is reached by an invalidation under
  Miri. All 17 `stylo` library tests pass.
- **Viper proof.** `miri`: Viper's slow Miri run (`miri_slow_all` in its
  `scripts/gate/miri.sh`) requires the two tests by name,
  `incremental::a_mixed_case_attribute_restyles_what_lowercase_selectors_read`
  and `incremental::display_none_subtrees_follow_changes_anywhere`, under
  Stacked Borrows and Tree Borrows; without the patch both fail on the
  assertion.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** A candidate for test tooling; draft to the owner first.

## Patch `chrome-layout-properties`

- **Problem.** Servo builds parse `contain`, `backdrop-filter` and
  `offset-path` only behind `layout.unimplemented`, and build
  `content-visibility` and `transform-box` for Gecko only, refusing
  `transform-box`'s `content-box` and `stroke-box`; Chrome 154 parses all
  of them, and `will-change` keeps their names. A stylesheet's
  declarations of them were dropped.
- **Change.** The two preference gates and the two Gecko-only engines go;
  `content-visibility`'s `Animate`, `ComputeSquaredDistance` and
  `ToAnimatedZero` impls, which the generated animation code needs, are
  built for servo too; servo's `all_transform_boxes_are_enabled` answers
  `true`. `container-type` and `container-name` follow from `chrome-prefs`.
  Viper's decision rule: with the patch, its layout corpus (development
  split) loses no value and its `contain` test meets Chrome's values (a
  contained float's box 50px tall, the next block at y 58), so every
  property landed.
- **Spots.** `style/properties/longhands.toml` (five properties),
  `style/properties/helpers/animated_properties.mako.rs` (the import and
  three impls), `style/values/specified/transform.rs`
  (`all_transform_boxes_are_enabled`) and `style/values/specified/box.rs`
  (the test module).
- **Fork tests.** `chrome_layout_properties_parse_in_servo_builds`; all 20
  `stylo` library tests pass.
- **Viper proofs.** `chrome_layout_properties_parse_and_compute` in the
  gate's patch proofs, and viper-layout's
  `contained_floats_stay_inside_their_box`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `chrome-selector-names`

- **Problem.** Chrome 154 parses the selectors of its stylesheets and of
  its DOM selector APIs with one parser, so both accept the same
  pseudo-class and pseudo-element names. Viper's DOM-API parser accepted
  Chrome's names (`chrome-dom-api-pseudos`), but stylesheets parse with
  servo's `SelectorParser`, which refused them: a rule whose selector list
  named `:dir()`, `:-webkit-any()`, `::-webkit-scrollbar` or another of them
  was dropped where Chrome keeps it. Servo also parses names Chrome refuses
  (`:-moz-meter-optimum`, `:lang("en")`, `::color-swatch`,
  `::slider-thumb`), so such rules were kept where Chrome drops them.
- **Change.** The names, their tables and their argument grammars move from
  Viper's DOM-API parser into `style/servo/chrome_names.rs`, which servo's
  `SelectorParser` consults before its own names, in every origin:
  `parse_non_ts_pseudo_class` (the 36 plain pseudo-classes of Viper's probe
  as `NonTSPseudoClass::Unmatched` with their serialization, and the three
  aliases), `parse_non_ts_functional_pseudo_class` (`:dir(<ident>)` as
  `NonTSPseudoClass::Dir`, `:host-context(<compound-selector>)` and
  `:active-view-transition-type(<ident>#)` as `Unmatched`),
  `parse_pseudo_element` (every `-webkit-` name and the 14 plain
  pseudo-elements as `PseudoElement::Unmatched`), the new
  `parse_functional_pseudo_element` (`::highlight(<ident>)`,
  `::cue(<compound-selector-list>)`, `::scroll-button(* | <direction>)` and
  the five `::view-transition-*(<pt-name-and-class-selector>)` parts) and
  `parse_compound_is_alias` (`-webkit-any`). Outside user-agent sheets the
  parser refuses `-moz-*` and `-servo-*` names (a pseudo-class or a
  pseudo-element), a string in `:lang()` (an identifier only) and
  `::color-swatch`, `::slider-fill`, `::slider-thumb` and `::slider-track`;
  user-agent sheets keep servo's own names. Viper's DOM-API parser now
  delegates every name to this parser and keeps only its own refusal of
  form state it cannot match yet. Viper's goldens of Chrome 154
  (`sheet-names.html`) decided the `-internal-` names: Chrome's stylesheets
  keep `:-internal-text-field`, `:-internal-autofill-selected` and
  `:-internal-dialog-in-top-layer` as its DOM APIs do, so no name is
  accepted by the DOM APIs alone.
- **Spots.** `style/servo/chrome_names.rs` (the tables, the grammars, the
  author-sheet refusals and the tests), `style/servo/mod.rs` (the module),
  `style/servo/selector_parser.rs` (the five parser methods, and the
  `chrome-dom-api-pseudos` comments and test that said servo never builds
  its variants).
- **Fork tests.** `chrome_selector_names_parse_in_every_origin`,
  `chrome_selector_names_refuse_bad_arguments` and
  `chrome_selector_names_refused_outside_user_agent_sheets`, and
  `chrome_dom_api_pseudos_internal_name_stays_unparsed`; all 23 `stylo`
  library tests and the 18 `selectors` tests pass.
- **Viper proofs.** `chrome_selector_names_in_stylesheets` in the gate's
  patch proofs, the computed-value golden suite (`sheet-names.html`, 51
  cases) and the selector golden suite (`names-*`, every result unchanged).
- **Commands.** `cargo test -p stylo --lib`, `cargo test -p selectors`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `chrome-after-pseudo-element`

- **Problem.** Chrome 154 accepts after each pseudo-element the
  pseudo-classes its parser's table names for it, in stylesheets and in its
  DOM selector APIs alike (Chromium's `IsPseudoClassValidAfterPseudoElement`
  and `IsSimpleSelectorValidAfterPseudoElement`, BSD,
  `third_party/blink/renderer/core/css/parser/css_selector_parser.cc` at
  154.0.8037.57): `::-webkit-scrollbar:horizontal`,
  `::-webkit-scrollbar-button:decrement`, `::-webkit-scrollbar:disabled`,
  `::selection:window-inactive`, `::-webkit-foo:focus-visible`,
  `::view-transition-old(*):only-child`, `::search-text:current`,
  `::scroll-marker:target-current`, `::scroll-button(up):focus`, and it
  refuses others that upstream takes (`::-webkit-scrollbar:focus`,
  `::view-transition-group(*):hover`, `::highlight(x):hover`,
  `::-webkit-scrollbar:-webkit-any(:hover)`). `:is()`, `:where()` and
  `:not()` are always valid there, and what they contain is checked against
  the same pseudo-element: forgivingly in the first two. Upstream accepts
  only a user-action pseudo-class (`:hover`, `:active`, `:focus`), and only
  after a pseudo-element that accepts state pseudo-classes.
- **Change.** `selectors`: the parsing state `CHECK_AFTER_PSEUDO_ELEMENT`,
  set after a pseudo-element that is not element-backed (and not after
  `::slotted()`), under which the pseudo-classes that follow it, and those
  inside its `:is()`, `:where()` and `:not()`, parse without upstream's
  checks; the compound then hands each simple selector to the new `Parser`
  hook `valid_after_pseudo_element` (`check_after_pseudo_element`, which
  forgives an invalid item of `:is()` or `:where()` as written and fails a
  `:not()` with one). The hook's default is upstream's rule, so upstream's
  tests and every other `Parser` are unchanged. `:-webkit-any()`
  (`selectors-compound-is-alias`) is refused after a pseudo-element, as
  Chrome refuses it. A second hook,
  `parse_non_ts_pseudo_class_after_pseudo_element` (by default
  `parse_non_ts_pseudo_class`), lets an embedder parse a name there that it
  refuses elsewhere (Viper refuses form state it cannot match yet, but
  `::-webkit-scrollbar:disabled` matches no element). The hooks are on
  `Parser`, not on the `PseudoElement` trait, which has no `SelectorImpl`
  to name the pseudo-classes. `stylo`: `chrome_after.rs` reads an
  `UnmatchedKind` (`ScrollbarPart`, `WebkitCustom`, `ViewTransitionPart`,
  `SearchText`, `ScrollMarker`, `ScrollMarkerGroup`, `ScrollButton`, `Cue`,
  `Column`, `Other`) from the name `PseudoElement::Unmatched` holds (since
  `selector-names-hook`; before, the variant carried the kind, set by
  `chrome_names.rs`); every name serializes, cascades and is indexed as
  before. `index()` and
  `from_eager_index` match the variants instead of `as usize` and a
  transmute, which need an enum without fields. `style/servo/chrome_after.rs`
  is servo's hook: Chrome's table for the kinds, `::selection`
  (`:window-inactive`), `::file-selector-button` (the user actions) and,
  for the other servo pseudo-elements, nothing (`::before`, `::after` and
  `::marker` take `:hover` only behind a runtime flag Chrome 154 leaves
  off). Chrome's user-action set adds `:focus-visible` and `:focus-within`.
- **The goldens decide.** Where Viper's goldens of Chrome 154 differ from a
  reading of the table they win: the plain `::cue` takes the user actions
  (`names-after-cue-hover`) and `::cue(b)` nothing; `::view-transition`
  itself takes no `:only-child` (`names-after-view-transition-only-child`);
  `::placeholder` takes no user action (`names-after-placeholder-hover`).
- **Spots.** `selectors/parser.rs`: the state flag and the three
  `SelectorParsingState` checks, the two `Parser` hooks, the compound loop,
  `check_after_pseudo_element` and `valid_after`, the refusals of a
  functional pseudo-class and of a non-user-action one, `:-webkit-any()`,
  and the test parser's switch. `style/servo/`: `chrome_after.rs` (new),
  `chrome_names.rs` (the kinds), `mod.rs`, `selector_parser.rs`
  (`Unmatched`'s kind, `index`, `from_eager_index`, `PSEUDO_COUNT`, the
  hook, the tests).
- **Fork tests.** `after_pseudo_element_default_keeps_upstream_rule` and
  `after_pseudo_element_parses_its_own_names` (`selectors`; every input of
  the first but `::before:-webkit-any(:hover)` gives the same result on the
  parser before the patch), `chrome_pseudo_classes_after_pseudo_elements`
  and `chrome_after_pseudo_element_indices_stay_distinct` (`stylo`); all 20
  `selectors` tests and 25 `stylo` library tests pass.
- **Viper proofs.** `chrome_pseudo_classes_after_pseudo_elements` (the
  `cascade` test binary) in the gate's patch proofs, the selector golden
  suite (`names-after-*`, 53 cases) and the computed-value golden suite
  (`sheet-after-pseudo.html`, 51 cases).
- **Commands.** `cargo test -p selectors`, `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `servo-display-contents-unboxing`

- **Problem.** CSS Display 3 Appendix B makes `display: contents` compute
  to `display: none` on the elements whose rendering is not their
  children's boxes: br, wbr, meter, progress, canvas, embed, object, audio,
  iframe, img, video, frame, frameset, input, textarea and select; on an
  `svg` element whose parent is not an SVG element; on SVG elements other
  than a nested `svg`, `a`, `g`, `use`, `tspan` and `textPath`; and on
  MathML elements. Chrome 154 reports `none` there (Viper's computed-value
  golden `display-contents-unboxing-display`: every HTML element of the
  list, the top-level `svg`, a `rect`, `math` and `mi`; `button`,
  `details`, `fieldset`, `legend`, a nested `svg` and a `g` keep
  `contents`). Stylo has the rule, `is_effective_display_none_for_display_contents`
  and the element half of `adjust_for_prohibited_display_contents`, in
  Gecko builds only, so servo builds reported `contents`.
- **Change.** Both functions and `is_topmost_svg_svg_element` are built in
  servo builds too, and `adjust` calls the adjustment in both. Servo's
  atom set has none of the element names, so a small `element_name!`
  macro takes them from `atom!` in Gecko builds and from the local names
  (`local_name!`) in servo builds. The pseudo-element branch, which turns
  `display: contents` on `::before` and `::after` into `inline`, stays
  Gecko-only: servo builds leave a pseudo-element's `contents` alone, as
  before, and no Viper golden shows Chrome changing it. Frames and
  framesets then compute `block` (`chrome-frames-and-textarea`).
- **Spots.** `style/style_adjuster.rs`: the element-name type and macro,
  `is_topmost_svg_svg_element`, `is_effective_display_none_for_display_contents`,
  `adjust_for_prohibited_display_contents` and its call in `adjust`.
- **Fork tests.** None: the adjustment needs an element and a style
  context, which the fork's tests do not build. All 27 `stylo` library
  tests pass.
- **Viper proofs.** `servo_display_contents_unboxing` (the cascade test
  binary) and the computed-value fixture `display-contents-unboxing.html`;
  `viper-layout` reads the computed `display` for these elements since.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Candidate (it is the specification's rule); draft to the
  owner first.

## Patch `chrome-frames-and-textarea`

- **Problem.** Chrome 154 computes `display: block`, `position: static`
  and `float: none` for every HTML `frame` and `frameset` element, whatever
  the author asks (`display: none` and `contents` included; both elements
  always have a box), and computes a `textarea`'s `overflow-x` and
  `overflow-y` of `visible` as `auto` (Viper's computed-value goldens
  `frameset-display-display`, `-position`, `-float` and
  `overflow-controls-overflow-x`, `-y`). Upstream has neither rule.
- **Change.** In servo builds, a new adjustment right after the Appendix B
  one: for an HTML `frame` or `frameset` (not a pseudo-element) it sets the
  three values; for an HTML `textarea` it turns each `visible` axis into
  `auto`, before `adjust_for_overflow` reconciles the two axes.
- **Spots.** `style/style_adjuster.rs`: `adjust_for_frames_and_textarea`
  and its call in `adjust`.
- **Fork tests.** None, for the reason above. All 27 `stylo` library
  tests pass.
- **Viper proofs.** `chrome_frames_and_textarea` (the cascade test binary),
  the computed-value fixtures `frameset-display.html` and
  `overflow-controls.html`, and `viper-layout`'s frameset boxes
  (`framesets_lay_out_their_frames_as_chrome`).
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `line-height-whole-percent`

- **Problem.** Chrome 154 computes a percentage `line-height` from the
  percentage truncated toward zero to a whole percent: `130.5%` of 20px is
  26px, `99.9%` of 13px is 12.87px, `0.5%` is 0px (Viper's computed-value
  golden `line-height-percent-line-height`, four font sizes). Stylo
  computes the exact percentage (26.1px, 12.987px, 0.05px).
- **Change.** In servo builds the percentage arm of
  `LineHeight::to_computed_value` multiplies the font size by the whole
  percent and divides by 100 once, in `f64`, so a whole `130%` of 20px is
  exactly 26px (`1.3_f32 * 20.0` is not). The parser keeps the percentage
  as its number over 100 in `f32`, so `whole_percent` first recovers the
  number as written from the fraction's shortest decimal (`1.3_f32 *
  100.0` is not exactly 130), then truncates it.
- **Spots.** `style/values/specified/font.rs`: `whole_percent`,
  `whole_percent_line_height`, the percentage arm, the test module.
- **Fork tests.** `line_height_whole_percent_truncates_the_number_as_written`;
  all 27 `stylo` library tests pass.
- **Viper proofs.** `line_height_whole_percent` (the cascade test binary,
  worked by hand) and the computed-value fixture `line-height-percent.html`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `line-height-calc-layout-unit`

- **Problem.** Chrome 154 resolves a line-height `calc()` that mixes a
  percentage and a length in its layout unit, 1/64 px: the percentage is of
  the font size truncated to 1/64 px, and the result is truncated to
  1/64 px (`calc(130.5% + 0px)` of 10px is 13.046875px, reported as
  13.0469px; `calc(1000% + 0px)` of 10.99px is 109.84375px, not 109.890625px).
  A `calc()` of lengths only is that length (`calc(13.05px + 0px)` is
  13.05px) and one of percentages only is that percentage
  (`calc(50% + 50%)` of 10.99px is 10.99px) (Viper's computed-value
  goldens `line-height-calc-line-height` and
  `line-height-percent-line-height`). Stylo computes every `calc()`
  exactly.
- **Change.** In servo builds the `calc()` arm reads the specified
  expression's leaves (its computed form drops a `+ 0px`, which Chrome
  keeps): lengths only give the computed length, percentages only go
  through `line-height-whole-percent`'s rule, and a mix is resolved against
  the font size truncated to 1/64 px and truncated to 1/64 px
  (`layout_unit`).
- **Spots.** `style/values/specified/font.rs`: `layout_unit`, the `calc()`
  arm, the test module.
- **Fork tests.** `line_height_calc_layout_unit_truncates_to_64ths`; all 27
  `stylo` library tests pass.
- **Viper proofs.** `line_height_calc_layout_unit` (the cascade test
  binary, worked by hand) and the computed-value fixture
  `line-height-calc.html`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Chrome-specific, so not offered.

## Patch `stylesheet-parse-options`

- **Problem.** What Stylo's parsers accept is fixed by the build and by
  global preferences: an embedder cannot parse one document's style sheets
  at one CSS level and another's at another, nor decide a choice per parse.
  Viper needs that for the `s` attribute flag ([Selectors 4 section
  6.3](https://drafts.csswg.org/selectors-4/#attribute-case)): its Basic
  dialect accepts the flag, its legacy dialect refuses it, and both style
  documents in one process. The earlier patch `chrome-attr-s-flag` made
  servo's `SelectorParser` refuse the flag for every sheet.
- **Change.** A new type `style::stylesheets::ParseOptions`
  (`#[non_exhaustive]`; `Default` is upstream's behaviour; built as
  `ParseOptions::default()` with fields set after) is a public field of
  `ParserContext`, which `ParserContext::new` sets to the default, and of
  `SelectorParser`. Every `SelectorParser` built from a context (style
  rules and nested rules, `@supports selector()`, `@scope`) copies the
  context's options; the context-free `parse_author_origin_no_namespace`
  uses the default. `StylesheetContents::from_str_with_options`,
  `Stylesheet::from_str_with_options` and
  `parse_style_attribute_with_options` take the options; the forms without
  `_with_options` pass the default, so their behaviour is unchanged. The
  first field, `attribute_s_flag` (default `true`), is servo's answer to
  `selectors-attr-flag-hook`'s `parse_case_sensitive_attribute_flag`: when
  `false`, a selector using the flag is invalid, so a style rule whose
  selector list holds it is dropped and a forgiving list drops just that
  selector. Declaration blocks hold no selector, so the field changes
  nothing in a style attribute; its `_with_options` form is there for
  later fields. Not covered: the CSSOM's rule insertion and the other
  entry points parse with the default.
- **Spots.** `style/stylesheets/parse_options.rs` (new: the type and the
  test module), `style/stylesheets/mod.rs` (module and export),
  `style/parser.rs` (the field and its default), `style/selector_parser.rs`
  (the field; `parse_author_origin_no_namespace`),
  `style/servo/selector_parser.rs` (the hook, which replaces
  `chrome-attr-s-flag`'s, and that patch's test module removed),
  `style/stylesheets/stylesheet.rs` (the two `from_str_with_options` and
  `parse_rules`), `style/properties/declaration_block.rs`
  (`parse_style_attribute_with_options`), `style/stylesheets/rule_parser.rs`,
  `supports_rule.rs` and `scope_rule.rs` (the options copied), and the
  test-only struct literals in `style/servo/chrome_names.rs` and
  `chrome_after.rs` (the default).
- **Fork tests.** `stylesheet_parse_options_default_accepts_the_s_flag`,
  `stylesheet_parse_options_refuse_the_s_flag` and
  `stylesheet_parse_options_style_attribute`; all 29 `stylo` library
  tests and the 20 `selectors` tests pass.
- **Viper proof.** `stylesheet_parse_options_decide_the_s_flag` (the
  cascade test binary): the legacy dialect refuses the flag in a
  stylesheet, the Basic dialect accepts it.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Generic (no default changes), so a candidate; a draft goes
  to the Viper owner first, and nothing is sent without the owner's
  go-ahead.

## Patch `value-grammar-options`

- **Problem.** Servo builds parse some standard grammars only for Gecko or
  behind `layout.unimplemented`, and an embedder had no way to give one
  document extra spellings or refusals and another the standards'
  answers.
- **Change.** `ParseOptions` (from `stylesheet-parse-options`) gains two
  lists, empty by default, so the default is the grammars:
  `value_aliases: &'static [(&'static str, &'static str, &'static str)]`,
  `(property, spelling, keyword)`, and
  `refused_values: &'static [(&'static str, &'static str)]`,
  `(property, keyword or function)`, and a third,
  `unsupported_values` (the same shape), which only a supports condition
  reads (`Declaration::eval`, so `@supports` and `CSS.supports`: CSS
  Conditional 3 section 6.1's values without a usable level of support).
  `PropertyDeclaration::parse_into`
  first asks `ParseOptions::rewrite_value` about the declared property's
  name (shorthand or longhand as declared, no expansion to sub-properties):
  when a list names it, the value's tokens are walked at every nesting
  depth; a listed identifier or function name refuses the declaration,
  and each whole identifier equal to a listed spelling is replaced by its
  keyword, and the rewritten text is parsed instead. Both compare ASCII
  case-insensitively and apply in every origin; refusals are matched on
  the value as written. The grammar enablements carry no option:
  `text-decoration-line`'s Gecko keyword list in servo builds,
  `TextIndent::parse` without its servo early stop, `counter-reset` and
  `counter-increment` without `layout.unimplemented`, `counter-set`,
  `text-wrap-style` (with `pretty`, servo builds) and the `text-wrap`
  shorthand. `chrome-value-parsing`'s derive aliases and spellings
  (`TextAlignKeyword`, `TextAlign::MatchParent`, `CursorKind`,
  `FontSizeKeyword`, `unicode-bidi`'s `servo_aliases`) and its refusals
  (`TextTransform::validate_mixed_flags`, `reversed()`, `symbols()`) are
  gone: those files equal upstream's again. Not covered: a value holding
  `var()` is substituted later with the default options.
- **Spots.** `style/stylesheets/parse_options.rs` (the fields,
  `rewrite_value`, `unsupported` and the test module),
  `style/stylesheets/supports_rule.rs` (`Declaration::eval`),
  `style/stylesheets/mod.rs` (the
  crate export), `style/properties/mod.rs` (`parse_into`, and upstream's
  body as `parse_value_into`), `style/values/specified/text.rs`
  (`TextDecorationLine`, `TextIndent::parse`),
  `style/properties/longhands.toml` (the three counter properties,
  `text-wrap-style`), `style/properties/shorthands.toml` and
  `style/properties/shorthands.rs` (`text-wrap`), and the import of
  `chrome-layout-properties`' test in `style/values/specified/box.rs`,
  whose helper this module's tests now hold.
- **Fork tests.** `value_grammar_options_default_parses_the_grammars`,
  `value_grammar_options_alias_whole_identifiers`,
  `value_grammar_options_refuse_names_at_any_depth` and
  `value_grammar_options_refuse_unsupported_values_in_supports_only`.
- **Viper proofs.** `value_grammar_basic_answers` (Viper's Basic lists) and
  `value_grammar_options_take_a_test_list` (a test list through the
  options), in the gate's patch proofs.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** The options are generic, so a candidate; the grammar
  enablements are upstream's own grammars. A draft goes to the Viper owner
  first, and nothing is sent without the owner's go-ahead.

## Patch `servo-scripting-media-feature`

- **Problem.** Upstream's servo media feature list lacks Media Queries 5
  section 9.1's `scripting` feature, so `(scripting)` was an unknown
  feature (Gecko's evaluator asks its document).
- **Change.** A `Scripting` keyword (`none`, `initial-only`, `enabled`) and
  a `scripting` feature that always evaluates to `enabled`, because the
  embedder runs scripts (Viper's S3-SCRIPTING-OFF may add a device flag
  for documents whose scripts are off), with its atom. Split from
  `chrome-value-parsing` unchanged.
- **Spots.** `style/servo/media_features.rs` (the feature and its test
  module) and `stylo_atoms/static_atoms.txt` (the atom).
- **Fork tests.** `servo_scripting_media_feature_parses`.
- **Viper proof.** `servo_scripting_media_feature_is_enabled`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Standard, so a candidate; a draft goes to the Viper owner
  first.

## Patch `to-arc-provenance`

- **Problem.** `ContainerCondition`'s container lookup turned the
  container's style into an `Arc` with `ComputedValues::to_arc`, which
  calls `Arc::from_raw_addrefed(self)` on `&self`. That reference covers
  the values only, not the reference count in front of them: the fault
  `style-struct-arc-borrow` and `stylesheet-contents-arc` fix elsewhere,
  undefined behaviour under both of Miri's aliasing models. Servo builds
  reach it as soon as they parse `@container` (`servo-container-queries`).
- **Change.** `ContainerLookupResult` gains a lifetime and holds a
  `ContainerStyle`: `Borrowed` for the style the caller passed down (an
  originating element's, while a pseudo-element is matched) and `Shared`
  for a clone of the element data's primary `Arc`; it dereferences to
  `ComputedValues`. `Context::for_container_query_evaluation` borrows the
  style instead of taking an `Arc`, as it only ever read it. Servo builds
  drop `ComputedValues::to_arc`; Gecko's (`gecko.mako.rs`) is unchanged.
- **Spots.** `style/stylesheets/container_rule.rs` (`ContainerStyle`,
  `traverse_container`, `valid_container_info`, `find_container`,
  `matches`), `style/values/computed/mod.rs`
  (`for_container_query_evaluation`) and
  `style/properties/properties.mako.rs` (servo's `to_arc`).
- **Fork tests.** None: a container lookup needs a `TElement` with element
  data and a stylist, which the fork's tests do not build. All `stylo`
  library tests pass.
- **Viper proof.** `a_container_query_matches_under_miri` styles a
  document whose rules match through both lookups (an element's, and a
  `::before`'s through its originating element's style). The gate runs it as
  a named proof; Miri runs it under Stacked Borrows and Tree Borrows in the
  nightly slow run (`--test api`), and Viper's PR recorded both runs.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** A generic soundness fix: candidate, draft to the owner
  first.

## Patch `servo-container-queries`

- **Problem.** Upstream parses `@container` rules
  (`stylesheets/rule_parser.rs`) and the container relative lengths
  `cqw`, `cqh`, `cqi`, `cqb`, `cqmin` and `cqmax`
  (`values/specified/length.rs`) in Gecko builds only, builds the
  `container` shorthand for Gecko only, and leaves `style()` queries and
  `container-type` and `container-name` off in servo's preferences. CSS
  Conditional 5 defines all of them (sections 5.2 to 5.4, 6.2 and 7).
  Upstream also flags a style that read a `style()` query or a container
  unit, but not one that read a size or name-only query, so an embedder
  cannot tell which styles a container's size may change.
- **Change.** The two `cfg!(feature = "gecko")` guards go; the shorthand
  is built in servo builds behind `layout.container-queries.enabled`, the
  preference of its longhands; `layout.container-queries.enabled` (from
  `chrome-prefs`) and `layout.css.style-queries.enabled` are true. The
  rules take part in servo's cascade, rule tree and invalidation through
  the code Gecko builds already use. `ComputedValueFlags` gains
  `DEPENDS_ON_CONTAINER_QUERY` (bit 28), a cascade input that
  `ContainerCondition::matches` sets for every condition it evaluates.
  Units in `@page` stay refused, as in Gecko builds.
- **Spots.** `style/stylesheets/rule_parser.rs`,
  `style/values/specified/length.rs`, `style/stylesheets/container_rule.rs`
  (`matches`, and the test module),
  `style/properties/computed_value_flags.rs`,
  `style/properties/shorthands.toml`, `style/properties/shorthands.rs`,
  `stylo_static_prefs/preferences.toml` and `stylo_static_prefs/src/lib.rs`.
- **Fork tests.** `servo_container_queries_parse_container_rules` (the
  preludes and their serialization, invalid preludes dropped,
  `<general-enclosed>` kept), `servo_container_queries_parse_container_units`,
  `servo_container_queries_parse_the_container_shorthand` and
  `test_basic_preferences`.
- **Viper proof.** `servo_container_queries_evaluate`; Viper's own parse
  suite and cascade tests hold the rest (V10b-10a).
- **Commands.** `cargo test -p stylo --lib`, `cargo test -p
  stylo_static_prefs`.
- **Upstream.** Standard, so a candidate; a draft goes to the Viper owner
  first.

## Patch `container-size-px`

- **Problem.** `TElement::query_container_size` answered in app units
  (1/60 px). An embedder whose layout measures in finer units (1/64 px,
  say) loses the size's exact value: a 333.328125 px container reads
  333.333 px, so `(width < 333.33px)` fails and `100cqw` computes the
  wrong length.
- **Change.** The trait method answers
  `Size2D<Option<CSSPixelLength>>`; `ContainerInfo` and
  `ContainerSizeQueryResult` keep CSS px, the size features read them
  directly, `cqmin` and `cqmax` compare them with `min` and `max`, and the
  small-viewport fallback converts its app units to px.
  `Orientation::eval` is generic over the unit, so media and container
  queries share it. Gecko's implementation converts its app units, which
  is exact.
- **Spots.** `style/dom.rs`, `style/stylesheets/container_rule.rs`,
  `style/values/specified/length.rs`, `style/queries/values.rs` and
  `style/gecko/wrapper.rs` (not built by Viper).
- **Fork tests.** `container_size_px_keeps_sizes_exact`.
- **Viper proof.** `container_size_px_compares_exactly`.
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Generic; a candidate, drafted to the owner first.

## Patch `line-height-number-f64`

- **Problem.** CSS Inline 3 section 5.1 makes a unitless `line-height`'s
  used value the number times the element's font size. Upstream keeps the
  computed number in `f32` and folds a `calc()` of numbers in `f32`, so
  `calc(20 / 12)` of 12px is a little under 20px, and a layout that
  truncates to a fixed-point unit (1/64 px) loses a unit there; so does a
  plain number with more digits than `f32` holds (`1.66666667` of 12px).
- **Change.** In servo builds the computed number is `LineHeightNumber`,
  an `f64`. The specified number keeps the parsed number (its
  serialization and `f32` computation are unchanged) and the number as
  written in `f64`: the number token's text, or a math function of
  number tokens combined by `+`, `-`, `*`, `/`, parentheses and nested
  `calc()`, evaluated in `f64` (CSS Values 4 section 10.1; section 5
  leaves a number's precision to the implementation). Any other math
  function (percentages, other functions, constants) and a result that
  `f64` does not round to the `f32` value (one clamped by range checking,
  section 10.12, or an infinite one) keep the `f32` value. The resolved value and the `lh`
  unit multiply the `f64` number by the font size in `f64`, rounded once
  to `f32`. The specified value is larger, so the `line-height`
  declaration is boxed (`longhands.toml`, and the `font` shorthand boxes
  its longhand). Other builds keep the `f32` computed number.
- **Spots.** `style/values/specified/font.rs` (`LineHeightNumber`,
  `exact_number` and its parser, `parse_line_height_number`,
  `to_computed_value`, `from_computed_value`, the test module),
  `style/values/computed/font.rs` (the computed type and the resolved
  value), `style/device/servo.rs` (`calc_line_height`),
  `style/properties/longhands.toml` and `style/properties/shorthands.rs`.
- **Fork tests.** `line_height_number_f64_reads_numbers_as_written`,
  `line_height_number_f64_computes_the_number_as_written`.
- **Viper proof.** `line_height_number_f64`; Viper's layout multiplies
  the number in `f64` (its ADR 0024).
- **Commands.** `cargo test -p stylo --lib`.
- **Upstream.** Generic; a candidate, drafted to the owner first.

## Patch `selector-names-hook`

- **Problem.** Which pseudo-class and pseudo-element names a sheet
  accepts, with which arguments, and what may follow a pseudo-element, is
  fixed in servo's parser: upstream's names and rule, or this branch's
  default tables. An embedder whose DOM selector APIs follow a table of
  its own cannot make its stylesheets agree with them.
- **Change.** `ParseOptions::selector_names` holds an embedder's
  `SelectorNames` (`stylesheets/parse_options.rs`, exported from
  `stylesheets`), default `None`. When it is set, servo's `SelectorParser`
  first refuses, outside user-agent sheets, every name the hook's
  `refuses` names, then asks `pseudo_class` or `pseudo_element` (with
  itself, the enclosing parser, and the arguments of a functional name),
  then parses servo's own names;
  `valid_after_pseudo_element` takes the hook's `valid_after`, and without
  an answer upstream's rule (a user-action pseudo-class after a
  pseudo-element that accepts state pseudo-classes);
  `parse_compound_is_alias` takes the hook's `compound_is_alias`. An
  embedder parses the selectors nested in arguments, such as a compound
  selector, with the parser it is given, so they keep its declared
  namespaces, origin, supports-condition mode and options (the hook
  included), as the default tables' nested selectors do. Without a
  hook nothing changes: the default tables answer, and `:lang()` with a
  string stays refused outside user-agent sheets only then.
  `PseudoElement::Unmatched` holds its name instead of the default
  table's kind, which the default tables' module now reads from the name; so
  `PseudoElement` is no longer `Copy` (two clones in `matching.rs`, one in
  `ComputedValues::pseudo`) and servo's `ComputedValues` is 248 bytes.
- **Spots.** `style/stylesheets/parse_options.rs` (the field, the trait,
  `PartialEq` by hook identity and `Debug` by hand, the test module),
  `style/stylesheets/mod.rs`, `style/servo/selector_parser.rs` (the
  variant, `PSEUDO_COUNT`, `embedder_name` and the five `Parser` methods),
  `style/servo/chrome_names.rs`, `style/servo/chrome_after.rs`,
  `style/matching.rs`, `style/properties/properties.mako.rs`.
- **Fork tests.** `selector_names_hook_decides_the_names`,
  `selector_names_hook_refuses_outside_user_agent_sheets`,
  `selector_names_hook_decides_what_follows_a_pseudo_element`,
  `selector_names_hook_in_stylesheets` (`:lang()` takes a string again
  under a hook; distinct zero-sized hooks compare unequal),
  `selector_names_hook_nests_with_the_enclosing_parser` (the test hook's
  `:host-context()` keeps a declared `svg` namespace in `svg|a` and refuses
  an unknown name in `:is()` in a supports condition, as the default tables
  do), with two test implementations.
- **Viper proof.** `selector_names_hook_in_stylesheets_and_queries`:
  Viper's Basic names in its sheets and its DOM selector APIs alike.
- **Commands.** `cargo test -p stylo --features servo --lib`.
- **Upstream.** Generic; a candidate, drafted to the owner first.

## Patch `percentage-number-f64`

- **Problem.** CSS Values 4 section 5.1.3 defines a `<percentage>` by
  the number written. Upstream keeps a percentage as that number divided
  by 100 in `f32`, so a number with more significant digits than `f32`
  holds cannot be recovered from it: `66.666667%` is the fraction
  `0.6666667`, which reads back as `66.66667`, where the number written,
  rounded to `f32`, is `66.666664`. A layout that resolves a percentage as
  the number written times its basis over 100 (777px of it is 518px from
  the read-back number and 517.984375px from the written one, in a
  1/64 px unit) cannot get the second.
- **Change.** In servo builds `NoCalcPercentage` keeps its fraction as a
  `PercentageFraction`, an `f64` (other builds keep upstream's `f32`,
  which their C layout needs). `LengthPercentage`'s parser reads the
  percentage token's text (after the whitespace and comments `next`
  skips) and keeps its number over 100 in `f64` when that rounds to the
  token's `unit_value`, else the `unit_value` (`from_token`); every other
  constructor widens its `f32`. `get` rounds to `f32`, so every value
  servo computes, serializes and reifies is unchanged. The computed
  `LengthPercentage`'s inline number is a `PercentageFraction` too
  (lengths keep their `f32`, widened): `new_percent_fraction` keeps the
  specified fraction when it rounds to the computed percentage (so a
  clamped or normalized value keeps its own), and
  `to_percentage_fraction` returns it. `Clone`, `map_lengths` and a
  positive percentage's `clamp_to_non_negative` keep the fraction, and
  `PartialEq` compares inline numbers as kept. A percentage computed
  otherwise (a `calc()`, an interpolation, `from_computed_value`) keeps
  its `f32`. The computed `LengthPercentage` grows from 8 to 16 bytes in
  64-bit servo builds, and with it the style structs that hold one (the
  `Position` struct from 448 to 568 bytes, `Margin` from 72 to 104,
  `Padding` from 32 to 64, `Border` from 392 to 488).
- **Spots.** `style/values/specified/percentage.rs`
  (`PercentageFraction`, `NoCalcPercentage`, `from_token`, `fraction`,
  `get`, the conversions, the test module), `style/values/specified/length.rs`
  (`LengthPercentage::parse_internal`), `style/values/computed/length_percentage.rs`
  (the type, `new_length`, `new_percent`, `new_percent_fraction`,
  `to_percentage_fraction`, `unpack`, `unpack_mut`, `map_lengths`,
  `clamp_to_non_negative`, `PartialEq`, `Clone` and the specified value's
  `to_computed_value`).
- **Fork tests.** `percentage_number_f64_keeps_the_number_written`,
  `percentage_number_f64_computes_the_number_written`.
- **Viper proof.** `percentage_number_f64`; Viper's layout resolves the
  number written (its ADR 0023).
- **Commands.** `cargo test -p stylo --features servo --lib`.
- **Upstream.** Generic; a candidate, drafted to the owner first.

## Patch `servo-device-media-features`

- **Problem.** Upstream's servo media feature list lacks Media Queries 5's
  `color-gamut` (section 6.4), `dynamic-range` (6.5),
  `prefers-reduced-motion` (12.1), `prefers-contrast` (12.3) and
  `forced-colors` (12.4), so a query on one parses as `<general-enclosed>`
  and is unknown: neither `(prefers-reduced-motion: no-preference)` nor its
  negation matches. Servo's `Device::forced_colors` is fixed at `none`
  (upstream's gecko build reads its document).
- **Change.** The four keyword types (`ColorGamut` and `DynamicRange`
  ordered from the narrowest, `PrefersReducedMotion`, `PrefersContrast`;
  `forced-colors` keeps the shared `ForcedColors`) and
  `DeviceMediaFeatures`, the embedder's five settings, whose default is
  `srgb`, `standard`, `no-preference`, `no-preference` and `none`. The
  servo `Device` keeps one (`set_device_media_features`,
  `device_media_features`), and `forced_colors` returns its setting, so
  `active` also makes the cascade ignore author colors, as CSS Color
  Adjustment 1 section 3 asks and upstream's gecko build already does. The five
  features join the servo list with their atoms. A gamut or range matches
  every value up to the device's; in a boolean context every gamut and
  range matches, and `no-preference` and `none` do not (Media Queries 5's
  boolean-context rule). `Device::new` keeps its signature.
- **Spots.** `style/servo/media_features.rs` (the types, the evaluators,
  the five list entries and the test module), `style/device/servo.rs` (the
  field, its default, the accessors and `forced_colors`) and
  `stylo_atoms/static_atoms.txt` (the five atoms).
- **Fork tests.** `servo_device_media_features_evaluate`.
- **Viper proof.** `servo_device_media_features_take_the_config`.
- **Commands.** `cargo test -p stylo --features servo --lib`.
- **Upstream.** Standard, so a candidate; a draft goes to the Viper owner
  first.

## Patch `vertical-align-longhand`

- **Problem.** Upstream parses `vertical-align` only as CSS Inline 3's
  shorthand of `alignment-baseline`, `baseline-shift` and
  `baseline-source` (section 4.2, a Working Draft), so a computed
  `alignment-baseline` cannot tell an author's declaration of it from a
  `vertical-align` one, and `vertical-align` resets `baseline-source`. CSS
  2.1 (section 10.8.1) and SVG 1.1 (section 10.9.2) define `vertical-align`
  as one property, with `alignment-baseline` and `baseline-shift` apart
  from it; an embedder had no way to parse that model.
- **Change.** `ParseOptions::vertical_align_longhand`, default `false`
  (upstream's behaviour). When set:
  - `vertical-align` takes CSS 2.1's grammar: one alignment keyword
    (`baseline`, `text-top`, `text-bottom`, `middle`) or one shift (`sub`,
    `super`, `top`, `bottom`, a length or a percentage), never `center`,
    `first` or `last` or two values; it still declares the box's
    `alignment_baseline`, `baseline_shift` and `baseline_source` (the last
    `auto`), which then hold its value alone, so layout reads the first two
    as before;
  - a declaration of `alignment-baseline`, `baseline-shift` or
    `baseline-source` declares the internal longhand
    `-x-alignment-baseline`, `-x-baseline-shift` or `-x-baseline-source`
    instead, CSS-wide keywords and `var()` included, so each cascades on
    its own; `-x-alignment-baseline` takes SVG 1.1's initial value `auto`
    with servo's alignment keywords (`baseline`, `text-bottom`, `middle`,
    `text-top`), and `-x-baseline-source` affects layout as
    `baseline-source` does: since the box's `baseline_source` is then
    always `auto`, a layout reads `-x-baseline-source` where the box's is
    `auto`, which is exact in both models (without the option nothing
    declares the internal longhand).

  The internal longhands are not enabled in content: no sheet names them,
  and `all` does not reset them (nor, so, `baseline-source` for layout). A
  declaration block serializes them by their internal names, and its
  `getPropertyValue`, `setProperty` and `removeProperty` of the public
  names miss them. Sheets parsed without the option (the user-agent
  sheets) keep the shorthand. A value substituted from `var()` in
  `vertical-align` parses as without the option, as for `value_aliases`.
- **Spots.** `style/stylesheets/parse_options.rs` (the field, its default,
  comparison and debug output, `declared_id`, public so an embedder reads
  the three computed values where they are held, and the test module),
  `style/properties/mod.rs` (`parse_value_into` asks `declared_id`),
  `style/properties/shorthands.rs` (CSS 2.1's grammar in
  `vertical_align::parse_value`) and `style/properties/longhands.toml` (the
  three internal longhands, servo only; `-x-baseline-source` with
  `baseline-source`'s layout damage).
- **Fork tests.** `vertical_align_longhand_default_keeps_the_shorthand`,
  `vertical_align_longhand_separates_the_parts` and
  `vertical_align_longhand_options_compare`.
- **Viper proof.** `vertical_align_longhand_parts_cascade_apart`.
- **Commands.** `cargo test -p stylo --features servo --lib`.
- **Upstream.** Embedder control, so a candidate; a draft goes to the
  Viper owner first.

## Patch `engine-gates`

- **Problem.** Upstream builds 46 standard longhands and 13 shorthands for
  Gecko only (`engine = "gecko"`), so a servo build's parser drops their
  declarations and its computed style has no value for them, although
  their specifications define them for every engine. The longhands:
  `scroll-snap-type`, `scroll-snap-align`, `scroll-snap-stop` and the sixteen
  `scroll-margin-*` and `scroll-padding-*` longhands (CSS Scroll Snap 1);
  `break-before`, `break-after`, `break-inside` (CSS Fragmentation 3);
  `column-fill`, `column-rule-color`, `column-rule-style`,
  `column-rule-width` (CSS Multi-column 1); `contain-intrinsic-width`,
  `-height`, `-inline-size`, `-block-size` (CSS Box Sizing 4);
  `offset-anchor`, `offset-distance`, `offset-position`, `offset-rotate`
  (CSS Motion Path 1); `overflow-anchor` (CSS Scroll Anchoring 1);
  `anchor-name`, `anchor-scope`, `position-anchor`, `position-try-order`,
  `position-visibility` (CSS Anchor Positioning 1); `resize` (CSS UI 4);
  `scrollbar-gutter` (CSS Overflow 3); `line-clamp` (CSS Overflow 4, the
  longhand of `-webkit-line-clamp`); `shape-outside`, `shape-margin`,
  `shape-image-threshold` (CSS Shapes 1). The shorthands: `scroll-margin`,
  `scroll-margin-block`, `scroll-margin-inline`, `scroll-padding`,
  `scroll-padding-block`, `scroll-padding-inline`, `page-break-before`,
  `page-break-after`, `page-break-inside`, `offset`,
  `contain-intrinsic-size`, `column-rule` and `-webkit-line-clamp`.
- **Change.** The `engine = "gecko"` line of each goes (the Gecko
  preference and FFI names stay, which servo builds ignore), and the
  `offset` and `column_rule` modules of `shorthands.rs` lose their
  `#[cfg(feature = "gecko")]`. Nothing else changes: the value types were
  already built for both engines, and a servo layout that does not use a
  value ignores it. The longhands keep upstream's `affects` (layout,
  overflow, paint or none), so their restyle damage is upstream's.
- **Known difference (not patched).** Stylo's `BreakBetween` has no
  `column`, `avoid-page`, `avoid-column`, `recto` or `verso` in either
  build, though CSS Fragmentation 3 section 3.1 defines them, so
  `break-before: column` is still dropped; `BreakWithin` has all four of
  its values. Viper registers the gap; a later patch of the grammar is its
  own row.
- **Spots.** `style/properties/longhands.toml` (46 entries, the reason in
  full at `scroll-snap-type`), `style/properties/shorthands.toml` (13
  entries), `style/properties/shorthands.rs` (two module gates) and
  `style/properties/declaration_block.rs` (the test module).
- **Fork tests.** `engine_gates_parse_the_standard_longhands` and
  `engine_gates_parse_the_shorthands`; all 51 `stylo` library tests pass.
- **Viper proof.** `engine_gates`, which computes values of the
  longhands through Viper's cascade and `computed_value`.
- **Commands.** `cargo test -p stylo --features servo --lib`.
- **Upstream.** Standard, so a candidate; a draft goes to the Viper owner
  first.

## Patch `normal-color-scheme-option`

- **Problem.** CSS Color Adjustment 1 section 2.1 says an element whose
  `color-scheme` is `normal` is not aware of colour schemes and is rendered
  with the user agent's default scheme. Upstream's servo `Device` answers
  `normal` (no scheme listed) like `light dark` (both listed): with the
  user's preferred scheme, so under a dark preference a page that lists no
  scheme gets the dark system colours (`CanvasText` white) and the dark
  branch of `light-dark()`, and an embedder cannot keep such a page's
  authored contrast.
- **Change.** `NormalColorScheme` (`#[non_exhaustive]`: `Preferred`, the
  default and upstream's answer, `Light`, `Dark`), the embedder's choice,
  kept by the servo `Device` (`set_normal_color_scheme`,
  `normal_color_scheme`). `is_dark_color_scheme` reads it only when
  neither `light` nor `dark` is listed; a single listed scheme and
  `light dark` keep upstream's answers. `Device::new` keeps its
  signature. Like `set_color_scheme`, a change needs the embedder to
  restyle.
- **Spots.** `style/device/servo.rs` (the type, the field, its default,
  the accessors, the branch in `is_dark_color_scheme` and the test
  module).
- **Fork tests.** `normal_color_scheme_option_decides_normal_only`; all 52
  `stylo` library tests pass.
- **Viper proof.** `normal_color_scheme_takes_the_config`, which computes
  `color: CanvasText` through Viper's cascade under a dark preference.
- **Commands.** `cargo test -p stylo --features servo --lib`.
- **Upstream.** Embedder control, so a candidate; a draft goes to the
  Viper owner first.

## Retired patches

Patches that left this branch. Their code is gone from the branch head;
the commits that carried them stay, as every pinned commit does.

| Id | Retired by (Viper task) | Why | Last pinned commit carrying it | Replaced by |
| --- | --- | --- | --- | --- |
| `chrome-attr-s-flag` | STY-FORK-1 | It made servo's `SelectorParser` refuse the `s` attribute flag in every sheet; whether a sheet accepts the flag is now the embedder's choice, per parse | `c4343e2a` | `stylesheet-parse-options` (`attribute_s_flag: false` gives its answer) |
| `chrome-value-parsing` | STY-FORK-2 | It hard-coded Chrome 154's extra spellings and refusals in servo builds through the derives; they are now the embedder's lists, per parse, and the standard parts are their own patches | `36cc25a7` | `value-grammar-options` (the lists; Chrome's are the Chrome 154 provider's) and `servo-scripting-media-feature` |

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
