use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::Vec3;
use libc;

pub type Modifier = i32;

#[no_mangle]
pub static Modifier_Null: Modifier = 0 << 0;

#[no_mangle]
pub static Modifier_Alt: Modifier = 1 << 0;

#[no_mangle]
pub static Modifier_Ctrl: Modifier = 1 << 1;

#[no_mangle]
pub static Modifier_Shift: Modifier = 1 << 2;

#[no_mangle]
pub unsafe extern "C" fn Modifier_ToString(mut modifier: Modifier) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 512] = [0; 512];
    if modifier == Modifier_Null {
        return b"Modifier_Null\0" as *const u8 as *const libc::c_char;
    }
    let mut modifiers: [Modifier; 3] = [Modifier_Alt, Modifier_Ctrl, Modifier_Shift];
    let mut names: [*const libc::c_char; 3] = [
        b"Modifier_Alt\0" as *const u8 as *const libc::c_char,
        b"Modifier_Ctrl\0" as *const u8 as *const libc::c_char,
        b"Modifier_Shift\0" as *const u8 as *const libc::c_char,
    ];
    let mut start: *mut libc::c_char = buffer.as_mut_ptr();
    let mut sep: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    while i < modifiers.len() as i32 {
        if modifier & modifiers[i as usize] == modifiers[i as usize] {
            len += libc::snprintf(
                start.offset(len as isize),
                (buffer.len() as i32 - len) as usize,
                b"%s%s\0" as *const u8 as *const libc::c_char,
                sep,
                names[i as usize],
            );
            sep = b" | \0" as *const u8 as *const libc::c_char;
            modifier &= !modifiers[i as usize];
        }
        i += 1;
    }
    if modifier != 0 {
        len += libc::snprintf(
            start.offset(len as isize),
            (buffer.len() as i32 - len) as usize,
            b"%sUnknown (%i)\0" as *const u8 as *const libc::c_char,
            sep,
            modifier,
        );
    }
    buffer.as_mut_ptr() as *const libc::c_char
}
