use crate::internal::Memory::*;
use crate::DataFormat::*;
use crate::Math::IVec3;
use crate::Math::Vec3;
use crate::Math::*;
use crate::Mesh::*;
use crate::PixelFormat::*;
use crate::Tex3D::*;
use libc;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SDF {
    pub size: IVec3,
    pub data: *mut Cell,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cell {
    pub value: f32,
    pub normal: Vec3,
}

#[no_mangle]
pub unsafe extern "C" fn SDF_Create(mut sx: i32, mut sy: i32, mut sz: i32) -> *mut SDF {
    let mut this: *mut SDF = MemAlloc(::core::mem::size_of::<SDF>()) as *mut SDF;
    (*this).size = IVec3::new(sx, sy, sz);
    (*this).data = MemAlloc((::core::mem::size_of::<Cell>()).wrapping_mul((sx * sy * sz) as usize))
        as *mut Cell;
    MemZero(
        (*this).data as *mut libc::c_void,
        (::core::mem::size_of::<Cell>())
            .wrapping_mul(sx as usize)
            .wrapping_mul(sy as usize)
            .wrapping_mul(sz as usize),
    );
    this
}

#[no_mangle]
pub unsafe extern "C" fn SDF_FromTex3D(mut tex: *mut Tex3D) -> *mut SDF {
    let mut this: *mut SDF = MemAlloc(::core::mem::size_of::<SDF>()) as *mut SDF;
    Tex3D_GetSize(tex, &mut (*this).size);
    (*this).data = MemAlloc(
        (::core::mem::size_of::<Cell>())
            .wrapping_mul(((*this).size.x * (*this).size.y * (*this).size.z) as usize),
    ) as *mut Cell;
    Tex3D_GetData(
        tex,
        (*this).data as *mut libc::c_void,
        PixelFormat_RGBA,
        DataFormat_Float,
    );
    this
}

#[no_mangle]
pub unsafe extern "C" fn SDF_Free(mut this: *mut SDF) {
    MemFree((*this).data as *const libc::c_void);
    MemFree(this as *const libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn SDF_ToMesh(mut this: *mut SDF) -> *mut Mesh {
    let mut mesh: *mut Mesh = Mesh_Create();
    let cells: IVec3 = IVec3 {
        x: (*this).size.x - 1_i32,
        y: (*this).size.y - 1_i32,
        z: (*this).size.z - 1_i32,
    };
    let cellsF: Vec3 = Vec3 {
        x: cells.x as f32,
        y: cells.y as f32,
        z: cells.z as f32,
    };
    let stride: IVec3 = IVec3 {
        x: 1_i32,
        y: (*this).size.x,
        z: (*this).size.x * (*this).size.y,
    };
    let cellStride: IVec3 = IVec3 {
        x: 1_i32,
        y: cells.x,
        z: cells.x * cells.y,
    };
    let mut indices: *mut i32 = MemAlloc(
        (::core::mem::size_of::<i32>()).wrapping_mul((cells.x * cells.y * cells.z) as usize),
    ) as *mut i32;
    let vp: [Vec3; 8] = [
        Vec3::new(0.0f32, 0.0f32, 0.0f32),
        Vec3::new(1.0f32, 0.0f32, 0.0f32),
        Vec3::new(0.0f32, 1.0f32, 0.0f32),
        Vec3::new(1.0f32, 1.0f32, 0.0f32),
        Vec3::new(0.0f32, 0.0f32, 1.0f32),
        Vec3::new(1.0f32, 0.0f32, 1.0f32),
        Vec3::new(0.0f32, 1.0f32, 1.0f32),
        Vec3::new(1.0f32, 1.0f32, 1.0f32),
    ];
    let edgeTable: [[i32; 2]; 12] = [
        [0_i32, 1_i32],
        [2_i32, 3_i32],
        [4_i32, 5_i32],
        [6_i32, 7_i32],
        [0_i32, 2_i32],
        [1_i32, 3_i32],
        [4_i32, 6_i32],
        [5_i32, 7_i32],
        [0_i32, 4_i32],
        [1_i32, 5_i32],
        [2_i32, 6_i32],
        [3_i32, 7_i32],
    ];
    let mut z: i32 = 0_i32;
    while z < cells.z {
        let mut z0: f32 = z as f32 / cells.z as f32;
        let mut y: i32 = 0_i32;
        while y < cells.y {
            let mut y0: f32 = y as f32 / cells.y as f32;
            let mut x: i32 = 0_i32;
            while x < cells.x {
                let mut x0: f32 = x as f32 / cells.x as f32;
                let mut cell: IVec3 = IVec3 { x: x, y: y, z: z };
                let cellIndex = IVec3::dot(cellStride, IVec3::new(x, y, z));
                let mut base: *const Cell =
                    ((*this).data).offset(IVec3::dot(stride, IVec3::new(x, y, z)) as isize);
                let mut v: [*const Cell; 8] = [
                    base,
                    base.offset(stride.x as isize),
                    base.offset(stride.y as isize),
                    base.offset(stride.x as isize).offset(stride.y as isize),
                    base.offset(stride.z as isize),
                    base.offset(stride.z as isize).offset(stride.x as isize),
                    base.offset(stride.z as isize).offset(stride.y as isize),
                    base.offset(stride.z as isize)
                        .offset(stride.y as isize)
                        .offset(stride.x as isize),
                ];
                let mut mask: i32 = 0_i32;
                mask |= if (*v[0]).value > 0.0f32 {
                    0x1_i32
                } else {
                    0_i32
                };
                mask |= if (*v[1]).value > 0.0f32 {
                    0x2_i32
                } else {
                    0_i32
                };
                mask |= if (*v[2]).value > 0.0f32 {
                    0x4_i32
                } else {
                    0_i32
                };
                mask |= if (*v[3]).value > 0.0f32 {
                    0x8_i32
                } else {
                    0_i32
                };
                mask |= if (*v[4]).value > 0.0f32 {
                    0x10_i32
                } else {
                    0_i32
                };
                mask |= if (*v[5]).value > 0.0f32 {
                    0x20_i32
                } else {
                    0_i32
                };
                mask |= if (*v[6]).value > 0.0f32 {
                    0x40_i32
                } else {
                    0_i32
                };
                mask |= if (*v[7]).value > 0.0f32 {
                    0x80_i32
                } else {
                    0_i32
                };
                if mask == 0_i32 || mask == 0xff_i32 {
                    *indices.offset(cellIndex as isize) = -1_i32;
                } else {
                    let mut tw: f32 = 0.0f32;
                    let mut offset: Vec3 = Vec3 {
                        x: 0.0f32,
                        y: 0.0f32,
                        z: 0.0f32,
                    };
                    let mut n: Vec3 = Vec3 {
                        x: 0.0f32,
                        y: 0.0f32,
                        z: 0.0f32,
                    };
                    let mut i: i32 = 0_i32;
                    while i < 12_i32 {
                        let mut i0: i32 = edgeTable[i as usize][0];
                        let mut i1: i32 = edgeTable[i as usize][1];
                        let mut v0: *const Cell = v[i0 as usize];
                        let mut v1: *const Cell = v[i1 as usize];
                        if !(((*v0).value > 0.0f32) as i32 == ((*v1).value > 0.0f32) as i32) {
                            let mut t: f32 =
                                Saturate(((*v0).value / ((*v0).value - (*v1).value)) as f64) as f32;
                            offset += vp[i0 as usize].lerp(vp[i1 as usize], t);
                            n += (*v0).normal.lerp((*v1).normal, t);
                            tw += 1.0f32;
                        }
                        i += 1;
                    }
                    offset /= tw;
                    n = n.normalize();
                    let mut p: Vec3 = Vec3::new(x0, y0, z0) + (offset / cellsF);
                    p = p * 2.0f32 - 1.0f32;
                    *indices.offset(cellIndex as isize) = Mesh_GetVertexCount(mesh);
                    Mesh_AddVertex(mesh, p.x, p.y, p.z, n.x, n.y, n.z, 1.0f32, 0.0f32);
                    let mut i_0: i32 = 0_i32;
                    while i_0 < 3_i32 {
                        let mut j: i32 = (i_0 + 1_i32) % 3_i32;
                        let mut k: i32 = (i_0 + 2_i32) % 3_i32;
                        if !(*(&mut cell.x as *mut i32).offset(j as isize) == 0_i32
                            || *(&mut cell.x as *mut i32).offset(k as isize) == 0_i32)
                        {
                            let mut du: i32 = *(&cellStride.x as *const i32).offset(j as isize);
                            let mut dv: i32 = *(&cellStride.x as *const i32).offset(k as isize);
                            let mut i0_0: i32 = *indices.offset(cellIndex as isize);
                            let mut i1_0: i32 = *indices.offset((cellIndex - du) as isize);
                            let mut i2: i32 = *indices.offset((cellIndex - du - dv) as isize);
                            let mut i3: i32 = *indices.offset((cellIndex - dv) as isize);
                            if !(i1_0 < 0_i32 || i2 < 0_i32 || i3 < 0_i32) {
                                if (*v[0]).value > 0.0f32 {
                                    Mesh_AddQuad(mesh, i0_0, i3, i2, i1_0);
                                } else {
                                    Mesh_AddQuad(mesh, i0_0, i1_0, i2, i3);
                                }
                            }
                        }
                        i_0 += 1;
                    }
                }
                x += 1;
            }
            y += 1;
        }
        z += 1;
    }
    MemFree(indices as *const libc::c_void);
    mesh
}

#[no_mangle]
pub unsafe extern "C" fn SDF_Clear(mut this: *mut SDF, mut value: f32) {
    let mut size: u64 = ((*this).size.x * (*this).size.y * (*this).size.z) as u64;
    let mut pCell: *mut Cell = (*this).data;
    let mut i: u64 = 0_i32 as u64;
    while i < size {
        let fresh0 = pCell;
        pCell = pCell.offset(1);
        (*fresh0).value = value;
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn SDF_ComputeNormals(mut this: *mut SDF) {
    let stride: IVec3 = IVec3 {
        x: 1_i32,
        y: (*this).size.x,
        z: (*this).size.x * (*this).size.y,
    };
    let mut z: i32 = 1_i32;
    while z < (*this).size.z - 1_i32 {
        let mut y: i32 = 1_i32;
        while y < (*this).size.y - 1_i32 {
            let mut x: i32 = 1_i32;
            while x < (*this).size.x - 1_i32 {
                let mut cell: *mut Cell = ((*this).data)
                    .offset((x * stride.x) as isize)
                    .offset((y * stride.y) as isize)
                    .offset((z * stride.z) as isize);
                let mut x0: *const Cell = cell.offset(-(stride.x as isize));
                let mut x1: *const Cell = cell.offset(stride.x as isize);
                let mut y0: *const Cell = cell.offset(-(stride.y as isize));
                let mut y1: *const Cell = cell.offset(stride.y as isize);
                let mut z0: *const Cell = cell.offset(-(stride.z as isize));
                let mut z1: *const Cell = cell.offset(stride.z as isize);
                (*cell).normal = Vec3::new(
                    (*x1).value - (*x0).value,
                    (*y1).value - (*y0).value,
                    (*z1).value - (*z0).value,
                )
                .normalize();
                x += 1;
            }
            y += 1;
        }
        z += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SDF_Set(
    mut this: *mut SDF,
    mut x: i32,
    mut y: i32,
    mut z: i32,
    mut value: f32,
) {
    (*((*this).data).offset((x + (*this).size.x * (y + (*this).size.y * z)) as isize)).value =
        value;
}

#[no_mangle]
pub unsafe extern "C" fn SDF_SetNormal(
    mut this: *mut SDF,
    mut x: i32,
    mut y: i32,
    mut z: i32,
    mut normal: *const Vec3,
) {
    (*((*this).data).offset((x + (*this).size.x * (y + (*this).size.y * z)) as isize)).normal =
        *normal;
}
