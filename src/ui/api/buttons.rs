use three_d::egui::{self, ImageButton, Ui};

use crate::ui::{icon, response::Responsive, UiData};

pub struct DecoradedButton {
    pub border: f32,
    pub size: (f32, f32),
    pub hover_color: egui::Color32,
}

pub trait DecoradedButtons {
    fn add_toggle_button<T: 'static + Into<usize> + std::clone::Clone + Copy>(
        &mut self,
        data: &mut UiData,
        button: &DecoradedButton,
        t: T,
        callback: Box<dyn FnOnce(&mut UiData)>,
    );

    fn add_responsive_button<T: 'static + Into<usize> + std::clone::Clone + Copy>(
        &mut self,
        data: &mut UiData,
        button: &DecoradedButton,
        t: T,
        callback: Box<dyn FnOnce(&mut UiData)>,
    );
}

impl DecoradedButtons for Ui {
    fn add_toggle_button<T: 'static + Into<usize> + std::clone::Clone + Copy>(
        &mut self,
        data: &mut UiData,
        button: &DecoradedButton,
        t: T,
        callback: Box<dyn FnOnce(&mut UiData)>,
    ) {
        let icon = icon::ICONTABLE.get_icon(t).unwrap();

        let image_button =
            ImageButton::new(icon.texture_id(self.ctx()), icon.size_vec2()).frame(false);

        self.allocate_ui(
            [button.size.0 + button.border, button.size.1 + button.border].into(),
            move |ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        let clicked;

                        {
                            let mut ui_state = data.state.borrow_mut();
                            let prev_response = ui_state.responses.get_button_response(t).unwrap();

                            if prev_response.hovered() {
                                ui.painter().rect_filled(
                                    ui.available_rect_before_wrap(),
                                    0.0,
                                    button.hover_color,
                                );
                            }

                            let response =
                                ui.add_sized([button.size.0, button.size.1], image_button);

                            ui_state.responses.update_button_response(t, &response);

                            clicked = response.clicked();
                        }

                        if clicked {
                            callback(data);
                        }
                    },
                );
            },
        );
    }

    fn add_responsive_button<T: 'static + Into<usize> + std::clone::Clone + Copy>(
        &mut self,
        data: &mut UiData,
        button: &DecoradedButton,
        t: T,
        callback: Box<dyn FnOnce(&mut UiData)>,
    ) {
        let icon = icon::ICONTABLE.get_icon(t).unwrap();

        let image_button =
            ImageButton::new(icon.texture_id(self.ctx()), icon.size_vec2()).frame(false);

        self.allocate_ui(
            [button.size.0 + button.border, button.size.1 + button.border].into(),
            |ui| {
                let clicked;

                {
                    let mut ui_state = data.state.borrow_mut();
                    let prev_response = ui_state.responses.get_button_response(t).unwrap();

                    if prev_response.hovered() {
                        ui.painter().rect_filled(
                            ui.available_rect_before_wrap(),
                            0.0,
                            button.hover_color,
                        );
                    }

                    let response = ui.add_sized([button.size.0, button.size.1], image_button);

                    ui_state.responses.update_button_response(t, &response);

                    clicked = response.clicked();
                }

                if clicked {
                    callback(data);
                    /*
                                       data.borrow_shared_state()
                       .writer_environment_event
                       .send(crate::environment::EnvironmentEvent::SendOrientation(t))
                    */
                }
            },
        );
    }
}
