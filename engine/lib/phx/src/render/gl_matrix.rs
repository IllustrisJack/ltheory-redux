use super::*;
use crate::math::*;

/* NOTE : LoadMatrix expects column-major memory layout. */

#[no_mangle]
pub extern "C" fn GLMatrix_Clear() {
    glcheck!(gl::LoadIdentity());
}

#[no_mangle]
pub extern "C" fn GLMatrix_Load(matrix: &mut Matrix) {
    glcheck!(gl::LoadMatrixf(matrix.as_ref().as_ptr()));
}

#[no_mangle]
pub extern "C" fn GLMatrix_LookAt(eye: &DVec3, at: &DVec3, up: &DVec3) {
    let matrix = glam::DMat4::look_at_rh(*eye, *at, *up);
    glcheck!(gl::MultMatrixd(matrix.as_ref().as_ptr()));
}

#[no_mangle]
pub extern "C" fn GLMatrix_ModeP() {
    glcheck!(gl::MatrixMode(gl::PROJECTION));
}

#[no_mangle]
pub extern "C" fn GLMatrix_ModeWV() {
    glcheck!(gl::MatrixMode(gl::MODELVIEW));
}

#[no_mangle]
pub extern "C" fn GLMatrix_Mult(matrix: &mut Matrix) {
    glcheck!(gl::MultMatrixf(matrix.as_ref().as_ptr()));
}

#[no_mangle]
pub extern "C" fn GLMatrix_Perspective(fovy: f64, aspect: f64, z0: f64, z1: f64) {
    let matrix = glam::DMat4::perspective_rh_gl(fovy, aspect, z0, z1);
    glcheck!(gl::MultMatrixd(matrix.as_ref().as_ptr()));
}

#[no_mangle]
pub extern "C" fn GLMatrix_Pop() {
    glcheck!(gl::PopMatrix());
}

#[no_mangle]
pub extern "C" fn GLMatrix_Push() {
    glcheck!(gl::PushMatrix());
}

#[no_mangle]
pub extern "C" fn GLMatrix_PushClear() {
    glcheck!(gl::PushMatrix());
    glcheck!(gl::LoadIdentity());
}

#[no_mangle]
pub extern "C" fn GLMatrix_Get() -> Option<Box<Matrix>> {
    let mut matrix_mode: gl::types::GLint = 0;
    glcheck!(gl::GetIntegerv(gl::MATRIX_MODE, &mut matrix_mode));

    let matrix_enum = match matrix_mode as u32 {
        gl::MODELVIEW => gl::MODELVIEW_MATRIX,
        gl::PROJECTION => gl::PROJECTION_MATRIX,
        _ => return None,
    };

    let mut matrix = Matrix::IDENTITY;
    glcheck!(gl::GetFloatv(matrix_enum, matrix.as_mut().as_mut_ptr()));

    Some(Box::new(matrix))
}

#[no_mangle]
pub extern "C" fn GLMatrix_RotateX(angle: f64) {
    glcheck!(gl::Rotated(angle, 1.0, 0.0, 0.0));
}

#[no_mangle]
pub extern "C" fn GLMatrix_RotateY(angle: f64) {
    glcheck!(gl::Rotated(angle, 0.0, 1.0, 0.0));
}

#[no_mangle]
pub extern "C" fn GLMatrix_RotateZ(angle: f64) {
    glcheck!(gl::Rotated(angle, 0.0, 0.0, 1.0));
}

#[no_mangle]
pub extern "C" fn GLMatrix_Scale(x: f64, y: f64, z: f64) {
    glcheck!(gl::Scaled(x, y, z));
}

#[no_mangle]
pub extern "C" fn GLMatrix_Translate(x: f64, y: f64, z: f64) {
    glcheck!(gl::Translated(x, y, z));
}
