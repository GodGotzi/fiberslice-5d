use egui::epaint::ahash::HashMap;
use egui::{Align, Direction, Ui};

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
    open_tab_index: usize,
}

impl <'a> TabbedView<'a> {
    pub fn new(tabs: &'a mut Vec<Tab<'a>>, _start_index: &'a mut usize) -> Self {

        Self {
            tabs,
            open_tab_index: 0,
        }
    }

    pub fn show(&'a mut self, ui: &'a mut Ui) {
        ui.separator();

        ui.horizontal(|ui| {
            //horizontal layout centered each tab

            let layout = egui::Layout {
                main_dir: Direction::LeftToRight,
                main_wrap: true,
                main_align: Align::Center,
                main_justify: false,
                cross_align: Align::Center,
                cross_justify: true,
            };

            ui.with_layout(layout, |ui| {
                for i in 0..self.tabs.len() {
                    let tab = &mut self.tabs[i];

                    ui.selectable_value(&mut self.open_tab_index, i, tab.title);
                    if (i + 1) < self.tabs.len() {
                        ui.separator();
                    }
                }
            });
        });

        ui.separator();
        (self.tabs[self.open_tab_index].show)(ui);
    }
}