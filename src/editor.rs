use crate::app::ModuleState;
use egui::{Context, Ui, Window};

pub fn repl_buttons(ui: &mut Ui) {
    ui.set_min_width(100.0);

    ui.vertical(|ui| {
        if ui.button("Send Code").clicked() {
            // send code to repl
        }

        if ui.button("Undefine Code").clicked() {
            // undefine the code in the repl
        }
    });
}

pub fn make_module_editor(ctx: &Context, id: i32, state: &mut ModuleState) {
    Window::new(&id.to_string())
        .open(&mut state.closed)
        .show(ctx, |ui| {
            ui.label(format!("Window: {}", id));

            ui.horizontal(|mut ui| {
                ui.code_editor(&mut state.source);
                repl_buttons(&mut ui);
            });

            //if ui.button("click me to close").clicked() {
            //          on_close(state);
            //}
        });
}
