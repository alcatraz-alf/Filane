use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_sidebar(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(5.0);

                // QUICK ACCESS Section
                ui.indent("quick_access", |ui| { // Use indent for consistent left padding
                    ui.horizontal(|ui| {
                        let arrow = if self.sidebar_quick_access_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new(arrow).size(14.0).color(self.palette().accent)).frame(false)).clicked() {
                            self.sidebar_quick_access_expanded = !self.sidebar_quick_access_expanded;
                        }
                        ui.label(
                            egui::RichText::new("QUICK ACCESS")
                                .size(13.0)
                                .strong()
                                .color(self.palette().text_dim),
                        );
                    });

                    if self.sidebar_quick_access_expanded {
                        ui.add_space(5.0);
                        let quick_items = crate::bookmarks::get_quick_access_items();
                        for item in quick_items {
                            let text = format!("{} {}", item.icon, item.name);
                            let response = ui.add(
                                egui::Button::new(egui::RichText::new(text).size(14.0))
                                    .frame(false)
                                    .min_size(egui::vec2(ui.available_width(), 32.0)),
                            );

                            if response.clicked() {
                                let path = item.path.clone();
                                let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                self.status_message = format!("Navigated to {}", path.display());
                            }
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // BOOKMARKS Section
                ui.indent("bookmarks", |ui| { // Use indent for consistent left padding
                    ui.horizontal(|ui| {
                        let arrow = if self.sidebar_bookmarks_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new(arrow).size(14.0).color(self.palette().accent)).frame(false)).clicked() {
                            self.sidebar_bookmarks_expanded = !self.sidebar_bookmarks_expanded;
                        }
                        ui.label(
                            egui::RichText::new("BOOKMARKS")
                                .size(13.0)
                                .strong()
                                .color(self.palette().text_dim),
                        );

                        // Add button positioned to the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new("➕").size(13.0))).clicked() {
                                self.show_add_bookmark_dialog = true;
                                self.new_bookmark_name = self
                                    .get_active_pane()
                                    .current_path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Bookmark")
                                    .to_string();
                            }
                        });
                    });

                    if self.sidebar_bookmarks_expanded {
                        ui.add_space(5.0);
                        let bookmarks = self.bookmark_manager.get_bookmarks().to_vec();
                        if bookmarks.is_empty() {
                            ui.label(
                                egui::RichText::new("No bookmarks")
                                    .size(13.0)
                                    .color(egui::Color32::GRAY),
                            );
                        } else {
                            let mut remove_index: Option<usize> = None;

                            for (_idx, bookmark) in bookmarks.iter().enumerate() {
                                let text = format!("{} {}", bookmark.icon, bookmark.name);
                                let response = ui.add(
                                    egui::Button::new(egui::RichText::new(text).size(14.0))
                                        .frame(false)
                                        .min_size(egui::vec2(ui.available_width(), 32.0)),
                                );

                                if response.clicked() {
                                    let path = bookmark.path.clone();
                                    let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                    self.status_message =
                                        format!("Navigated to {}", path.display());
                                }

                                // Right-click to remove
                                if response.secondary_clicked() {
                                    remove_index = Some(_idx);
                                }
                            }

                            if let Some(idx) = remove_index {
                                let _ = self.bookmark_manager.remove_bookmark(idx);
                                self.status_message = "Bookmark removed".to_string();
                            }
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // DEVICES Section
                ui.indent("devices", |ui| { // Use indent for consistent left padding
                    ui.horizontal(|ui| {
                        let arrow = if self.sidebar_devices_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        if ui.add_sized([24.0, 24.0], egui::Button::new(egui::RichText::new(arrow).size(14.0).color(self.palette().accent)).frame(false)).clicked() {
                            self.sidebar_devices_expanded = !self.sidebar_devices_expanded;
                        }
                        ui.label(
                            egui::RichText::new("DEVICES")
                                .size(13.0)
                                .strong()
                                .color(self.palette().text_dim),
                        );
                    });

                    if self.sidebar_devices_expanded {
                        ui.add_space(5.0);
                        let mounts = crate::filesystem::get_mount_points();
                        for mount in mounts.iter().take(5) {
                            let icon = if mount.is_removable { "🔌" } else { "💾" };
                            let label = if let Some(name) = mount.mount_point.file_name() {
                                name.to_string_lossy().to_string()
                            } else {
                                mount.mount_point.display().to_string()
                            };

                            let text = format!("{} {}", icon, label);
                            let response = ui.add(
                                egui::Button::new(egui::RichText::new(text).size(14.0))
                                    .frame(false)
                                    .min_size(egui::vec2(ui.available_width(), 32.0)),
                            );

                            if response.clicked() {
                                let path = mount.mount_point.clone();
                                let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                self.status_message = format!("Navigated to {}", path.display());
                            }
                        }

                        if mounts.len() > 5 {
                            ui.add_space(5.0);
                            if ui
                                .button(egui::RichText::new("View all...").size(13.0))
                                .clicked()
                            {
                                self.show_mounts_dialog = true;
                            }
                        }
                    }
                });

                ui.add_space(10.0);
            });
    }
}
