use std::sync::{Arc, Mutex};

use crate::utils::task::TaskWithResult;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ForceVisualizer {
    result: Option<Arc<Mutex<TaskWithResult<Vec<u32>>>>>,
}

impl ForceVisualizer {
    pub fn new() -> Self {
        Self::default()
    }
}
