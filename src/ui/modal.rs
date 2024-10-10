use parking_lot::RwLock;

static MODAL: RwLock<Option<Modal>> = RwLock::new(None);

pub struct Modal {}
