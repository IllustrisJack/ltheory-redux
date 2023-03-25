use crate::internal::Memory::*;
use crate::Common::*;
use crate::Math::Box3;
use crate::Math::Vec3;
use crate::Math::Vec4;
use crate::Quat::*;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Matrix {
    pub m: [f32; 16],
}

#[inline]
unsafe extern "C" fn Float_ApproximatelyEqual(x: f64, y: f64) -> bool {
    f64::abs(x - y) < 1e-3f64
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Clone(this: *const Matrix) -> *mut Matrix {
    let mut clone = MemNew!(Matrix);
    *clone = *this;
    clone
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Free(this: *mut Matrix) {
    MemFree(this as *const _);
}

unsafe extern "C" fn Matrix_IOInverse(in_0: *const Matrix, out: *mut Matrix) {
    let mut src: *const f32 = in_0 as *const f32;
    let mut dst: *mut f32 = out as *mut f32;
    *dst.offset(0) = *src.offset(5) * *src.offset(10) * *src.offset(15)
        - *src.offset(5) * *src.offset(11) * *src.offset(14)
        - *src.offset(9) * *src.offset(6) * *src.offset(15)
        + *src.offset(9) * *src.offset(7) * *src.offset(14)
        + *src.offset(13) * *src.offset(6) * *src.offset(11)
        - *src.offset(13) * *src.offset(7) * *src.offset(10);
    *dst.offset(4) = -*src.offset(4) * *src.offset(10) * *src.offset(15)
        + *src.offset(4) * *src.offset(11) * *src.offset(14)
        + *src.offset(8) * *src.offset(6) * *src.offset(15)
        - *src.offset(8) * *src.offset(7) * *src.offset(14)
        - *src.offset(12) * *src.offset(6) * *src.offset(11)
        + *src.offset(12) * *src.offset(7) * *src.offset(10);
    *dst.offset(8) = *src.offset(4) * *src.offset(9) * *src.offset(15)
        - *src.offset(4) * *src.offset(11) * *src.offset(13)
        - *src.offset(8) * *src.offset(5) * *src.offset(15)
        + *src.offset(8) * *src.offset(7) * *src.offset(13)
        + *src.offset(12) * *src.offset(5) * *src.offset(11)
        - *src.offset(12) * *src.offset(7) * *src.offset(9);
    *dst.offset(12) = -*src.offset(4) * *src.offset(9) * *src.offset(14)
        + *src.offset(4) * *src.offset(10) * *src.offset(13)
        + *src.offset(8) * *src.offset(5) * *src.offset(14)
        - *src.offset(8) * *src.offset(6) * *src.offset(13)
        - *src.offset(12) * *src.offset(5) * *src.offset(10)
        + *src.offset(12) * *src.offset(6) * *src.offset(9);
    *dst.offset(1) = -*src.offset(1) * *src.offset(10) * *src.offset(15)
        + *src.offset(1) * *src.offset(11) * *src.offset(14)
        + *src.offset(9) * *src.offset(2) * *src.offset(15)
        - *src.offset(9) * *src.offset(3) * *src.offset(14)
        - *src.offset(13) * *src.offset(2) * *src.offset(11)
        + *src.offset(13) * *src.offset(3) * *src.offset(10);
    *dst.offset(5) = *src.offset(0) * *src.offset(10) * *src.offset(15)
        - *src.offset(0) * *src.offset(11) * *src.offset(14)
        - *src.offset(8) * *src.offset(2) * *src.offset(15)
        + *src.offset(8) * *src.offset(3) * *src.offset(14)
        + *src.offset(12) * *src.offset(2) * *src.offset(11)
        - *src.offset(12) * *src.offset(3) * *src.offset(10);
    *dst.offset(9) = -*src.offset(0) * *src.offset(9) * *src.offset(15)
        + *src.offset(0) * *src.offset(11) * *src.offset(13)
        + *src.offset(8) * *src.offset(1) * *src.offset(15)
        - *src.offset(8) * *src.offset(3) * *src.offset(13)
        - *src.offset(12) * *src.offset(1) * *src.offset(11)
        + *src.offset(12) * *src.offset(3) * *src.offset(9);
    *dst.offset(13) = *src.offset(0) * *src.offset(9) * *src.offset(14)
        - *src.offset(0) * *src.offset(10) * *src.offset(13)
        - *src.offset(8) * *src.offset(1) * *src.offset(14)
        + *src.offset(8) * *src.offset(2) * *src.offset(13)
        + *src.offset(12) * *src.offset(1) * *src.offset(10)
        - *src.offset(12) * *src.offset(2) * *src.offset(9);
    *dst.offset(2) = *src.offset(1) * *src.offset(6) * *src.offset(15)
        - *src.offset(1) * *src.offset(7) * *src.offset(14)
        - *src.offset(5) * *src.offset(2) * *src.offset(15)
        + *src.offset(5) * *src.offset(3) * *src.offset(14)
        + *src.offset(13) * *src.offset(2) * *src.offset(7)
        - *src.offset(13) * *src.offset(3) * *src.offset(6);
    *dst.offset(6) = -*src.offset(0) * *src.offset(6) * *src.offset(15)
        + *src.offset(0) * *src.offset(7) * *src.offset(14)
        + *src.offset(4) * *src.offset(2) * *src.offset(15)
        - *src.offset(4) * *src.offset(3) * *src.offset(14)
        - *src.offset(12) * *src.offset(2) * *src.offset(7)
        + *src.offset(12) * *src.offset(3) * *src.offset(6);
    *dst.offset(10) = *src.offset(0) * *src.offset(5) * *src.offset(15)
        - *src.offset(0) * *src.offset(7) * *src.offset(13)
        - *src.offset(4) * *src.offset(1) * *src.offset(15)
        + *src.offset(4) * *src.offset(3) * *src.offset(13)
        + *src.offset(12) * *src.offset(1) * *src.offset(7)
        - *src.offset(12) * *src.offset(3) * *src.offset(5);
    *dst.offset(14) = -*src.offset(0) * *src.offset(5) * *src.offset(14)
        + *src.offset(0) * *src.offset(6) * *src.offset(13)
        + *src.offset(4) * *src.offset(1) * *src.offset(14)
        - *src.offset(4) * *src.offset(2) * *src.offset(13)
        - *src.offset(12) * *src.offset(1) * *src.offset(6)
        + *src.offset(12) * *src.offset(2) * *src.offset(5);
    *dst.offset(3) = -*src.offset(1) * *src.offset(6) * *src.offset(11)
        + *src.offset(1) * *src.offset(7) * *src.offset(10)
        + *src.offset(5) * *src.offset(2) * *src.offset(11)
        - *src.offset(5) * *src.offset(3) * *src.offset(10)
        - *src.offset(9) * *src.offset(2) * *src.offset(7)
        + *src.offset(9) * *src.offset(3) * *src.offset(6);
    *dst.offset(7) = *src.offset(0) * *src.offset(6) * *src.offset(11)
        - *src.offset(0) * *src.offset(7) * *src.offset(10)
        - *src.offset(4) * *src.offset(2) * *src.offset(11)
        + *src.offset(4) * *src.offset(3) * *src.offset(10)
        + *src.offset(8) * *src.offset(2) * *src.offset(7)
        - *src.offset(8) * *src.offset(3) * *src.offset(6);
    *dst.offset(11) = -*src.offset(0) * *src.offset(5) * *src.offset(11)
        + *src.offset(0) * *src.offset(7) * *src.offset(9)
        + *src.offset(4) * *src.offset(1) * *src.offset(11)
        - *src.offset(4) * *src.offset(3) * *src.offset(9)
        - *src.offset(8) * *src.offset(1) * *src.offset(7)
        + *src.offset(8) * *src.offset(3) * *src.offset(5);
    *dst.offset(15) = *src.offset(0) * *src.offset(5) * *src.offset(10)
        - *src.offset(0) * *src.offset(6) * *src.offset(9)
        - *src.offset(4) * *src.offset(1) * *src.offset(10)
        + *src.offset(4) * *src.offset(2) * *src.offset(9)
        + *src.offset(8) * *src.offset(1) * *src.offset(6)
        - *src.offset(8) * *src.offset(2) * *src.offset(5);
    let mut det: f32 = 1.0f32
        / (*src.offset(0) * *dst.offset(0)
            + *src.offset(1) * *dst.offset(4)
            + *src.offset(2) * *dst.offset(8)
            + *src.offset(3) * *dst.offset(12));
    let mut i: i32 = 0;
    while i < 16 {
        *dst.offset(i as isize) *= det;
        i += 1;
    }
}

unsafe extern "C" fn Matrix_IOTranspose(in_0: *const Matrix, out: *mut Matrix) {
    let mut src: *const f32 = in_0 as *const f32;
    let mut dst: *mut f32 = out as *mut f32;
    *dst.offset(0) = *src.offset(0);
    *dst.offset(1) = *src.offset(4);
    *dst.offset(2) = *src.offset(8);
    *dst.offset(3) = *src.offset(12);
    *dst.offset(4) = *src.offset(1);
    *dst.offset(5) = *src.offset(5);
    *dst.offset(6) = *src.offset(9);
    *dst.offset(7) = *src.offset(13);
    *dst.offset(8) = *src.offset(2);
    *dst.offset(9) = *src.offset(6);
    *dst.offset(10) = *src.offset(10);
    *dst.offset(11) = *src.offset(14);
    *dst.offset(12) = *src.offset(3);
    *dst.offset(13) = *src.offset(7);
    *dst.offset(14) = *src.offset(11);
    *dst.offset(15) = *src.offset(15);
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Equal(a: *const Matrix, b: *const Matrix) -> bool {
    let mut i: i32 = 0;
    while i < 16 {
        if (*a).m[i as usize] != (*b).m[i as usize] {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_ApproximatelyEqual(a: *const Matrix, b: *const Matrix) -> bool {
    let mut i: i32 = 0;
    while i < 16 {
        if !Float_ApproximatelyEqual((*a).m[i as usize] as f64, (*b).m[i as usize] as f64) {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Inverse(this: *const Matrix) -> *mut Matrix {
    let mut result: Matrix = Matrix { m: [0.; 16] };
    Matrix_IOInverse(this, &mut result);
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_InverseTranspose(this: *const Matrix) -> *mut Matrix {
    let mut inverse: Matrix = Matrix { m: [0.; 16] };
    let mut result: Matrix = Matrix { m: [0.; 16] };
    Matrix_IOInverse(this, &mut inverse);
    Matrix_IOTranspose(&mut inverse, &mut result);
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Sum(a: *const Matrix, b: *const Matrix) -> *mut Matrix {
    let mut result: Matrix = Matrix { m: [0.; 16] };
    let mut i: i32 = 0;
    while i < 16 {
        result.m[i as usize] = (*a).m[i as usize] + (*b).m[i as usize];
        i += 1;
    }
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Transpose(this: *const Matrix) -> *mut Matrix {
    let mut result: Matrix = Matrix { m: [0.; 16] };
    Matrix_IOTranspose(this, &mut result);
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_IInverse(this: *mut Matrix) {
    let mut result: Matrix = Matrix { m: [0.; 16] };
    Matrix_IOInverse(this, &mut result);
    *this = result;
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_IScale(this: *mut Matrix, scale: f32) {
    let mut m: *mut f32 = ((*this).m).as_mut_ptr();
    *m.offset(0) *= scale;
    *m.offset(1) *= scale;
    *m.offset(2) *= scale;
    *m.offset(4) *= scale;
    *m.offset(5) *= scale;
    *m.offset(6) *= scale;
    *m.offset(8) *= scale;
    *m.offset(9) *= scale;
    *m.offset(10) *= scale;
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_ITranspose(this: *mut Matrix) {
    let mut result: Matrix = Matrix { m: [0.; 16] };
    Matrix_IOTranspose(this, &mut result);
    *this = result;
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Identity() -> *mut Matrix {
    let identity: Matrix = Matrix {
        m: [
            1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
            0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&identity)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_LookAt(
    pos: *const Vec3,
    at: *const Vec3,
    up: *const Vec3,
) -> *mut Matrix {
    let mut z: Vec3 = (*pos - *at).normalize();
    let mut x: Vec3 = Vec3::cross(*up, z).normalize();
    let mut y: Vec3 = Vec3::cross(z, x);
    let mut result: Matrix = Matrix {
        m: [
            x.x,
            y.x,
            z.x,
            (*pos).x,
            x.y,
            y.y,
            z.y,
            (*pos).y,
            x.z,
            y.z,
            z.z,
            (*pos).z,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_LookUp(
    pos: *const Vec3,
    look: *const Vec3,
    up: *const Vec3,
) -> *mut Matrix {
    let mut z: Vec3 = (*look * -1.0f32).normalize();
    let mut x: Vec3 = Vec3::cross(*up, z).normalize();
    let mut y: Vec3 = Vec3::cross(z, x);
    let mut result: Matrix = Matrix {
        m: [
            x.x,
            y.x,
            z.x,
            (*pos).x,
            x.y,
            y.y,
            z.y,
            (*pos).y,
            x.z,
            y.z,
            z.z,
            (*pos).z,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Perspective(
    degreesFovy: f32,
    aspect: f32,
    N: f32,
    F: f32,
) -> *mut Matrix {
    let mut rads: f64 = (std::f32::consts::PI * degreesFovy) as f64 / 360.0f64;
    let mut cot: f64 = 1.0f64 / f64::tan(rads);
    let mut result: Matrix = Matrix {
        m: [
            (cot / aspect as f64) as f32,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            cot as f32,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            (N + F) / (N - F),
            (2.0f64 * (F * N) as f64 / (N - F) as f64) as f32,
            0.0f32,
            0.0f32,
            -1.0f32,
            0.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Product(a: *const Matrix, b: *const Matrix) -> *mut Matrix {
    let mut result: Matrix = Matrix { m: [0.; 16] };
    let mut pResult: *mut f32 = (result.m).as_mut_ptr();
    let mut i: i32 = 0;
    while i < 4 {
        let mut j: i32 = 0;
        while j < 4 {
            let mut sum: f32 = 0.0f32;
            let mut k: i32 = 0;
            while k < 4 {
                sum += (*a).m[(4 * i + k) as usize] * (*b).m[(4 * k + j) as usize];
                k += 1;
            }
            let fresh0 = pResult;
            pResult = pResult.offset(1);
            *fresh0 = sum;
            j += 1;
        }
        i += 1;
    }
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_RotationX(rads: f32) -> *mut Matrix {
    let mut c: f32 = f64::cos(rads as f64) as f32;
    let mut s: f32 = f64::sin(rads as f64) as f32;
    let mut result: Matrix = Matrix {
        m: [
            1.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, c, -s, 0.0f32, 0.0f32, s, c, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_RotationY(rads: f32) -> *mut Matrix {
    let mut c: f32 = f64::cos(rads as f64) as f32;
    let mut s: f32 = f64::sin(rads as f64) as f32;
    let mut result: Matrix = Matrix {
        m: [
            c, 0.0f32, s, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, -s, 0.0f32, c, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_RotationZ(rads: f32) -> *mut Matrix {
    let mut c: f32 = f64::cos(rads as f64) as f32;
    let mut s: f32 = f64::sin(rads as f64) as f32;
    let mut result: Matrix = Matrix {
        m: [
            c, -s, 0.0f32, 0.0f32, s, c, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32,
            0.0f32, 0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Scaling(sx: f32, sy: f32, sz: f32) -> *mut Matrix {
    let mut result: Matrix = Matrix {
        m: [
            sx, 0.0f32, 0.0f32, 0.0f32, 0.0f32, sy, 0.0f32, 0.0f32, 0.0f32, 0.0f32, sz, 0.0f32,
            0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_SRT(
    sx: f32,
    sy: f32,
    sz: f32,
    ry: f32,
    rp: f32,
    rr: f32,
    tx: f32,
    ty: f32,
    tz: f32,
) -> *mut Matrix {
    let mut S: *mut Matrix = Matrix_Scaling(sx, sy, sz);
    let mut R: *mut Matrix = Matrix_YawPitchRoll(ry, rp, rr);
    let mut T: *mut Matrix = Matrix_Translation(tx, ty, tz);
    let mut TR: *mut Matrix = Matrix_Product(T, R);
    let mut TRS: *mut Matrix = Matrix_Product(TR, S);
    Matrix_Free(S);
    Matrix_Free(R);
    Matrix_Free(T);
    Matrix_Free(TR);
    TRS
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Translation(tx: f32, ty: f32, tz: f32) -> *mut Matrix {
    let mut result: Matrix = Matrix {
        m: [
            1.0f32, 0.0f32, 0.0f32, tx, 0.0f32, 1.0f32, 0.0f32, ty, 0.0f32, 0.0f32, 1.0f32, tz,
            0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_YawPitchRoll(yaw: f32, pitch: f32, roll: f32) -> *mut Matrix {
    let mut ca: f32 = f64::cos(roll as f64) as f32;
    let mut sa: f32 = f64::sin(roll as f64) as f32;
    let mut cb: f32 = f64::cos(yaw as f64) as f32;
    let mut sb: f32 = f64::sin(yaw as f64) as f32;
    let mut cy: f32 = f64::cos(pitch as f64) as f32;
    let mut sy: f32 = f64::sin(pitch as f64) as f32;
    let mut result: Matrix = Matrix {
        m: [
            ca * cb,
            ca * sb * sy - sa * cy,
            ca * sb * cy + sa * sy,
            0.0f32,
            sa * cb,
            sa * sb * sy + ca * cy,
            sa * sb * cy - ca * sy,
            0.0f32,
            -sb,
            cb * sy,
            cb * cy,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_MulBox(this: *const Matrix, out: *mut Box3, in_0: *const Box3) {
    let corners: [Vec3; 8] = [
        Vec3 {
            x: (*in_0).lower.x,
            y: (*in_0).lower.y,
            z: (*in_0).lower.z,
        },
        Vec3 {
            x: (*in_0).upper.x,
            y: (*in_0).lower.y,
            z: (*in_0).lower.z,
        },
        Vec3 {
            x: (*in_0).lower.x,
            y: (*in_0).upper.y,
            z: (*in_0).lower.z,
        },
        Vec3 {
            x: (*in_0).upper.x,
            y: (*in_0).upper.y,
            z: (*in_0).lower.z,
        },
        Vec3 {
            x: (*in_0).lower.x,
            y: (*in_0).lower.y,
            z: (*in_0).upper.z,
        },
        Vec3 {
            x: (*in_0).upper.x,
            y: (*in_0).lower.y,
            z: (*in_0).upper.z,
        },
        Vec3 {
            x: (*in_0).lower.x,
            y: (*in_0).upper.y,
            z: (*in_0).upper.z,
        },
        Vec3 {
            x: (*in_0).upper.x,
            y: (*in_0).upper.y,
            z: (*in_0).upper.z,
        },
    ];
    let mut result = Vec3::ZERO;
    Matrix_MulPoint(this, &mut result, corners[0].x, corners[0].y, corners[0].z);
    (*out).lower = result;
    (*out).upper = result;
    let mut i: i32 = 1;
    while i < 8 {
        Matrix_MulPoint(
            this,
            &mut result,
            corners[i as usize].x,
            corners[i as usize].y,
            corners[i as usize].z,
        );
        (*out).lower = Vec3::min((*out).lower, result);
        (*out).upper = Vec3::max((*out).upper, result);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_MulDir(
    this: *const Matrix,
    out: *mut Vec3,
    x: f32,
    y: f32,
    z: f32,
) {
    let mut m: *const f32 = ((*this).m).as_ptr();
    (*out).x = *m.offset(0) * x + *m.offset(1) * y + *m.offset(2) * z;
    (*out).y = *m.offset(4) * x + *m.offset(5) * y + *m.offset(6) * z;
    (*out).z = *m.offset(8) * x + *m.offset(9) * y + *m.offset(10) * z;
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_MulPoint(
    this: *const Matrix,
    out: *mut Vec3,
    x: f32,
    y: f32,
    z: f32,
) {
    let mut m: *const f32 = ((*this).m).as_ptr();
    (*out).x = *m.offset(0) * x + *m.offset(1) * y + *m.offset(2) * z + *m.offset(3);
    (*out).y = *m.offset(4) * x + *m.offset(5) * y + *m.offset(6) * z + *m.offset(7);
    (*out).z = *m.offset(8) * x + *m.offset(9) * y + *m.offset(10) * z + *m.offset(11);
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_MulVec(
    this: *const Matrix,
    out: *mut Vec4,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    let mut m: *const f32 = ((*this).m).as_ptr();
    (*out).x = *m.offset(0) * x + *m.offset(1) * y + *m.offset(2) * z + *m.offset(3) * w;
    (*out).y = *m.offset(4) * x + *m.offset(5) * y + *m.offset(6) * z + *m.offset(7) * w;
    (*out).z = *m.offset(8) * x + *m.offset(9) * y + *m.offset(10) * z + *m.offset(11) * w;
    (*out).w = *m.offset(12) * x + *m.offset(13) * y + *m.offset(14) * z + *m.offset(15) * w;
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_GetForward(this: *const Matrix, out: *mut Vec3) {
    (*out).x = -(*this).m[2];
    (*out).y = -(*this).m[6];
    (*out).z = -(*this).m[10];
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_GetRight(this: *const Matrix, out: *mut Vec3) {
    (*out).x = (*this).m[0];
    (*out).y = (*this).m[4];
    (*out).z = (*this).m[8];
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_GetUp(this: *const Matrix, out: *mut Vec3) {
    (*out).x = (*this).m[1];
    (*out).y = (*this).m[5];
    (*out).z = (*this).m[9];
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_GetPos(this: *const Matrix, out: *mut Vec3) {
    (*out).x = (*this).m[3];
    (*out).y = (*this).m[7];
    (*out).z = (*this).m[11];
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_GetRow(this: *const Matrix, out: *mut Vec4, row: i32) {
    (*out).x = (*this).m[(4 * row + 0) as usize];
    (*out).y = (*this).m[(4 * row + 1) as usize];
    (*out).z = (*this).m[(4 * row + 2) as usize];
    (*out).w = (*this).m[(4 * row + 3) as usize];
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_FromBasis(
    x: *const Vec3,
    y: *const Vec3,
    z: *const Vec3,
) -> *mut Matrix {
    let mut result: Matrix = Matrix {
        m: [
            (*x).x,
            (*y).x,
            (*z).x,
            0.0f32,
            (*x).y,
            (*y).y,
            (*z).y,
            0.0f32,
            (*x).z,
            (*y).z,
            (*z).z,
            0.0f32,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_FromPosRot(pos: *const Vec3, rot: *const Quat) -> *mut Matrix {
    let mut x = Vec3::ZERO;
    Quat_GetAxisX(rot, &mut x);
    let mut y = Vec3::ZERO;
    Quat_GetAxisY(rot, &mut y);
    let mut z = Vec3::ZERO;
    Quat_GetAxisZ(rot, &mut z);
    let mut result: Matrix = Matrix {
        m: [
            x.x,
            y.x,
            z.x,
            (*pos).x,
            x.y,
            y.y,
            z.y,
            (*pos).y,
            x.z,
            y.z,
            z.z,
            (*pos).z,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_FromPosRotScale(
    pos: *const Vec3,
    rot: *const Quat,
    scale: f32,
) -> *mut Matrix {
    let mut x = Vec3::ZERO;
    Quat_GetAxisX(rot, &mut x);
    let mut y = Vec3::ZERO;
    Quat_GetAxisY(rot, &mut y);
    let mut z = Vec3::ZERO;
    Quat_GetAxisZ(rot, &mut z);
    let mut result: Matrix = Matrix {
        m: [
            scale * x.x,
            scale * y.x,
            scale * z.x,
            (*pos).x,
            scale * x.y,
            scale * y.y,
            scale * z.y,
            (*pos).y,
            scale * x.z,
            scale * y.z,
            scale * z.z,
            (*pos).z,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_FromPosBasis(
    pos: *const Vec3,
    x: *const Vec3,
    y: *const Vec3,
    z: *const Vec3,
) -> *mut Matrix {
    let mut result: Matrix = Matrix {
        m: [
            (*x).x,
            (*y).x,
            (*z).x,
            (*pos).x,
            (*x).y,
            (*y).y,
            (*z).y,
            (*pos).y,
            (*x).z,
            (*y).z,
            (*z).z,
            (*pos).z,
            0.0f32,
            0.0f32,
            0.0f32,
            1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_FromQuat(q: *const Quat) -> *mut Matrix {
    let mut x = Vec3::ZERO;
    Quat_GetAxisX(q, &mut x);
    let mut y = Vec3::ZERO;
    Quat_GetAxisY(q, &mut y);
    let mut z = Vec3::ZERO;
    Quat_GetAxisZ(q, &mut z);
    let mut result: Matrix = Matrix {
        m: [
            x.x, y.x, z.x, 0.0f32, x.y, y.y, z.y, 0.0f32, x.z, y.z, z.z, 0.0f32, 0.0f32, 0.0f32,
            0.0f32, 1.0f32,
        ],
    };
    Matrix_Clone(&mut result)
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_ToQuat(this: *const Matrix, q: *mut Quat) {
    let mut m: *const f32 = this as *const f32;
    let mut x: Vec3 = Vec3::new(*m.offset(0), *m.offset(4), *m.offset(8));
    let mut y: Vec3 = Vec3::new(*m.offset(1), *m.offset(5), *m.offset(9));
    let mut z: Vec3 = Vec3::new(*m.offset(2), *m.offset(6), *m.offset(10));
    Quat_FromBasis(&mut x, &mut y, &mut z, q);
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_Print(this: *const Matrix) {
    let mut i: i32 = 0;
    while i < 4 {
        let mut j: i32 = 0;
        while j < 4 {
            libc::printf(
                b"%f \0" as *const u8 as *const libc::c_char,
                (*this).m[(4 * i + j) as usize] as f64,
            );
            j += 1;
        }
        libc::printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Matrix_ToString(this: *const Matrix) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 512] = [0; 512];
    let mut m: *const f32 = ((*this).m).as_ptr();
    libc::snprintf(
        buffer.as_mut_ptr(),
        (std::mem::size_of::<[libc::c_char; 512]>())
            .wrapping_div(std::mem::size_of::<libc::c_char>())
           ,
        b"[%+.2f, %+.2f, %+.2f, %+.2f]\n[%+.2f, %+.2f, %+.2f, %+.2f]\n[%+.2f, %+.2f, %+.2f, %+.2f]\n[%+.2f, %+.2f, %+.2f, %+.2f]\0"
            as *const u8 as *const libc::c_char,
        *m.offset(0) as f64,
        *m.offset(1) as f64,
        *m.offset(2) as f64,
        *m.offset(3) as f64,
        *m.offset(4) as f64,
        *m.offset(5) as f64,
        *m.offset(6) as f64,
        *m.offset(7) as f64,
        *m.offset(8) as f64,
        *m.offset(9) as f64,
        *m.offset(10) as f64,
        *m.offset(11) as f64,
        *m.offset(12) as f64,
        *m.offset(13) as f64,
        *m.offset(14) as f64,
        *m.offset(15) as f64,
    );
    buffer.as_mut_ptr() as *const libc::c_char
}
