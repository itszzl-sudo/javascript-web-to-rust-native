//! Vue Demo with egui - 展示 Vue 组件在原生窗口中渲染

use eframe::egui;
use jrust_runtime::document::Document;
use jrust_runtime::element::Element;
use jrust_runtime::core::{JsValue, JsObject};

struct VueApp {
    title: String,
    count: i32,
    document: Document,
}

impl Default for VueApp {
    fn default() -> Self {
        Self {
            title: "JRust Vue Demo".to_string(),
            count: 0,
            document: Document::new(),
        }
    }
}

impl eframe::App for VueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading(&self.title);
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Increment").clicked() {
                        self.count += 1;
                        println!("Button clicked! Count: {}", self.count);
                    }
                    
                    if ui.button("Decrement").clicked() {
                        self.count -= 1;
                        println!("Button clicked! Count: {}", self.count);
                    }
                });
                
                ui.add_space(10.0);
                ui.label(format!("Count: {}", self.count));
                
                ui.add_space(20.0);
                ui.separator();
                
                ui.collapsing("DOM Structure", |ui| {
                    ui.label("Document structure:");
                    ui.label("- <div id='app'>");
                    ui.label(format!("  - <h1>{}", self.title));
                    ui.label("  - <button>Increment");
                    ui.label("  - <button>Decrement");
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    println!("=== Vue Demo with egui ===");
    println!("Starting native window...");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("JRust Vue Demo"),
        ..Default::default()
    };
    
    eframe::run_native(
        "JRust Vue Demo",
        options,
        Box::new(|_cc| Ok(Box::new(VueApp::default()))),
    )
}
