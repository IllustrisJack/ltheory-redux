use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::Vec3;
use libc;

#[no_mangle]
pub unsafe extern "C" fn Memory_Alloc(mut size: usize) -> *mut libc::c_void {
    libc::malloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn Memory_Calloc(mut n: usize, mut size: usize) -> *mut libc::c_void {
    libc::calloc(n, size)
}

#[no_mangle]
pub unsafe extern "C" fn Memory_Free(mut ptr: *mut libc::c_void) {
    libc::free(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn Memory_MemCopy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: usize,
) {
    libc::memcpy(dst, src, size);
}

#[no_mangle]
pub unsafe extern "C" fn Memory_MemMove(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: usize,
) {
    libc::memmove(dst, src, size);
}

#[no_mangle]
pub unsafe extern "C" fn Memory_Realloc(
    mut ptr: *mut libc::c_void,
    mut newSize: usize,
) -> *mut libc::c_void {
    libc::realloc(ptr, newSize)
}
