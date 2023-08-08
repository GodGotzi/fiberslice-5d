use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use three_d::{Vector2, Vector3, Vector4};

pub trait PrettyFormat {
    fn pretty_format(&self) -> String;
}

impl<T: PrettyFormat> PrettyFormat for Vec<T> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| p.pretty_format())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl<T: PrettyFormat> PrettyFormat for &Vec<T> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| p.pretty_format())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<u32> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<f64> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<f32> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<u16> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<u8> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<i8> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<i16> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<i32> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<i64> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl PrettyFormat for Vec<i128> {
    fn pretty_format(&self) -> String {
        let str_vec = self
            .iter()
            .map(|p| format!("{:.2}", p))
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", str_vec)
    }
}

impl<N: Add<Output = N> + Sub<Output = N> + Display> PrettyFormat for Vector3<N> {
    fn pretty_format(&self) -> String {
        format!("Vec3(x={:.2}, y={:.2}, z={:.2})", self.x, self.y, self.z)
    }
}

impl<N: Add<Output = N> + Sub<Output = N> + Display> PrettyFormat for Vector4<N> {
    fn pretty_format(&self) -> String {
        format!(
            "Vec4(x={:.2}, y={:.2}, z={:.2}, w={:.2})",
            self.x, self.y, self.z, self.w
        )
    }
}

impl<N: Add<Output = N> + Sub<Output = N> + Display> PrettyFormat for Vector2<N> {
    fn pretty_format(&self) -> String {
        format!("Vec2(x={:.2}, y={:.2})", self.x, self.y)
    }
}
