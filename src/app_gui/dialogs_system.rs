use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_mounts_dialog(&mut self, ctx: &egui::Context) {
        // Mount Points Dialog
        if self.show_mounts_dialog {
            let mut close_dialog = false;

            egui::Window::new("💾 Mount Points")
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 500.0])
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(12.0)
                        .inner_margin(15.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);

                        ui.label(
                            egui::RichText::new("Available mount points and storage devices")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(189, 193, 198)),
                        );

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        let mounts = crate::filesystem::get_mount_points();

                        egui::ScrollArea::vertical()
                            .max_height(350.0)
                            .show(ui, |ui| {
                                for mount in &mounts {
                                    let icon = if mount.is_removable { "🔌" } else { "💾" };
                                    let usage = mount.usage_percentage();
                                    let used = mount.total_space - mount.available_space;

                                    let usage_color = if usage > 90.0 {
                                        egui::Color32::from_rgb(242, 139, 130)
                                    } else if usage > 70.0 {
                                        egui::Color32::from_rgb(255, 193, 7)
                                    } else {
                                        egui::Color32::from_rgb(129, 201, 149)
                                    };

                                    egui::Frame::default()
                                        .fill(egui::Color32::from_rgb(50, 52, 55))
                                        .rounding(8.0)
                                        .inner_margin(12.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(egui::RichText::new(icon).size(24.0));
                                                ui.add_space(8.0);

                                                ui.vertical(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(
                                                                mount
                                                                    .mount_point
                                                                    .display()
                                                                    .to_string(),
                                                            )
                                                            .size(14.0)
                                                            .strong()
                                                            .color(egui::Color32::from_rgb(
                                                                138, 180, 248,
                                                            )),
                                                        );

                                                        if mount.is_removable {
                                                            ui.label(
                                                                egui::RichText::new("Removable")
                                                                    .size(10.0)
                                                                    .color(
                                                                        egui::Color32::from_rgb(
                                                                            189, 147, 249,
                                                                        ),
                                                                    ),
                                                            );
                                                        }
                                                    });

                                                    ui.add_space(4.0);

                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "Device: {}",
                                                                mount.device_name
                                                            ))
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(
                                                                189, 193, 198,
                                                            )),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new("•").size(11.0),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "Type: {}",
                                                                mount.file_system
                                                            ))
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(
                                                                189, 193, 198,
                                                            )),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new("•").size(11.0),
                                                        );
                                                        ui.label(
                                                            egui::RichText::new(&mount.disk_kind)
                                                                .size(11.0)
                                                                .color(egui::Color32::from_rgb(
                                                                    189, 193, 198,
                                                                )),
                                                        );
                                                    });

                                                    ui.add_space(6.0);

                                                    ui.horizontal(|ui| {
                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "{} / {}",
                                                                crate::filesystem::format_size(
                                                                    used
                                                                ),
                                                                crate::filesystem::format_size(
                                                                    mount.total_space
                                                                )
                                                            ))
                                                            .size(11.0),
                                                        );

                                                        ui.label(
                                                            egui::RichText::new("•").size(11.0),
                                                        );

                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "Available: {}",
                                                                crate::filesystem::format_size(
                                                                    mount.available_space
                                                                )
                                                            ))
                                                            .size(11.0)
                                                            .color(egui::Color32::from_rgb(
                                                                129, 201, 149,
                                                            )),
                                                        );
                                                    });

                                                    ui.add_space(6.0);

                                                    let progress_width =
                                                        ui.available_width() - 80.0;
                                                    ui.horizontal(|ui| {
                                                        let progress_bar =
                                                            egui::ProgressBar::new(usage / 100.0)
                                                                .desired_width(progress_width)
                                                                .fill(usage_color);
                                                        ui.add(progress_bar);

                                                        ui.label(
                                                            egui::RichText::new(format!(
                                                                "{:.1}%",
                                                                usage
                                                            ))
                                                            .size(11.0)
                                                            .strong()
                                                            .color(usage_color),
                                                        );
                                                    });

                                                    ui.add_space(6.0);

                                                    if ui
                                                        .button(
                                                            egui::RichText::new("📂 Open")
                                                                .size(11.0),
                                                        )
                                                        .clicked()
                                                    {
                                                        let active_pane =
                                                            self.get_active_pane_mut();
                                                        if let Ok(()) = active_pane
                                                            .navigate_to(mount.mount_point.clone())
                                                        {
                                                            self.status_message = format!(
                                                                "Navigated to {}",
                                                                mount.mount_point.display()
                                                            );
                                                            close_dialog = true;
                                                        }
                                                    }
                                                });
                                            });
                                        });

                                    ui.add_space(8.0);
                                }

                                if mounts.is_empty() {
                                    ui.vertical_centered(|ui| {
                                        ui.add_space(40.0);
                                        ui.label(
                                            egui::RichText::new("No mount points found")
                                                .size(14.0)
                                                .color(egui::Color32::from_rgb(154, 160, 166)),
                                        );
                                    });
                                }
                            });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add_sized(
                                            [100.0, 32.0],
                                            egui::Button::new(
                                                egui::RichText::new("Close").size(12.0),
                                            )
                                            .fill(egui::Color32::from_rgb(66, 70, 77))
                                            .rounding(6.0),
                                        )
                                        .clicked()
                                    {
                                        close_dialog = true;
                                    }
                                },
                            );
                        });
                    });
                });

            if close_dialog {
                self.show_mounts_dialog = false;
            }
        }

    }

    pub(crate) fn render_bookmark_dialog(&mut self, ctx: &egui::Context) {
        // Add Bookmark Dialog
        if self.show_add_bookmark_dialog {
            egui::Window::new("⭐ Add Bookmark")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(egui::Color32::from_rgb(45, 47, 50))
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);

                        ui.label(egui::RichText::new("Bookmark name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.new_bookmark_name)
                                .hint_text("Enter bookmark name..."),
                        );
                        response.request_focus();

                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Path: {}",
                                self.get_active_pane().current_path.display()
                            ))
                            .size(11.0)
                            .color(egui::Color32::from_rgb(138, 180, 248)),
                        );

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Add").size(14.0))
                                        .fill(egui::Color32::from_rgb(40, 167, 69))
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.new_bookmark_name.is_empty() {
                                    let path = self.get_active_pane().current_path.clone();
                                    match self.bookmark_manager.add_bookmark(
                                        self.new_bookmark_name.clone(),
                                        path.clone(),
                                        "📁".to_string(),
                                    ) {
                                        Ok(_) => {
                                            self.status_message = format!(
                                                "Added bookmark: {}",
                                                self.new_bookmark_name
                                            );
                                        }
                                        Err(e) => {
                                            self.status_message =
                                                format!("Error adding bookmark: {}", e);
                                        }
                                    }
                                    self.new_bookmark_name.clear();
                                    self.show_add_bookmark_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(egui::Color32::from_rgb(66, 70, 77))
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.new_bookmark_name.clear();
                                self.show_add_bookmark_dialog = false;
                            }
                        });
                    });
                });
        }

    }

}
