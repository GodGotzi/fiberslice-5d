use glam::{vec4, Vec3, Vec4};
use rether::{
    model::BufferLocation,
    picking::{Hitbox, Ray},
    vertex::Vertex,
    Rotate, Scale, SimpleGeometry, Translate,
};

use crate::{
    geometry::{
        mesh::{construct_triangle_vertices, Mesh, WireMesh},
        BoundingBox, ProfileExtrusion, QuadFace, SelectBox,
    },
    viewer::{ToVisual, Visual},
};

use super::{path::PathModul, tree::ToolpathTree, DisplaySettings};

#[derive(Debug, Clone)]
pub struct ProfileCross {
    pub a: Vec3,
    pub c: Vec3,
    pub b: Vec3,
    pub d: Vec3,
}

impl ProfileCross {
    pub fn from_direction(
        direction: Vec3,
        (horizontal_radius, vertical_radius): (f32, f32),
    ) -> Self {
        let horizontal = if direction.z.abs() > 0.0 {
            direction.cross(Vec3::X)
        } else {
            direction.cross(Vec3::Z)
        };

        let vertical = direction.cross(horizontal);

        Self {
            a: vertical.normalize() * vertical_radius,
            c: vertical.normalize() * -vertical_radius,
            b: horizontal.normalize() * horizontal_radius,
            d: horizontal.normalize() * -horizontal_radius,
        }
    }

    pub fn axis_aligned(self) -> Self {
        let horizontal = self.b - self.d;
        let vertical = self.a - self.c;

        let corner = self.a - (horizontal / 2.0);

        ProfileCross {
            a: corner,
            c: corner - vertical + horizontal,
            b: corner - vertical,
            d: corner + horizontal,
        }
    }

    pub fn scaled(self, scale: f32) -> Self {
        let diagonal_1 = (self.a - self.c) * scale;
        let diagonal_2 = (self.b - self.d) * scale;

        let center = (self.a + self.c + self.b + self.d) / 4.0;

        Self {
            a: center + diagonal_1 / 2.0,
            c: center - diagonal_1 / 2.0,
            b: center + diagonal_2 / 2.0,
            d: center - diagonal_2 / 2.0,
        }
    }

    pub fn min(&self) -> Vec3 {
        self.a.min(self.c).min(self.b).min(self.d)
    }

    pub fn max(&self) -> Vec3 {
        self.a.max(self.c).max(self.b).max(self.d)
    }
}

impl ProfileCross {
    fn with_offset(&self, offset: Vec3) -> Self {
        Self {
            a: self.a + offset,
            c: self.c + offset,
            b: self.b + offset,
            d: self.d + offset,
        }
    }
}

impl Translate for ProfileCross {
    fn translate(&mut self, translation: Vec3) {
        self.a += translation;
        self.c += translation;
        self.b += translation;
        self.d += translation;
    }
}

impl Rotate for ProfileCross {
    fn rotate(&mut self, rotation: glam::Quat) {
        self.a = rotation * self.a;
        self.c = rotation * self.c;
        self.b = rotation * self.b;
        self.d = rotation * self.d;
    }
}

impl Scale for ProfileCross {
    fn scale(&mut self, scale: Vec3) {
        let diagonal_1 = (self.a - self.c) * scale;
        let diagonal_2 = (self.b - self.d) * scale;

        let center = (self.a + self.c + self.b + self.d) / 4.0;

        self.a = center + diagonal_1 / 2.0;
        self.c = center - diagonal_1 / 2.0;
        self.b = center + diagonal_2 / 2.0;
        self.d = center - diagonal_2 / 2.0;
    }
}

pub struct ProfileCrossMesh {
    profile: ProfileCross,
    color: Option<Vec4>,
}

impl ProfileCrossMesh {
    pub fn from_profile(profile: ProfileCross) -> Self {
        Self {
            profile,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

impl Mesh<6> for ProfileCrossMesh {
    fn to_triangle_vertices(&self) -> [Vertex; 6] {
        construct_triangle_vertices(
            [
                self.profile.a,
                self.profile.d,
                self.profile.c,
                self.profile.a,
                self.profile.c,
                self.profile.b,
            ],
            self.color.unwrap_or(Vec4::new(0.0, 0.0, 0.0, 1.0)),
        )
    }

    fn to_triangle_vertices_flipped(&self) -> [Vertex; 6] {
        construct_triangle_vertices(
            [
                self.profile.a,
                self.profile.c,
                self.profile.d,
                self.profile.a,
                self.profile.b,
                self.profile.c,
            ],
            self.color.unwrap_or(Vec4::new(0.0, 0.0, 0.0, 1.0)),
        )
    }
}

pub struct PathMesh {
    profile_start: ProfileCross,
    profile_end: ProfileCross,
    color: Option<Vec4>,
}

impl PathMesh {
    pub fn from_profiles(profile_start: ProfileCross, profile_end: ProfileCross) -> Self {
        Self {
            profile_start,
            profile_end,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

impl Mesh<24> for PathMesh {
    fn to_triangle_vertices(&self) -> [Vertex; 24] {
        construct_triangle_vertices(
            [
                // asdasd
                self.profile_end.d,
                self.profile_end.a,
                self.profile_start.a,
                self.profile_end.d,
                self.profile_start.a,
                self.profile_start.d,
                // asdasd
                self.profile_end.c,
                self.profile_end.d,
                self.profile_start.c,
                self.profile_end.d,
                self.profile_start.d,
                self.profile_start.c,
                // asdasd
                self.profile_end.b,
                self.profile_end.c,
                self.profile_start.c,
                self.profile_end.b,
                self.profile_start.c,
                self.profile_start.b,
                // asdasd
                self.profile_end.a,
                self.profile_end.b,
                self.profile_start.a,
                self.profile_end.b,
                self.profile_start.b,
                self.profile_start.a,
            ],
            self.color.unwrap_or(Vec4::new(0.0, 0.0, 0.0, 1.0)),
        )
    }
}

pub struct PathConnectionMesh {
    profile_start: ProfileCross,
    profile_end: ProfileCross,
    color: Option<Vec4>,
}

impl PathConnectionMesh {
    pub fn from_profiles(profile_start: ProfileCross, profile_end: ProfileCross) -> Self {
        Self {
            profile_start,
            profile_end,
            color: None,
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Some(color);
        self
    }
}

impl Mesh<12> for PathConnectionMesh {
    fn to_triangle_vertices(&self) -> [Vertex; 12] {
        construct_triangle_vertices(
            [
                self.profile_start.d,
                self.profile_end.d,
                self.profile_start.a,
                // asdasd
                self.profile_start.c,
                self.profile_start.d,
                self.profile_end.d,
                // asdasd
                self.profile_start.b,
                self.profile_end.b,
                self.profile_start.c,
                // asdasd
                self.profile_end.b,
                self.profile_start.b,
                self.profile_start.a,
            ],
            self.color.unwrap_or(Vec4::new(0.0, 0.0, 0.0, 1.0)),
        )
    }
}

impl PathModul {
    pub(super) fn to_model(&self, settings: &DisplaySettings) -> ToolpathTree {
        let mut vertices = Vec::new();
        let mut offsets: Vec<usize> = Vec::new();
        let mut sub_handles = Vec::new();

        let color = self
            .state
            .print_type
            .as_ref()
            .unwrap_or(&crate::slicer::print_type::PrintType::Unknown);

        let mut bounding_box = BoundingBox::default();

        let mut last_cross: Option<ProfileCross> = None;

        for (index, line) in self.lines.iter().enumerate() {
            // let line = line.into_flipped_yz();
            let direction = line.direction();

            let profile = ProfileCross::from_direction(
                direction,
                (settings.vertical / 2.0, settings.horizontal / 2.0),
            );

            let profile_start = profile.with_offset(line.start);
            let profile_end = profile.with_offset(line.end);

            let profile_start_mesh =
                ProfileCrossMesh::from_profile(profile_start.clone()).with_color(color.into());
            let profile_end_mesh =
                ProfileCrossMesh::from_profile(profile_end.clone()).with_color(color.into());

            if index == self.lines.len() - 1 {
                vertices.extend_from_slice(&profile_end_mesh.to_triangle_vertices());
                offsets.push(vertices.len());
            }

            if line.print {
                if let Some(last) = last_cross.take() {
                    vertices.extend_from_slice(
                        &PathConnectionMesh::from_profiles(last, profile_start.clone())
                            .with_color(color.into())
                            .to_triangle_vertices(),
                    );
                } else {
                    vertices.extend_from_slice(&profile_start_mesh.to_triangle_vertices_flipped());
                }

                let path_mesh = PathMesh::from_profiles(profile_start, profile_end.clone())
                    .with_color(color.into());

                let path_mesh_vertices = path_mesh.to_triangle_vertices();

                vertices.extend_from_slice(&path_mesh_vertices);

                let path_hitbox = PathHitbox::from(path_mesh);

                bounding_box.expand_min(path_hitbox.get_min());
                bounding_box.expand_max(path_hitbox.get_max());

                let sub_model = ToolpathTree::create_path(
                    path_hitbox,
                    BufferLocation {
                        offset: vertices.len(),
                        size: path_mesh_vertices.len(),
                    },
                );

                sub_handles.push(sub_model);

                last_cross = Some(profile_end);
            } else if let Some(last) = last_cross.take() {
                vertices.extend_from_slice(
                    &ProfileCrossMesh::from_profile(last)
                        .with_color(color.into())
                        .to_triangle_vertices(),
                );

                offsets.push(vertices.len());
            }
        }

        ToolpathTree::create_root_with_models(
            bounding_box,
            SimpleGeometry::init(vertices),
            sub_handles,
        )
    }
}

impl From<PathMesh> for PathHitbox {
    fn from(val: PathMesh) -> Self {
        PathHitbox {
            north_west: QuadFace {
                normal: (val.profile_end.a - val.profile_start.a)
                    .cross(val.profile_start.d - val.profile_start.a),
                point: val.profile_start.a,
                max: val
                    .profile_end
                    .a
                    .max(val.profile_start.a)
                    .max(val.profile_start.d)
                    .max(val.profile_end.d),
                min: val
                    .profile_end
                    .a
                    .min(val.profile_start.a)
                    .min(val.profile_start.d)
                    .min(val.profile_end.d),
            },
            north_east: QuadFace {
                normal: (val.profile_end.d - val.profile_start.d)
                    .cross(val.profile_start.c - val.profile_start.d),
                point: val.profile_start.d,
                max: val
                    .profile_end
                    .d
                    .max(val.profile_start.d)
                    .max(val.profile_start.c)
                    .max(val.profile_end.c),
                min: val
                    .profile_end
                    .d
                    .min(val.profile_start.d)
                    .min(val.profile_start.c)
                    .min(val.profile_end.c),
            },
            south_west: QuadFace {
                normal: (val.profile_end.c - val.profile_start.c)
                    .cross(val.profile_start.b - val.profile_start.c),
                point: val.profile_start.c,
                max: val
                    .profile_end
                    .c
                    .max(val.profile_start.c)
                    .max(val.profile_start.b)
                    .max(val.profile_end.b),
                min: val
                    .profile_end
                    .c
                    .min(val.profile_start.c)
                    .min(val.profile_start.b)
                    .min(val.profile_end.b),
            },
            south_east: QuadFace {
                normal: (val.profile_end.b - val.profile_start.b)
                    .cross(val.profile_start.a - val.profile_start.b),
                point: val.profile_start.b,
                max: val
                    .profile_end
                    .b
                    .max(val.profile_start.b)
                    .max(val.profile_start.a)
                    .max(val.profile_end.a),
                min: val
                    .profile_end
                    .b
                    .min(val.profile_start.b)
                    .min(val.profile_start.a)
                    .min(val.profile_end.a),
            },
            visual: ProfileExtrusion::new(
                val.profile_start.axis_aligned(),
                val.profile_end.axis_aligned(),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathHitbox {
    visual: ProfileExtrusion,

    north_west: QuadFace,
    north_east: QuadFace,
    south_west: QuadFace,
    south_east: QuadFace,
}

impl Translate for PathHitbox {
    fn translate(&mut self, translation: Vec3) {
        self.north_west.translate(translation);
        self.north_east.translate(translation);
        self.south_west.translate(translation);
        self.south_east.translate(translation);

        self.visual.translate(translation);
    }
}

impl Rotate for PathHitbox {
    fn rotate(&mut self, rotation: glam::Quat) {
        self.north_west.rotate(rotation);
        self.north_east.rotate(rotation);
        self.south_west.rotate(rotation);
        self.south_east.rotate(rotation);

        self.visual.rotate(rotation);
    }
}

impl Scale for PathHitbox {
    fn scale(&mut self, scale: Vec3) {
        self.north_west.scale(scale);
        self.north_east.scale(scale);
        self.south_west.scale(scale);
        self.south_east.scale(scale);

        self.visual.scale(scale);
    }
}

impl Hitbox for PathHitbox {
    fn check_hit(&self, ray: &Ray) -> Option<f32> {
        let faces = [
            &self.north_west,
            &self.north_east,
            &self.south_west,
            &self.south_east,
        ];

        let mut min = None;

        for quad_face in faces {
            let distance = quad_face.check_hit(ray);

            if let Some(distance) = distance {
                if min.unwrap_or(f32::MAX) > distance || min.is_none() {
                    min = Some(distance);
                }
            }
        }

        min
    }

    fn expand_hitbox(&mut self, _box: &dyn Hitbox) {
        // Not expandable
        // TODO either figure out how to expand this or remove this method for this type or make it clear that this is not expandable
    }

    fn set_enabled(&mut self, _enabled: bool) {}

    fn enabled(&self) -> bool {
        true
    }

    fn get_min(&self) -> Vec3 {
        self.north_west
            .min
            .min(self.north_east.min)
            .min(self.south_west.min)
            .min(self.south_east.min)
    }

    fn get_max(&self) -> Vec3 {
        self.north_west
            .max
            .max(self.north_east.max)
            .max(self.south_west.max)
            .max(self.south_east.max)
    }
}

impl ToVisual<72, 48> for PathHitbox {
    fn to_visual(&self) -> Visual<72, 48> {
        let select_smaller_box: SelectBox = SelectBox::from(self.visual.clone())
            .with_color(vec4(1.0, 0.0, 0.0, 1.0), vec4(0.0, 0.0, 0.0, 1.0));

        let select_box = SelectBox::from(self.visual.clone().scaled(2.0))
            .with_corner_expansion(0.35)
            .with_color(Vec4::W, Vec4::W);

        let vertices = select_box.to_triangle_vertices();

        let mut wires = [Vertex::default(); 48];

        wires[..24].clone_from_slice(&select_smaller_box.to_wire_vertices());

        wires[24..].clone_from_slice(&select_box.to_wire_vertices());

        Visual { vertices, wires }
    }
}
