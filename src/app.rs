use std::{collections::HashMap, fmt::format, sync::{mpsc::channel, Arc, Mutex, RwLock}, thread::spawn};

use egui::{Context, DragValue, RichText, Ui, Window};
use reqwest::blocking::Client;

use crate::editor::make_module_editor;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ModuleState {
    pub closed: bool,
    pub source: String,
}

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    /// The port of the nrepl server
    pub nrepl_port: i32,

    /// The port for the streamline server
    pub streamline_server_port: i32,

    /// The host for the nrepl server
    pub nrepl_host: String,

    /// The host for the streamline server
    pub streamline_host: String,

    #[serde(skip)]
    /// The reqwest client
    pub client: Client,
}

trait StreamlineClient {
    fn send_code(&self, code: &String) -> String;
    fn undefine_module(&self, module_name: &String) -> String;
    fn load_blocks(&self, start_block: i32, stop_block: i32) -> String;
    fn execute_module(&self, start_block: i32, stop_block:i32, module_name: &String) -> String;
}

impl StreamlineClient for RwLock<AppConfig> {
    fn send_code(&self, code: &String) -> String {
        let data = self.read().unwrap();
        let url = format!(
            "http://{}:{}/nrepl",
            data.streamline_host, data.streamline_server_port
        );

        let mut map = HashMap::new();
        map.insert("src", code);

        let res = data.client.post(url).json(&map).send().unwrap();

        res.text().unwrap()
    }

    fn undefine_module(&self, module_name: &String) -> String {
        todo!("Should undefined the module in the repl")
    }

    fn load_blocks(&self, start_block: i32, stop_block: i32) -> String {
        todo!("Should Load Blocks in the Repl")
    }

    fn execute_module(&self, start_block: i32, stop_block: i32, module_name: &String) -> String {
        todo!("Should execute the module for the selected block range")
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            nrepl_port: 7869,
            streamline_server_port: 8080,
            nrepl_host: String::from("localhost"),
            streamline_host: String::from("localhost"),
            client: Client::new(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    /// A hashmap from id -> code for that module
    modules: HashMap<i32, ModuleState>,

    /// Toggles whether to show the config panel or not
    show_config: bool,

    /// App Config
    config: Arc<RwLock<AppConfig>>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            modules: Default::default(),
            show_config: false,
            config: Arc::new(RwLock::new(AppConfig::default())),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn render_config(app: &mut TemplateApp, ctx: &Context) {
    Window::new("App Config")
        .open(&mut app.show_config)
        .show(ctx, |ui| {
            egui::Grid::new("App Config")
                .num_columns(1)
                .min_col_width(40.0)
                .max_col_width(75.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        let mut config = app.config.write().unwrap();
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Nrepl Port:").size(15.0));
                            ui.add(DragValue::new(&mut config.nrepl_port));
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Streamline Server Port:").size(15.0));
                            ui.add(
                                DragValue::new(&mut config.streamline_server_port)
                                    .clamp_range(1000..=10000),
                            );
                        });
                    });
                });
        });
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                    ui.menu_button("Config", |ui| {
                        ui.checkbox(&mut self.show_config, "Show Config Panel");
                    });
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            if self.show_config {
                render_config(self, ctx);
            }

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            if ui.button("Send code").clicked() {
                let code = String::from("stream foo_bar;");

                let resp = self.config.send_code(&code);

                println!("{}", resp);
            }

            for i in 1..4 {
                let module = match self.modules.get_mut(&i) {
                    Some(module) => module,
                    None => {
                        self.modules.insert(
                            i,
                            ModuleState {
                                closed: true,
                                source: String::from("Hello world!"),
                            },
                        );
                        self.modules.get_mut(&i).unwrap()
                    }
                };

                make_module_editor(ctx, i, module);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
