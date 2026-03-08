// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    array,
    time::{Duration, Instant},
};

use eframe::egui::{self, ColorImage, TextureHandle};

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
    start: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            texture: None,
            show_diagnostics: false,
            start: Instant::now(),
        }
    }
}

const IMAGE_RES: usize = 200;
impl MyApp {
    fn generate_image(&mut self) -> egui::ColorImage {
        const DIM: usize = 3 * IMAGE_RES * IMAGE_RES;
        let x = Instant::now().duration_since(self.start).as_millis();

        // blinking sin wave
        let impulse = (x as f64 / 500.0).sin().powi(2) * 150.0 + 105.0;
        debug_assert!(
            impulse > 105.0 && impulse < 256.0,
            "{impulse} not in 105.0..256.0"
        );

        let dist = |p1, p2| {
            let (x1, y1): (usize, usize) = p1;
            let (x2, y2): (usize, usize) = p2;

            let dx = (x2 as f64 - x1 as f64).powi(2);
            let dy = (y2 as f64 - y1 as f64).powi(2);

            (dx + dy).sqrt()
        };

        let data: [u8; DIM] = array::from_fn(|i| {
            let i = i / 3;
            let impulse = impulse as u8;
            let center = IMAGE_RES / 2;
            let d = dist((i / IMAGE_RES, i % IMAGE_RES), (center, center));
            let max_d = dist((0, 0), (center, center));

            let rescale = d / max_d;
            debug_assert!(
                rescale > 0.0 - f64::EPSILON && rescale < 1.0 + f64::EPSILON,
                "{rescale} not in 0.0..1.0"
            );

            (rescale * impulse as f64) as u8
        });

        ColorImage::from_rgb([IMAGE_RES, IMAGE_RES], &data)
    }
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint_after(Duration::from_millis(20));

            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            let img_data = self.generate_image();
            let texture: &mut egui::TextureHandle =
                self.texture.get_or_insert_with(|| {
                    // Load the texture only once.
                    ui.ctx().load_texture(
                        "my-image",
                        img_data.clone(),
                        Default::default(),
                    )
                });
            texture.set(img_data, Default::default());

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
