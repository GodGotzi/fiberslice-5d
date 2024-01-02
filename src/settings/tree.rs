use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use three_d::egui::{CollapsingHeader, DragValue, Response, TextEdit};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingTree {
    changed: bool,
    root: TreeNode,
}

impl Default for SettingTree {
    fn default() -> Self {
        let mut tree = SettingTree {
            root: TreeNode::Branch {
                changed: false,
                description: "root".to_string(),
                children: std::collections::HashMap::new(),
            },
            changed: false,
        };

        let mut general = TreeNode::Branch {
            changed: false,
            description: "General".to_string(),
            children: std::collections::HashMap::new(),
        };

        general.add(
            "z_offset",
            TreeNode::Leaf {
                changed: false,
                description: "Z Offset".to_string(),
                unit: Some("mm".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        tree.add("general", general);

        let mut limits = TreeNode::Branch {
            changed: false,
            description: "Limits".to_string(),
            children: std::collections::HashMap::new(),
        };

        let mut max_feedrates = TreeNode::Branch {
            changed: false,
            description: "Max Feedrates".to_string(),
            children: std::collections::HashMap::new(),
        };

        max_feedrates.add(
            "movements_x",
            TreeNode::Leaf {
                changed: false,
                description: "X".to_string(),
                unit: Some("mm/s".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        max_feedrates.add(
            "movements_y",
            TreeNode::Leaf {
                changed: false,
                description: "Y".to_string(),
                unit: Some("mm/s".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        max_feedrates.add(
            "movements_z",
            TreeNode::Leaf {
                changed: false,
                description: "Z".to_string(),
                unit: Some("mm/s".to_string()),
                value: NodeValue::Float(0.0),
            },
        );

        limits.add("max_feedrates", max_feedrates);

        tree.add("limits", limits);

        tree
    }
}

impl SettingTree {
    pub fn show(&mut self, ui: &mut three_d::egui::Ui) {
        self.changed = false;

        if let TreeNode::Branch { children, .. } = &mut self.root {
            for (_, child) in children.iter_mut() {
                if child.show(ui) {
                    self.changed = true;
                }
            }
        }
    }

    fn get(&self, path: &str) -> Option<&TreeNode> {
        let mut current = &self.root;
        let mut queue = path.split('.').rev().collect::<Vec<&str>>();

        while let Some(path) = queue.pop() {
            match current {
                TreeNode::Branch { children, .. } => {
                    if children.contains_key(path) {
                        current = children.get(path).unwrap();
                        continue;
                    } else {
                        return None;
                    }
                }
                TreeNode::Leaf { .. } => {
                    panic!("Path {} is not a branch", path);
                }
            }
        }

        Some(current)
    }

    fn add(&mut self, path: &str, node: TreeNode) {
        match self.root {
            TreeNode::Branch {
                ref mut children, ..
            } => {
                children.insert(path.to_string(), node);
            }
            TreeNode::Leaf { .. } => {
                panic!("Path {} is not a branch", path);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum TreeNode {
    Branch {
        changed: bool,
        children: HashMap<String, TreeNode>,
        description: String,
    },
    Leaf {
        changed: bool,
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

                *changed
            }
            TreeNode::Leaf {
                value,
                description,
                unit,
                changed,
            } => {
                ui.horizontal(|ui| {
                    *changed = value.show(description, unit.as_ref(), ui).changed();
                });

                *changed
            }
        }
    }

    fn add(&mut self, path: &str, node: TreeNode) {
        match self {
            TreeNode::Branch { children, .. } => {
                children.insert(path.to_string(), node);
            }
            TreeNode::Leaf { .. } => {
                panic!("Path {} is not a branch", path);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum NodeValue {
    String(String),
    Float(f32),
    Boolean(bool),
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
            NodeValue::Boolean(value) => ui.checkbox(value, ""),
        };

        if let Some(unit) = unit {
            ui.label(unit);
        }

        response
    }
}

mod test {

    #[test]
    pub fn test_tree() {
        let mut tree = super::SettingTree {
            root: super::TreeNode::Branch {
                changed: false,
                description: "root".to_string(),
                children: std::collections::HashMap::new(),
            },
            changed: false,
        };

        let mut general = super::TreeNode::Branch {
            changed: false,
            description: "General".to_string(),
            children: std::collections::HashMap::new(),
        };

        general.add(
            "z_offset",
            super::TreeNode::Leaf {
                changed: false,
                description: "Z Offset".to_string(),
                unit: Some("mm".to_string()),
                value: super::NodeValue::Float(0.0),
            },
        );

        tree.add("general", general);

        let mut limits = super::TreeNode::Branch {
            changed: false,
            description: "Limits".to_string(),
            children: std::collections::HashMap::new(),
        };

        let mut max_feedrates = super::TreeNode::Branch {
            changed: false,
            description: "Max Feedrates".to_string(),
            children: std::collections::HashMap::new(),
        };

        max_feedrates.add(
            "movements_x",
            super::TreeNode::Leaf {
                changed: false,
                description: "X".to_string(),
                unit: Some("mm/s".to_string()),
                value: super::NodeValue::Float(0.0),
            },
        );

        max_feedrates.add(
            "movements_y",
            super::TreeNode::Leaf {
                changed: false,
                description: "Y".to_string(),
                unit: Some("mm/s".to_string()),
                value: super::NodeValue::Float(0.0),
            },
        );

        max_feedrates.add(
            "movements_z",
            super::TreeNode::Leaf {
                changed: false,
                description: "Z".to_string(),
                unit: Some("mm/s".to_string()),
                value: super::NodeValue::Float(0.0),
            },
        );

        limits.add("max_feedrates", max_feedrates);

        tree.add("limits", limits);

        println!("{:#?}", tree);

        assert!(tree.get("limits.max_feedrates.movements_x").is_some());
        assert!(tree.get("limits.max_feedrates.movements_y").is_some());
        assert!(tree.get("limits.max_feedrates.movements_z").is_some());
        assert!(tree.get("general.z_offset").is_some());
    }
}
