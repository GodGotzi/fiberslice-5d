use bevy_egui::egui;
use bevy_egui::egui::style::WidgetVisuals;

pub fn customize_look_and_feel(mut visuals: egui::Visuals) -> egui::Visuals {
    visuals.selection.bg_fill = egui::Color32::from_rgb(76, 255, 0);
    visuals.selection.stroke.color = egui::Color32::from_rgb(0, 0, 0);

    disable_rounding(&mut visuals.widgets.noninteractive);
    disable_rounding(&mut visuals.widgets.active);
    disable_rounding(&mut visuals.widgets.hovered);
    disable_rounding(&mut visuals.widgets.inactive);
    disable_rounding(&mut visuals.widgets.open);

    visuals
}

pub fn disable_rounding(widget_visuals: &mut WidgetVisuals) {
    widget_visuals.rounding.ne = 0.0;
    widget_visuals.rounding.nw = 0.0;
    widget_visuals.rounding.se = 0.0;
    widget_visuals.rounding.sw = 0.0;
}