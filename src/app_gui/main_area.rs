use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_main_area(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(6.0, 2.0)),
            )
            .show(ctx, |ui| {
                let _full_rect = ui.available_rect_before_wrap();
                let preview_height = if self.show_preview_panel { 200.0 } else { 0.0 };
                let spacing = if self.show_preview_panel { 4.0 } else { 0.0 };
                
                // Use StripBuilder for proper responsive layout with exact sizes
                egui_extras::StripBuilder::new(ui)
                    .size(egui_extras::Size::remainder()) // Panes area
                    .size(egui_extras::Size::exact(preview_height + spacing)) // Preview
                    .vertical(|mut strip| {
                        // Panes strip - horizontal layout
                        strip.cell(|ui| {
                            // Calculate available width and split evenly between panes
                            let available_width = ui.available_width();
                            let transfer_btn_width = 50.0;
                            let pane_width = (available_width - transfer_btn_width) / 2.0;
                            
                            egui_extras::StripBuilder::new(ui)
                                .size(egui_extras::Size::exact(pane_width)) // Left pane - exact half
                                .size(egui_extras::Size::exact(transfer_btn_width)) // Transfer buttons - fixed width
                                .size(egui_extras::Size::exact(pane_width)) // Right pane - exact half
                                .horizontal(|mut strip| {
                                    // Left pane
                                    strip.cell(|ui| {
                                        ui.set_clip_rect(ui.max_rect());
                                        self.render_pane(ui, 0, ctx, pane_width);
                                    });
                                    
                                    // Transfer buttons
                                    strip.cell(|ui| {
                                        self.render_transfer_buttons(ui);
                                    });
                                    
                                    // Right pane
                                    strip.cell(|ui| {
                                        ui.set_clip_rect(ui.max_rect());
                                        self.render_pane(ui, 1, ctx, pane_width);
                                    });
                                });
                        });
                        
                        // Preview panel
                        if self.show_preview_panel {
                            strip.cell(|ui| {
                                self.render_preview_panel(ui, ctx);
                            });
                        }
                    });
            });

        // Handle keyboard shortcuts
    }
}
