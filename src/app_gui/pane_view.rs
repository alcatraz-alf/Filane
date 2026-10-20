use super::FileManagerApp;
use egui;
use std::path::PathBuf;

impl FileManagerApp {
    pub(crate) fn render_function_bar(&mut self, ui: &mut egui::Ui) {
        // Theme-aware button styling: never hardcode the dark palette so
        // Light/Solarized themes don't get broken dark bars.
        let visuals = ui.ctx().style().visuals.clone();
        let btn_fill = visuals.widgets.inactive.bg_fill;
        let btn_hover = visuals.widgets.hovered.bg_fill;

        ui.horizontal(|ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(14.0, 10.0);
            ui.style_mut().visuals.widgets.inactive.fg_stroke.color =
                visuals.widgets.inactive.fg_stroke.color;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = btn_hover;

            let func_button = |ui: &mut egui::Ui, label: &str| {
                ui.add_sized(
                    [95.0, 38.0],
                    egui::Button::new(egui::RichText::new(label).size(13.0))
                        .fill(btn_fill)
                        .rounding(6.0),
                )
            };

            // Modern function buttons with tooltips
            if func_button(ui, "💾 F2")
                .on_hover_text("Refresh both panes")
                .clicked()
            {
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
                self.status_message = "Refreshed both panes".to_string();
            }

            if func_button(ui, "📋 F5")
                .on_hover_text("Copy selected item")
                .clicked()
            {
                self.copy_to_clipboard();
            }

            if func_button(ui, "✂ F6")
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
                .add_enabled_ui(paste_enabled, |ui| func_button(ui, "📎 F7"))
                .inner
                .on_hover_text(paste_tooltip);

            if paste_btn.clicked() && paste_enabled {
                let _ = self.paste_from_clipboard();
            }

            if func_button(ui, "🗑 F8")
                .on_hover_text("Move selected item to Trash")
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
                            .fill(self.palette().danger)
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
        let visuals = ui.ctx().style().visuals.clone();
        // Center the (now two) buttons vertically
        ui.add_space(ui.available_height() / 2.0 - 100.0);

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

            // Only the active pane's direction is shown: no permanently
            // disabled half of the panel. Arrow flips with the active pane.
            let (label_dir, hint_dir) = if self.active_pane == 0 {
                ("➡", "to the Right pane")
            } else {
                ("⬅", "to the Left pane")
            };
            let hint_from = if self.active_pane == 0 {
                "Select an item in the left pane"
            } else {
                "Select an item in the right pane"
            };

            // Copy button (theme accent)
            let copy_clicked = ui
                .add_enabled_ui(can_transfer, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(
                            egui::RichText::new(format!("{}\n📋", label_dir))
                                .size(14.0)
                                .color(visuals.widgets.active.fg_stroke.color),
                        )
                        .fill(visuals.widgets.active.bg_fill)
                        .rounding(8.0),
                    )
                })
                .inner
                .on_hover_text(if can_transfer {
                    format!("Copy selected item\n{}", hint_dir)
                } else {
                    hint_from.to_string()
                })
                .clicked();

            if copy_clicked && can_transfer {
                self.transfer_selected(
                    self.active_pane,
                    crate::ops::Operation::Copy { sources: Vec::new(), dest: PathBuf::new() },
                );
            }

            ui.add_space(12.0);

            // Move button (fixed warning orange, readable on all themes)
            let move_clicked = ui
                .add_enabled_ui(can_transfer, |ui| {
                    ui.add_sized(
                        [50.0, 50.0],
                        egui::Button::new(
                            egui::RichText::new(format!("{}\n✂", label_dir))
                                .size(14.0)
                                .color(egui::Color32::BLACK),
                        )
                        .fill(self.palette().warning)
                        .rounding(8.0),
                    )
                })
                .inner
                .on_hover_text(if can_transfer {
                    format!("Move selected item\n{}", hint_dir)
                } else {
                    hint_from.to_string()
                })
                .clicked();

            if move_clicked && can_transfer {
                self.transfer_selected(
                    self.active_pane,
                    crate::ops::Operation::Move { sources: Vec::new(), dest: PathBuf::new() },
                );
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
                    2.0_f32,
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
                .stroke(egui::Stroke::new(1.0_f32, visuals.widgets.noninteractive.bg_fill))
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
                                            .color(self.palette().text_dim),
                                    );
                                    break;
                                }

                                if ui
                                    .add_sized(
                                        [estimated_width.min(150.0), 18.0],
                                        egui::Button::new(
                                            egui::RichText::new(&display_name)
                                                .size(15.0)
                                                .color(self.palette().accent),
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
                                            .color(self.palette().text_dim),
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
                // Reserve room for the table's vertical scrollbar so the
                // right-aligned Size suffix (KB/MB) is never covered.
                let table_width = (ui.available_width() - 12.0).max(320.0);
                
                // Capture shift state before entering closures
                let shift_pressed = ctx.input(|i| i.modifiers.shift);
                let ctrl_pressed = ctx.input(|i| i.modifiers.ctrl);
                
                // Empty-state tracking: when the table renders no rows we
                // draw an explanatory message centered in the pane.
                let mut is_table_empty = false;
                let mut empty_reason = "";
                let mut table_rect = egui::Rect::NOTHING;

                // Use allocate_ui to constrain the table width
                ui.allocate_ui(egui::Vec2::new(table_width, ui.available_height()), |ui| {
                    ui.set_max_width(table_width);
                    ui.set_clip_rect(ui.max_rect());
                    table_rect = ui.max_rect();
                    
                    // Calculate column widths to fit within table_width
                    let date_width = 140.0;
                    let size_width = 110.0;
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
                            if ui.add(egui::Button::new(egui::RichText::new(text)
                                    .strong()
                                    .color(self.palette().text_dim))
                                .frame(false)).clicked() {
                                let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                pane.toggle_sort(crate::pane::SortBy::Name);
                            }
                        });
                        
                        // Date Header
                        header.col(|ui| {
                             let text = format!("Modified{}", sort_icon(crate::pane::SortBy::Date));
                             if ui.add(egui::Button::new(egui::RichText::new(text)
                                    .strong()
                                    .color(self.palette().text_dim))
                                .frame(false)).clicked() {
                                 let pane = if pane_index == 0 { &mut self.left_pane } else { &mut self.right_pane };
                                 pane.toggle_sort(crate::pane::SortBy::Date);
                             }
                        });

                        // Size Header
                        header.col(|ui| {
                             let text = format!("Size{}", sort_icon(crate::pane::SortBy::Size));
                             if ui.add(egui::Button::new(egui::RichText::new(text)
                                    .strong()
                                    .color(self.palette().text_dim))
                                .frame(false)).clicked() {
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

                        if filtered_items.is_empty() {
                            is_table_empty = true;
                            empty_reason = if !filter_text.is_empty() {
                                "No files match the filter"
                            } else {
                                "Folder is empty"
                            };
                        }

                        body.rows(text_height, filtered_items.len(), |mut row| {
                            let item_idx = row.index();
                            let (i, item) = filtered_items[item_idx];

                            // Better icons based on file type
                            let (icon, icon_color) = if item.is_dir {
                                if item.name == ".." {
                                    ("⬆", egui::Color32::from_rgb(189, 147, 249))
                                } else {
                                    ("📁", self.palette().warning)
                                }
                            } else {
                                let ext = item.path.extension().and_then(|e| e.to_str()).unwrap_or("");
                                match ext {
                                    "rs" => ("🦀", egui::Color32::from_rgb(222, 165, 132)),
                                    "py" => ("🐍", egui::Color32::from_rgb(87, 171, 224)),
                                    "js" | "ts" | "jsx" | "tsx" | "html" | "css" => ("🌐", egui::Color32::from_rgb(240, 219, 79)),
                                    "json" | "yaml" | "toml" | "xml" => ("⚙", egui::Color32::from_rgb(139, 233, 253)),
                                    "md" | "txt" => ("📝", egui::Color32::from_rgb(189, 147, 249)),
                                    "zip" | "tar" | "gz" | "rar" => ("📦", self.palette().accent),
                                    "png" | "jpg" | "jpeg" | "gif" => ("🖼", egui::Color32::from_rgb(80, 250, 123)),
                                    "mp3" | "wav" | "ogg" => ("🎵", egui::Color32::from_rgb(139, 233, 253)),
                                    "mp4" | "avi" | "mov" => ("🎬", self.palette().accent),
                                    "exe" | "sh" | "bat" => ("⚡", egui::Color32::from_rgb(80, 250, 123)),
                                    _ => ("📄", self.palette().text_dim),
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

                                    let text_color = if is_selected || is_multi_selected {
                                        visuals.strong_text_color()
                                    } else {
                                        // Theme-aware: LIGHT_GRAY broke Light themes
                                        visuals.widgets.noninteractive.fg_stroke.color
                                    };
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
                                    ui.add_space(10.0);
                                    let size_str = if item.is_dir {
                                        self.dir_size_for(ctx, &item.path)
                                            .map(crate::filesystem::format_size)
                                            .unwrap_or_else(|| "…".to_string())
                                    } else {
                                        crate::filesystem::format_size(item.size)
                                    };
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
                                } else if ctrl_pressed {
                                    // Non-contiguous toggle selection (Explorer-style)
                                    pane.selected_index = i;
                                    if pane.is_item_selected(i) {
                                        pane.selected_items.retain(|&x| x != i);
                                    } else {
                                        pane.selected_items.push(i);
                                    }
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

                // Empty-state message drawn over the (row-less) table area
                if is_table_empty {
                    ui.painter().text(
                        egui::pos2(table_rect.center().x, table_rect.center().y + 20.0),
                        egui::Align2::CENTER_CENTER,
                        empty_reason,
                        egui::FontId::proportional(14.0),
                        visuals.widgets.noninteractive.fg_stroke.color,
                    );
                }
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
