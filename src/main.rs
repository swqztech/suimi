#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod fonts;
mod generate;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 520.0])
            .with_min_inner_size([420.0, 480.0])
            .with_title("随密"),
        ..Default::default()
    };
    eframe::run_native(
        "随密",
        options,
        Box::new(|cc| {
            fonts::setup_cjk_fonts(&cc.egui_ctx);
            Ok(Box::new(app::SuimiApp::new()))
        }),
    )
}
