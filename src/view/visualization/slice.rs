pub struct Preview {
    pub layer_amount: Option<u32>,
}

impl Default for Preview {
    fn default() -> Self {
        Self {
            layer_amount: Some(100),
        }
    }
}
