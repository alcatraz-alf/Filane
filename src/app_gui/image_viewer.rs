use super::FileManagerApp;
use egui;

impl FileManagerApp {
    pub(crate) fn render_image_viewer(&mut self, ctx: &egui::Context) {
        // Image Viewer Window
        if let Some(viewer) = &mut self.image_viewer {
            let mut close_viewer = false;

            egui::Window::new(format!("🖼 {}", viewer.image_name))
                .collapsible(false)
                .resizable(true)
                .default_size([800.0, 600.0])
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    // Load image if not loaded
                    if viewer.texture.is_none() {
                        match image::open(&viewer.image_path) {
                            Ok(img) => {
                                let size = [img.width() as usize, img.height() as usize];
                                let rgba = img.to_rgba8();
                                let pixels = rgba.as_flat_samples();

                                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                    size,
                                    pixels.as_slice(),
                                );

                                viewer.texture = Some(ctx.load_texture(
                                    &viewer.image_name,
                                    color_image,
                                    egui::TextureOptions::LINEAR,
                                ));
                            }
                            Err(e) => {
                                ui.label(
                                    egui::RichText::new(format!("Error loading image: {}", e))
                                        .color(egui::Color32::RED)
                                        .size(14.0),
                                );
                                if ui.button("Close").clicked() {
                                    close_viewer = true;
                                }
                                return;
                            }
                        }
                    }

                    // Toolbar
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Zoom:").size(12.0));
                        if ui.button("➖").clicked() {
                            viewer.zoom = (viewer.zoom * 0.8).max(0.1);
                        }
                        ui.label(
                            egui::RichText::new(format!("{:.0}%", viewer.zoom * 100.0)).size(12.0),
                        );
                        if ui.button("➕").clicked() {
                            viewer.zoom = (viewer.zoom * 1.2).min(10.0);
                        }
                        if ui.button("🔄 Reset").clicked() {
                            viewer.zoom = 1.0;
                            viewer.offset = egui::Vec2::ZERO;
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("✗ Close (ESC)").clicked() {
                                close_viewer = true;
                            }
                        });
                    });

                    ui.separator();

                    // Image display area
                    if let Some(texture) = &viewer.texture {
                        let _available_size = ui.available_size();
                        let img_size = texture.size_vec2();
                        let scaled_size = img_size * viewer.zoom;

                        // Scrollable area for panning
                        egui::ScrollArea::both()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let (rect, response) = ui.allocate_exact_size(
                                    scaled_size,
                                    egui::Sense::click_and_drag(),
                                );

                                // Handle dragging
                                if response.dragged() {
                                    viewer.offset += response.drag_delta();
                                }

                                // Handle scroll wheel zoom
                                if response.hovered() {
                                    let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                                    if scroll_delta != 0.0 {
                                        let zoom_delta = 1.0 + scroll_delta * 0.001;
                                        viewer.zoom = (viewer.zoom * zoom_delta).clamp(0.1, 10.0);
                                    }
                                }

                                // Draw image
                                ui.painter().image(
                                    texture.id(),
                                    rect,
                                    egui::Rect::from_min_max(
                                        egui::pos2(0.0, 0.0),
                                        egui::pos2(1.0, 1.0),
                                    ),
                                    egui::Color32::WHITE,
                                );

                                // Show image info
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Size: {}x{} | Zoom: {:.0}%",
                                        img_size.x,
                                        img_size.y,
                                        viewer.zoom * 100.0
                                    ))
                                    .size(11.0)
                                    .color(egui::Color32::GRAY),
                                );
                            });
                    }
                });

            // Handle ESC key to close
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                close_viewer = true;
            }

            if close_viewer {
                self.close_image_viewer();
            }
        }
    }
}
