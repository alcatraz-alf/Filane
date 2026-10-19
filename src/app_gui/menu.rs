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
                    ui.menu_button("Files", |ui| {
                        if ui.button("Refresh (F2)").clicked() {
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                            self.status_message = "Refreshed".to_string();
                            ui.close_menu();
                        }

                        ui.separator();

                        // Toggle hidden files
                        let hidden_text = if self.show_hidden_files {
                            "☑ Show Hidden Files (Ctrl+H)"
                        } else {
                            "☐ Show Hidden Files (Ctrl+H)"
                        };
                        if ui.button(hidden_text).clicked() {
                            self.show_hidden_files = !self.show_hidden_files;
                            let _ = self.left_pane.refresh();
                            let _ = self.right_pane.refresh();
                            self.status_message = if self.show_hidden_files {
                                "Showing hidden files".to_string()
                            } else {
                                "Hiding hidden files".to_string()
                            };
                            ui.close_menu();
                        }

                        // Toggle preview panel
                        let preview_text = if self.show_preview_panel {
                            "☑ Show Preview Panel (Space)"
                        } else {
                            "☐ Show Preview Panel (Space)"
                        };
                        if ui.button(preview_text).clicked() {
                            self.show_preview_panel = !self.show_preview_panel;
                            ui.close_menu();
                        }

                        let sidebar_text = if self.show_sidebar {
                            "☑ Show Sidebar (Ctrl+B)"
                        } else {
                            "☐ Show Sidebar (Ctrl+B)"
                        };
                        if ui.button(sidebar_text).clicked() {
                            self.show_sidebar = !self.show_sidebar;
                            ui.close_menu();
                        }

                        ui.separator();

                        // Theme selector
                        if ui.button("🎨 Themes...").clicked() {
                            self.show_theme_selector = true;
                            ui.close_menu();
                        }

                        ui.separator();
                        if ui.button("Exit (Alt+F4)").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("Bookmarks", |ui| {
                        if ui.button("⭐ Add Current Folder").clicked() {
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
                            ui.label("No bookmarks yet");
                        } else {
                            for (_idx, bookmark) in bookmarks.iter().enumerate() {
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

                    ui.menu_button("Commands", |ui| {
                        if ui.button("💾 Mount Points").clicked() {
                            self.show_mounts_dialog = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📁 New Folder (Ctrl+N)").clicked() {
                            self.show_new_folder_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("✏ Rename (Shift+F2)").clicked() {
                            if let Some(item) = self.get_active_pane().get_selected_item() {
                                if item.name != ".." {
                                    self.rename_new_name = item.name.clone();
                                    self.show_rename_dialog = true;
                                }
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📄 Open File (F3)").clicked() {
                            let _ = self.open_file_with_default_app();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("🔍 Advanced Search (Ctrl+Shift+F)").clicked() {
                            self.search_criteria.search_path =
                                self.get_active_pane().current_path.clone();
                            self.show_search_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("⚖ Compare Files (Ctrl+D)").clicked() {
                            let _ = self.compare_selected_files();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("📋 Copy (F5)").clicked() {
                            self.copy_to_clipboard();
                            ui.close_menu();
                        }
                        if ui.button("✂ Cut/Move (F6)").clicked() {
                            self.cut_to_clipboard();
                            ui.close_menu();
                        }
                        if ui.button("📎 Paste (F7)").clicked() {
                            let _ = self.paste_from_clipboard();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("🗑 Delete (F8)").clicked() {
                            if let Some(item) = self.get_active_pane().get_selected_item() {
                                self.item_to_delete = Some(item.name.clone());
                                self.show_delete_confirm = true;
                            }
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("⚡ About").clicked() {
                            self.show_about_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("💝 Donate").clicked() {
                        if let Err(e) = open::that("https://www.paypal.com/paypalme/AchmadFachrie") {
                        self.status_message = format!("Failed to open PayPal: {}", e);
                        } else {
                        self.status_message = "Opening PayPal donation page...".to_string();
                        }
                            ui.close_menu();
                        }
                        ui.separator();
                        ui.label("Keyboard Shortcuts:");
                        ui.label("• Shift+F2: Rename");
                        ui.label("• F2: Refresh");
                        ui.label("• F3: Open file");
                        ui.label("• F5: Copy");
                        ui.label("• F6: Cut/Move");
                        ui.label("• F7: Paste");
                        ui.label("• F8: Delete");
                        ui.label("• Space: Toggle preview");
                        ui.label("• Ctrl+H: Toggle hidden");
                        ui.label("• Ctrl+F: Filter");
                        ui.label("• Tab: Switch pane");
                        ui.label("• Alt+←/→: Back/Forward");
                    });
                });
            });

    }
}
