use egui::Context;

pub fn side_panel_ui(ctx: &Context) {
    egui::SidePanel::right("egui_demo_panel")
        .resizable(true)
        .default_width(150.0)
        .show(ctx, |ui| {
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