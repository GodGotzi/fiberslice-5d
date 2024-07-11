use std::collections::HashMap;

pub struct WidgetServer {
    widgets: HashMap<String, Widget>,
}

pub struct Widget {
    enabled: bool,
}
