use super::*;
use crate::math::*;

#[derive(Clone)]
#[repr(C)]
pub struct BoxMesh {
    pub elem: Vec<Box0>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Box0 {
    pub p: Vec3,
    pub s: Vec3,
    pub r: Vec3,
    pub b: Vec3,
}

static mut K_FACE_ORIGIN: [Vec3; 6] = [
    Vec3::new(-1.0f32, -1.0f32, 1.0f32),
    Vec3::new(-1.0f32, -1.0f32, -1.0f32),
    Vec3::new(1.0f32, -1.0f32, -1.0f32),
    Vec3::new(-1.0f32, -1.0f32, -1.0f32),
    Vec3::new(-1.0f32, 1.0f32, -1.0f32),
    Vec3::new(-1.0f32, -1.0f32, -1.0f32),
];

static mut K_FACE_U: [Vec3; 6] = [
    Vec3::new(2.0f32, 0.0f32, 0.0f32),
    Vec3::new(0.0f32, 2.0f32, 0.0f32),
    Vec3::new(0.0f32, 2.0f32, 0.0f32),
    Vec3::new(0.0f32, 0.0f32, 2.0f32),
    Vec3::new(0.0f32, 0.0f32, 2.0f32),
    Vec3::new(2.0f32, 0.0f32, 0.0f32),
];

static mut K_FACE_V: [Vec3; 6] = [
    Vec3::new(0.0f32, 2.0f32, 0.0f32),
    Vec3::new(2.0f32, 0.0f32, 0.0f32),
    Vec3::new(0.0f32, 0.0f32, 2.0f32),
    Vec3::new(0.0f32, 2.0f32, 0.0f32),
    Vec3::new(2.0f32, 0.0f32, 0.0f32),
    Vec3::new(0.0f32, 0.0f32, 2.0f32),
];

#[no_mangle]
pub extern "C" fn BoxMesh_Create() -> Box<BoxMesh> {
    Box::new(BoxMesh { elem: Vec::new() })
}

#[no_mangle]
pub extern "C" fn BoxMesh_Free(_: Box<BoxMesh>) {}

#[no_mangle]
pub extern "C" fn BoxMesh_Add(this: &mut BoxMesh, p: &Vec3, s: &Vec3, r: &Vec3, b: &Vec3) {
    this.elem.push(Box0 {
        p: *p,
        s: *s,
        r: *r,
        b: *b,
    });
}

#[no_mangle]
pub unsafe extern "C" fn BoxMesh_GetMesh(this: &mut BoxMesh, res: i32) -> Box<Mesh> {
    let mut mesh = Mesh_Create();
    Mesh_ReserveVertexData(mesh.as_mut(), 6 * res * res * this.elem.len() as i32);
    Mesh_ReserveIndexData(mesh.as_mut(), 12 * (res - 1) * (res - 1));

    for box3 in this.elem.iter() {
        let lower: Vec3 = Vec3::new(
            (*box3).b.x - 1.0f32,
            (*box3).b.y - 1.0f32,
            (*box3).b.z - 1.0f32,
        );
        let upper: Vec3 = Vec3::new(
            1.0f32 - (*box3).b.x,
            1.0f32 - (*box3).b.y,
            1.0f32 - (*box3).b.z,
        );
        let rot: Box<Matrix> = Matrix_YawPitchRoll((*box3).r.x, (*box3).r.y, (*box3).r.z);

        for face in 0..6 {
            let o: Vec3 = K_FACE_ORIGIN[face as usize];
            let du: Vec3 = K_FACE_U[face as usize];
            let dv: Vec3 = K_FACE_V[face as usize];
            let n: Vec3 = Vec3::cross(du, dv).normalize();

            for iu in 0..res {
                let u: f32 = iu as f32 / (res - 1) as f32;
                for iv in 0..res {
                    let v: f32 = iv as f32 / (res - 1) as f32;
                    let mut p: Vec3 = o + (du * u) + (dv * v);
                    let clamped: Vec3 = Vec3::clamp(p, lower, upper);
                    let proj: Vec3 = p - clamped;
                    p = clamped + (proj.normalize() * (*box3).b);
                    p *= (*box3).s;
                    let mut rp = Vec3::ZERO;
                    Matrix_MulPoint(rot.as_ref(), &mut rp, p.x, p.y, p.z);
                    p = rp + (*box3).p;

                    if iu != 0 && iv != 0 {
                        let off: i32 = Mesh_GetVertexCount(mesh.as_mut());
                        Mesh_AddQuad(mesh.as_mut(), off, off - res, off - res - 1, off - 1);
                    }
                    Mesh_AddVertex(mesh.as_mut(), p.x, p.y, p.z, n.x, n.y, n.z, u, v);
                }
            }
        }
    }

    mesh
}
