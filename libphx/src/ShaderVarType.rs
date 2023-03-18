use crate::internal::Memory::*;
use crate::Tex1D::*;
use crate::Tex2D::*;
use crate::Tex3D::*;
use crate::TexCube::*;
use crate::Matrix::*;
use glam::Vec3;
use glam::{IVec2, IVec3, IVec4, Vec2};
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type ShaderVarType = i32;

#[no_mangle]
pub unsafe extern "C" fn ShaderVarType_FromStr(mut s: *const libc::c_char) -> ShaderVarType {
    let mut i: ShaderVarType = 0x1_i32;
    while i <= 0xd_i32 {
        if StrEqual(s, ShaderVarType_GetGLSLName(i)) {
            return i;
        }
        i += 1;
    }
    0_i32
}

#[no_mangle]
pub unsafe extern "C" fn ShaderVarType_GetGLSLName(mut this: ShaderVarType) -> *const libc::c_char {
    match this {
        1 => return b"float\0" as *const u8 as *const libc::c_char,
        2 => return b"vec2\0" as *const u8 as *const libc::c_char,
        3 => return b"vec3\0" as *const u8 as *const libc::c_char,
        4 => return b"vec4\0" as *const u8 as *const libc::c_char,
        5 => return b"int\0" as *const u8 as *const libc::c_char,
        6 => return b"ivec2\0" as *const u8 as *const libc::c_char,
        7 => return b"ivec3\0" as *const u8 as *const libc::c_char,
        8 => return b"ivec4\0" as *const u8 as *const libc::c_char,
        9 => return b"mat4\0" as *const u8 as *const libc::c_char,
        10 => return b"sampler1D\0" as *const u8 as *const libc::c_char,
        11 => return b"sampler2D\0" as *const u8 as *const libc::c_char,
        12 => return b"sampler3D\0" as *const u8 as *const libc::c_char,
        13 => return b"samplerCube\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn ShaderVarType_GetName(mut this: ShaderVarType) -> *const libc::c_char {
    match this {
        1 => return b"float\0" as *const u8 as *const libc::c_char,
        2 => return b"float2\0" as *const u8 as *const libc::c_char,
        3 => return b"float3\0" as *const u8 as *const libc::c_char,
        4 => return b"float4\0" as *const u8 as *const libc::c_char,
        5 => return b"int\0" as *const u8 as *const libc::c_char,
        6 => return b"int2\0" as *const u8 as *const libc::c_char,
        7 => return b"int3\0" as *const u8 as *const libc::c_char,
        8 => return b"int4\0" as *const u8 as *const libc::c_char,
        9 => return b"Matrix\0" as *const u8 as *const libc::c_char,
        10 => return b"Tex1D\0" as *const u8 as *const libc::c_char,
        11 => return b"Tex2D\0" as *const u8 as *const libc::c_char,
        12 => return b"Tex3D\0" as *const u8 as *const libc::c_char,
        13 => return b"TexCube\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn ShaderVarType_GetSize(mut this: ShaderVarType) -> i32 {
    match this {
        1 => {
            return ::core::mem::size_of::<f32>() as i32;
        }
        2 => return ::core::mem::size_of::<Vec2>() as i32,
        3 => return ::core::mem::size_of::<Vec3>() as i32,
        4 => return ::core::mem::size_of::<Vec4f>() as i32,
        5 => return ::core::mem::size_of::<i32>() as i32,
        6 => return ::core::mem::size_of::<IVec2>() as i32,
        7 => return ::core::mem::size_of::<IVec3>() as i32,
        8 => return ::core::mem::size_of::<IVec4>() as i32,
        9 => return ::core::mem::size_of::<*mut Matrix>() as i32,
        10 => return ::core::mem::size_of::<*mut Tex1D>() as i32,
        11 => return ::core::mem::size_of::<*mut Tex2D>() as i32,
        12 => return ::core::mem::size_of::<*mut Tex3D>() as i32,
        13 => {
            return ::core::mem::size_of::<*mut TexCube>() as i32;
        }
        _ => {}
    }
    0
}
