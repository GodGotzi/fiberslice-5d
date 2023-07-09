
use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Ui};

use crate::{prelude::*, config};


pub struct Toolbar;

impl Toolbar {
    pub fn new() -> Self {
        Self { }
    }
}

impl super::Component<Toolbar> for Toolbar {

    fn show(&mut self, ctx: &egui::Context, 
        _ui: Option<&mut Ui>,
        _mode_ctx: Option<&mut Mode>,
        gui_interface: &mut ResMut<super::Interface>,          
        item_wrapper: &mut ResMut<AsyncWrapper>, 
    ) {

        let response = egui::SidePanel::left("toolbar")
            .resizable(false)
            .default_width(config::gui::TOOLBAR_W)
            .show(ctx, |ui| {

                item_wrapper.register(Item::ToolbarWidth(Some(ui.available_width())));
                
            }).response;

        let rect = response.rect;

        gui_interface.register_boundary(
            super::Boundary::new(rect.min.x, rect.min.y, rect.width(), rect.height())
        );
    }
}
