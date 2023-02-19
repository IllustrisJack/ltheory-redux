use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
extern "C" {
    // fn __fpclassifyf(_: libc::c_float) -> libc::c_int;
    // fn __fpclassifyd(_: libc::c_double) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn Fatal(_: cstr, _: ...);
}
pub type int32_t = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type cstr = *const libc::c_char;
pub type int32 = int32_t;
pub type uint32 = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Plane {
    pub n: Vec3,
    pub d: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
}
pub type Error = uint32;
#[inline]
unsafe extern "C" fn Sqrtf(mut t: libc::c_float) -> libc::c_float {
    return sqrt(t as libc::c_double) as libc::c_float;
}
#[inline]
unsafe extern "C" fn Min(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}

#[no_mangle]
pub unsafe extern "C" fn Triangle_ToPlane(
    mut tri: *const Triangle,
    mut plane: *mut Plane,
) {
    let mut v: *const Vec3 = ((*tri).vertices).as_ptr();
    let mut e1: Vec3 = *v.offset(1) -  *v.offset(0);
    let mut e2: Vec3 = *v.offset(2) -  *v.offset(0);
    let mut n: Vec3 = Vec3::cross(e1, e2).normalize();
    let mut centroid: Vec3 = *v.offset(0);
    centroid += *v.offset(1);
    centroid += *v.offset(2);
    centroid /= 3.0f32;
    (*plane).n = n;
    (*plane).d = Vec3::dot(centroid, n);
}
#[no_mangle]
pub unsafe extern "C" fn Triangle_ToPlaneFast(
    mut triangle: *const Triangle,
    mut plane: *mut Plane,
) {
    let mut v: *const Vec3 = ((*triangle).vertices).as_ptr();
    let mut e1: Vec3 = *v.offset(1) - *v.offset(0);
    let mut e2: Vec3 = *v.offset(2) - *v.offset(0);
    let mut n: Vec3 = Vec3::cross(e1, e2);
    (*plane).n = n;
    (*plane).d = Vec3::dot(*v.offset(0), n);
}
#[no_mangle]
pub unsafe extern "C" fn Triangle_GetArea(mut tri: *const Triangle) -> libc::c_float {
    let mut e1 = (*tri).vertices[1] - (*tri).vertices[0];
    let mut e2 = (*tri).vertices[2] - (*tri).vertices[1];
    return 0.5f32 * Vec3::cross(e1, e2).length();
}
#[no_mangle]
pub unsafe extern "C" fn Triangle_Validate(mut tri: *const Triangle) -> Error {
    let mut v: *const Vec3 = ((*tri).vertices).as_ptr();
    let mut i: int32 = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut e: Error = Vec3_Validate(*v.offset(i as isize));
        if e != 0 as libc::c_int as libc::c_uint {
            return 0x400000 as libc::c_int as libc::c_uint | e;
        }
        i += 1;
    }
    let mut eq01 = *v.offset(0) == *v.offset(1);
    let mut eq12 = *v.offset(1) == *v.offset(2);
    let mut eq20 = *v.offset(2) == *v.offset(0);
    if eq01 as libc::c_int != 0 || eq12 as libc::c_int != 0 || eq20 as libc::c_int != 0 {
        return (0x400000 as libc::c_int | 0x40 as libc::c_int) as Error;
    }
    let mut e01 = (*v.offset(0)).distance(*v.offset(1));
    let mut e12 = (*v.offset(1)).distance(*v.offset(2));
    let mut e20 = (*v.offset(2)).distance(*v.offset(0));
    let mut shortest: libc::c_float = Min(
        Min(e01 as libc::c_double, e12 as libc::c_double),
        e20 as libc::c_double,
    ) as libc::c_float;
    if (shortest as libc::c_double) < 0.75f32 as libc::c_double * 1e-4f64 {
        return (0x400000 as libc::c_int | 0x8 as libc::c_int) as Error;
    }
    return 0 as libc::c_int as Error;
}
