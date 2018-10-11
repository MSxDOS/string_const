// Copyright 2018 MSxDOS
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use super::common::*;

pub fn prepare_utf8(mut input: String) -> String {
    let end = input.rfind('"').expect("Malformed string");
    input.insert(end, '\0');
    input
}
pub fn prepare_utf16(input: &str) -> (String, usize) {
    let (lit, is_raw) = parse(input);
    let lit = lit.get(..lit.len() - 1).unwrap_or(lit);
    if is_raw {
        create_u16_array(lit)
    } else {
        create_u16_array(&parse_sequences(lit).expect("Malformed string"))
    }
}
fn parse_sequences(input: &str) -> Option<String> {
    use std::char::from_u32;
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
                'x' => {
                    let h = chars.next()?.to_digit(16)?;
                    let l = chars.next()?.to_digit(16)?;
                    output.push((h << 4 | l) as u8 as char);
                }
                'u' => {
                    chars.next();
                    let mut seq = 0;
                    while let Some(d) = chars.next()?.to_digit(16) {
                        seq <<= 4;
                        seq |= d;
                    }
                    output.push(from_u32(seq)?)
                }
                other => panic!(r"Unknown escape sequence: \{}.", other),
            }
        } else {
            output.push(c);
        }
    }
    Some(output)
}
