//! Optional C ABI surface (enabled with feature `ffi`).
//!
//! This module is intentionally minimal — full bindings live in
//! `doldskrift-bindings`.

use crate::{decode, encode};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Encode a NUL-terminated UTF-8 C string to a newly allocated encoded C string.
///
/// Caller must free with [`dsk_string_free`].
///
/// # Safety
/// `input` must be a valid NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn dsk_encode(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = CStr::from_ptr(input);
    let Ok(text) = cstr.to_str() else {
        return std::ptr::null_mut();
    };
    let Ok(encoded) = encode(text) else {
        return std::ptr::null_mut();
    };
    CString::new(encoded)
        .map(|c| c.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Decode a NUL-terminated encoded C string.
///
/// Caller must free with [`dsk_string_free`].
///
/// # Safety
/// `input` must be a valid NUL-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn dsk_decode(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let cstr = CStr::from_ptr(input);
    let Ok(text) = cstr.to_str() else {
        return std::ptr::null_mut();
    };
    let Ok(decoded) = decode(text) else {
        return std::ptr::null_mut();
    };
    CString::new(decoded)
        .map(|c| c.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

/// Free a string returned by this FFI.
///
/// # Safety
/// `s` must have been allocated by [`dsk_encode`] / [`dsk_decode`], or be null.
#[no_mangle]
pub unsafe extern "C" fn dsk_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}
