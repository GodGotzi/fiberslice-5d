pub mod path;
pub mod settings;

use settings::tree::QuickSettings;

#[derive(Debug)]
pub struct Slicer {
    pub fiber_settings: QuickSettings,
    pub topology_settings: QuickSettings,
    pub view_settings: QuickSettings,
}

impl Default for Slicer {
    fn default() -> Self {
        Self {
            fiber_settings: QuickSettings::new("settings/main.yaml"),
            topology_settings: QuickSettings::new("settings/main.yaml"),
            view_settings: QuickSettings::new("settings/main.yaml"),
        }
    }
}
