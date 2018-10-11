// Copyright 2018 MSxDOS
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![cfg(windows)]
#![allow(bad_style)]

extern crate proc_macro;

mod attrib;
mod common;
mod derive;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn define(_: TokenStream, input: TokenStream) -> TokenStream {
    use proc_macro::TokenTree::{Ident, Literal, Punct};
    match &input.into_iter().collect::<Vec<_>>()[..] {
        [Ident(_), Ident(name), Punct(_), Ident(ty), Punct(_), Literal(val), Punct(_)] => {
            format!(
                "#[doc={0}]pub const {1}: {2} = {2}::new({3}, &{4});",
                val,
                name,
                ty,
                attrib::prepare_utf8(val.to_string()),
                attrib::prepare_utf16(&val.to_string()).0,
            )
        }
        other => panic!("Incorrect syntax: {:#?}", other)
    }.parse().unwrap()
}

#[proc_macro_derive(Wide)]
pub fn wide(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();
    let (string, length) = derive::prepare_utf16(
        &input[input.find('=').unwrap() + 1 .. input.rfind('"').unwrap()],
    );
    format!(
        "const UTF16: [u16; {}] = {};",
        length,
        string,
    ).parse().unwrap()
}
