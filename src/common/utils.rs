#[cfg(not(feature = "remote"))]
use alloc::string::String;

#[cfg(not(feature = "remote"))]
use crate::ntapi::def::UnicodeString;

/// Computes the DJB2 hash for the given buffer
pub fn dbj2_hash(buffer: &[u8]) -> u32 {
    let mut hsh: u32 = 5381;
    let mut iter: usize = 0;
    let mut cur: u8;

    while iter < buffer.len() {
        cur = buffer[iter];

        if cur == 0 {
            iter += 1;
            continue;
        }

        if cur >= ('a' as u8) {
            cur -= 0x20;
        }

        hsh = ((hsh << 5).wrapping_add(hsh)) + cur as u32;
        iter += 1;
    }
    hsh
}

/// Calculates the length of a C-style null-terminated string.
pub fn get_cstr_len(pointer: *const char) -> usize {
    let mut tmp: u64 = pointer as u64;

    unsafe {
        while *(tmp as *const u8) != 0 {
            tmp += 1;
        }
    }

    (tmp - pointer as u64) as _
}

pub fn string_length_w(string: *const u16) -> usize {
    unsafe {
        let mut string2 = string;
        while !(*string2).is_null() {
            string2 = string2.add(1);
        }
        string2.offset_from(string) as usize
    }
}

// Utility function for checking null terminator for u8 and u16
trait IsNull {
    fn is_null(&self) -> bool;
}

impl IsNull for u16 {
    fn is_null(&self) -> bool {
        *self == 0
    }
}

#[cfg(not(feature = "remote"))]
pub fn unicodestring_to_string(unicode_string: &UnicodeString) -> Option<String> {
    if unicode_string.length == 0 || unicode_string.buffer.is_null() {
        return None;
    }

    let slice = unsafe {
        core::slice::from_raw_parts(unicode_string.buffer, (unicode_string.length / 2) as usize)
    };

    String::from_utf16(slice).ok()
}

#[cfg(feature = "verbose")]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => ({
        libc_println!($($arg)*);
    });
}

#[cfg(not(feature = "verbose"))]
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}