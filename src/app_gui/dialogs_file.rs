use super::FileManagerApp;
use egui;
use std::fs;

impl FileManagerApp {
    pub(crate) fn render_properties_dialog(&mut self, ctx: &egui::Context) {
        // Properties Dialog
        if self.show_properties_dialog {
            if let Some(item) = self.properties_item.clone() {
                egui::Window::new("ℹ Properties")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .frame(
                        egui::Frame::window(&ctx.style())
                            .fill(self.palette().bg)
                            .rounding(12.0)
                            .inner_margin(20.0),
                    )
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(5.0);

                            let (icon, icon_color) = if item.is_dir {
                                ("📁", self.palette().warning)
                            } else {
                                ("📄", self.palette().text_dim)
                            };

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(icon).size(32.0).color(icon_color));
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(&item.name).size(16.0).strong());
                            });

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(10.0);

                            egui::Grid::new("properties_grid")
                                .num_columns(2)
                                .spacing([20.0, 8.0])
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new("Type:").strong());
                                    ui.label(if item.is_dir { "Directory" } else { "File" });
                                    ui.end_row();

                                    ui.label(egui::RichText::new("Size:").strong());
                                    ui.label(crate::filesystem::format_size(item.size));
                                    ui.end_row();

                                    ui.label(egui::RichText::new("Modified:").strong());
                                    ui.label(crate::filesystem::format_date(item.modified));
                                    ui.end_row();

                                    // Permissions
                                    ui.label(egui::RichText::new("Permissions:").strong());
                                    match crate::filesystem::get_permissions(&item.path) {
                                        Ok((summary, details)) => {
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    egui::RichText::new(summary)
                                                        .family(egui::FontFamily::Monospace)
                                                        .color(egui::Color32::from_rgb(
                                                            138, 180, 248,
                                                        )),
                                                );
                                                ui.add_space(5.0);
                                                ui.label(
                                                    egui::RichText::new(details).size(11.0).color(
                                                        self.palette().text_dim,
                                                    ),
                                                );
                                            });
                                        }
                                        Err(_) => {
                                            ui.label(
                                                egui::RichText::new("N/A")
                                                    .color(egui::Color32::GRAY),
                                            );
                                        }
                                    }
                                    ui.end_row();

                                    ui.label(egui::RichText::new("Path:").strong());
                                    ui.label(item.path.display().to_string());
                                    ui.end_row();

                                    // Symlink target (if the item is a link)
                                    if crate::filesystem::is_symlink(&item.path) {
                                        ui.label(egui::RichText::new("Symlink →:").strong());
                                        ui.label(
                                            crate::filesystem::symlink_target(&item.path)
                                                .unwrap_or_else(|| "(unknown)".to_string()),
                                        );
                                        ui.end_row();
                                    }

                                    // Editable chmod field (Unix only)
                                    #[cfg(unix)]
                                    {
                                        ui.label(egui::RichText::new("Change mode:").strong());
                                        ui.horizontal(|ui| {
                                            let edit = ui.add_sized(
                                                [70.0, 24.0],
                                                egui::TextEdit::singleline(&mut self.chmod_input)
                                                    .hint_text("e.g. 644")
                                                    .font(egui::TextStyle::Monospace),
                                            );
                                            if ui.button("Apply").clicked() && !self.chmod_input.is_empty() {
                                                match u32::from_str_radix(&self.chmod_input, 8) {
                                                    Ok(mode) => {
                                                        let exclude = item.path.clone();
                                                        match crate::filesystem::set_permissions(&exclude, mode) {
                                                            Ok(_) => {
                                                                self.status_message =
                                                                    format!("✅ Permissions changed to {:o} on {}", mode, item.name);
                                                            }
                                                            Err(e) => {
                                                                self.status_message =
                                                                    format!("❌ chmod failed: {}", e);
                                                            }
                                                        }
                                                    }
                                                    Err(_) => {
                                                        self.status_message =
                                                            "Invalid octal value".to_string();
                                                    }
                                                }
                                            }
                                            let _ = edit;
                                        });
                                    }
                                    ui.end_row();
                                });

                            ui.add_space(15.0);
                            ui.separator();
                            ui.add_space(10.0);

                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 100.0);
                                if ui
                                    .add_sized(
                                        [200.0, 36.0],
                                        egui::Button::new(egui::RichText::new("Close").size(14.0))
                                            .fill(self.palette().accent)
                                            .rounding(6.0),
                                    )
                                    .clicked()
                                {
                                    self.show_properties_dialog = false;
                                    self.properties_item = None;
                                }
                            });
                        });
                    });
            }
        }

    }

    pub(crate) fn render_new_folder_dialog(&mut self, ctx: &egui::Context) {
        // New Folder Dialog
        if self.show_new_folder_dialog {
            egui::Window::new("📁 Create New Folder")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(self.palette().bg)
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Folder name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.new_folder_name)
                                .hint_text("Enter folder name..."),
                        );
                        response.request_focus();

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Create").size(14.0))
                                        .fill(self.palette().success)
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.new_folder_name.is_empty() {
                                    let new_path = self
                                        .get_active_pane()
                                        .current_path
                                        .join(&self.new_folder_name);
                                    match fs::create_dir(&new_path) {
                                        Ok(_) => {
                                            self.status_message =
                                                format!("Created folder: {}", self.new_folder_name);
                                            let _ = self.get_active_pane_mut().refresh();
                                        }
                                        Err(e) => {
                                            self.status_message =
                                                format!("Error creating folder: {}", e);
                                        }
                                    }
                                    self.new_folder_name.clear();
                                    self.show_new_folder_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(self.palette().hover_bg)
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.new_folder_name.clear();
                                self.show_new_folder_dialog = false;
                            }
                        });
                    });
                });
        }

    }

    pub(crate) fn render_new_file_dialog(&mut self, ctx: &egui::Context) {
        if self.show_new_file_dialog {
            egui::Window::new("📄 Create New File")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(self.palette().bg)
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("File name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.new_file_name)
                                .hint_text("Enter file name (e.g. notes.txt)..."),
                        );

                        if response.changed() && !self.new_file_name.contains('.') {
                            ui.label(
                                egui::RichText::new("Tip: add an extension, e.g. notes.txt")
                                    .weak()
                                    .size(12.0),
                            );
                        }
                        response.request_focus();

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Create").size(14.0))
                                        .fill(self.palette().success)
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.new_file_name.is_empty() {
                                    let new_path = self
                                        .get_active_pane()
                                        .current_path
                                        .join(&self.new_file_name);
                                    if new_path.exists() {
                                        self.status_message =
                                            format!("File already exists: {}", self.new_file_name);
                                    } else {
                                        match fs::write(&new_path, b"") {
                                            Ok(_) => {
                                                self.status_message =
                                                    format!("Created file: {}", self.new_file_name);
                                                let _ = self.get_active_pane_mut().refresh();
                                            }
                                            Err(e) => {
                                                self.status_message =
                                                    format!("Error creating file: {}", e);
                                            }
                                        }
                                    }
                                    self.new_file_name.clear();
                                    self.show_new_file_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(self.palette().hover_bg)
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.new_file_name.clear();
                                self.show_new_file_dialog = false;
                            }
                        });
                    });
                });
        }
    }

    pub(crate) fn render_rename_dialog(&mut self, ctx: &egui::Context) {
        // Rename Dialog
        if self.show_rename_dialog {
            egui::Window::new("✏ Rename")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    egui::Frame::window(&ctx.style())
                        .fill(self.palette().bg)
                        .rounding(12.0)
                        .inner_margin(20.0),
                )
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("New name:").size(14.0));
                        ui.add_space(5.0);
                        let response = ui.add_sized(
                            [300.0, 30.0],
                            egui::TextEdit::singleline(&mut self.rename_new_name),
                        );
                        response.request_focus();

                        ui.add_space(15.0);
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✓ Rename").size(14.0))
                                        .fill(self.palette().success)
                                        .rounding(6.0),
                                )
                                .clicked()
                                || (response.lost_focus()
                                    && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                            {
                                if !self.rename_new_name.is_empty() {
                                    if let Some(item) = self.get_active_pane().get_selected_item() {
                                        let old_path = item.path.clone();
                                        let new_path = item
                                            .path
                                            .parent()
                                            .unwrap_or(item.path.as_ref())
                                            .join(&self.rename_new_name);
                                        match fs::rename(&old_path, &new_path) {
                                            Ok(_) => {
                                                self.status_message = format!(
                                                    "Renamed: {} → {}",
                                                    item.name, self.rename_new_name
                                                );
                                                let _ = self.get_active_pane_mut().refresh();
                                            }
                                            Err(e) => {
                                                self.status_message =
                                                    format!("Error renaming: {}", e);
                                            }
                                        }
                                    }
                                    self.rename_new_name.clear();
                                    self.show_rename_dialog = false;
                                }
                            }
                            ui.add_space(10.0);
                            if ui
                                .add_sized(
                                    [140.0, 36.0],
                                    egui::Button::new(egui::RichText::new("✗ Cancel").size(14.0))
                                        .fill(self.palette().hover_bg)
                                        .rounding(6.0),
                                )
                                .clicked()
                            {
                                self.rename_new_name.clear();
                                self.show_rename_dialog = false;
                            }
                        });
                    });
                });
        }

    }

}
