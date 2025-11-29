use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "CatSay GUI",
        options,
        Box::new(|_cc| Ok(Box::new(CatSayApp::default()))),
    )
}

struct CatSayApp {
    message: String,
    is_dead: bool,
    show_image: bool,
    cat_texture: Option<egui::TextureHandle>,
    cat_dead_texture: Option<egui::TextureHandle>,
}

impl Default for CatSayApp {
    fn default() -> Self {
        Self {
            message: String::new(),
            is_dead: false,
            show_image: false,
            cat_texture: None,
            cat_dead_texture: None,
        }
    }
}

impl CatSayApp {
    fn load_image(
        ctx: &egui::Context,
        texture: &mut Option<egui::TextureHandle>,
        name: &str,
        path: &str,
    ) {
        if texture.is_none() {
            if let Ok(image) = image::open(path) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    size,
                    pixels.as_slice(),
                );
                *texture = Some(ctx.load_texture(
                    name,
                    color_image,
                    egui::TextureOptions::default(),
                ));
            }
        }
    }
}

impl eframe::App for CatSayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load images lazily
        Self::load_image(ctx, &mut self.cat_texture, "cat", "./images/cat.png");
        Self::load_image(ctx, &mut self.cat_dead_texture, "cat_dead", "./images/cat_dead.png");

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐱 CatSay GUI");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Message:");
                ui.text_edit_singleline(&mut self.message);
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Cat Status:");
                ui.checkbox(&mut self.is_dead, "Is Dead");
            });

            ui.add_space(10.0);

            if ui.button("Generate").clicked() {
                self.show_image = true;
            }

            ui.add_space(20.0);

            if self.show_image {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        // Display the message in a speech bubble style
                        let message_text = if self.message.is_empty() {
                            "Meow! 🐱".to_string()
                        } else {
                            format!("{}\n \\n \\", self.message)
                        };

                        ui.label(
                            egui::RichText::new(message_text)
                                .size(16.0)
                                .color(egui::Color32::from_rgb(50, 50, 50)),
                        );

                        ui.add_space(10.0);

                        // Display the appropriate cat image
                        let texture = if self.is_dead {
                            self.cat_dead_texture.as_ref()
                        } else {
                            self.cat_texture.as_ref()
                        };

                        if let Some(tex) = texture {
                            let size = tex.size_vec2();
                            let max_width = 300.0;
                            let scale = if size[0] > max_width {
                                max_width / size[0]
                            } else {
                                1.0
                            };
                            ui.image((tex.id(), size * scale));
                        } else {
                            ui.label("⚠️ Image not found. Make sure cat images are in ./images/");
                        }
                    });
                });
            }
        });
    }
}
