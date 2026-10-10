use super::FileManagerApp;
use egui;
use std::path::PathBuf;

impl FileManagerApp {
    pub(crate) fn render_function_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(14.0, 10.0);
            ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                egui::Color32::from_rgb(232, 234, 237);
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                egui::Color32::from_rgb(66, 70, 77);

            // Modern function buttons with tooltips
            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("💾 F2").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Refresh both panes")
                .clicked()
            {
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = "Refreshed both panes".to_string();
            }

            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("📋 F5").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Copy selected item")
                .clicked()
            {
                self.copy_to_clipboard();
            }

            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("✂ F6").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Cut/Move selected item")
                .clicked()
            {
                self.cut_to_clipboard();
            }

            let paste_enabled = self.clipboard.is_some();
            let paste_tooltip = if paste_enabled {
                "Paste from clipboard"
            } else {
                "Clipboard is empty"
            };

            let paste_btn = ui
                .add_enabled_ui(paste_enabled, |ui| {
                    ui.add_sized(
                        [95.0, 38.0],
                        egui::Button::new(egui::RichText::new("📎 F7").size(13.0))
                            .fill(egui::Color32::from_rgb(55, 58, 64))
                            .rounding(6.0),
                    )
                })
                .inner
                .on_hover_text(paste_tooltip);

            if paste_btn.clicked() && paste_enabled {
                let _ = self.paste_from_clipboard();
            }

            if ui
                .add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new("🗑 F8").size(13.0))
                        .fill(egui::Color32::from_rgb(55, 58, 64))
                        .rounding(6.0),
                )
                .on_hover_text("Delete selected item")
                .clicked()
            {
                if let Some(item) = self.get_active_pane().get_selected_item() {
                    self.item_to_delete = Some(item.name.clone());
                    self.show_delete_confirm = true;
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_sized(
                        [95.0, 38.0],
                        egui::Button::new(egui::RichText::new("✗ Exit").size(13.0))
                            .fill(egui::Color32::from_rgb(184, 51, 63))
                            .rounding(6.0),
                    )
                    .clicked()
                {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }

    pub(crate) fn render_transfer_buttons(&mut self, ui: &mut egui::Ui) {
        // Add vertical spacing to center the buttons
        ui.add_space(ui.available_height() / 2.0 - 120.0);

        ui.vertical_centered(|ui| {
            ui.style_mut().spacing.item_spacing.y = 12.0;

            // Get selected item info
            let has_selection = self.get_active_pane().get_selected_item().is_some();
            let selected_name = self
                .get_active_pane()
                .get_selected_item()
                .map(|item| item.name.clone())
                .unwrap_or_default();
            let is_parent_dir = selected_name == "..";
            let can_transfer = has_selection && !is_parent_dir;

            // Copy Left to Right button
            let copy_left_to_right_enabled = self.active_pane == 0 && can_transfer;
            let copy_l2r_tooltip = if copy_left_to_right_enabled {
                "Copy selected item\nfrom Left to Right"
            } else {
                "Select an item in left pane"
            };

            let copy_l2r_clicked = ui
                .add_enabled_ui(copy_left_to_right_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("➡\n📋").size(14.0))
                            .fill(egui::Color32::from_rgb(66, 133, 244))
                            .rounding(8.0),
                    )
                    .on_hover_text(copy_l2r_tooltip)
                })
                .inner
                .clicked();

            if copy_l2r_clicked && copy_left_to_right_enabled {
                self.transfer_selected(0, crate::ops::Operation::Copy { sources: Vec::new(), dest: PathBuf::new() });
            }

            // Copy Right to Left button
            let copy_right_to_left_enabled = self.active_pane == 1 && can_transfer;
            let copy_r2l_tooltip = if copy_right_to_left_enabled {
                "Copy selected item\nfrom Right to Left"
            } else {
                "Select an item in right pane"
            };

            let copy_r2l_clicked = ui
                .add_enabled_ui(copy_right_to_left_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("⬅\n📋").size(14.0))
                            .fill(egui::Color32::from_rgb(66, 133, 244))
                            .rounding(8.0),
                    )
                    .on_hover_text(copy_r2l_tooltip)
                })
                .inner
                .clicked();

            if copy_r2l_clicked && copy_right_to_left_enabled {
                self.transfer_selected(1, crate::ops::Operation::Copy { sources: Vec::new(), dest: PathBuf::new() });
            }

            ui.add_space(20.0);

            // Move Left to Right button
            let move_left_to_right_enabled = self.active_pane == 0 && can_transfer;
            let move_l2r_tooltip = if move_left_to_right_enabled {
                "Move selected item\nfrom Left to Right"
            } else {
                "Select an item in left pane"
            };

            let move_l2r_clicked = ui
                .add_enabled_ui(move_left_to_right_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("➡\n✂").size(14.0))
                            .fill(egui::Color32::from_rgb(255, 152, 0))
                            .rounding(8.0),
                    )
                    .on_hover_text(move_l2r_tooltip)
                })
                .inner
                .clicked();

            if move_l2r_clicked && move_left_to_right_enabled {
                self.transfer_selected(0, crate::ops::Operation::Move { sources: Vec::new(), dest: PathBuf::new() });
            }

            // Move Right to Left button
            let move_right_to_left_enabled = self.active_pane == 1 && can_transfer;
            let move_r2l_tooltip = if move_right_to_left_enabled {
                "Move selected item\nfrom Right to Left"
            } else {
                "Select an item in right pane"
            };

            let move_r2l_clicked = ui
                .add_enabled_ui(move_right_to_left_enabled, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(egui::RichText::new("⬅\n✂").size(14.0))
                            .fill(egui::Color32::from_rgb(255, 152, 0))
                            .rounding(8.0),
                    )
                    .on_hover_text(move_r2l_tooltip)
                })
                .inner
                .clicked();

            if move_r2l_clicked && move_right_to_left_enabled {
                self.transfer_selected(1, crate::ops::Operation::Move { sources: Vec::new(), dest: PathBuf::new() });
            }
        });
    }

    pub(crate) fn render_pane(&mut self, ui: &mut egui::Ui, pane_index: usize, ctx: &egui::Context, max_width: f32) {
        let is_active = self.active_pane == pane_index;
        let visuals = ctx.style().visuals.clone();
        
        // Force the UI to respect the allocated width
        ui.set_max_width(max_width);
        ui.set_width(max_width);

        // Collect all data we need before borrowing
        let (current_path, items, selected_index, sort_by, sort_order) = {
            let pane = if pane_index == 0 {
                &self.left_pane
            } else {
                &self.right_pane
            };
            (
                pane.current_path.clone(),
                pane.items.clone(),
                pane.selected_index,
                pane.sort_by,
                pane.sort_order,
            )
        };

        let frame_style = if is_active {
            egui::Frame::default()
                .fill(visuals.panel_fill)
                .stroke(egui::Stroke::new(
                    2.0,
                    visuals.widgets.active.bg_fill,
                ))
                .rounding(8.0)
                .inner_margin(0.0)
                .shadow(egui::epaint::Shadow {
                    offset: egui::vec2(0.0, 2.0),
                    blur: 8.0,
                    spread: 0.0,
                    color: egui::Color32::from_black_alpha(60),
                })
        } else {
            egui::Frame::default()
                .fill(visuals.panel_fill)
                .stroke(egui::Stroke::new(1.0, visuals.widgets.noninteractive.bg_fill))
                .rounding(8.0)
                .inner_margin(0.0)
        };

        // Use full available space but constrain width
        let available_width = ui.available_width();
        let available_height = ui.available_height();
        
        frame_style.show(ui, |ui| {
            // IMPORTANT: Set max width to prevent pane from expanding beyond allocated space
            ui.set_max_width(available_width);
            ui.set_width(available_width);
            
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0; // No vertical spacing!
                // Force this to take full height but respect width constraint
                ui.set_max_width(available_width);
                ui.set_height(available_height);

                // Header with breadcrumb navigation - Modern, compact
                let header_resp = egui::Frame::default()
                    .fill(visuals.widgets.noninteractive.bg_fill)
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(2.0, 0.0);
                        ui.set_height(20.0);
                        ui.horizontal(|ui| {
                            // Back button
                            let can_go_back = if pane_index == 0 {
                                self.left_pane.can_go_back()
                            } else {
                                self.right_pane.can_go_back()
                            };

                            let back_btn = ui
                                .add_enabled(
                                    can_go_back,
                                    egui::Button::new(egui::RichText::new("◀").size(16.0))
                                        .fill(visuals.widgets.inactive.bg_fill)
                                        .rounding(4.0)
                                        .min_size(egui::vec2(28.0, 24.0)),
                                )
                                .on_hover_text("Back (Alt+Left)");

                            if back_btn.clicked() && can_go_back {
                                let pane = if pane_index == 0 {
                                    &mut self.left_pane
                                } else {
                                    &mut self.right_pane
                                };
                                let _ = pane.navigate_back();
                            }

                            // Forward button
                            let can_go_forward = if pane_index == 0 {
                                self.left_pane.can_go_forward()
                            } else {
                                self.right_pane.can_go_forward()
                            };

                            let forward_btn = ui
                                .add_enabled(
                                    can_go_forward,
                                    egui::Button::new(egui::RichText::new("▶").size(16.0))
                                        .fill(visuals.widgets.inactive.bg_fill)
                                        .rounding(4.0)
                                        .min_size(egui::vec2(28.0, 24.0)),
                                )
                                .on_hover_text("Forward (Alt+Right)");

                            if forward_btn.clicked() && can_go_forward {
                                let pane = if pane_index == 0 {
                                    &mut self.left_pane
                                } else {
                                    &mut self.right_pane
                                };
                                let _ = pane.navigate_forward();
                            }

                            ui.add_space(6.0);
                            ui.label(egui::RichText::new("💾").size(14.0));
                            ui.add_space(4.0);

                            // Breadcrumb navigation - single line, truncate if needed
                            let components: Vec<_> = current_path.components().collect();
                            let available_width = ui.available_width();
                            let mut used_width = 0.0;

                            for (idx, component) in components.iter().enumerate() {
                                let comp_str = component.as_os_str().to_string_lossy();
                                let display_name = if comp_str == "/" {
                                    "/".to_string()
                                } else {
                                    comp_str.to_string()
                                };

                                let estimated_width = display_name.len() as f32 * 6.0 + 10.0;

                                // Stop if we're running out of space
                                if used_width + estimated_width > available_width - 30.0
                                    && idx < components.len() - 1
                                {
                                    ui.label(
                                        egui::RichText::new("...")
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(154, 160, 166)),
                                    );
                                    break;
                                }

                                if ui
                                    .add_sized(
                                        [estimated_width.min(150.0), 18.0],
                                        egui::Button::new(
                                            egui::RichText::new(&display_name)
                                                .size(15.0)
                                                .color(egui::Color32::from_rgb(138, 180, 248)),
                                        )
                                        .frame(false),
                                    )
                                    .clicked()
                                {
                                    // Navigate to clicked path segment
                                    let mut target_path = std::path::PathBuf::new();
                                    for i in 0..=idx {
                                        target_path.push(components[i]);
                                    }
                                    let pane = if pane_index == 0 {
                                        &mut self.left_pane
                                    } else {
                                        &mut self.right_pane
                                    };
                                    let _ = pane.navigate_to(target_path);
                                }

                                used_width += estimated_width;

                                if idx < components.len() - 1 {
                                    ui.label(
                                        egui::RichText::new("›")
                                            .size(11.0)
                                            .color(egui::Color32::from_rgb(154, 160, 166)),
                                    );
                                    used_width += 10.0;
                                }
                            }
                        });
                    });
                // Solid bottom divider for header
                let hr = header_resp.response.rect;
                let y = hr.bottom() - 0.5;
                ui.painter().line_segment(
                    [egui::pos2(hr.left(), y), egui::pos2(hr.right(), y)],
                    egui::Stroke {
                        width: 1.0,
                        color: visuals.widgets.noninteractive.bg_fill,
                    },
                );

                // Filter input (if filter mode is active and this is the active pane)
                if self.filter_mode && self.active_pane == pane_index {
                    egui::Frame::default()
                        .fill(visuals.widgets.inactive.bg_fill)
                        .inner_margin(egui::Margin::symmetric(10.0, 4.0))
                        .show(ui, |ui| {
                            ui.set_height(24.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new("🔍")
                                        .size(13.0)
                                        .color(visuals.widgets.active.bg_fill),
                                );
                                ui.add_space(6.0);
                                let filter_response = ui.add(
                                    egui::TextEdit::singleline(if pane_index == 0 {
                                        &mut self.left_pane.filter_text
                                    } else {
                                        &mut self.right_pane.filter_text
                                    })
                                    .hint_text("Type to filter...")
                                    .desired_width(ui.available_width()),
                                );
                                filter_response.request_focus();
                            });
                        });
                }

                // Table for file list - wrapped in a constrained area
                use egui_extras::{TableBuilder, Column};

                let text_height = 28.0; // Smaller row height for compact view
                let table_width = ui.available_width();
                
                // Capture shift state before entering closures
                let shift_pressed = ctx.input(|i| i.modifiers.shift);
                
                // Use allocate_ui to constrain the table width
                ui.allocate_ui(egui::Vec2::new(table_width, ui.available_height()), |ui| {
                    ui.set_max_width(table_width);
                    ui.set_clip_rect(ui.max_rect());
                    
                    // Calculate column widths to fit within table_width
                    let date_width = 140.0;
                    let size_width = 90.0;
                    let name_width = (table_width - date_width - size_width - 20.0).max(100.0);
                    
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(false) // Disable resize to prevent overflow
                        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                        .column(Column::exact(name_width).clip(true)) // Name - exact width
                        .column(Column::exact(date_width)) // Date
                        .column(Column::exact(size_width)) // Size
                        .sense(egui::Sense::click()) // Row selection
                    .header(26.0, |mut header| {
                        let sort_icon = |current_sort: crate::pane::SortBy| -> &str {
                            if sort_by == current_sort {
                                match sort_order {
                                    crate::pane::SortOrder::Ascending => " ▲",
                                    crate::pane::SortOrder::Descending => " ▼",
                                }
                            } else {
                                ""
                            }
                        };
                        
                        // Name Header
                        header.col(|ui| {
                            let text = format!("Name{}", sort_icon(crate::pane::SortBy::Name));
                            if ui.add(egui::Button::new(egui::RichText::new(text).strong()).frame(false)).clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                pane.toggle_sort(crate::pane::SortBy::Name);
                            }
                        });
                        
                        // Date Header
                        header.col(|ui| {
                             let text = format!("Modified{}", sort_icon(crate::pane::SortBy::Date));
                             if ui.add(egui::Button::new(egui::RichText::new(text).strong()).frame(false)).clicked() {
                                 let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                 pane.toggle_sort(crate::pane::SortBy::Date);
                             }
                        });

                        // Size Header
                        header.col(|ui| {
                             let text = format!("Size{}", sort_icon(crate::pane::SortBy::Size));
                             if ui.add(egui::Button::new(egui::RichText::new(text).strong()).frame(false)).clicked() {
                                 let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                 pane.toggle_sort(crate::pane::SortBy::Size);
                             }
                        });
                    })
                    .body(|body| {
                        // Get filter text for active pane
                        let filter_text = if pane_index == 0 {
                            self.left_pane.filter_text.to_lowercase()
                        } else {
                            self.right_pane.filter_text.to_lowercase()
                        };

                        // Filter items based on show_hidden_files setting AND filter text AND extension filters
                        let filtered_items: Vec<(usize, &crate::filesystem::FileItem)> = items
                            .iter()
                            .enumerate()
                            .filter(|(_, item)| {
                                if item.name == ".." { return true; }
                                if !filter_text.is_empty() && !crate::fuzzy::filter_match(&item.name, &filter_text) { return false; }
                                if !self.should_show_file(item) { return false; }
                                if self.show_hidden_files { true } else { !item.name.starts_with('.') }
                            })
                            .collect();

                        body.rows(text_height, filtered_items.len(), |mut row| {
                            let item_idx = row.index();
                            let (i, item) = filtered_items[item_idx];

                            // Better icons based on file type
                            let (icon, icon_color) = if item.is_dir {
                                if item.name == ".." {
                                    ("⬆", egui::Color32::from_rgb(189, 147, 249))
                                } else {
                                    ("📁", egui::Color32::from_rgb(255, 184, 108))
                                }
                            } else {
                                let ext = item.path.extension().and_then(|e| e.to_str()).unwrap_or("");
                                match ext {
                                    "rs" => ("🦀", egui::Color32::from_rgb(222, 165, 132)),
                                    "py" => ("🐍", egui::Color32::from_rgb(87, 171, 224)),
                                    "js" | "ts" | "jsx" | "tsx" | "html" | "css" => ("🌐", egui::Color32::from_rgb(240, 219, 79)),
                                    "json" | "yaml" | "toml" | "xml" => ("⚙", egui::Color32::from_rgb(139, 233, 253)),
                                    "md" | "txt" => ("📝", egui::Color32::from_rgb(189, 147, 249)),
                                    "zip" | "tar" | "gz" | "rar" => ("📦", egui::Color32::from_rgb(255, 121, 198)),
                                    "png" | "jpg" | "jpeg" | "gif" => ("🖼", egui::Color32::from_rgb(80, 250, 123)),
                                    "mp3" | "wav" | "ogg" => ("🎵", egui::Color32::from_rgb(139, 233, 253)),
                                    "mp4" | "avi" | "mov" => ("🎬", egui::Color32::from_rgb(255, 121, 198)),
                                    "exe" | "sh" | "bat" => ("⚡", egui::Color32::from_rgb(80, 250, 123)),
                                    _ => ("📄", egui::Color32::from_rgb(189, 193, 198)),
                                }
                            };

                            let is_selected = i == selected_index;
                            let pane = if pane_index == 0 { &self.left_pane } else { &self.right_pane };
                            let is_multi_selected = pane.is_item_selected(i);
                            
                            row.set_selected(is_selected || is_multi_selected); 

                            // Name Column
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(icon).size(16.0).color(icon_color));
                                    
                                    // Git status
                                    if let Some(ref status) = item.git_status {
                                        ui.label(egui::RichText::new(status.icon()).size(12.0).color(status.color()));
                                    }

                                    let text_color = if is_selected { egui::Color32::WHITE } else { egui::Color32::LIGHT_GRAY };
                                    ui.label(egui::RichText::new(&item.name).size(13.0).color(text_color));
                                });
                            });

                            // Date Column
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(egui::RichText::new(crate::filesystem::format_date(item.modified)).size(11.0).monospace());
                                });
                            });

                            // Size Column
                            row.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let size_str = if item.is_dir { "<DIR>".to_string() } else { crate::filesystem::format_size(item.size) };
                                    ui.label(egui::RichText::new(size_str).size(11.0).monospace());
                                });
                            });

                            // Row Interaction
                            let response = row.response();
                            if response.clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                
                                if shift_pressed {
                                    if pane.selection_anchor.is_none() {
                                        pane.selection_anchor = Some(pane.selected_index);
                                    }
                                    pane.selected_index = i;
                                    let anchor = pane.selection_anchor.unwrap();
                                    let start = anchor.min(i);
                                    let end = anchor.max(i);
                                    pane.selected_items = (start..=end).collect();
                                } else {
                                    pane.clear_selection();
                                    pane.selected_index = i;
                                }
                                
                                self.active_pane = pane_index;
                                if self.show_preview_panel { self.update_previews(); }
                            }
                            if response.double_clicked() {
                                if item.is_dir {
                                    let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                    if let Err(e) = pane.enter_directory() {
                                        self.status_message = format!("Error: {}", e);
                                    }
                                } else {
                                    let _ = self.open_file_with_default_app();
                                }
                            }
                            if response.secondary_clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                pane.selected_index = i;
                                self.active_pane = pane_index;
                                if let Some(pos) = response.interact_pointer_pos() {
                                    self.context_menu_pos = pos;
                                    self.show_context_menu = true;
                                    self.context_menu_item_index = i;
                                }
                            }
                        });
                    });
                }); // Close allocate_ui
            });
        });

        // Handle click to activate pane and ensure hover detection
        let pane_rect = ui.min_rect();
        if ui.rect_contains_pointer(pane_rect) {
            // Activate pane on click
            if ctx.input(|i| i.pointer.primary_clicked()) {
                self.active_pane = pane_index;
            }

            // Ensure this pane captures hover for scroll
            ui.interact(
                pane_rect,
                egui::Id::new(format!("pane_interact_{}", pane_index)),
                egui::Sense::hover(),
            );
        }
    }
}
