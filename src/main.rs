// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::array;

use eframe::egui::{self, ColorImage, TextureHandle};
use rand::{RngExt, SeedableRng, rngs::SmallRng};

fn main() -> eframe::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_pixels_per_point(4.0);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    texture: Option<TextureHandle>,
    show_diagnostics: bool,
    rng: SmallRng,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            texture: None,
            show_diagnostics: false,
            rng: SmallRng::seed_from_u64(43),
        }
    }
}

const IMAGE_RES: usize = 200;
fn generate_image(rng: &mut SmallRng) -> egui::ColorImage {
    const DIM: usize = 3 * IMAGE_RES * IMAGE_RES;
    let data: [u8; DIM] = array::from_fn(|_i| rng.random_range(0..200) as u8);

    return ColorImage::from_rgb([IMAGE_RES, IMAGE_RES], &data);
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();

            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            let texture: &mut egui::TextureHandle =
                self.texture.get_or_insert_with(|| {
                    // Load the texture only once.
                    ui.ctx().load_texture(
                        "my-image",
                        generate_image(&mut self.rng),
                        Default::default(),
                    )
                });
            texture.set(generate_image(&mut self.rng), Default::default());

            // Show the image:
            ui.image((texture.id(), texture.size_vec2()));

            // FPS counter
            ui.label(format!("FPS: {:.1}", 1.0 / ctx.input(|i| i.stable_dt)));

            // Diagnostics
            if ui.button("Show diagnostics").clicked() {
                self.show_diagnostics = !self.show_diagnostics;
            }

            egui::Window::new("Diagnostics")
                .open(&mut self.show_diagnostics)
                .scroll(true)
                .default_height(100.0)
                .show(ctx, |ui| {
                    ctx.inspection_ui(ui);
                });
        });
    }
}
