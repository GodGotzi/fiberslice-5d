use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RawSettingObject {
    pub fields: HashMap<String, String>,
    pub children: HashMap<String, RawSettingObject>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingObject {
    pub fields: HashMap<String, Setting>,
    pub children: HashMap<String, SettingObject>,
}

impl From<RawSettingObject> for SettingObject {
    fn from(raw: RawSettingObject) -> Self {
        let mut fields = HashMap::new();
        let mut children = HashMap::new();

        for (key, value) in raw.fields {
            fields.insert(key, Setting::String(value));
        }

        for (key, value) in raw.children {
            children.insert(key, SettingObject::from(value));
        }

        Self { fields, children }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Setting {
    String(String),
    Float(f32),
    Integer(i32),
    Boolean(bool),
}