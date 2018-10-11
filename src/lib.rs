// Copyright 2018 MSxDOS
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![cfg(windows)]
#![no_std]
#![allow(bad_style)]

extern crate string_const_proc;

use core::fmt;
use core::ops::Deref;
use string_const_proc::define;

pub struct StringConst(&'static str, &'static [u16]);

impl StringConst {
    // Public const constructor, to be stabilized in 1.31, I guess:
    // https://github.com/rust-lang/rust/issues/53555
    #[inline]
    pub const fn new(utf8: &'static str, utf16: &'static [u16]) -> StringConst {
        StringConst(utf8, utf16)
    }
    #[inline]
    pub fn utf8(&self) -> &'static str {
        self.0
    }
    #[inline]
    pub fn utf16(&self) -> &'static [u16] {
        self.1
    }
}
// Non-null-terminated methods
impl AsRef<str> for StringConst {
    #[inline]
    fn as_ref(&self) -> &'static str {
        self.0.get(..self.0.len() - 1).unwrap()
    }
}

impl AsRef<[u16]> for StringConst {
    #[inline]
    fn as_ref(&self) -> &'static [u16] {
        self.1.get(..self.1.len() - 1).unwrap()
    }
}
// Allow as_ptr() to give us *const u16
impl Deref for StringConst {
    type Target = [u16];
    #[inline]
    fn deref(&self) -> &[u16] {
        self.as_ref()
    }
}
// Allow print(ln) and the rest to 'just work'
impl fmt::Display for StringConst {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

macro_rules! STRING {
    (const $name:ident = $value:expr) => {
        #[doc = $value] // Workaround for 'cargo doc' not showing a value
        pub const $name: $crate::StringConst =
            $crate::StringConst::new(
                concat!($value, '\0'),
                &{
                    #[derive($crate::string_const_proc::Wide)]
                    #[deprecated(note = $value)] // #hackery; parse the $value as a part of this
                    struct _Dummy;               // attribute, create a constant and return it.
                    UTF16
                },
            );
    };
}

STRING!{const TEST = "\u{22}Test\x20string:\n☺"}
STRING!{const RAW_TEST = r"\u{22}Raw test\x20string:\n☺"}
#[define] const ATTRIB_TEST: StringConst = "\u{22}Attrib test\x20string:\n☺";
#[define] const RAW_ATTRIB_TEST: StringConst = r"\u{22}Raw attrib test\x20string:\n☺";
