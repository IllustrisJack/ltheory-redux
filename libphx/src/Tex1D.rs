use crate::internal::Memory::*;
use crate::Bytes::*;
use crate::Common::*;
use crate::DataFormat::*;
use crate::Math::Vec3;
use crate::PixelFormat::*;
use crate::TexFilter::*;
use crate::TexFormat::*;
use crate::TexWrapMode::*;
use crate::GL::gl;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tex1D {
    pub _refCount: u32,
    pub handle: u32,
    pub size: i32,
    pub format: TexFormat,
}

#[inline]
unsafe extern "C" fn Tex1D_Init() {
    gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_Create(mut size: i32, mut format: TexFormat) -> *mut Tex1D {
    if !TexFormat_IsValid(format) {
        Fatal(
            b"Tex1D_Create: Invalid texture format requested\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut this = MemNew!(Tex1D);
    (*this)._refCount = 1;
    (*this).size = size;
    (*this).format = format;
    gl::GenTextures(1, &mut (*this).handle);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::TexImage1D(
        gl::TEXTURE_1D,
        0,
        (*this).format,
        (*this).size,
        0,
        (if TexFormat_IsColor(format) as i32 != 0 {
            gl::RED
        } else {
            gl::DEPTH_COMPONENT
        }) as gl::types::GLenum,
        gl::UNSIGNED_BYTE,
        std::ptr::null(),
    );
    Tex1D_Init();
    gl::BindTexture(gl::TEXTURE_1D, 0);
    this
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_Acquire(this: *mut Tex1D) {
    (*this)._refCount = ((*this)._refCount).wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_Free(this: *mut Tex1D) {
    if !this.is_null() && {
        (*this)._refCount = ((*this)._refCount).wrapping_sub(1);
        (*this)._refCount <= 0
    } {
        gl::DeleteTextures(1, &mut (*this).handle);
        MemFree(this as *const _);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_Draw(
    this: *mut Tex1D,
    mut x: f32,
    mut y: f32,
    mut xs: f32,
    mut ys: f32,
) {
    gl::Enable(gl::TEXTURE_1D);
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::Begin(gl::QUADS);
    gl::TexCoord1f(0.0f32);
    gl::Vertex2f(x, y);
    gl::Vertex2f(x, y + ys);
    gl::TexCoord1f(1.0f32);
    gl::Vertex2f(x + xs, y + ys);
    gl::Vertex2f(x + xs, y);
    gl::End();
    gl::Disable(gl::TEXTURE_1D);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_GenMipmap(this: *mut Tex1D) {
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::GenerateMipmap(gl::TEXTURE_1D);
    gl::BindTexture(gl::TEXTURE_1D, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_GetFormat(this: *mut Tex1D) -> TexFormat {
    (*this).format
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_GetData(
    this: *mut Tex1D,
    mut data: *mut libc::c_void,
    mut pf: PixelFormat,
    mut df: DataFormat,
) {
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::GetTexImage(
        gl::TEXTURE_1D,
        0,
        pf as gl::types::GLenum,
        df as gl::types::GLenum,
        data,
    );
    gl::BindTexture(gl::TEXTURE_1D, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_GetDataBytes(
    this: *mut Tex1D,
    mut pf: PixelFormat,
    mut df: DataFormat,
) -> *mut Bytes {
    let mut size: i32 = (*this).size * DataFormat_GetSize(df) * PixelFormat_Components(pf);
    let mut data: *mut Bytes = Bytes_Create(size as u32);
    Tex1D_GetData(this, Bytes_GetData(data), pf, df);
    Bytes_Rewind(data);
    data
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_GetHandle(this: *mut Tex1D) -> u32 {
    (*this).handle
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_GetSize(this: *mut Tex1D) -> u32 {
    (*this).size as u32
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_SetData(
    this: *mut Tex1D,
    mut data: *const libc::c_void,
    mut pf: PixelFormat,
    mut df: DataFormat,
) {
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::TexImage1D(
        gl::TEXTURE_1D,
        0,
        (*this).format,
        (*this).size,
        0,
        pf as gl::types::GLenum,
        df as gl::types::GLenum,
        data,
    );
    gl::BindTexture(gl::TEXTURE_1D, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_SetDataBytes(
    this: *mut Tex1D,
    mut data: *mut Bytes,
    mut pf: PixelFormat,
    mut df: DataFormat,
) {
    Tex1D_SetData(this, Bytes_GetData(data), pf, df);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_SetMagFilter(this: *mut Tex1D, mut filter: TexFilter) {
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MAG_FILTER, filter);
    gl::BindTexture(gl::TEXTURE_1D, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_SetMinFilter(this: *mut Tex1D, mut filter: TexFilter) {
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MIN_FILTER, filter);
    gl::BindTexture(gl::TEXTURE_1D, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_SetTexel(
    this: *mut Tex1D,
    mut x: i32,
    mut r: f32,
    mut g: f32,
    mut b: f32,
    mut a: f32,
) {
    let mut rgba: [f32; 4] = [r, g, b, a];
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::TexSubImage1D(
        gl::TEXTURE_1D,
        0,
        x,
        1,
        gl::RGBA,
        gl::FLOAT,
        rgba.as_mut_ptr() as *const _,
    );
    gl::BindTexture(gl::TEXTURE_1D, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Tex1D_SetWrapMode(this: *mut Tex1D, mut mode: TexWrapMode) {
    gl::BindTexture(gl::TEXTURE_1D, (*this).handle);
    gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_WRAP_S, mode);
    gl::BindTexture(gl::TEXTURE_1D, 0);
}
