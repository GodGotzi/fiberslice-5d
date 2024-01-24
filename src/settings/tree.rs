use std::{cell::RefCell, collections::HashMap, usize};

use serde::{Deserialize, Serialize};
use three_d::egui::{CollapsingHeader, DragValue, Response, TextEdit};

use crate::prelude::{SharedMut, WrappedSharedMut};

type RawSettings = HashMap<String, NodeValue>;

#[derive(Debug)]
pub struct Setting {
    file_path: String,
    raw: SharedMut<RawSettings>,
    tree: WrappedSharedMut<SettingTree>,
}

impl Setting {
    pub fn new(file_path: &str) -> Self {
        let raw = SharedMut::from_inner(RawSettings::new());

        let map: HashMap<String, RawSetting> =
            serde_yaml::from_str(std::fs::read_to_string(file_path).unwrap().as_str()).unwrap();

        let tree = SettingTree::from(map);

        tree.collect_values_into(&raw, &|_| true, &|raw, path, node| {
            if let TreeNode::Value { value, .. } = node {
                raw.write().insert(path.to_string(), value.clone());
            }
        });

        Self {
            file_path: file_path.to_string(),
            raw,
            tree: WrappedSharedMut::from_inner(tree),
        }
    }

    pub fn show(&self, ui: &mut three_d::egui::Ui) {
        //check if the settingtree has changed
        if self.tree.write().inner.show(ui) {
            //if it has changed, update the raw settings
            let filter = |node: &TreeNode| match node {
                TreeNode::Branch { changed, .. } => *changed,
                TreeNode::Value { changed, .. } => *changed,
            };

            self.tree
                .read()
                .inner
                .collect_values_into(&self.raw, &filter, &|raw, path, node| {
                    if let TreeNode::Value { value, .. } = node {
                        raw.write().insert(path.to_string(), value.clone());
                    }
                });

            println!("changed");
            println!("{:#?}", self.raw);
        }
    }

    fn save(&self) {
        let content = serde_yaml::to_string(&self.tree.read().inner.root_children).unwrap();
        std::fs::write(self.file_path.as_str(), content).unwrap();
    }
}

impl Drop for Setting {
    fn drop(&mut self) {
        self.save();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingTree {
    root_children: HashMap<String, TreeNode>,
}

impl Default for SettingTree {
    fn default() -> Self {
        test::create_test_tree()
    }
}

impl SettingTree {
    pub fn show(&mut self, ui: &mut three_d::egui::Ui) -> bool {
        let mut has_changed = false;

        for (_, child) in self.root_children.iter_mut() {
            if child.show(ui) {
                has_changed = true;
            }
        }

        has_changed
    }

    fn collect_values_into<T: Copy>(
        &self,
        raw: T,
        filter: &dyn Fn(&TreeNode) -> bool,
        insert: &dyn Fn(T, &str, &TreeNode),
    ) {
        for (key, node) in self.root_children.iter() {
            if filter(node) {
                collect_values_into(raw, node, key.as_str(), filter, insert);
            }
        }
    }

    fn add(&mut self, path: &str, node: TreeNode) {
        self.root_children.insert(path.to_string(), node);
    }
}

fn collect_values_into<T: Copy>(
    raw: T,
    node: &TreeNode,
    path: &str,
    filter: &dyn Fn(&TreeNode) -> bool,
    insert: &dyn Fn(T, &str, &TreeNode),
) {
    match node {
        TreeNode::Branch { children, .. } => {
            for (key, child) in children.iter() {
                if filter(child) {
                    collect_values_into(
                        raw,
                        child,
                        format!("{}.{}", path, key).as_str(),
                        filter,
                        insert,
                    );
                }
            }
        }
        TreeNode::Value { .. } => {
            insert(raw, path, node);
        }
    }
}

impl From<HashMap<String, RawSetting>> for SettingTree {
    fn from(map: HashMap<String, RawSetting>) -> Self {
        let mut root_children: HashMap<String, TreeNode> = HashMap::new();

        for (key, raw_setting) in map.iter() {
            let mut children = &mut root_children;
            let mut path = key.split('.').peekable();

            while let Some(key) = path.next() {
                if children.contains_key(key) {
                    if path.peek().is_some() {
                        children = children.get_mut(key).unwrap().children();
                    } else {
                        panic!("Key {} already exists", key);
                    }
                } else if path.peek().is_some() {
                    children = children
                        .entry(key.to_string())
                        .or_insert(TreeNode::Branch {
                            changed: false,

                            weight: raw_setting.weight,
                            description: raw_setting.description.clone(),
                            children: HashMap::new(),
                        })
                        .children();
                } else {
                    children.insert(
                        key.to_string(),
                        TreeNode::Value {
                            changed: false,

                            weight: raw_setting.weight,
                            description: raw_setting.description.clone(),
                            unit: raw_setting.unit.clone(),
                            value: raw_setting.value.clone(),
                        },
                    );

                    break;
                }
            }
        }

        Self { root_children }
    }
}

impl From<&SettingTree> for HashMap<String, RawSetting> {
    fn from(tree: &SettingTree) -> Self {
        let map = RefCell::new(HashMap::new());

        tree.collect_values_into(&map, &|_| true, &|map, path, node| {
            if let TreeNode::Value {
                weight,
                value,
                description,
                unit,
                ..
            } = node
            {
                map.borrow_mut().insert(
                    path.to_string(),
                    RawSetting {
                        weight: *weight,
                        value: value.clone(),
                        description: description.clone(),
                        unit: unit.clone(),
                    },
                );
            }
        });

        map.into_inner()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TreeNode {
    Branch {
        #[serde(skip)]
        changed: bool,

        weight: usize,
        children: HashMap<String, TreeNode>,
        description: String,
    },
    Value {
        #[serde(skip)]
        changed: bool,

        weight: usize,
        value: NodeValue,
        description: String,
        unit: Option<String>,
    },
}

impl TreeNode {
    fn show(&mut self, ui: &mut three_d::egui::Ui) -> bool {
        match self {
            TreeNode::Branch {
                children,
                description,
                changed,
                ..
            } => {
                let mut has_changed = false;

                CollapsingHeader::new(description.as_str())
                    .default_open(true)
                    .show(ui, |ui| {
                        for (_, child) in children.iter_mut() {
                            if child.show(ui) {
                                has_changed = true;
                            }
                        }
                    });

                *changed = has_changed;
                has_changed
            }
            TreeNode::Value {
                value,
                description,
                unit,
                changed,
                ..
            } => {
                ui.horizontal(|ui| {
                    *changed = value.show(description, unit.as_ref(), ui).changed();
                });

                *changed
            }
        }
    }

    fn children(&mut self) -> &mut HashMap<String, TreeNode> {
        match self {
            TreeNode::Branch { children, .. } => children,
            TreeNode::Value { .. } => panic!("Path is not a branch"),
        }
    }

    fn add(&mut self, path: &str, node: TreeNode) {
        match self {
            TreeNode::Branch { children, .. } => {
                children.insert(path.to_string(), node);
            }
            TreeNode::Value { .. } => {
                panic!("Path {} is not a branch", path);
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct RawSetting {
    weight: usize,
    value: NodeValue,
    description: String,
    unit: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum NodeValue {
    String(String),
    Float(f32),
    Bool(bool),
}

impl NodeValue {
    fn show(
        &mut self,
        description: &str,
        unit: Option<&String>,
        ui: &mut three_d::egui::Ui,
    ) -> Response {
        crate::config::gui::settings::SETTINGS_LABEL.label(ui, description);

        let response = match self {
            NodeValue::String(value) => ui.add(TextEdit::singleline(value)),
            NodeValue::Float(value) => ui.add(DragValue::new(value).max_decimals(3)),
            NodeValue::Bool(value) => ui.checkbox(value, ""),
        };

        if let Some(unit) = unit {
            ui.label(unit);
        }

        response
    }
}

mod test {
    use crate::settings::tree::NodeValue;

    use super::SettingTree;

    #[test]
    fn test_str_from_into_raw() {
        let content = serde_yaml::to_string(&create_test_tree().root_children).unwrap();
        let children = serde_yaml::from_str(content.as_str()).unwrap();

        let tree = SettingTree {
            root_children: children,
        };

        let map: std::collections::HashMap<String, super::RawSetting> =
            std::collections::HashMap::from(&tree);

        let content = serde_yaml::to_string(&map).unwrap();

        let map2: std::collections::HashMap<String, super::RawSetting> =
            serde_yaml::from_str(content.as_str()).unwrap();

        let tree2 = SettingTree::from(map2.clone());

        let map3: std::collections::HashMap<String, super::RawSetting> =
            std::collections::HashMap::from(&tree2);

        assert_eq!(map, map2);
        assert_eq!(map, map3);
    }

    pub(super) fn create_test_tree() -> SettingTree {
        let mut tree = SettingTree {
            root_children: std::collections::HashMap::new(),
        };

        let mut general = super::TreeNode::Branch {
            changed: false,
            weight: 0,
            description: "General".to_string(),
            children: std::collections::HashMap::new(),
        };

        general.add(
            "z_offset",
            super::TreeNode::Value {
                changed: false,
                weight: 0,
                description: "Z Offset".to_string(),
                unit: Some("mm".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        tree.add("general", general);

        let mut limits = super::TreeNode::Branch {
            changed: false,

            weight: 1,
            description: "Limits".to_string(),
            children: std::collections::HashMap::new(),
        };

        let mut max_feedrates = super::TreeNode::Branch {
            changed: false,

            weight: 0,
            description: "Max Feedrates".to_string(),
            children: std::collections::HashMap::new(),
        };

        max_feedrates.add(
            "movements_x",
            super::TreeNode::Value {
                changed: false,

                weight: 0,
                description: "X".to_string(),
                unit: Some("mm/s".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        max_feedrates.add(
            "movements_y",
            super::TreeNode::Value {
                changed: false,

                weight: 1,
                description: "Y".to_string(),
                unit: Some("mm/s".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        max_feedrates.add(
            "movements_z",
            super::TreeNode::Value {
                changed: false,

                weight: 2,
                description: "Z".to_string(),
                unit: Some("mm/s".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        limits.add("max_feedrates", max_feedrates);

        tree.add("limits", limits);

        tree
    }

    #[test]
    fn test_from_into_raw() {
        let mut raw = std::collections::HashMap::new();
        raw.insert(
            "general.nice.huhu".to_string(),
            super::RawSetting {
                weight: 0,
                value: NodeValue::Float(0.0),
                description: "Z Huhu".to_string(),
                unit: Some("mm".to_string()),
            },
        );

        raw.insert(
            "general.nice2.dir.nextval".to_string(),
            super::RawSetting {
                weight: 0,
                value: NodeValue::Float(0.0),
                description: "Z Haha".to_string(),
                unit: Some("mm".to_string()),
            },
        );

        raw.insert(
            "general1.nice2.haha4".to_string(),
            super::RawSetting {
                weight: 0,
                value: NodeValue::Float(0.0),
                description: "Z Haha4".to_string(),
                unit: Some("mm".to_string()),
            },
        );

        let tree = super::SettingTree::from(raw);

        let raw = std::collections::HashMap::from(&tree);

        let new_tree = super::SettingTree::from(raw.clone());

        let new_raw = std::collections::HashMap::from(&new_tree);

        assert_eq!(raw, new_raw);

        let tree = create_test_tree();

        let raw = std::collections::HashMap::from(&tree);

        let new_tree = super::SettingTree::from(raw.clone());

        let new_raw = std::collections::HashMap::from(&new_tree);

        assert_eq!(raw, new_raw);

        //assert_eq!(serde_json::to_string(&tree).unwrap(), None);
    }
}
