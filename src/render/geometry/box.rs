use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};

use crate::render::vertex::Vertex;

/// Calculates the vertecies of a box.
///
/// Arguments:
///
/// * `position`: The position of the center of the box.
/// * `size`: The outer dimensions of the box.
/// * `rotation`: The `XYZ` - Euler angles which represent the rotation of the
/// box around its center.
#[cfg(feature = "indexed")]
pub fn get_box_vertecies(
    index_offset: u32,
    position: Vec3,
    size: Vec3,
    rotation: Vec3,
) -> (Vec<Vertex>, Vec<u32>) {
    // Calculate transformation
    let transform = Mat4::from_translation(position)
        * Mat4::from_quat(Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation.x,
            rotation.y,
            rotation.z,
        ))
        * Mat4::from_scale(size);

    // Vertecies for a box
    let points = vec![
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, -0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, 0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, -0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, 0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, -0.5f32, -0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, 0.5f32, -0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, -0.5f32, -0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, 0.5f32, -0.5f32, 1.0f32)),
    ];

    // Calulate normal vectors
    let front = (Vec3::from_slice(&points[1]) - Vec3::from_slice(&points[2]))
        .cross(Vec3::from_slice(&points[0]) - Vec3::from_slice(&points[2]))
        .normalize()
        .to_array();
    let back = (Vec3::from_slice(&points[6]) - Vec3::from_slice(&points[5]))
        .cross(Vec3::from_slice(&points[4]) - Vec3::from_slice(&points[5]))
        .normalize()
        .to_array();
    let left = (Vec3::from_slice(&points[1]) - Vec3::from_slice(&points[0]))
        .cross(Vec3::from_slice(&points[4]) - Vec3::from_slice(&points[0]))
        .normalize()
        .to_array();
    let right = (Vec3::from_slice(&points[6]) - Vec3::from_slice(&points[2]))
        .cross(Vec3::from_slice(&points[3]) - Vec3::from_slice(&points[2]))
        .normalize()
        .to_array();
    let top = (Vec3::from_slice(&points[7]) - Vec3::from_slice(&points[3]))
        .cross(Vec3::from_slice(&points[1]) - Vec3::from_slice(&points[3]))
        .normalize()
        .to_array();
    let bottom = (Vec3::from_slice(&points[0]) - Vec3::from_slice(&points[2]))
        .cross(Vec3::from_slice(&points[6]) - Vec3::from_slice(&points[2]))
        .normalize()
        .to_array();

    // TODO: Find out why this method does not work.
    /* let inverse_transform = transform.inverse();

    let front = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, 0.0, 1.0, 1.0));
    let back = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, 0.0, -1.0, 1.0));
    let left = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(-1.0, 0.0, 0.0, 1.0));
    let right = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(1.0, 0.0, 0.0, 1.0));
    let top = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, 1.0, 0.0, 1.0));
    let bottom = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, -1.0, 0.0, 1.0)); */

    let vertices = vec![
        // Front
        Vertex {
            position: points[0],
            tex_coords: [0.0, 1.0],
            normal: front,
        },
        Vertex {
            position: points[2],
            tex_coords: [1.0, 1.0],
            normal: front,
        },
        Vertex {
            position: points[1],
            tex_coords: [0.0, 0.0],
            normal: front,
        },
        Vertex {
            position: points[3],
            tex_coords: [1.0, 0.0],
            normal: front,
        },
        // Back
        Vertex {
            position: points[4],
            tex_coords: [1.0, 1.0],
            normal: back,
        },
        Vertex {
            position: points[6],
            tex_coords: [0.0, 1.0],
            normal: back,
        },
        Vertex {
            position: points[5],
            tex_coords: [1.0, 0.0],
            normal: back,
        },
        Vertex {
            position: points[7],
            tex_coords: [0.0, 0.0],
            normal: back,
        },
        // Left
        Vertex {
            position: points[4],
            tex_coords: [0.0, 1.0],
            normal: left,
        },
        Vertex {
            position: points[5],
            tex_coords: [0.0, 0.0],
            normal: left,
        },
        Vertex {
            position: points[0],
            tex_coords: [1.0, 1.0],
            normal: left,
        },
        Vertex {
            position: points[1],
            tex_coords: [1.0, 0.0],
            normal: left,
        },
        // Right
        Vertex {
            position: points[6],
            tex_coords: [1.0, 1.0],
            normal: right,
        },
        Vertex {
            position: points[7],
            tex_coords: [1.0, 0.0],
            normal: right,
        },
        Vertex {
            position: points[2],
            tex_coords: [0.0, 1.0],
            normal: right,
        },
        Vertex {
            position: points[3],
            tex_coords: [0.0, 0.0],
            normal: right,
        },
        // Top
        Vertex {
            position: points[5],
            tex_coords: [0.0, 0.0],
            normal: top,
        },
        Vertex {
            position: points[1],
            tex_coords: [0.0, 1.0],
            normal: top,
        },
        Vertex {
            position: points[7],
            tex_coords: [1.0, 0.0],
            normal: top,
        },
        Vertex {
            position: points[3],
            tex_coords: [1.0, 1.0],
            normal: top,
        },
        // Bottom
        Vertex {
            position: points[4],
            tex_coords: [0.0, 0.0],
            normal: bottom,
        },
        Vertex {
            position: points[0],
            tex_coords: [1.0, 0.0],
            normal: bottom,
        },
        Vertex {
            position: points[6],
            tex_coords: [0.0, 1.0],
            normal: bottom,
        },
        Vertex {
            position: points[2],
            tex_coords: [1.0, 1.0],
            normal: bottom,
        },
    ];

    let indices = vec![
        // Front
        index_offset,
        1 + index_offset,
        3 + index_offset,
        index_offset,
        3 + index_offset,
        2 + index_offset,
        // Back
        7 + index_offset,
        5 + index_offset,
        4 + index_offset,
        7 + index_offset,
        4 + index_offset,
        6 + index_offset,
        // Left
        11 + index_offset,
        9 + index_offset,
        8 + index_offset,
        11 + index_offset,
        8 + index_offset,
        10 + index_offset,
        // Right
        12 + index_offset,
        13 + index_offset,
        15 + index_offset,
        12 + index_offset,
        15 + index_offset,
        14 + index_offset,
        // Top
        16 + index_offset,
        17 + index_offset,
        19 + index_offset,
        16 + index_offset,
        19 + index_offset,
        18 + index_offset,
        // Bottom
        23 + index_offset,
        21 + index_offset,
        20 + index_offset,
        23 + index_offset,
        20 + index_offset,
        22 + index_offset,
    ];

    (vertices, indices)
}

/// Calculates the vertecies of a box.
///
/// Arguments:
///
/// * `position`: The position of the center of the box.
/// * `size`: The outer dimensions of the box.
/// * `rotation`: The `XYZ` - Euler angles which represent the rotation of the
/// box around its center.
#[cfg(not(feature = "indexed"))]
pub fn get_box_vertecies(
    _index_offset: u32,
    position: Vec3,
    size: Vec3,
    rotation: Vec3,
) -> (Vec<Vertex>, Vec<u32>) {
    // Calculate transformation
    let transform = Mat4::from_translation(position)
        * Mat4::from_quat(Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation.x,
            rotation.y,
            rotation.z,
        ))
        * Mat4::from_scale(size);

    // Vertecies for a box
    let points = [
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, -0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, 0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, -0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, 0.5f32, 0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, -0.5f32, -0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(-0.5f32, 0.5f32, -0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, -0.5f32, -0.5f32, 1.0f32)),
        homogenous_vector_to_array_3d(transform * Vec4::new(0.5f32, 0.5f32, -0.5f32, 1.0f32)),
    ];

    // Calulate normal vectors
    let front = (Vec3::from_slice(&points[1]) - Vec3::from_slice(&points[2]))
        .cross(Vec3::from_slice(&points[0]) - Vec3::from_slice(&points[2]))
        .normalize()
        .to_array();
    let back = (Vec3::from_slice(&points[6]) - Vec3::from_slice(&points[5]))
        .cross(Vec3::from_slice(&points[4]) - Vec3::from_slice(&points[5]))
        .normalize()
        .to_array();
    let left = (Vec3::from_slice(&points[1]) - Vec3::from_slice(&points[0]))
        .cross(Vec3::from_slice(&points[4]) - Vec3::from_slice(&points[0]))
        .normalize()
        .to_array();
    let right = (Vec3::from_slice(&points[6]) - Vec3::from_slice(&points[2]))
        .cross(Vec3::from_slice(&points[3]) - Vec3::from_slice(&points[2]))
        .normalize()
        .to_array();
    let top = (Vec3::from_slice(&points[7]) - Vec3::from_slice(&points[3]))
        .cross(Vec3::from_slice(&points[1]) - Vec3::from_slice(&points[3]))
        .normalize()
        .to_array();
    let bottom = (Vec3::from_slice(&points[0]) - Vec3::from_slice(&points[2]))
        .cross(Vec3::from_slice(&points[6]) - Vec3::from_slice(&points[2]))
        .normalize()
        .to_array();

    // TODO: Find out why this method does not work.
    /* let inverse_transform = transform.inverse();

    let front = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, 0.0, 1.0, 1.0));
    let back = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, 0.0, -1.0, 1.0));
    let left = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(-1.0, 0.0, 0.0, 1.0));
    let right = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(1.0, 0.0, 0.0, 1.0));
    let top = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, 1.0, 0.0, 1.0));
    let bottom = homogenous_vector_to_array_3d(inverse_transform * Vec4::new(0.0, -1.0, 0.0, 1.0)); */

    let vertices = vec![
        // Front
        Vertex {
            position: points[0],
            tex_coords: [0.0, 1.0],
            normal: front,
        },
        Vertex {
            position: points[2],
            tex_coords: [1.0, 1.0],
            normal: front,
        },
        Vertex {
            position: points[3],
            tex_coords: [1.0, 0.0],
            normal: front,
        },
        Vertex {
            position: points[0],
            tex_coords: [0.0, 1.0],
            normal: front,
        },
        Vertex {
            position: points[3],
            tex_coords: [1.0, 0.0],
            normal: front,
        },
        Vertex {
            position: points[1],
            tex_coords: [0.0, 0.0],
            normal: front,
        },
        // Back
        Vertex {
            position: points[7],
            tex_coords: [0.0, 0.0],
            normal: back,
        },
        Vertex {
            position: points[6],
            tex_coords: [0.0, 1.0],
            normal: back,
        },
        Vertex {
            position: points[4],
            tex_coords: [1.0, 1.0],
            normal: back,
        },
        Vertex {
            position: points[7],
            tex_coords: [0.0, 0.0],
            normal: back,
        },
        Vertex {
            position: points[4],
            tex_coords: [1.0, 1.0],
            normal: back,
        },
        Vertex {
            position: points[5],
            tex_coords: [1.0, 0.0],
            normal: back,
        },
        // Left
        Vertex {
            position: points[1],
            tex_coords: [1.0, 0.0],
            normal: left,
        },
        Vertex {
            position: points[5],
            tex_coords: [0.0, 0.0],
            normal: left,
        },
        Vertex {
            position: points[4],
            tex_coords: [0.0, 1.0],
            normal: left,
        },
        Vertex {
            position: points[1],
            tex_coords: [1.0, 0.0],
            normal: left,
        },
        Vertex {
            position: points[4],
            tex_coords: [0.0, 1.0],
            normal: left,
        },
        Vertex {
            position: points[0],
            tex_coords: [1.0, 1.0],
            normal: left,
        },
        // Right
        Vertex {
            position: points[6],
            tex_coords: [1.0, 1.0],
            normal: right,
        },
        Vertex {
            position: points[7],
            tex_coords: [1.0, 0.0],
            normal: right,
        },
        Vertex {
            position: points[3],
            tex_coords: [0.0, 0.0],
            normal: right,
        },
        Vertex {
            position: points[6],
            tex_coords: [1.0, 1.0],
            normal: right,
        },
        Vertex {
            position: points[3],
            tex_coords: [0.0, 0.0],
            normal: right,
        },
        Vertex {
            position: points[2],
            tex_coords: [0.0, 1.0],
            normal: right,
        },
        // Top
        Vertex {
            position: points[5],
            tex_coords: [0.0, 0.0],
            normal: top,
        },
        Vertex {
            position: points[1],
            tex_coords: [0.0, 1.0],
            normal: top,
        },
        Vertex {
            position: points[3],
            tex_coords: [1.0, 1.0],
            normal: top,
        },
        Vertex {
            position: points[5],
            tex_coords: [0.0, 0.0],
            normal: top,
        },
        Vertex {
            position: points[3],
            tex_coords: [1.0, 1.0],
            normal: top,
        },
        Vertex {
            position: points[7],
            tex_coords: [1.0, 0.0],
            normal: top,
        },
        // Bottom
        Vertex {
            position: points[2],
            tex_coords: [1.0, 1.0],
            normal: bottom,
        },
        Vertex {
            position: points[0],
            tex_coords: [1.0, 0.0],
            normal: bottom,
        },
        Vertex {
            position: points[4],
            tex_coords: [0.0, 0.0],
            normal: bottom,
        },
        Vertex {
            position: points[2],
            tex_coords: [1.0, 1.0],
            normal: bottom,
        },
        Vertex {
            position: points[4],
            tex_coords: [0.0, 0.0],
            normal: bottom,
        },
        Vertex {
            position: points[6],
            tex_coords: [0.0, 1.0],
            normal: bottom,
        },
    ];

    let indices = vec![];

    (vertices, indices)
}

fn homogenous_vector_to_array_3d(vector: Vec4) -> [f32; 3] {
    vector.xyz().to_array()
}
