use egui::ImageButton;
use egui::{Context, SidePanel};

use crate::config;
use crate::ui::boundary::Boundary;
use crate::ui::tools::ToolState;
use crate::ui::Component;
use crate::ui::ComponentState;
use crate::ui::UiState;
use crate::GlobalState;
use crate::RootEvent;

#[derive(Debug)]
pub struct ToolBarState {
    enabled: bool,
    boundary: Boundary,
}

impl ToolBarState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            boundary: Boundary::zero(),
        }
    }
}

impl ComponentState for ToolBarState {
    fn get_boundary(&self) -> Boundary {
        self.boundary
    }

    fn get_enabled(&mut self) -> &mut bool {
        &mut self.enabled
    }

    fn get_name(&self) -> &str {
        "Toolbar"
    }
}

pub struct Toolbar<'a> {
    state: &'a mut ToolBarState,
    tools: &'a mut [&'a mut dyn ToolState],
}

impl<'a> Toolbar<'a> {
    pub fn with_state(state: &'a mut ToolBarState) -> Self {
        Self {
            state,
            tools: &mut [],
        }
    }

    pub fn with_tools(mut self, tools: &'a mut [&'a mut dyn ToolState]) -> Self {
        self.tools = tools;
        self
    }
}

impl<'a> Component for Toolbar<'a> {
    fn show(&mut self, ctx: &Context, _shared_state: &(UiState, GlobalState<RootEvent>)) {
        if self.state.enabled {
            self.state.boundary = SidePanel::left("toolbar")
                .resizable(false)
                .default_width(config::gui::TOOLBAR_W)
                .show(ctx, |ui| {
                    for tool in self.tools.iter_mut() {
                        let button = config::gui::TOOL_TOGGLE_BUTTON;

                        let icon = tool.get_icon();

                        let image_button = ImageButton::new(icon).frame(true);

                        ui.allocate_ui(
                            [button.size.0 + button.border, button.size.1 + button.border].into(),
                            |ui| {
                                let response =
                                    ui.add_sized([button.size.0, button.size.1], image_button);

                                if response.clicked() {
                                    *tool.get_enabled() = !*tool.get_enabled();
                                }
                            },
                        );
                    }
                })
                .response
                .into();
        }
    }
}
