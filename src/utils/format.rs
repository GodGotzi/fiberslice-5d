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
