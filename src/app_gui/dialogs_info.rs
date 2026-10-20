use super::FileManagerApp;
use super::Theme;
use egui;

impl FileManagerApp {
    pub(crate) fn render_help_dialog(&mut self, ctx: &egui::Context) {
        // Keyboard shortcuts cheat sheet
        if self.show_help_dialog {
            egui::Window::new("⌨  Keyboard Shortcuts")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(self.palette().bg)
                        .rounding(12.0)
                        .inner_margin(25.0),
                )
                .show(ctx, |ui| {
                    const SECTIONS: &[(&str, &[(&str, &str)])] = &[
                        (
                            "📂 Navigation",
                            &[
                                ("Tab", "Switch between panes"),
                                ("Enter", "Open file / directory"),
                                ("Backspace", "Go up one level"),
                                ("Alt+Left", "Go back in history"),
                                ("Ctrl+H", "Show / hide hidden files"),
                                ("Ctrl+B", "Show / hide sidebar"),
                                ("Ctrl+T", "Open terminal here"),
                            ],
                        ),
                        (
                            "✂️ Files",
                            &[
                                ("Ctrl+C / Ctrl+X", "Copy / cut to clipboard"),
                                ("Ctrl+V / F7", "Paste"),
                                ("F5", "Copy (Total Commander style)"),
                                ("F6", "Move (Total Commander style)"),
                                ("F2", "Refresh both panes"),
                                ("Shift+F2", "Rename selected item"),
                                ("Ctrl+N", "Create new folder"),
                                ("Ctrl+Shift+N", "Create new file"),
                                ("Delete / F8", "Move to Trash"),
                                ("Shift+Delete", "Delete permanently (no Trash)"),
                                ("Ctrl+D", "Compare two selected files"),
                            ],
                        ),
                        (
                            "🎛 View",
                            &[
                                ("Ctrl+F", "Quick filter (fuzzy match)"),
                                ("F3", "Open file with default app"),
                                ("Space", "Toggle preview panel"),
                                ("Ctrl+= / Ctrl+-", "Bigger / smaller UI font"),
                                ("F1", "Show this help"),
                                ("ESC", "Close dialog / clear selection"),
                            ],
                        ),
                    ];
                    egui::ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                        for (title, rows) in SECTIONS {
                            ui.add_space(8.0);
                            ui.label(
                                egui::RichText::new(*title)
                                    .size(14.0)
                                    .strong()
                                    .color(self.palette().accent),
                            );
                            ui.separator();
                            for (key, desc) in *rows {
                                ui.horizontal(|ui| {
                                    ui.monospace(
                                        egui::RichText::new(*key)
                                            .size(13.0)
                                            .color(self.palette().warning),
                                    );
                                    ui.label(
                                        egui::RichText::new(" — ").weak(),
                                );
                                    ui.label(egui::RichText::new(*desc).size(13.0));
                                });
                            }
                        }
                    });
            ui.add_space(12.0);
            ui.vertical_centered(|ui| {
                if ui
                    .add_sized(
                        [140.0, 34.0],
                        egui::Button::new(egui::RichText::new("✗ Close").size(13.0))
                            .fill(self.palette().hover_bg),
                    )
                    .clicked()
                {
                    self.show_help_dialog = false;
                }
            });
                });
        }
    }

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
                        .fill(self.palette().bg)
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
                                .color(self.palette().accent),
                        );
                        ui.add_space(5.0);
                        if ui
                            .add_sized(
                                [200.0, 36.0],
                                egui::Button::new(egui::RichText::new("☕ Donate via PayPal").size(13.0))
                                    .fill(self.palette().accent)
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
                                    .fill(self.palette().accent)
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

                        // UI scale (font size) setting
                        ui.label(egui::RichText::new("UI Scale / Font Size:").size(14.0).strong());
                        ui.add_space(6.0);
                        let zoom_pct = format!("{:.0}%", self.zoom_level * 100.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized([28.0, 24.0], egui::Button::new("−"))
                                .clicked()
                            {
                                self.set_zoom_level(self.zoom_level - 0.1);
                            }
                            ui.add_sized([90.0, 24.0], egui::Slider::new(
                                &mut self.zoom_level,
                                0.7..=2.0,
                            ));
                            if ui
                                .add_sized([28.0, 24.0], egui::Button::new("+"))
                                .clicked()
                            {
                                self.set_zoom_level(self.zoom_level + 0.1);
                            }
                            ui.label(
                                egui::RichText::new(&zoom_pct)
                                    .size(12.0)
                                    .color(self.palette().warning),
                            );
                        });
                        ui.add_space(8.0);
                        let mut check_startup = self.check_updates_on_startup;
                        if ui
                            .checkbox(&mut check_startup, "🔍 Check for updates on startup (weekly)")
                            .clicked()
                        {
                            self.check_updates_on_startup = check_startup;
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
