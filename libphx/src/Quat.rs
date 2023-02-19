use ::libc;
use glam::Vec3;
use crate::internal::Memory::*;
extern "C" {
    // fn __fpclassifyf(_: libc::c_float) -> libc::c_int;
    // fn __fpclassifyd(_: libc::c_double) -> libc::c_int;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn Fatal(_: cstr, _: ...);
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type uint32_t = libc::c_uint;
pub type cstr = *const libc::c_char;
pub type uint32 = uint32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Quat {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}

pub type Error = uint32;
#[inline]
unsafe extern "C" fn Abs(mut t: libc::c_double) -> libc::c_double {
    return fabs(t);
}
#[inline]
unsafe extern "C" fn Sqrtf(mut t: libc::c_float) -> libc::c_float {
    return sqrt(t as libc::c_double) as libc::c_float;
}
#[inline]
unsafe extern "C" fn ClampUnit(mut t: libc::c_double) -> libc::c_double {
    t = if t > 1.0f64 { 1.0f64 } else { t };
    t = if t < -1.0f64 { -1.0f64 } else { t };
    return t;
}
#[inline]
unsafe extern "C" fn Sqrt(mut t: libc::c_double) -> libc::c_double {
    return sqrt(t);
}
#[inline]
unsafe extern "C" fn Acos(mut t: libc::c_double) -> libc::c_double {
    return acos(t);
}
#[inline]
unsafe extern "C" fn Asin(mut t: libc::c_double) -> libc::c_double {
    return asin(t);
}
#[inline]
unsafe extern "C" fn Cos(mut t: libc::c_double) -> libc::c_double {
    return cos(t);
}
#[inline]
unsafe extern "C" fn Sin(mut t: libc::c_double) -> libc::c_double {
    return sin(t);
}
#[inline]
unsafe extern "C" fn Float_Validate(mut x: libc::c_double) -> Error {
    let mut classification: libc::c_int = if ::core::mem::size_of::<libc::c_double>()
        as libc::c_ulong == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        f32::classify(x as libc::c_float) as libc::c_int
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        f64::classify(x) as libc::c_int
    } else {3
    };
    match classification {
        2 => return 0x4 as libc::c_int as Error,
        5 => return 0x8 as libc::c_int as Error,
        1 => return 0x20 as libc::c_int as Error,
        3 | 4 => return 0 as libc::c_int as Error,
        _ => {
            Fatal(
                b"Float_Validate: Unhandled case: %i\0" as *const u8
                    as *const libc::c_char,
                classification,
            );
        }
    }
    return 0 as libc::c_int as Error;
}
#[inline]
unsafe extern "C" fn Float_ApproximatelyEqual(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> bool {
    return Abs(x - y) < 1e-3f64;
}

#[no_mangle]
pub extern "C" fn Quat_Create(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) -> Quat {
    Quat { x, y, z, w }
}
#[no_mangle]
pub unsafe extern "C" fn Quat_GetAxisX(mut q: *const Quat, mut out: *mut Vec3) {
    // (*out) = (*q).
    (*out).x = 1.0f32 - 2.0f32 * ((*q).y * (*q).y + (*q).z * (*q).z);
    (*out).y = 2.0f32 * ((*q).x * (*q).y + (*q).z * (*q).w);
    (*out).z = 2.0f32 * ((*q).x * (*q).z - (*q).y * (*q).w);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_GetAxisY(mut q: *const Quat, mut out: *mut Vec3) {
    (*out).x = 2.0f32 * ((*q).x * (*q).y - (*q).z * (*q).w);
    (*out).y = 1.0f32 - 2.0f32 * ((*q).x * (*q).x + (*q).z * (*q).z);
    (*out).z = 2.0f32 * ((*q).y * (*q).z + (*q).x * (*q).w);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_GetAxisZ(mut q: *const Quat, mut out: *mut Vec3) {
    (*out).x = 2.0f32 * ((*q).x * (*q).z + (*q).y * (*q).w);
    (*out).y = 2.0f32 * ((*q).y * (*q).z - (*q).x * (*q).w);
    (*out).z = 1.0f32 - 2.0f32 * ((*q).x * (*q).x + (*q).y * (*q).y);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_GetForward(mut q: *const Quat, mut out: *mut Vec3) {
    Quat_GetAxisZ(q, out);
    (*out).x = -(*out).x;
    (*out).y = -(*out).y;
    (*out).z = -(*out).z;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_GetRight(mut q: *const Quat, mut out: *mut Vec3) {
    Quat_GetAxisX(q, out);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_GetUp(mut q: *const Quat, mut out: *mut Vec3) {
    Quat_GetAxisY(q, out);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Identity(mut out: *mut Quat) {
    (*out).x = 0.0f32;
    (*out).y = 0.0f32;
    (*out).z = 0.0f32;
    (*out).w = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Canonicalize(mut q: *const Quat, mut out: *mut Quat) {
    let mut value: libc::c_float = if !Float_ApproximatelyEqual(
        (*q).w as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).w
    } else if !Float_ApproximatelyEqual(
        (*q).z as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).z
    } else if !Float_ApproximatelyEqual(
        (*q).y as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).y
    } else if !Float_ApproximatelyEqual(
        (*q).x as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).x
    } else {
        0.0f32
    };
    if value < 0.0f32 {
        (*out).x = -(*q).x;
        (*out).y = -(*q).y;
        (*out).z = -(*q).z;
        (*out).w = -(*q).w;
    } else {
        (*out).x = (*q).x;
        (*out).y = (*q).y;
        (*out).z = (*q).z;
        (*out).w = (*q).w;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Quat_ICanonicalize(mut q: *mut Quat) {
    let mut value: libc::c_float = if !Float_ApproximatelyEqual(
        (*q).w as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).w
    } else if !Float_ApproximatelyEqual(
        (*q).z as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).z
    } else if !Float_ApproximatelyEqual(
        (*q).y as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).y
    } else if !Float_ApproximatelyEqual(
        (*q).x as libc::c_double,
        0.0f32 as libc::c_double,
    ) {
        (*q).x
    } else {
        0.0f32
    };
    if value < 0.0f32 {
        (*q).x = -(*q).x;
        (*q).y = -(*q).y;
        (*q).z = -(*q).z;
        (*q).w = -(*q).w;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Dot(
    mut q: *const Quat,
    mut p: *const Quat,
) -> libc::c_float {
    return (*q).x * (*p).x + (*q).y * (*p).y + (*q).z * (*p).z + (*q).w * (*p).w;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Equal(mut q: *const Quat, mut p: *const Quat) -> bool {
    let mut cq = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    Quat_Canonicalize(q, &mut cq);
    let mut cp = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    Quat_Canonicalize(p, &mut cp);
    return cq.x == cp.x && cq.y == cp.y && cq.z == cp.z && cq.w == cp.w;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_ApproximatelyEqual(
    mut q: *const Quat,
    mut p: *const Quat,
) -> bool {
    let mut cq = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    Quat_Canonicalize(q, &mut cq);
    let mut cp = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    Quat_Canonicalize(p, &mut cp);
    return Abs((cq.x - cp.x) as libc::c_double) < 1e-3f32 as libc::c_double
        && Abs((cq.y - cp.y) as libc::c_double) < 1e-3f32 as libc::c_double
        && Abs((cq.z - cp.z) as libc::c_double) < 1e-3f32 as libc::c_double
        && Abs((cq.w - cp.w) as libc::c_double) < 1e-3f32 as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Inverse(mut q: *const Quat, mut out: *mut Quat) {
    let mut magSq: libc::c_float = (*q).x * (*q).x + (*q).y * (*q).y + (*q).z * (*q).z
        + (*q).w * (*q).w;
    (*out).x = -(*q).x / magSq;
    (*out).y = -(*q).y / magSq;
    (*out).z = -(*q).z / magSq;
    (*out).w = (*q).w / magSq;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_IInverse(mut q: *mut Quat) {
    let mut magSq: libc::c_float = (*q).x * (*q).x + (*q).y * (*q).y + (*q).z * (*q).z
        + (*q).w * (*q).w;
    (*q).x = -(*q).x / magSq;
    (*q).y = -(*q).y / magSq;
    (*q).z = -(*q).z / magSq;
    (*q).w = (*q).w / magSq;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Lerp(
    mut q: *const Quat,
    mut p: *const Quat,
    mut t: libc::c_float,
    mut out: *mut Quat,
) {
    let mut d: libc::c_float = Quat_Dot(p, q);
    let mut dp = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    if d < 0.0f32 {
        dp.x = -(*p).x;
        dp.y = -(*p).y;
        dp.z = -(*p).z;
        dp.w = -(*p).w;
    } else {
        dp = *p;
    }
    let mut x: libc::c_float = (*q).x + (dp.x - (*q).x) * t;
    let mut y: libc::c_float = (*q).y + (dp.y - (*q).y) * t;
    let mut z: libc::c_float = (*q).z + (dp.z - (*q).z) * t;
    let mut w: libc::c_float = (*q).w + (dp.w - (*q).w) * t;
    let mut rcpMag: libc::c_float = (1.0f32 as libc::c_double
        / Sqrt((x * x + y * y + z * z + w * w) as libc::c_double)) as libc::c_float;
    (*out).x = x * rcpMag;
    (*out).y = y * rcpMag;
    (*out).z = z * rcpMag;
    (*out).w = w * rcpMag;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_ILerp(
    mut q: *mut Quat,
    mut p: *const Quat,
    mut t: libc::c_float,
) {
    let mut d: libc::c_float = Quat_Dot(p, q);
    let mut dp = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    if d < 0.0f32 {
        dp.x = -(*p).x;
        dp.y = -(*p).y;
        dp.z = -(*p).z;
        dp.w = -(*p).w;
    } else {
        dp = *p;
    }
    let mut x: libc::c_float = (*q).x + (dp.x - (*q).x) * t;
    let mut y: libc::c_float = (*q).y + (dp.y - (*q).y) * t;
    let mut z: libc::c_float = (*q).z + (dp.z - (*q).z) * t;
    let mut w: libc::c_float = (*q).w + (dp.w - (*q).w) * t;
    let mut rcpMag: libc::c_float = (1.0f32 as libc::c_double
        / Sqrt((x * x + y * y + z * z + w * w) as libc::c_double)) as libc::c_float;
    (*q).x = x * rcpMag;
    (*q).y = y * rcpMag;
    (*q).z = z * rcpMag;
    (*q).w = w * rcpMag;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Mul(
    mut q: *const Quat,
    mut p: *const Quat,
    mut out: *mut Quat,
) {
    let mut qv: Vec3 = {
        let mut init = Vec3 {
            x: (*q).x,
            y: (*q).y,
            z: (*q).z,
        };
        init
    };
    let mut pv: Vec3 = {
        let mut init = Vec3 {
            x: (*p).x,
            y: (*p).y,
            z: (*p).z,
        };
        init
    };
    let mut rv: Vec3 = (qv * (*p).w) + (pv * (*q).w) + Vec3::cross(qv, pv);
    (*out).x = rv.x;
    (*out).y = rv.y;
    (*out).z = rv.z;
    (*out).w = (*q).w * (*p).w - Vec3::dot(qv, pv);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_IMul(mut q: *mut Quat, mut p: *const Quat) {
    let qv: Vec3 = {
        let mut init = Vec3 {
            x: (*q).x,
            y: (*q).y,
            z: (*q).z,
        };
        init
    };
    let pv: Vec3 = {
        let mut init = Vec3 {
            x: (*p).x,
            y: (*p).y,
            z: (*p).z,
        };
        init
    };
    let rv: Vec3 = (qv * (*p).w) + (pv * (*q).w) + Vec3::cross(qv, pv);
    (*q).x = rv.x;
    (*q).y = rv.y;
    (*q).z = rv.z;
    (*q).w = (*q).w * (*p).w - Vec3::dot(qv, pv);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_MulV(
    mut q: *const Quat,
    mut v: *const Vec3,
    mut out: *mut Vec3,
) {
    let mut u: Vec3 = {
        let mut init = Vec3 {
            x: (*q).x,
            y: (*q).y,
            z: (*q).z,
        };
        init
    };
    let mut w: libc::c_float = (*q).w;
    let mut t: Vec3 = Vec3::cross(u, *v);
    *out = (u * 2.0f32 * Vec3::dot(u, *v)) + ((*v) * (2.0f32 * w * w - 1.0f32)) + (t * 2.0f32 * w);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Normalize(mut q: *const Quat, mut out: *mut Quat) {
    let mut mag: libc::c_float = Sqrt(
        ((*q).x * (*q).x + (*q).y * (*q).y + (*q).z * (*q).z + (*q).w * (*q).w)
            as libc::c_double,
    ) as libc::c_float;
    (*out).x = (*q).x / mag;
    (*out).y = (*q).y / mag;
    (*out).z = (*q).z / mag;
    (*out).w = (*q).w / mag;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_INormalize(mut q: *mut Quat) {
    let mut mag: libc::c_float = Sqrt(
        ((*q).x * (*q).x + (*q).y * (*q).y + (*q).z * (*q).z + (*q).w * (*q).w)
            as libc::c_double,
    ) as libc::c_float;
    (*q).x /= mag;
    (*q).y /= mag;
    (*q).z /= mag;
    (*q).w /= mag;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Scale(
    mut q: *const Quat,
    mut scale: libc::c_float,
    mut out: *mut Quat,
) {
    (*out).x = scale * (*q).x;
    (*out).y = scale * (*q).y;
    (*out).z = scale * (*q).z;
    (*out).w = scale * (*q).w;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_IScale(mut q: *mut Quat, mut scale: libc::c_float) {
    (*q).x *= scale;
    (*q).y *= scale;
    (*q).z *= scale;
    (*q).w *= scale;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Slerp(
    mut q: *const Quat,
    mut p: *const Quat,
    mut t: libc::c_float,
    mut out: *mut Quat,
) {
    let mut np = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    Quat_Normalize(p, &mut np);
    let mut d: libc::c_float = Quat_Dot(q, p);
    if d < 0.0f32 {
        np.x = -np.x;
        np.y = -np.y;
        np.z = -np.z;
        np.w = -np.w;
        d = -d;
    }
    if d > 0.9995f32 {
        Quat_Lerp(q, p, t, out);
        return;
    }
    d = ClampUnit(d as libc::c_double) as libc::c_float;
    let mut angle: libc::c_float = (t as libc::c_double * Acos(d as libc::c_double))
        as libc::c_float;
    let mut c = Quat_Create(
        (*p).x - d * (*q).x,
        (*p).y - d * (*q).y,
        (*p).z - d * (*q).z,
        (*p).w - d * (*q).w,
    );
    Quat_INormalize(&mut c);
    let mut fa: libc::c_float = Cos(angle as libc::c_double) as libc::c_float;
    let mut fc: libc::c_float = Sin(angle as libc::c_double) as libc::c_float;
    (*out).x = fa * (*q).x + fc * c.x;
    (*out).y = fa * (*q).y + fc * c.y;
    (*out).z = fa * (*q).z + fc * c.z;
    (*out).w = fa * (*q).w + fc * c.w;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_ISlerp(
    mut q: *mut Quat,
    mut p: *const Quat,
    mut t: libc::c_float,
) {
    let mut np = Quat_Create(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    Quat_Normalize(p, &mut np);
    let mut d: libc::c_float = Quat_Dot(q, p);
    if d < 0.0f32 {
        np.x = -np.x;
        np.y = -np.y;
        np.z = -np.z;
        np.w = -np.w;
        d = -d;
    }
    if d > 0.9995f32 {
        Quat_ILerp(q, p, t);
        return;
    }
    d = ClampUnit(d as libc::c_double) as libc::c_float;
    let mut angle: libc::c_float = (t as libc::c_double * Acos(d as libc::c_double))
        as libc::c_float;
    let mut c = Quat_Create(
        (*p).x - d * (*q).x,
        (*p).y - d * (*q).y,
        (*p).z - d * (*q).z,
        (*p).w - d * (*q).w,
    );
    Quat_INormalize(&mut c);
    let mut fa: libc::c_float = Cos(angle as libc::c_double) as libc::c_float;
    let mut fc: libc::c_float = Sin(angle as libc::c_double) as libc::c_float;
    (*q).x = fa * (*q).x + fc * c.x;
    (*q).y = fa * (*q).y + fc * c.y;
    (*q).z = fa * (*q).z + fc * c.z;
    (*q).w = fa * (*q).w + fc * c.w;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_ToString(mut q: *const Quat) -> cstr {
    static mut buffer: [libc::c_char; 512] = [0; 512];
    snprintf(
        buffer.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 512]>())
            .wrapping_div(::core::mem::size_of::<libc::c_char>())
            as libc::c_int as libc::size_t,
        b"(%.4f, %.4f, %.4f, %.4f)\0" as *const u8 as *const libc::c_char,
        (*q).x as libc::c_double,
        (*q).y as libc::c_double,
        (*q).z as libc::c_double,
        (*q).w as libc::c_double,
    );
    return buffer.as_mut_ptr() as cstr;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_Validate(mut q: *const Quat) -> Error {
    let mut e: Error = 0 as libc::c_int as Error;
    e |= Float_Validate((*q).x as libc::c_double);
    e |= Float_Validate((*q).y as libc::c_double);
    e |= Float_Validate((*q).z as libc::c_double);
    e |= Float_Validate((*q).w as libc::c_double);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_FromAxisAngle(
    mut axis: *const Vec3,
    mut radians: libc::c_float,
    mut out: *mut Quat,
) {
    radians *= 0.5f32;
    let mut v: Vec3 = *axis * Sin(radians as libc::c_double) as libc::c_float;
    (*out).x = v.x;
    (*out).y = v.y;
    (*out).z = v.z;
    (*out).w = Cos(radians as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Quat_FromBasis(
    mut x: *const Vec3,
    mut y: *const Vec3,
    mut z: *const Vec3,
    mut out: *mut Quat,
) {
    let mut r: libc::c_float = (*x).x + (*y).y + (*z).z;
    if r > 0.0f32 {
        (*out)
            .w = (Sqrt((r + 1.0f32) as libc::c_double) * 0.5f32 as libc::c_double)
            as libc::c_float;
        let mut w4: libc::c_float = 1.0f32 / (4.0f32 * (*out).w);
        (*out).x = ((*y).z - (*z).y) * w4;
        (*out).y = ((*z).x - (*x).z) * w4;
        (*out).z = ((*x).y - (*y).x) * w4;
    } else if (*x).x > (*y).y && (*x).x > (*z).z {
        (*out)
            .x = (Sqrt((1.0f32 + (*x).x - (*y).y - (*z).z) as libc::c_double)
            * 0.5f32 as libc::c_double) as libc::c_float;
        let mut x4: libc::c_float = 1.0f32 / (4.0f32 * (*out).x);
        (*out).y = ((*y).x + (*x).y) * x4;
        (*out).z = ((*z).x + (*x).z) * x4;
        (*out).w = ((*y).z - (*z).y) * x4;
    } else if (*y).y > (*z).z {
        (*out)
            .y = (Sqrt((1.0f32 + (*y).y - (*x).x - (*z).z) as libc::c_double)
            * 0.5f32 as libc::c_double) as libc::c_float;
        let mut y4: libc::c_float = 1.0f32 / (4.0f32 * (*out).y);
        (*out).x = ((*y).x + (*x).y) * y4;
        (*out).z = ((*z).y + (*y).z) * y4;
        (*out).w = ((*z).x - (*x).z) * y4;
    } else {
        (*out)
            .z = (Sqrt((1.0f32 + (*z).z - (*x).x - (*y).y) as libc::c_double)
            * 0.5f32 as libc::c_double) as libc::c_float;
        let mut z4: libc::c_float = 1.0f32 / (4.0f32 * (*out).z);
        (*out).x = ((*z).x + (*x).z) * z4;
        (*out).y = ((*z).y + (*y).z) * z4;
        (*out).w = ((*x).y - (*y).x) * z4;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Quat_FromLookUp(
    mut look: *const Vec3,
    mut up: *const Vec3,
    mut out: *mut Quat,
) {
    let mut z: Vec3 = (*look * -1.0f32).normalize();
    let mut x: Vec3 = Vec3::cross(*up, z).normalize();
    let mut y: Vec3 = Vec3::cross(z, x);
    Quat_FromBasis(&mut x, &mut y, &mut z, out);
}
#[no_mangle]
pub unsafe extern "C" fn Quat_FromRotateTo(
    mut from: *const Vec3,
    mut to: *const Vec3,
    mut out: *mut Quat,
) {
    let mut axis: Vec3 = Vec3::cross((*from).normalize(), (*to).normalize());
    let mut angle = f32::asin(axis.length());
    Quat_FromAxisAngle(&mut axis, angle, out);
}
