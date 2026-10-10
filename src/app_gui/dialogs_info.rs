use super::FileManagerApp;
use super::Theme;
use egui;

impl FileManagerApp {
    pub(crate) fn render_about_dialog(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        // About Dialog
        if self.show_about_dialog {
            egui::Window::new("About")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(12.0)
                        .inner_margin(25.0),
                )
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("⚡").size(64.0));
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("Filane - Dual Pane FM")
                                .size(20.0)
                                .strong(),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new(format!("Version {}", env!("CARGO_PKG_VERSION")))
                                .size(12.0)
                                .color(visuals.widgets.noninteractive.fg_stroke.color),
                        );
                        ui.add_space(15.0);
                        ui.label(egui::RichText::new("A modern, fast file manager").size(13.0));
                        ui.label(egui::RichText::new("built with Rust and egui").size(13.0));
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("Developed by AI Fach")
                                .size(13.0)
                                .color(visuals.widgets.active.bg_fill),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("💝 Support this project")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(255, 121, 198)),
                        );
                        ui.add_space(5.0);
                        if ui
                            .add_sized(
                                [200.0, 36.0],
                                egui::Button::new(egui::RichText::new("☕ Donate via PayPal").size(13.0))
                                    .fill(egui::Color32::from_rgb(0, 112, 186))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            if let Err(e) = open::that("https://www.paypal.com/paypalme/AchmadFachrie") {
                                self.status_message = format!("Failed to open PayPal: {}", e);
                            } else {
                                self.status_message = "Opening PayPal donation page...".to_string();
                            }
                        }
                        ui.add_space(20.0);
                        if ui
                            .add_sized(
                                [200.0, 36.0],
                                egui::Button::new(egui::RichText::new("Close").size(14.0))
                                    .fill(egui::Color32::from_rgb(66, 133, 244))
                                    .rounding(6.0),
                            )
                            .clicked()
                        {
                            self.show_about_dialog = false;
                        }
                        ui.add_space(10.0);
                    });
                });
        }

    }

    pub(crate) fn render_theme_selector(&mut self, ctx: &egui::Context) {
        // Theme Selector Dialog
        if self.show_theme_selector {
            let visuals = ctx.style().visuals.clone();
            
            egui::Window::new("🎨 Theme Selector")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(visuals.panel_fill)
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Choose a theme:").size(14.0).strong());
                        ui.add_space(10.0);

                        for theme in Theme::all() {
                            let is_current = self.current_theme == theme;
                            let button_text = if is_current {
                                format!("✓ {}", theme.name())
                            } else {
                                format!("   {}", theme.name())
                            };

                            let button = ui.add_sized(
                                [280.0, 36.0],
                                egui::Button::new(egui::RichText::new(button_text).size(14.0))
                                    .fill(if is_current {
                                        visuals.widgets.active.bg_fill
                                    } else {
                                        visuals.widgets.inactive.bg_fill
                                    })
                                    .rounding(6.0),
                            );

                            if button.clicked() {
                                self.current_theme = theme;
                                self.status_message = format!("Theme changed to: {}", theme.name());
                            }

                            ui.add_space(6.0);
                        }

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 100.0);
                            if ui
                                .add_sized(
                                    [200.0, 36.0],
                                    egui::Button::new(egui::RichText::new("Close").size(14.0))
                                        .fill(visuals.widgets.inactive.bg_fill)
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.show_theme_selector = false;
                            }
                        });
                    });
                });
        }

    }

}
