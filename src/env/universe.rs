use std::collections::HashMap;

pub struct Universe {
    parts_server: PartServer,
}

pub struct PartServer {
    parts_map: HashMap<String, ()>,
}
