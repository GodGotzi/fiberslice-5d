use egui::epaint::ahash::HashMap;
use egui::Ui;

#[derive(Clone, Copy, PartialEq)]
pub struct Tab<'a> {
    title: &'a str,
    show: fn(&'a mut Ui),
}

impl <'a> Tab<'a> {
    pub fn new(title: &'a str, show: fn(&'a mut Ui)) -> Self {
        Self {
            title,
            show,
        }
    }
}

pub struct TabbedView<'a> {
    tabs: &'a mut Vec<Tab<'a>>,
    open_tab_index: &'a mut usize,
}

impl <'a> TabbedView<'a> {
    pub fn new(tabs: &'a mut Vec<Tab<'a>>, start_index: &'a mut usize) -> Self {

        Self {
            tabs,
            open_tab_index: start_index,
        }
    }

    pub fn show(&'a mut self, ui: &'a mut Ui) {
        ui.separator();
        ui.horizontal(|ui| {
            for i in 0..self.tabs.len() {
                let tab = &mut self.tabs[i];
                ui.selectable_value(self.open_tab_index, i, tab.title);
            }
        });

        ui.separator();
        (self.tabs[*self.open_tab_index].show)(ui);
    }
}