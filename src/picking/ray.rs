use glam::Vec3;

use crate::geometry::BoundingBox;

pub const EPSILON: f32 = 0.0001;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn from_view(
        viewport: (f32, f32, f32, f32),
        position: (f32, f32),
        view: glam::Mat4,
        proj: glam::Mat4,
        eye: glam::Vec3,
    ) -> Self {
        // Unpack the viewport parameters
        let (x, y, width, height) = viewport;

        // Convert screen space position to normalized device coordinates (NDC)
        let ndc_x = 1.0 - (2.0 * (position.0 - x)) / width;
        let ndc_y = (2.0 * (position.1 - y)) / height - 1.0; // flip y-axis

        println!("ndc_x: {}, ndc_y: {}", ndc_x, ndc_y);

        // Clip space coordinates
        let clip_coords = glam::Vec4::new(ndc_x, ndc_y, -1.0, 1.0);

        // Convert clip space coordinates to eye space using the inverse projection matrix
        let inv_proj = proj.inverse();
        let eye_coords = inv_proj * clip_coords;

        // Homogeneous divide (perspective divide) to get eye space coordinates
        let eye_coords = glam::Vec4::new(eye_coords.x, eye_coords.y, -1.0, 0.0);

        // Convert eye space coordinates to world space using the inverse view matrix
        let inv_view = view.inverse();
        let world_coords = inv_view * eye_coords;

        // Normalize the direction
        let direction = (Vec3::new(world_coords.x, world_coords.y, world_coords.z)).normalize();

        Ray {
            origin: eye,
            direction,
        }
    }

    #[allow(dead_code)]
    pub fn closest_distance_box(&self, bounding_box: &BoundingBox) -> Option<f32> {
        // check if ray origin is inside the bounding box
        if bounding_box.contains(self.origin) {
            return Some(0.0);
        }

        let mut min = None;

        for (plane_dir, (a, b, c, d)) in bounding_box.faces_with_edges() {
            let intersection = self.intersection_plane(plane_dir, a);

            let max_face = a.max(b).max(c).max(d);
            let min_face = a.min(b).min(c).min(d);

            // check if the intersection point is inside the face with epsilon
            if (max_face.x + EPSILON) >= intersection.x
                && intersection.x >= (min_face.x - EPSILON)
                && (max_face.y + EPSILON) >= intersection.y
                && intersection.y >= (min_face.y - EPSILON)
                && (max_face.z + EPSILON) >= intersection.z
                && intersection.z >= (min_face.z - EPSILON)
            {
                let distance = (intersection - self.origin).length();
                if min.unwrap_or(f32::MAX) > distance || min.is_none() {
                    min = Some(distance);
                }
            }
        }

        min
    }

    #[allow(dead_code)]
    pub fn intersection_plane(&self, plane: Vec3, point: Vec3) -> Vec3 {
        let d = plane.dot(self.direction);
        if d.abs() > f32::EPSILON {
            let t = (point - self.origin).dot(plane) / d;
            self.origin + self.direction * t
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

/*

#[test]
fn test_ray() {
    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 1.0, 1.0),
    };

    let bounding_box = BoundingBox::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    assert!(ray.intersects_box(&bounding_box));
    assert_eq!(ray.closest_distance_box(&bounding_box), Some(0.0));
}

#[test]
fn test_ray2() {
    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 2.0, 1.0),
    };

    let bounding_box = BoundingBox::new(Vec3::new(2.0, 60.0, 2.0), Vec3::new(300.0, 300.0, 300.0));

    // assert!(ray.intersects_box(&bounding_box));
    assert_eq!(ray.closest_distance_box(&bounding_box), Some(73.484695));
}

#[test]
fn test_ray3() {
    let ray = Ray {
        origin: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(1.0, 1.0, 1.0),
    };

    let bounding_box = BoundingBox::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(300.0, 300.0, 300.0));

    assert!(ray.intersects_box(&bounding_box));
    assert_eq!(ray.closest_distance_box(&bounding_box), Some(3.4641016));
}

*/
