/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::atomic::{AtomicBool, AtomicI32};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[test]
fn test_basic_preferences() {
    assert!(!pref!("layout.unimplemented"));
    set_pref!("layout.unimplemented", true);
    assert!(pref!("layout.unimplemented"));

    // VIPER-PATCH(chrome-prefs): the compiled default is 0 (no thread pool).
    assert_eq!(pref!("layout.threads"), 0);
    set_pref!("layout.threads", 42);
    assert_eq!(pref!("layout.threads"), 42);

    // VIPER-PATCH(chrome-prefs): Chrome parses `writing-mode` and `subgrid`.
    assert!(pref!("layout.writing-mode.enabled"));
    assert!(pref!("layout.css.grid-template-subgrid-value.enabled"));
    assert!(pref!("layout.container-queries.enabled"));
}
