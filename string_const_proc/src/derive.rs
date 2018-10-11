// Copyright 2018 MSxDOS
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use super::common::*;

pub fn prepare_utf16(input: &str) -> (String, usize) {
    let (lit, is_raw) = parse(input);
    if is_raw {
        create_u16_array(lit)
    } else {
        create_u16_array(&parse_sequences(lit))
    }
}
fn parse_sequences(input: &str) -> String {
    let mut output = String::with_capacity(input.chars().count());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next().expect(r"Malformed string: '\' is not followed by anything.") {
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '\\' => output.push('\\'),
                '"' => output.push('"'),
                '\'' => output.push('\''),
                'u' => {
                    assert!(
                        chars.next() == Some('{')
                            && chars.next() == Some('0')
                            && chars.next() == Some('}'),
                        "String '{}' contains unicode sequences that are not {{0}}.\n\
                        This is not expected."
                    );
                    output.push('\0')
                }
                other => panic!(r"Unknown escape sequence: \{}.", other),
            }
        } else {
            output.push(c);
        }
    }
    output
}
