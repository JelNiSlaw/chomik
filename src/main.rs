#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows

use std::fs;

use eframe::egui::{self, Area, ColorImage};
use eframe::emath::{Align2, Vec2};
use eframe::epaint::TextureHandle;
use eframe::CreationContext;
use image::ImageFormat;

fn main() {
    eframe::run_native(
        "ChomikBox",
        eframe::NativeOptions {
            always_on_top: true,
            decorated: false,
            transparent: true,
            ..Default::default()
        },
        Box::new(|ctx| Box::new(ChomikBox::new(ctx))),
    );
}

struct ChomikBox {
    frames: Vec<TextureHandle>,
    counter: usize,
}

impl ChomikBox {
    fn new(ctx: &CreationContext) -> Self {
        let files = fs::read_dir("assets/hamster").unwrap();
        let mut images = Vec::with_capacity(files.size_hint().0);
        for (i, file) in files.enumerate() {
            println!("{i}");
            let file = file.unwrap();
            let bytes = fs::read(file.path()).unwrap();
            let image = image::load_from_memory_with_format(&bytes, ImageFormat::Png).unwrap();
            let image = ColorImage::from_rgba_unmultiplied(
                [image.width() as _, image.height() as _],
                image.as_bytes(),
            );
            let image = ctx
                .egui_ctx
                .load_texture(file.file_name().to_string_lossy(), image);
            images.push(image);
        }
        Self {
            frames: images,
            counter: 0,
        }
    }
}

impl eframe::App for ChomikBox {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        frame.drag_window();
        Area::new("hamster")
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                let image = self.frames.get(self.counter).unwrap();
                ui.image(image, image.size_vec2());
            });
        self.counter = (self.counter + 1) % self.frames.len();
    }

    fn clear_color(&self, _: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
}
