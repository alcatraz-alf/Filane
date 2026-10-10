use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            let shift_pressed = i.modifiers.shift;
            
            if i.key_pressed(egui::Key::ArrowUp) {
                let pane = self.get_active_pane_mut();
                if shift_pressed {
                    pane.move_up_with_selection();
                } else {
                    pane.clear_selection();
                    pane.move_up();
                }
                if self.show_preview_panel {
                    self.update_previews();
                }
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                let pane = self.get_active_pane_mut();
                if shift_pressed {
                    pane.move_down_with_selection();
                } else {
                    pane.clear_selection();
                    pane.move_down();
                }
                if self.show_preview_panel {
                    self.update_previews();
                }
            }
            if i.key_pressed(egui::Key::Enter) {
                if let Err(e) = self.get_active_pane_mut().enter_directory() {
                    self.status_message = format!("Error: {}", e);
                }
            }
            
            if i.key_pressed(egui::Key::Escape) {
                self.get_active_pane_mut().clear_selection();
            }

            // Tab to switch panes
            if i.key_pressed(egui::Key::Tab) {
                self.active_pane = if self.active_pane == 0 { 1 } else { 0 };
            }

            // Alt+Left - Navigate back
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowLeft) {
                if let Err(e) = self.get_active_pane_mut().navigate_back() {
                    self.status_message = format!("Error: {}", e);
                } else {
                    self.status_message = "Navigated back".to_string();
                }
            }

            // Alt+Right - Navigate forward
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowRight) {
                if let Err(e) = self.get_active_pane_mut().navigate_forward() {
                    self.status_message = format!("Error: {}", e);
                } else {
                    self.status_message = "Navigated forward".to_string();
                }
            }

            // F2 - Refresh (Total Commander style)
            if i.key_pressed(egui::Key::F2) {
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = "Refreshed both panes".to_string();
            }

            // F3 - Open file with default app (Total Commander style)
            if i.key_pressed(egui::Key::F3) {
                let _ = self.open_file_with_default_app();
            }

            // F5 - Copy (Total Commander style)
            if i.key_pressed(egui::Key::F5) {
                self.copy_to_clipboard();
            }

            // F6 - Cut/Move (Total Commander style)
            if i.key_pressed(egui::Key::F6) {
                self.cut_to_clipboard();
            }

            // F7 - Paste
            if i.key_pressed(egui::Key::F7) {
                let _ = self.paste_from_clipboard();
            }

            // F8 - Delete (Total Commander style)
            if i.key_pressed(egui::Key::F8) {
                let selected_items = self.get_active_pane().get_selected_items();
                if !selected_items.is_empty() {
                    let count = selected_items.len();
                    let names: Vec<String> = selected_items.iter().map(|item| item.name.clone()).collect();
                    self.item_to_delete = Some(if count == 1 {
                        names[0].clone()
                    } else {
                        format!("{} items", count)
                    });
                    self.show_delete_confirm = true;
                }
            }

            // Ctrl+H - Toggle hidden files
            if i.modifiers.ctrl && i.key_pressed(egui::Key::H) {
                self.show_hidden_files = !self.show_hidden_files;
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = if self.show_hidden_files {
                    "Showing hidden files".to_string()
                } else {
                    "Hiding hidden files".to_string()
                };
            }

            // Ctrl+B - Toggle sidebar
            if i.modifiers.ctrl && i.key_pressed(egui::Key::B) {
                self.show_sidebar = !self.show_sidebar;
                self.status_message = if self.show_sidebar {
                    "Sidebar shown".to_string()
                } else {
                    "Sidebar hidden".to_string()
                };
            }

            // Ctrl+F - Toggle filter mode
            if i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::F) {
                self.filter_mode = !self.filter_mode;
                if !self.filter_mode {
                    // Clear filter when exiting
                    self.get_active_pane_mut().filter_text.clear();
                }
                self.status_message = if self.filter_mode {
                    "Filter mode: Type to filter files".to_string()
                } else {
                    "Filter cleared".to_string()
                };
            }

            // Ctrl+Shift+F - Advanced Search
            if i.modifiers.ctrl && i.modifiers.shift && i.key_pressed(egui::Key::F) {
                self.search_criteria.search_path = self.get_active_pane().current_path.clone();
                self.show_search_dialog = true;
            }

            // ESC - Exit filter mode or close dialogs
            if i.key_pressed(egui::Key::Escape) {
                let op_running = self
                    .op_runner
                    .as_ref()
                    .map(|r| !r.is_finished())
                    .unwrap_or(false);
                if op_running {
                    // Cancel the running file operation
                    if let Some(runner) = &self.op_runner {
                        runner.cancel();
                        self.status_message = "Cancelling operation...".to_string();
                    }
                } else if self.show_progress_dialog {
                    self.show_progress_dialog = false;
                    self.op_result_message = None;
                } else if self.filter_mode {
                    self.filter_mode = false;
                    self.get_active_pane_mut().filter_text.clear();
                    self.status_message = "Filter cleared".to_string();
                } else if self.show_new_folder_dialog {
                    self.show_new_folder_dialog = false;
                    self.new_folder_name.clear();
                } else if self.show_rename_dialog {
                    self.show_rename_dialog = false;
                    self.rename_new_name.clear();
                } else if self.show_about_dialog {
                    self.show_about_dialog = false;
                } else if self.show_properties_dialog {
                    self.show_properties_dialog = false;
                    self.properties_item = None;
                } else if self.show_delete_confirm {
                    self.show_delete_confirm = false;
                    self.item_to_delete = None;
                } else if self.show_search_dialog {
                    self.show_search_dialog = false;
                } else if self.show_compare_dialog {
                    self.show_compare_dialog = false;
                    self.comparison_result = None;
                }
            }

            // Ctrl+D - Compare files
            if i.modifiers.ctrl && i.key_pressed(egui::Key::D) {
                let _ = self.compare_selected_files();
            }

            // Ctrl+N - New folder
            if i.modifiers.ctrl && i.key_pressed(egui::Key::N) {
                self.show_new_folder_dialog = true;
            }

            // Space - Toggle preview panel and update preview
            if i.key_pressed(egui::Key::Space) && !self.filter_mode {
                self.show_preview_panel = !self.show_preview_panel;
                if self.show_preview_panel {
                    self.update_previews();
                }
            }

            // Legacy shortcuts
            if i.modifiers.ctrl && i.key_pressed(egui::Key::C) {
                self.copy_to_clipboard();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::X) {
                self.cut_to_clipboard();
            }
            if i.modifiers.ctrl && i.key_pressed(egui::Key::V) {
                let _ = self.paste_from_clipboard();
            }
            if i.key_pressed(egui::Key::Delete) {
                if let Some(item) = self.get_active_pane().get_selected_item() {
                    self.item_to_delete = Some(item.name.clone());
                    self.show_delete_confirm = true;
                }
            }
        });
    }
}
