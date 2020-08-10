// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::CString;

use libc::{c_char, c_int};

use crate::{err::BlkidErr, Result};

// Shared code for encoding methods
fn string_shared<F>(string: &str, closure: F) -> Result<String>
where
    F: Fn(&CString, &mut Vec<u8>) -> c_int,
{
    // Per the documentation, the maximum buffer is 4 times
    // the length of the string.
    let mut buffer = vec![0u8; string.len() * 4];

    let cstring = CString::new(string)?;
    if closure(&cstring, &mut buffer) != 0 {
        return Err(BlkidErr::InvalidConv);
    }

    let first_null = buffer
        .iter()
        .position(|u| *u == 0)
        .ok_or_else(|| BlkidErr::Other("No null found in C string".to_string()))?;
    buffer.truncate(first_null);
    let buffer_cstring = CString::new(buffer)?;
    Ok(buffer_cstring.into_string().map_err(BlkidErr::IntoString)?)
}

/// Encode potentially unsafe characters in the given `string` parameter.
pub fn encode_string(string: &str) -> Result<String> {
    string_shared(string, |cstring, buffer| unsafe {
        libblkid_rs_sys::blkid_encode_string(
            cstring.as_ptr(),
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
        )
    })
}

/// Generate a safe string that allows ascii, hex-escaping, and utf8. Whitespaces
/// become `_`.
pub fn safe_string(string: &str) -> Result<String> {
    string_shared(string, |cstring, buffer| unsafe {
        libblkid_rs_sys::blkid_safe_string(
            cstring.as_ptr(),
            buffer.as_mut_ptr() as *mut i8,
            buffer.len(),
        )
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_string() {
        let encoded_string = encode_string("\\test string").unwrap();
        assert_eq!(encoded_string, "\\x5ctest\\x20string");
    }

    #[test]
    fn test_safe_string() {
        let safe_string = safe_string("test string").unwrap();
        assert_eq!(safe_string, "test_string");
    }
}
