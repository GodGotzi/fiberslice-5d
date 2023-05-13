use egui::Context;
use crate::component::tabbed_view::*;

pub fn side_panel_ui(ctx: &Context) {
    egui::SidePanel::right("egui_demo_panel")
        .resizable(true)
        .default_width(150.0)
        .show(ctx, |ui| {
            let mut tabs = vec![
                Tab::new("Slice Settings", |ui| {
                    ui.label("This is the content of tab 1");
                }),
                Tab::new("Filament Settings", |ui| {
                    ui.label("This is the content of tab 2");
                }),
                Tab::new("Printer Settings", |ui| {
                    ui.label("This is the content of tab 3");
                }),
            ];

            let mut start_index = 0;
            let mut tabview = TabbedView::new(&mut tabs, &mut start_index);

            tabview.show(ui);
            /*
            TabBar::new("Options").show(ui, |ui| {
                // Add a tab page
                TabPage::new("Slice Settings").show(ui, |ui| {
                    ui.label("This is the content of tab 1");
                });

                TabPage::new("Filament Settings").show(ui, |ui| {
                    ui.label("This is the content of tab 2");
                });

                // Add another tab page
                TabPage::new("Printer Settings").show(ui, |ui| {
                    ui.label("This is the content of tab 2");
                });
            });
             */
        });
}