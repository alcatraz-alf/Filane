use dual_pane_fm::app_gui::FileManagerApp;

fn main() -> eframe::Result<()> {
    // Load icon from embedded bytes
    let icon_bytes = include_bytes!("../../assets/icon.png");
    let icon = eframe::icon_data::from_png_bytes(icon_bytes).expect("Failed to load icon");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_min_inner_size([800.0, 500.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Filane - Dual Pane FM",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_zoom_factor(1.1);
            
            Ok(Box::new(FileManagerApp::new(cc)))
        }),
    )
}
