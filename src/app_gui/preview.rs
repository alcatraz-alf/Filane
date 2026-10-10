use super::{FileManagerApp, PreviewContent};
use egui;

impl FileManagerApp {
    pub(crate) fn update_previews(&mut self) {
        self.update_left_preview();
        self.update_right_preview();
    }

    pub(crate) fn update_left_preview(&mut self) {
        if let Some(item) = self.left_pane.get_selected_item() {
            self.preview_content_left = self.generate_preview_content(&item);
        } else {
            self.preview_content_left = None;
        }
    }

    pub(crate) fn update_right_preview(&mut self) {
        if let Some(item) = self.right_pane.get_selected_item() {
            self.preview_content_right = self.generate_preview_content(&item);
        } else {
            self.preview_content_right = None;
        }
    }

    pub(crate) fn generate_preview_content(&self, item: &crate::filesystem::FileItem) -> Option<PreviewContent> {
        if item.name == ".." {
            return None;
        }

        let path = &item.path;

        // Check if it's an image, text, or PDF
        if !item.is_dir {
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if matches!(
                    ext_lower.as_str(),
                    "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp"
                ) {
                    return Some(PreviewContent::Image(path.clone()));
                }

                // Check if it's a PDF
                if ext_lower == "pdf" {
                    // Try to render PDF and get page count
                    let (pages, image) = match crate::pdf_renderer::render_pdf_page(path) {
                        Ok(rendered_image) => {
                            // Also get page count
                            let page_count = crate::pdf_renderer::get_page_count(path)
                                .unwrap_or(1); // Default to 1 if we can't read page count
                            (page_count, Some(rendered_image))
                        }
                        Err(e) => {
                            // Rendering failed - try to get page count for fallback
                            eprintln!("PDF preview info: {}", e);
                            let page_count = crate::pdf_renderer::get_page_count(path)
                                .unwrap_or(0); // Default to 0 if we can't read it
                            (page_count, None)
                        }
                    };

                    return Some(PreviewContent::Pdf {
                        name: item.name.clone(),
                        size: crate::filesystem::format_size(item.size),
                        modified: crate::filesystem::format_date(item.modified),
                        pages,
                        image,
                    });
                }

                // Check if it's a text file
                if matches!(
                    ext_lower.as_str(),
                    "txt"
                        | "md"
                        | "rs"
                        | "toml"
                        | "json"
                        | "xml"
                        | "html"
                        | "css"
                        | "js"
                        | "py"
                        | "c"
                        | "cpp"
                        | "h"
                        | "hpp"
                        | "sh"
                        | "yaml"
                        | "yml"
                ) {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        let lines: Vec<&str> = content.lines().take(20).collect();
                        return Some(PreviewContent::Text(lines.join("\n")));
                    }
                }
            }
        }

        // Default: show file info
        let (summary, details) = crate::filesystem::get_permissions(path)
            .unwrap_or(("N/A".to_string(), "".to_string()));

        Some(PreviewContent::FileInfo {
            name: item.name.clone(),
            size: crate::filesystem::format_size(item.size),
            modified: crate::filesystem::format_date(item.modified),
            permissions: format!("{}\n{}", summary, details),
            is_dir: item.is_dir,
        })
    }

    pub(crate) fn render_preview_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let visuals = ctx.style().visuals.clone();
        let panel_fill = visuals.panel_fill;
        let stroke_color = visuals.widgets.noninteractive.bg_fill;
        
        egui::Frame::default()
            .fill(panel_fill)
            .stroke(egui::Stroke::new(1.0, stroke_color))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.set_height(188.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("👁 Dual Preview").size(14.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("✗").clicked() {
                            self.show_preview_panel = false;
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Fixed width layout with divider in the middle
                let available_width = ui.available_width();
                let divider_width = 2.0;
                let preview_width = (available_width - divider_width) / 2.0;

                ui.horizontal(|ui| {
                    ui.set_height(140.0);

                    // LEFT PREVIEW (fixed width)
                    ui.vertical(|ui| {
                        ui.set_width(preview_width);
                        ui.label(egui::RichText::new("Left Pane").size(11.0).strong());
                        self.render_preview_content(
                            ui,
                            ctx,
                            &self.preview_content_left.clone(),
                        );
                    });

                    // CENTER DIVIDER (fixed width)
                    ui.separator();

                    // RIGHT PREVIEW (fixed width)
                    ui.vertical(|ui| {
                        ui.set_width(preview_width);
                        ui.label(egui::RichText::new("Right Pane").size(11.0).strong());
                        self.render_preview_content(
                            ui,
                            ctx,
                            &self.preview_content_right.clone(),
                        );
                    });
                });
            });
    }

    pub(crate) fn render_preview_content(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        content: &Option<PreviewContent>,
    ) {
        if let Some(content) = content {
            match content {
                PreviewContent::Text(text) => {
                    egui::ScrollArea::vertical()
                        .max_height(120.0)
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(text)
                                    .family(egui::FontFamily::Monospace)
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(189, 193, 198)),
                            );
                        });
                }
                PreviewContent::Image(path) => match image::open(path) {
                    Ok(img) => {
                        let size = [img.width() as usize, img.height() as usize];
                        let rgba = img.to_rgba8();
                        let pixels = rgba.as_flat_samples();

                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            pixels.as_slice(),
                        );

                        let texture = ctx.load_texture(
                            format!("preview_image_{}", path.display()),
                            color_image,
                            egui::TextureOptions::LINEAR,
                        );

                        let img_size = texture.size_vec2();
                        let available_width = ui.available_width();
                        let scale = (available_width / img_size.x)
                            .min(120.0 / img_size.y)
                            .min(1.0);
                        let display_size = img_size * scale;

                        ui.add(egui::Image::new(&texture).max_size(display_size));
                    }
                    Err(e) => {
                        ui.label(
                            egui::RichText::new(format!("Error: {}", e))
                                .color(egui::Color32::RED)
                                .size(10.0),
                        );
                    }
                },
                PreviewContent::Pdf {
                    name,
                    size,
                    modified,
                    pages,
                    image,
                } => {
                    ui.vertical(|ui| {
                        // File info header
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("📋").size(16.0));
                            ui.add_space(6.0);
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(name).size(11.0).strong());
                                ui.add_space(1.0);
                                ui.label(
                                    egui::RichText::new(format!("Size: {} | Pages: {}", size, pages))
                                        .size(9.0)
                                        .color(egui::Color32::from_rgb(189, 193, 198)),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Modified: {}", modified))
                                        .size(9.0)
                                        .color(egui::Color32::from_rgb(189, 193, 198)),
                                );
                            });
                        });

                        ui.add_space(4.0);

                        // Display rendered image if available
                        if let Some(img) = image {
                            let img_size = [img.width() as usize, img.height() as usize];
                            let rgba = img.to_rgba8();
                            let pixels = rgba.as_flat_samples();

                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                img_size,
                                pixels.as_slice(),
                            );

                            let texture = ctx.load_texture(
                                format!("pdf_preview_{}", name),
                                color_image,
                                egui::TextureOptions::LINEAR,
                            );

                            let img_size = texture.size_vec2();
                            let available_width = ui.available_width();
                            let available_height = ui.available_height();
                            let scale = (available_width / img_size.x)
                                .min(available_height / img_size.y)
                                .min(1.0);
                            let display_size = img_size * scale;

                            ui.add(egui::Image::new(&texture).max_size(display_size));
                        } else {
                            ui.label(
                                egui::RichText::new("PDF preview not available")
                                    .size(9.0)
                                    .color(egui::Color32::GRAY),
                            );
                            ui.label(
                                egui::RichText::new("(PDFium library may not be installed)")
                                    .size(8.0)
                                    .color(egui::Color32::DARK_GRAY),
                            );
                        }
                    });
                }
                PreviewContent::FileInfo {
                    name,
                    size,
                    modified,
                    permissions,
                    is_dir,
                } => {
                    ui.horizontal(|ui| {
                        let icon = if *is_dir { "📁" } else { "📄" };
                        ui.label(egui::RichText::new(icon).size(20.0));
                        ui.add_space(6.0);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new(name).size(12.0).strong());
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new(format!("Size: {}", size))
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(189, 193, 198)),
                            );
                            ui.label(
                                egui::RichText::new(format!("Modified: {}", modified))
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(189, 193, 198)),
                            );
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new(permissions)
                                    .family(egui::FontFamily::Monospace)
                                    .size(9.0)
                                    .color(egui::Color32::from_rgb(138, 180, 248)),
                            );
                        });
                    });
                }
            }
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("No file selected")
                        .size(11.0)
                        .color(egui::Color32::GRAY),
                );
            });
        }
    }

}
