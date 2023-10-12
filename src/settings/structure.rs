use crate::ui::TextComponent;

pub trait Group {
    fn get_name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn get_inner_groups(&self) -> Vec<Box<dyn Group>>;
    fn get_settings(&self) -> Vec<Box<dyn TextComponent>>;
}
