use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_menu_bar(&mut self, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();

        // Top menu bar - Modern style
        egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::default()
                    .fill(visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                    visuals.widgets.inactive.fg_stroke.color;
                ui.style_mut().visuals.widgets.hovered.fg_stroke.color =
                    visuals.widgets.hovered.fg_stroke.color;

                egui::menu::bar(ui, |ui| {
                    // ---- File ------------------------------------------------
                    ui.menu_button("📄 File", |ui| {
                        if ui.button(format!("📁 New Folder {}", Self::hint("Ctrl+N"))).clicked() {
                            self.show_new_folder_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button(format!("📄 New File {}", Self::hint("Ctrl+Shift+N"))).clicked() {
                            self.show_new_file_dialog = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("📄 Open {}", Self::hint("F3"))).clicked() {
                            let _ = self.open_file_with_default_app();
                            ui.close_menu();
                        }
                        if ui.button(format!("🖥 Open Terminal Here {}", Self::hint("Ctrl+T"))).clicked() {
                            let _ = self.open_terminal_here();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("📋 Copy {}", Self::hint("F5"))).clicked() {
                            self.copy_to_clipboard();
                            ui.close_menu();
                        }
                        if ui.button(format!("✂ Cut {}", Self::hint("F6"))).clicked() {
                            self.cut_to_clipboard();
                            ui.close_menu();
                        }
                        if ui.button(format!("📎 Paste {}", Self::hint("F7"))).clicked() {
                            let _ = self.paste_from_clipboard();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("✏ Rename {}", Self::hint("Shift+F2"))).clicked() {
                            if let Some(item) = self.get_active_pane().get_selected_item() {
                                if item.name != ".." {
                                    self.rename_new_name = item.name.clone();
                                    self.show_rename_dialog = true;
                                }
                            }
                            ui.close_menu();
                        }
                        if ui.button(format!("🗑 Delete {}", Self::hint("F8"))).clicked() {
                            if let Some(item) = self.get_active_pane().get_selected_item() {
                                self.item_to_delete = Some(item.name.clone());
                                self.show_delete_confirm = true;
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("⏻ Exit {}", Self::hint("Alt+F4"))).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    // ---- View -------------------------------------------------
                    ui.menu_button("👁 View", |ui| {
                        let mut hidden = self.show_hidden_files;
                        if ui
                            .checkbox(&mut hidden, format!("Show Hidden Files {}", Self::hint("Ctrl+H")))
                            .clicked()
                        {
                            self.show_hidden_files = hidden;
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                            ui.close_menu();
                        }
                        let mut preview = self.show_preview_panel;
                        if ui
                            .checkbox(&mut preview, format!("Preview Panel {}", Self::hint("Space")))
                            .clicked()
                        {
                            self.show_preview_panel = preview;
                            ui.close_menu();
                        }
                        let mut sidebar = self.show_sidebar;
                        if ui
                            .checkbox(&mut sidebar, format!("Sidebar {}", Self::hint("Ctrl+B")))
                            .clicked()
                        {
                            self.show_sidebar = sidebar;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("🎨 Themes...")).clicked() {
                            self.show_theme_selector = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("🔄 Refresh {}", Self::hint("F2"))).clicked() {
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                            self.status_message = "Refreshed".to_string();
                            ui.close_menu();
                        }
                    });

                    // ---- Bookmarks -------------------------------------------
                    ui.menu_button("⭐ Bookmarks", |ui| {
                        if ui.button("➕ Add Current Folder").clicked() {
                            self.show_add_bookmark_dialog = true;
                            self.new_bookmark_name = self
                                .get_active_pane()
                                .current_path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("Bookmark")
                                .to_string();
                            ui.close_menu();
                        }
                        ui.separator();
                        let bookmarks = self.bookmark_manager.get_bookmarks().to_vec();
                        if bookmarks.is_empty() {
                            ui.weak("No bookmarks yet");
                        } else {
                            for bookmark in &bookmarks {
                                if ui
                                    .button(format!("{} {}", bookmark.icon, bookmark.name))
                                    .clicked()
                                {
                                    let path = bookmark.path.clone();
                                    let _ = self.get_active_pane_mut().navigate_to(path.clone());
                                    self.status_message =
                                        format!("Navigated to {}", path.display());
                                    ui.close_menu();
                                }
                            }
                        }
                    });

                    // ---- Tools -------------------------------------------------
                    ui.menu_button("🔧 Tools", |ui| {
                        if ui.button(format!("🔍 Advanced Search {}", Self::hint("Ctrl+Shift+F"))).clicked() {
                            self.search_criteria.search_path =
                                self.get_active_pane().current_path.clone();
                            self.show_search_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button(format!("⚖ Compare Files {}", Self::hint("Ctrl+D"))).clicked() {
                            let _ = self.compare_selected_files();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("💾 Mount Points").clicked() {
                            self.show_mounts_dialog = true;
                            ui.close_menu();
                        }
                    });

                    // ---- Help ---------------------------------------------------
                    ui.menu_button("❓ Help", |ui| {
                        if ui.button("🔍 Check for Updates...").clicked() {
                            if let super::update::UpdateStatus::Done { outcome } =
                                &self.update_status
                            {
                                match outcome {
                                    Ok(Some(info)) => {
                                        let dl = info
                                            .appimage_url
                                            .as_deref()
                                            .unwrap_or(info.release_url.as_str());
                                        self.status_message =
                                            format!("Update {} available — {}", info.tag, dl);
                                    }
                                    Ok(None) => {
                                        self.status_message = format!(
                                            "Up to date (v{})",
                                            env!("CARGO_PKG_VERSION")
                                        );
                                    }
                                    Err(e) => {
                                        self.status_message =
                                            format!("Update check failed: {}", e);
                                    }
                                }
                            }
                            self.start_update_check(false);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(format!("⌨ Keyboard Shortcuts {}", Self::hint("F1"))).clicked() {
                            self.show_help_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("⚡ About").clicked() {
                            self.show_about_dialog = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("💝 Donate via PayPal").clicked() {
                            if let Err(e) = open::that("https://www.paypal.com/paypalme/AchmadFachrie") {
                                self.status_message = format!("Failed to open PayPal: {}", e);
                            } else {
                                self.status_message = "Opening PayPal donation page...".to_string();
                            }
                            ui.close_menu();
                        }
                    });
                });
            });
    }

    /// Dim regular weight hint attached to a menu item label.
    fn hint(shortcut: &str) -> String {
        format!("  ({})", shortcut)
    }
}
