// Copyright 2018 MSxDOS
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
pub fn parse(input: &str) -> (&str, bool) {
    let start = input.find('"').expect("Malformed string") + 1;
    (
        input.get(start..).expect("Parsing failure"),
        input.get(..start).map(|p| p.contains('r')).unwrap_or(false),
    )
}
pub fn create_u16_array(input: &str) -> (String, usize) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    let (mut array, mut total_length) = (String::new(), 1);
    array.push('[');
    OsStr::new(input).encode_wide().for_each(|c| {
        array.push_str(&c.to_string());
        array.push(',');
        total_length += 1;
    });
    array.push_str("0u16]");
    (array, total_length)
}
