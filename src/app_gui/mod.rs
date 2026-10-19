use crate::pane::{Pane, SortBy, SortOrder};
use anyhow::Result;
use std::env;
use std::fs;
use std::path::PathBuf;

mod bars;
mod context_menu;
mod dialogs_file;
mod dialogs_info;
mod dialogs_ops;
mod dialogs_search;
mod dialogs_system;
mod image_viewer;
mod keys;
mod main_area;
mod menu;
mod pane_view;
mod preview;
mod sidebar;
mod sidebar_panel;
mod theme;

/// Preferences persisted across app restarts via egui storage.
#[derive(serde::Serialize, serde::Deserialize)]
struct AppPrefs {
    current_theme: Theme,
    show_hidden_files: bool,
    show_sidebar: bool,
    show_preview_panel: bool,
    sidebar_quick_access_expanded: bool,
    sidebar_bookmarks_expanded: bool,
    sidebar_devices_expanded: bool,
    filter_txt: bool,
    filter_image: bool,
    filter_pdf: bool,
    filter_doc: bool,
    filter_xls: bool,
    left_dir: PathBuf,
    right_dir: PathBuf,
    left_sort_by: SortBy,
    left_sort_order: SortOrder,
    right_sort_by: SortBy,
    right_sort_order: SortOrder,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ClipboardOperation {
    Copy,
    Cut,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ArchiveKind {
    Zip,
    TarGz,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    Dark,
    Light,
    Dracula,
    Nord,
    Monokai,
    SolarizedDark,
}

impl Theme {
    pub fn name(&self) -> &str {
        match self {
            Theme::Dark => "Dark (Default)",
            Theme::Light => "Light",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::Monokai => "Monokai",
            Theme::SolarizedDark => "Solarized Dark",
        }
    }

    pub fn all() -> Vec<Theme> {
        vec![
            Theme::Dark,
            Theme::Light,
            Theme::Dracula,
            Theme::Nord,
            Theme::Monokai,
            Theme::SolarizedDark,
        ]
    }
}

pub struct ImageViewerState {
    pub image_path: PathBuf,
    pub image_name: String,
    pub texture: Option<egui::TextureHandle>,
    pub zoom: f32,
    pub offset: egui::Vec2,
    pub dragging: bool,
    pub drag_start: egui::Pos2,
}

impl ImageViewerState {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self {
            image_path: path,
            image_name: name,
            texture: None,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            dragging: false,
            drag_start: egui::Pos2::ZERO,
        }
    }
}

/// A transient notification shown near the op site (bottom-center) that
/// auto-dismisses after a few seconds.
#[derive(Clone)]
pub struct Toast {
    pub text: String,
    pub kind: ToastKind,
    /// Seconds since creation, advanced each frame.
    pub age: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastKind {
    fn icon(&self) -> &'static str {
        match self {
            ToastKind::Info => "ℹ",
            ToastKind::Success => "✅",
            ToastKind::Warning => "⚠️",
            ToastKind::Error => "❌",
        }
    }

    fn color(&self) -> egui::Color32 {
        match self {
            ToastKind::Info => egui::Color32::from_rgb(138, 180, 248),
            ToastKind::Success => egui::Color32::from_rgb(129, 201, 149),
            ToastKind::Warning => egui::Color32::from_rgb(251, 188, 5),
            ToastKind::Error => egui::Color32::from_rgb(242, 139, 130),
        }
    }
}

pub struct FileManagerApp {
    pub left_pane: Pane,
    pub right_pane: Pane,
    pub active_pane: usize,
    pub status_message: String,
    pub show_delete_confirm: bool,
    pub delete_permanently: bool,
    pub item_to_delete: Option<String>,
    pub show_hidden_files: bool,
    pub(crate) clipboard: Option<(std::path::PathBuf, ClipboardOperation)>,
    pub show_context_menu: bool,
    pub context_menu_pos: egui::Pos2,
    pub context_menu_item_index: usize,
    pub context_menu_just_opened: bool,
    pub hovered_item: Option<(usize, usize)>,
    pub filter_mode: bool,
    pub show_new_folder_dialog: bool,
    pub new_folder_name: String,
    pub show_new_file_dialog: bool,
    pub new_file_name: String,
    pub show_rename_dialog: bool,
    pub rename_new_name: String,
    pub show_about_dialog: bool,
    pub show_properties_dialog: bool,
    pub properties_item: Option<crate::filesystem::FileItem>,
    pub current_theme: Theme,
    pub show_theme_selector: bool,
    pub image_viewer: Option<ImageViewerState>,
    pub show_preview_panel: bool,
    pub preview_content_left: Option<PreviewContent>,
    pub preview_content_right: Option<PreviewContent>,
    pub show_search_dialog: bool,
    pub search_criteria: crate::filesystem::SearchCriteria,
    pub search_results: Vec<crate::filesystem::FileItem>,
    pub search_in_progress: bool,
    pub search_min_size_text: String,
    pub search_max_size_text: String,
    pub search_days_ago: String,
    pub show_compare_dialog: bool,
    pub comparison_result: Option<crate::filesystem::FileComparison>,
    pub compare_scroll_offset: f32,
    pub show_mounts_dialog: bool,
    pub show_sidebar: bool,
    pub bookmark_manager: crate::bookmarks::BookmarkManager,
    pub show_add_bookmark_dialog: bool,
    pub new_bookmark_name: String,
    pub sidebar_quick_access_expanded: bool,
    pub sidebar_bookmarks_expanded: bool,
    pub sidebar_devices_expanded: bool,
    pub filter_txt: bool,
    pub filter_image: bool,
    pub filter_pdf: bool,
    pub filter_doc: bool,
    pub filter_xls: bool,
    pub op_runner: Option<crate::ops::OpRunner>,
    pub show_progress_dialog: bool,
    pub op_result_message: Option<String>,
    /// Transient UI notifications (auto-dismiss); replaces the old single
    /// status_message text buried at the end of the status bar.
    pub toasts: Vec<Toast>,
    /// Tracks status_message changes so each new message becomes a toast.
    last_status_message: String,
    // Paste conflict dialog state (destination file already exists)
    pub show_paste_conflict: bool,
    pub paste_conflict_source: Option<PathBuf>,
    pub paste_conflict_source_name: String,
    pub paste_conflict_dest_dir: Option<PathBuf>,
    pub(crate) paste_conflict_op: Option<ClipboardOperation>,
    pub paste_conflict_new_name: String,
}

#[derive(Clone)]
pub enum PreviewContent {
    Text(String),
    Image(PathBuf),
    Pdf {
        name: String,
        size: String,
        modified: String,
        pages: usize,
        image: Option<image::DynamicImage>,
    },
    FileInfo {
        name: String,
        size: String,
        modified: String,
        permissions: String,
        is_dir: bool,
    },
}

impl FileManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Try to get home directory first, fallback to current dir
        let start_dir = dirs::home_dir()
            .or_else(|| env::current_dir().ok())
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        // Load persisted preferences (falls back to defaults on first run)
        let prefs: Option<AppPrefs> = cc
            .storage
            .as_ref()
            .and_then(|storage| eframe::get_value(*storage, "app_prefs"));

        let left_dir = prefs
            .as_ref()
            .map(|p| p.left_dir.clone())
            .filter(|p| p.is_dir())
            .unwrap_or_else(|| start_dir.clone());
        let right_dir = prefs
            .as_ref()
            .map(|p| p.right_dir.clone())
            .filter(|p| p.is_dir())
            .unwrap_or(left_dir.clone());

        let left_pane = Pane::new(left_dir).unwrap_or_else(|e| {
            eprintln!("Error creating left pane: {}", e);
            Pane::new(start_dir.clone()).expect("Failed to create pane")
        });

        let right_pane = Pane::new(right_dir).unwrap_or_else(|e| {
            eprintln!("Error creating right pane: {}", e);
            Pane::new(start_dir.clone()).expect("Failed to create pane")
        });

        let mut app = Self {
            left_pane,
            right_pane,
            active_pane: 0,
            status_message: format!("Ready - Starting directory: {}", start_dir.display()),
            show_delete_confirm: false,
            delete_permanently: false,
            item_to_delete: None,
            show_hidden_files: false,
            clipboard: None,
            show_context_menu: false,
            context_menu_pos: egui::Pos2::ZERO,
            context_menu_item_index: 0,
            context_menu_just_opened: false,
            hovered_item: None,
            filter_mode: false,
            show_new_folder_dialog: false,
            new_folder_name: String::new(),
            show_new_file_dialog: false,
            new_file_name: String::new(),
            show_rename_dialog: false,
            rename_new_name: String::new(),
            show_about_dialog: false,
            show_properties_dialog: false,
            properties_item: None,
            current_theme: Theme::Dark,
            show_theme_selector: false,
            image_viewer: None,
            show_preview_panel: true,
            preview_content_left: None,
            preview_content_right: None,
            show_search_dialog: false,
            search_criteria: crate::filesystem::SearchCriteria::default(),
            search_results: Vec::new(),
            search_in_progress: false,
            search_min_size_text: String::new(),
            search_max_size_text: String::new(),
            search_days_ago: String::new(),
            show_compare_dialog: false,
            comparison_result: None,
            compare_scroll_offset: 0.0,
            show_mounts_dialog: false,
            show_sidebar: true,
            bookmark_manager: crate::bookmarks::BookmarkManager::load().unwrap_or_default(),
            show_add_bookmark_dialog: false,
            new_bookmark_name: String::new(),
            sidebar_quick_access_expanded: true,
            sidebar_bookmarks_expanded: true,
            sidebar_devices_expanded: true,
            filter_txt: false,
            filter_image: false,
            filter_pdf: false,
            filter_doc: false,
            filter_xls: false,
            op_runner: None,
            show_progress_dialog: false,
            op_result_message: None,
            show_paste_conflict: false,
            paste_conflict_source: None,
            paste_conflict_source_name: String::new(),
            paste_conflict_dest_dir: None,
            paste_conflict_op: None,
            paste_conflict_new_name: String::new(),
            toasts: Vec::new(),
            last_status_message: String::new(),
        };

        // Overlay persisted prefs on top of defaults
        if let Some(prefs) = prefs {
            app.current_theme = prefs.current_theme;
            app.show_hidden_files = prefs.show_hidden_files;
            app.show_sidebar = prefs.show_sidebar;
            app.show_preview_panel = prefs.show_preview_panel;
            app.sidebar_quick_access_expanded = prefs.sidebar_quick_access_expanded;
            app.sidebar_bookmarks_expanded = prefs.sidebar_bookmarks_expanded;
            app.sidebar_devices_expanded = prefs.sidebar_devices_expanded;
            app.filter_txt = prefs.filter_txt;
            app.filter_image = prefs.filter_image;
            app.filter_pdf = prefs.filter_pdf;
            app.filter_doc = prefs.filter_doc;
            app.filter_xls = prefs.filter_xls;
            app.left_pane.sort_by = prefs.left_sort_by;
            app.left_pane.sort_order = prefs.left_sort_order;
            app.right_pane.sort_by = prefs.right_sort_by;
            app.right_pane.sort_order = prefs.right_sort_order;
            let _ = app.left_pane.refresh();
            let _ = app.right_pane.refresh();
        }

        app
    }

    /// Push a toast notification (newest on top).
    pub fn push_toast(&mut self, text: impl Into<String>, kind: ToastKind) {
        self.toasts.insert(
            0,
            Toast {
                text: text.into(),
                kind,
                age: 0.0,
            },
        );
        // Cap so a burst of errors doesn't stack unbounded
        if self.toasts.len() > 5 {
            self.toasts.truncate(5);
        }
    }

    pub(crate) fn render_toasts(&mut self, ctx: &egui::Context) {
        if self.toasts.is_empty() {
            return;
        }

        let dt = ctx.input(|i| i.stable_dt);
        for toast in &mut self.toasts {
            toast.age += dt;
        }

        // Expire after 3.5s
        self.toasts.retain(|t| t.age < 3.5);
        if self.toasts.is_empty() {
            return;
        }

        let screen = ctx.screen_rect();
        egui::Area::new(egui::Id::new("toasts"))
            .anchor(
                egui::Align2::CENTER_BOTTOM,
                [0.0, -(if self.show_preview_panel { 236.0 } else { 36.0 })],
            )
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                let _ = screen;
                for toast in &self.toasts {
                    let fade_out_start = 3.0;
                    let alpha = (if toast.age < fade_out_start {
                        1.0
                    } else {
                        1.0 - (toast.age - fade_out_start) / 0.5
                    })
                    .clamp(0.0, 1.0);

                    let accent = toast.kind.color();
                    // Fade by multiplying colors (Frame has no opacity API)
                    let fill = egui::Color32::from_rgb(45, 47, 50).gamma_multiply(alpha);
                    let text_col =
                        egui::Color32::from_rgb(232, 234, 237).gamma_multiply(alpha);
                    egui::Frame::default()
                        .fill(fill)
                        .rounding(8.0)
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 2.0),
                            blur: 8.0,
                            spread: 0.0,
                            color: egui::Color32::from_black_alpha(100),
                        })
                        .stroke(egui::Stroke::new(2.0, accent.gamma_multiply(alpha)))
                        .inner_margin(egui::Margin::symmetric(14.0, 8.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("{} ", toast.kind.icon()))
                                        .size(14.0)
                                        .color(accent.gamma_multiply(alpha)),
                                );
                                ui.label(
                                    egui::RichText::new(&toast.text)
                                        .size(13.0)
                                        .color(text_col),
                                );
                            });
                        });
                    ui.add_space(6.0);
                }
            });

        // Keep repainting while toasts fade out
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }

    pub fn start_operation(&mut self, op: crate::ops::Operation) {
        if self.op_runner.is_some() {
            self.status_message = "Another operation is already running".to_string();
            return;
        }
        match crate::ops::OpRunner::start(op) {
            Ok(runner) => {
                self.op_runner = Some(runner);
                self.show_progress_dialog = true;
                self.op_result_message = None;
            }
            Err(e) => {
                self.status_message = format!("❌ Failed to start operation: {}", e);
            }
        }
    }

    pub fn poll_operation(&mut self, ctx: &egui::Context) {
        let mut finished = false;
        if let Some(runner) = &self.op_runner {
            ctx.request_repaint();
            if runner.is_finished() {
                finished = true;
            }
        }
        if finished {
            if let Some(mut runner) = self.op_runner.take() {
                runner.join();
                let snap = runner.snapshot();
                if let Some(err) = &snap.error {
                    self.status_message = format!("❌ {} failed: {}", snap.kind.label(), err);
                    self.op_result_message = Some(format!("Operation failed: {}", err));
                } else if snap.cancelled {
                    self.status_message = "⚠️ Operation cancelled".to_string();
                    self.op_result_message = Some("Operation cancelled".to_string());
                } else {
                    self.status_message = format!(
                        "✅ {} complete: {} ({} files)",
                        snap.kind.label(),
                        snap.title,
                        snap.files_total
                    );
                    self.op_result_message = Some(format!(
                        "✅ Completed successfully\n\n{} → {}",
                        snap.kind.label(),
                        snap.title
                    ));
                }
            }
            self.show_progress_dialog = true; // keep dialog to show result
            let _ = self.left_pane.refresh();
            let _ = self.right_pane.refresh();
        }
    }

    pub fn get_active_pane(&self) -> &Pane {
        if self.active_pane == 0 {
            &self.left_pane
        } else {
            &self.right_pane
        }
    }

    pub fn get_active_pane_mut(&mut self) -> &mut Pane {
        if self.active_pane == 0 {
            &mut self.left_pane
        } else {
            &mut self.right_pane
        }
    }

    pub fn refresh_pane(&mut self, pane_index: usize) -> Result<()> {
        if pane_index == 0 {
            self.left_pane.refresh()?;
        } else {
            self.right_pane.refresh()?;
        }
        Ok(())
    }

    fn is_image_file(path: &std::path::Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(
                ext.as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp"
            )
        } else {
            false
        }
    }

    pub fn open_image_viewer(&mut self, path: PathBuf, name: String) {
        self.image_viewer = Some(ImageViewerState::new(path, name));
    }

    pub fn close_image_viewer(&mut self) {
        self.image_viewer = None;
    }

    pub fn copy_to_clipboard(&mut self) {
        let item_data = self
            .get_active_pane()
            .get_selected_item()
            .map(|item| (item.path.clone(), item.name.clone()));

        if let Some((path, name)) = item_data {
            if name != ".." {
                self.clipboard = Some((path, ClipboardOperation::Copy));
                self.status_message = format!("Copied to clipboard: {}", name);
            }
        }
    }

    pub fn cut_to_clipboard(&mut self) {
        let item_data = self
            .get_active_pane()
            .get_selected_item()
            .map(|item| (item.path.clone(), item.name.clone()));

        if let Some((path, name)) = item_data {
            if name != ".." {
                self.clipboard = Some((path, ClipboardOperation::Cut));
                self.status_message = format!("Cut to clipboard: {}", name);
            }
        }
    }

    /// Show a conflict dialog when the paste destination already exists.
    fn open_paste_conflict(&mut self, source_path: PathBuf, op: ClipboardOperation) {
        let dest_dir = if self.active_pane == 0 {
            self.right_pane.current_path.clone()
        } else {
            self.left_pane.current_path.clone()
        };
        let file_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Suggest a unique name: "name (1).ext"
        let mut candidate = file_name.clone();
        let mut counter = 1;
        while dest_dir.join(&candidate).exists() {
            let stem = std::path::Path::new(&file_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&file_name);
            let ext = std::path::Path::new(&file_name)
                .extension()
                .and_then(|e| e.to_str());
            candidate = match ext {
                Some(ext) => format!("{} ({}).{}", stem, counter, ext),
                None => format!("{} ({})", stem, counter),
            };
            counter += 1;
        }

        self.show_paste_conflict = true;
        self.paste_conflict_source = Some(source_path);
        self.paste_conflict_source_name = file_name;
        self.paste_conflict_dest_dir = Some(dest_dir);
        self.paste_conflict_op = Some(op);
        self.paste_conflict_new_name = candidate;
    }

    fn begin_paste(&mut self, source_path: PathBuf, dest_dir: PathBuf, op: ClipboardOperation, consume_clipboard: bool) {
        let op_cmd = match op {
            ClipboardOperation::Copy => crate::ops::Operation::Copy {
                sources: vec![source_path],
                dest: dest_dir,
            },
            ClipboardOperation::Cut => crate::ops::Operation::Move {
                sources: vec![source_path],
                dest: dest_dir,
            },
        };
        self.start_operation(op_cmd);
        // Cut clipboard is consumed on successful paste; Copy persists.
        if !consume_clipboard && matches!(op, ClipboardOperation::Cut) {
            self.clipboard = None;
        }
    }

    pub fn paste_conflict_overwrite(&mut self) {
        let (source, dest_dir) = match (&self.paste_conflict_source, &self.paste_conflict_dest_dir) {
            (Some(s), Some(d)) => (s.clone(), d.clone()),
            _ => return,
        };
        let op = self.paste_conflict_op.unwrap_or(ClipboardOperation::Copy);
        let file_name = self.paste_conflict_source_name.clone();

        let existing = dest_dir.join(&file_name);
        let source_is_dir = fs::metadata(&source).map(|m| m.is_dir()).unwrap_or(false);
        if existing.is_dir() {
            // Source dir over existing dir: copy merges safely; move would fail
            if source_is_dir && matches!(op, ClipboardOperation::Copy) {
                self.begin_paste(source, dest_dir, op, false);
            } else {
                self.status_message =
                    format!("❌ Cannot overwrite folder '{}': destination exists", file_name);
            }
        } else if source_is_dir {
            // Dir replacing file: too destructive to do silently
            self.status_message =
                format!("❌ '{}' already exists as a file; rename instead", file_name);
        } else {
            match fs::remove_file(&existing) {
                Ok(_) => self.begin_paste(source, dest_dir, op, false),
                Err(e) => {
                    // Keep dialog open so the user can pick another option
                    self.status_message = format!("❌ Cannot overwrite: {}", e);
                    return;
                }
            }
        }
        self.close_paste_conflict();
    }

    pub fn paste_conflict_rename(&mut self) {
        let (source, dest_dir) = match (&self.paste_conflict_source, &self.paste_conflict_dest_dir) {
            (Some(s), Some(d)) => (s.clone(), d.clone()),
            _ => return,
        };
        let op = self.paste_conflict_op.unwrap_or(ClipboardOperation::Copy);
        let new_name = self.paste_conflict_new_name.trim().to_string();

        if new_name.is_empty() || new_name == "." || new_name == ".." {
            self.status_message = "Invalid name".to_string();
            return;
        }
        if dest_dir.join(&new_name).exists() {
            self.status_message = format!("❌ '{}' also exists, pick another name", new_name);
            return;
        }

        // Synchronous copy under the new name (paste handles a single item)
        let dest = dest_dir.join(&new_name);
        let result = if source.is_dir() {
            copy_dir_sync(&source, &dest)
        } else {
            fs::copy(&source, &dest)
                .map(|_| ())
                .map_err(anyhow::Error::from)
        };

        match result {
            Ok(_) => {
                if matches!(op, ClipboardOperation::Cut) {
                    // Move semantics: the original source no longer belongs here
                    let remove_result = if source.is_dir() {
                        fs::remove_dir_all(&source)
                    } else {
                        fs::remove_file(&source)
                    };
                    if let Err(e) = remove_result {
                        self.status_message =
                            format!("⚠ Copied as '{}', but failed to remove source: {}", new_name, e);
                        self.close_paste_conflict();
                        return;
                    }
                    self.clipboard = None;
                    self.status_message = format!("Moved as '{}'", new_name);
                } else {
                    self.status_message = format!("Copied as '{}'", new_name);
                }
                let _ = self.left_pane.refresh();
                let _ = self.right_pane.refresh();
            }
            Err(e) => {
                self.status_message = format!("❌ Paste failed: {}", e);
            }
        }
        self.close_paste_conflict();
    }

    pub fn paste_conflict_skip(&mut self) {
        let name = self.paste_conflict_source_name.clone();
        self.status_message = format!("Skipped '{}' (already exists)", name);
        self.close_paste_conflict();
    }

    pub fn paste_conflict_cancel(&mut self) {
        // Clipboard kept so the user can paste elsewhere
        self.close_paste_conflict();
    }

    fn close_paste_conflict(&mut self) {
        self.show_paste_conflict = false;
        self.paste_conflict_source = None;
        self.paste_conflict_source_name = String::new();
        self.paste_conflict_dest_dir = None;
        self.paste_conflict_op = None;
        self.paste_conflict_new_name = String::new();
    }

    pub fn paste_from_clipboard(&mut self) -> Result<()> {
        // Clone clipboard data to avoid borrow issues
        let clipboard_data = self.clipboard.clone();

        if let Some((source_path, operation)) = clipboard_data {
            let target_path = if self.active_pane == 0 {
                self.right_pane.current_path.clone()
            } else {
                self.left_pane.current_path.clone()
            };

            let file_name = source_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let dest_path = target_path.join(file_name);

            // Destination already exists: let the user resolve the conflict
            // (Overwrite / Rename / Skip) instead of failing outright.
            if dest_path.exists() {
                self.open_paste_conflict(source_path.clone(), operation);
                return Ok(());
            }

            let op = match operation {
                ClipboardOperation::Copy => crate::ops::Operation::Copy {
                    sources: vec![source_path.clone()],
                    dest: target_path,
                },
                ClipboardOperation::Cut => crate::ops::Operation::Move {
                    sources: vec![source_path.clone()],
                    dest: target_path,
                },
            };

            // Cut clipboard is consumed; copy clipboard persists for repeated paste
            if matches!(operation, ClipboardOperation::Cut) {
                self.clipboard = None;
            }
            self.start_operation(op);
        } else {
            self.status_message = "Clipboard is empty".to_string();
        }
        Ok(())
    }

    /// Transfer selected items from one pane to the other (background op).
    /// `op` is Copy or Move; `from_pane` is 0 (left) or 1 (right).
    pub fn transfer_selected(&mut self, from_pane: usize, op: crate::ops::Operation) {
        let (sources, dest) = {
            let (src_pane, dst_pane) = if from_pane == 0 {
                (&self.left_pane, &self.right_pane)
            } else {
                (&self.right_pane, &self.left_pane)
            };
            let sources: Vec<PathBuf> = src_pane
                .get_selected_items()
                .iter()
                .filter(|item| item.name != "..")
                .map(|item| item.path.clone())
                .collect();
            (sources, dst_pane.current_path.clone())
        };

        if sources.is_empty() {
            self.status_message = "No items selected".to_string();
            return;
        }

        let op = match op {
            crate::ops::Operation::Copy { .. } => crate::ops::Operation::Copy { sources, dest },
            _ => crate::ops::Operation::Move { sources, dest },
        };
        self.start_operation(op);
    }

    pub fn open_file_with_default_app(&mut self) -> Result<()> {
        let item_data = self
            .get_active_pane()
            .get_selected_item()
            .map(|item| (item.path.clone(), item.name.clone(), item.is_dir));

        if let Some((path, name, is_dir)) = item_data {
            if !is_dir && name != ".." {
                // Check if it's an image file - open in built-in viewer
                if Self::is_image_file(&path) {
                    self.open_image_viewer(path, name.clone());
                    self.status_message = format!("Opening image: {}", name);
                    return Ok(());
                }

                // Use system default app to open file
                #[cfg(target_os = "linux")]
                {
                    std::process::Command::new("xdg-open").arg(&path).spawn()?;
                    self.status_message = format!("Opening: {}", name);
                }

                #[cfg(target_os = "macos")]
                {
                    std::process::Command::new("open").arg(&path).spawn()?;
                    self.status_message = format!("Opening: {}", name);
                }

                #[cfg(target_os = "windows")]
                {
                    std::process::Command::new("cmd")
                        .args(["/C", "start", "", &path.to_string_lossy()])
                        .spawn()?;
                    self.status_message = format!("Opening: {}", name);
                }
            } else if is_dir && name != ".." {
                self.status_message = "Use Enter or double-click to open directories".to_string();
            }
        }
        Ok(())
    }

    /// Open the active pane's directory (or a selected folder) in a terminal
    /// emulator, trying the common ones in turn until one spawns.
    pub fn open_terminal_here(&mut self) -> Result<()> {
        let dir = match self.get_active_pane().get_selected_item().map(|i| i.path.clone()) {
            Some(p) if p.is_dir() && p.file_name().is_some() => p,
            _ => self.get_active_pane().current_path.clone(),
        };

        #[cfg(target_os = "linux")]
        {
            const CANDIDATES: &[(&str, &str)] = &[
                ("gnome-terminal", "--working-directory"),
                ("konsole", "--workdir"),
                ("alacritty", "--working-directory"),
                ("kitty", "--working-directory"),
                ("xfce4-terminal", "--working-directory"),
                ("terminator", "--working-directory"),
                ("tilix", "--working-directory"),
                ("x-terminal-emulator", "--working-directory"),
            ];
            for (cmd, flag) in CANDIDATES {
                let spawned = std::process::Command::new(cmd)
                    .arg(flag)
                    .arg(&dir)
                    .spawn();
                match spawned {
                    Ok(_) => {
                        self.status_message = format!("Opening terminal: {}", cmd);
                        return Ok(());
                    }
                    Err(_) => continue,
                }
            }
            self.status_message = "No terminal emulator found".to_string();
            anyhow::bail!("No terminal emulator found");
        }

        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").args(["-a", "Terminal"]).arg(&dir).spawn()?;
            self.status_message = "Opening Terminal...".to_string();
            Ok(())
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", "", "/D"])
                .arg(&dir)
                .spawn()?;
            self.status_message = "Opening terminal...".to_string();
            Ok(())
        }
    }

    /// Restore an item that is currently inside the OS trash back to its
    /// original location.
    pub fn restore_trashed_item(&mut self, path: &std::path::Path, _name: &str) {
        match crate::trash::restore_from_trash(path) {
            Ok(restored) => {
                self.status_message = format!("Restored to {}", restored.display());
                let _ = self.get_active_pane_mut().refresh();
                if self.show_preview_panel {
                    self.update_previews();
                }
            }
            Err(e) => {
                self.status_message = format!("❌ Restore failed: {}", e);
            }
        }
    }

    pub fn delete_selected_items(&mut self, permanent: bool) -> Result<()> {
        let items_to_delete: Vec<_> = self.get_active_pane()
            .get_selected_items()
            .iter()
            .filter(|item| item.name != "..")
            .map(|item| (item.path.clone(), item.is_dir, item.name.clone()))
            .collect();

        if permanent {
            let mut deleted_count = 0;
            for (path, is_dir, name) in &items_to_delete {
                let result = if *is_dir {
                    fs::remove_dir_all(path)
                } else {
                    fs::remove_file(path)
                };
                match result {
                    Ok(_) => deleted_count += 1,
                    Err(e) => {
                        self.status_message = format!("❌ Failed to delete '{}': {}", name, e);
                        anyhow::bail!("Delete failed: {}", e);
                    }
                }
            }

            self.status_message = if deleted_count == 1 {
                "Deleted 1 item permanently".to_string()
            } else {
                format!("Deleted {} items permanently", deleted_count)
            };
        } else {
            // Move to the OS trash instead of permanently deleting, so the
            // user can recover from accidental deletions.
            let mut moved_count = 0;
            for (path, _is_dir, name) in &items_to_delete {
                match crate::trash::move_to_trash(path) {
                    Ok(_) => moved_count += 1,
                    Err(e) => {
                        self.status_message =
                            format!("❌ Failed to move '{}' to trash: {}", name, e);
                        anyhow::bail!("Move to trash failed: {}", e);
                    }
                }
            }

            self.status_message = if moved_count == 1 {
                "Moved 1 item to Trash".to_string()
            } else {
                format!("Moved {} items to Trash", moved_count)
            };
        }

        self.get_active_pane_mut().clear_selection();
        self.get_active_pane_mut().refresh()?;
        Ok(())
    }

    pub(crate) fn compress_item(&mut self, kind: ArchiveKind) -> Result<()> {
        let item = self.get_active_pane().get_selected_item().cloned();

        if let Some(item) = item {
            if item.name != ".." {
                let archive_name = match kind {
                    ArchiveKind::Zip => format!("{}.zip", item.name),
                    ArchiveKind::TarGz => format!("{}.tar.gz", item.name),
                };
                let archive_path = item
                    .path
                    .parent()
                    .unwrap_or(item.path.as_path())
                    .join(&archive_name);

                if archive_path.exists() {
                    self.status_message = format!("❌ Archive already exists: {}", archive_name);
                    anyhow::bail!("Archive already exists");
                }

                let op = match kind {
                    ArchiveKind::Zip => crate::ops::Operation::CompressZip {
                        source: item.path.clone(),
                        dest_zip: archive_path,
                    },
                    ArchiveKind::TarGz => crate::ops::Operation::CompressTarGz {
                        source: item.path.clone(),
                        dest_path: archive_path,
                    },
                };
                self.start_operation(op);
            }
        }
        Ok(())
    }

    pub fn extract_archive(&mut self) -> Result<()> {
        let item = self.get_active_pane().get_selected_item().cloned();

        if let Some(item) = item {
            if item.name != ".." && !item.is_dir {
                let extract_dir = item.path.parent().unwrap_or(item.path.as_path());

                if item.name.ends_with(".zip") {
                    self.start_operation(crate::ops::Operation::ExtractZip {
                        archive: item.path.clone(),
                        dest_dir: extract_dir.to_path_buf(),
                    });
                } else if crate::filesystem::is_tar_gz(&item.path)
                    || crate::filesystem::is_tar(&item.path)
                {
                    self.start_operation(crate::ops::Operation::ExtractTar {
                        archive: item.path.clone(),
                        dest_dir: extract_dir.to_path_buf(),
                    });
                } else {
                    self.status_message =
                        "Only .zip, .tar.gz, .tgz and .tar files are supported".to_string();
                }
            }
        }
        Ok(())
    }

    pub fn compare_selected_files(&mut self) -> Result<()> {
        let left_item = self.left_pane.get_selected_item().cloned();
        let right_item = self.right_pane.get_selected_item().cloned();

        if let (Some(left), Some(right)) = (left_item, right_item) {
            if left.name == ".." || right.name == ".." {
                self.status_message = "Cannot compare parent directory".to_string();
                return Ok(());
            }

            if left.is_dir || right.is_dir {
                self.status_message = "Cannot compare directories (files only)".to_string();
                return Ok(());
            }

            match crate::filesystem::compare_files(&left.path, &right.path) {
                Ok(comparison) => {
                    self.comparison_result = Some(comparison.clone());
                    self.show_compare_dialog = true;
                    self.compare_scroll_offset = 0.0;

                    if comparison.are_identical {
                        self.status_message = "✅ Files are identical".to_string();
                    } else {
                        self.status_message = format!(
                            "Files differ: {} added, {} removed, {} modified",
                            comparison.right_only_lines,
                            comparison.left_only_lines,
                            comparison.modified_lines
                        );
                    }
                }
                Err(e) => {
                    self.status_message = format!("❌ Comparison failed: {}", e);
                }
            }
        } else {
            self.status_message = "Select files in both panes to compare".to_string();
        }

        Ok(())
    }

    pub fn should_show_file(&self, item: &crate::filesystem::FileItem) -> bool {
        if !self.filter_txt && !self.filter_image && !self.filter_pdf && !self.filter_doc && !self.filter_xls {
            return true;
        }

        if item.is_dir {
            return true;
        }

        if let Some(ext) = item.path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            
            if self.filter_txt && matches!(ext_lower.as_str(), "txt" | "log" | "md" | "json" | "xml" | "csv") {
                return true;
            }
            if self.filter_image && matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" | "svg") {
                return true;
            }
            if self.filter_pdf && ext_lower == "pdf" {
                return true;
            }
            if self.filter_doc && matches!(ext_lower.as_str(), "doc" | "docx" | "odt" | "rtf") {
                return true;
            }
            if self.filter_xls && matches!(ext_lower.as_str(), "xls" | "xlsx" | "ods" | "csv") {
                return true;
            }
        }

        false
    }
}

impl eframe::App for FileManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll background operation
        self.poll_operation(ctx);

        // Every status_message change surfaces as a transient toast so it
        // can't be lost at the end of a long status bar.
        if !self.status_message.is_empty() && self.status_message != self.last_status_message {
            let kind = if self.status_message.starts_with("❌") {
                ToastKind::Error
            } else if self.status_message.starts_with("⚠") {
                ToastKind::Warning
            } else if self.status_message.starts_with("✅") {
                ToastKind::Success
            } else {
                ToastKind::Info
            };
            self.last_status_message = self.status_message.clone();
            self.push_toast(self.status_message.clone(), kind);
        }

        // Apply modern visual style (zoom is user-controlled/persisted by egui)
        ctx.set_visuals(self.get_modern_visuals());
        self.render_menu_bar(ctx);
        self.render_filter_bar(ctx);
        self.render_function_bar_panel(ctx);
        self.render_status_bar(ctx);

        self.render_sidebar_widgets(ctx);
        self.render_main_area(ctx);
        self.handle_keyboard(ctx);

        self.render_image_viewer(ctx);
        self.render_context_menu(ctx);
        self.render_toasts(ctx);

        self.render_properties_dialog(ctx);
        self.render_new_folder_dialog(ctx);
        self.render_new_file_dialog(ctx);
        self.render_rename_dialog(ctx);
        self.render_about_dialog(ctx);
        self.render_theme_selector(ctx);
        self.render_search_dialog(ctx);
        self.render_compare_dialog(ctx);
        self.render_mounts_dialog(ctx);
        self.render_bookmark_dialog(ctx);
        self.render_delete_confirm(ctx);
        self.render_paste_conflict_dialog(ctx);
        self.render_progress_dialog(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let prefs = AppPrefs {
            current_theme: self.current_theme,
            show_hidden_files: self.show_hidden_files,
            show_sidebar: self.show_sidebar,
            show_preview_panel: self.show_preview_panel,
            sidebar_quick_access_expanded: self.sidebar_quick_access_expanded,
            sidebar_bookmarks_expanded: self.sidebar_bookmarks_expanded,
            sidebar_devices_expanded: self.sidebar_devices_expanded,
            filter_txt: self.filter_txt,
            filter_image: self.filter_image,
            filter_pdf: self.filter_pdf,
            filter_doc: self.filter_doc,
            filter_xls: self.filter_xls,
            left_dir: self.left_pane.current_path.clone(),
            right_dir: self.right_pane.current_path.clone(),
            left_sort_by: self.left_pane.sort_by,
            left_sort_order: self.left_pane.sort_order,
            right_sort_by: self.right_pane.sort_by,
            right_sort_order: self.right_pane.sort_order,
        };
        eframe::set_value(storage, "app_prefs", &prefs);
    }
}

/// Synchronous recursive copy used by the paste-conflict "Rename" action.
fn copy_dir_sync(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_dir_sync(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

