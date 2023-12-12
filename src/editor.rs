use crate::app::ModuleState;
use egui::{Context, Window};

pub fn make_module_editor(ctx: &Context, id: i32, state: &mut ModuleState) {
    Window::new(&id.to_string())
        .open(&mut state.closed)
        .show(ctx, |ui| {
            ui.label(format!("Window: {}", id));

            ui.code_editor(&mut state.source);

            //if ui.button("click me to close").clicked() {
            //          on_close(state);
            //}
        });
}
