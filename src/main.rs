// SPDX-FileCopyrightText: 2023 The Gamebox Developers
//
// SPDX-License-Identifier: Apache-2.0

//! Entrypoint to `Gamebox`.
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use extism::*;

fn main() {
    let url = Wasm::url("https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm");

    let manifest = Manifest::new([url]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let res = plugin.call::<&str, &str>("count_vowels", "Hello, world!").unwrap();
    println!("{}", res);
}
