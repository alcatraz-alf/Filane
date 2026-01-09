use crate::app::App;
use crate::filesystem::{format_size, get_mount_points};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};
use std::path::Path;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            if app.show_preview {
                Constraint::Percentage(60)
            } else {
                Constraint::Min(0)
            },
            if app.show_preview {
                Constraint::Percentage(40)
            } else {
                Constraint::Length(0)
            },
            Constraint::Length(2),
        ])
        .split(f.size());

    let title = Paragraph::new(
        "Filane - Dual Pane FM - Tab: Switch | Shift+↑↓: Multi-select | v: Move | d: Delete | Space: Preview | m: Mounts | q: Quit",
    )
    .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    if app.show_preview {
        // Main dual-pane area
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Draw left pane
        draw_pane(f, app, panes[0], 0);
        // Draw right pane
        draw_pane(f, app, panes[1], 1);

        // Draw preview pane
        draw_preview(f, app, chunks[2]);
    } else {
        // Main dual-pane area
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Draw left pane
        draw_pane(f, app, panes[0], 0);
        // Draw right pane
        draw_pane(f, app, panes[1], 1);
    }

    // Status bar
    let status_text = if let Some(item) = app.get_active_pane().get_selected_item() {
        let size_str = if item.is_dir {
            String::from("<DIR>")
        } else {
            format_size(item.size)
        };
        format!(
            "Selected: {} | Size: {} | Path: {}",
            item.name,
            size_str,
            item.path.display()
        )
    } else {
        String::from("No item selected")
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(status, chunks[chunks.len() - 1]);

    if app.show_mounts {
        draw_mounts_popup(f, app);
    }
}

fn draw_preview(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(" Preview ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Magenta));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let active_pane = app.get_active_pane();
    let selected_item = active_pane.get_selected_item();

    let content = if let Some(item) = selected_item {
        if item.is_dir {
            format!("Directory: {}", item.name)
        } else {
            match get_file_type(&item.path) {
                FileType::Text => preview_text_file(&item.path, inner_area.height as usize),
                FileType::Image => String::from("Image preview not supported in TUI"),
                FileType::Pdf => preview_pdf_file(&item.path),
                FileType::Other => format!("File: {} | Size: {}", item.name, format_size(item.size)),
            }
        }
    } else {
        String::from("No item selected")
    };

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(paragraph, inner_area);
}

#[derive(Debug, PartialEq)]
enum FileType {
    Text,
    Image,
    Pdf,
    Other,
}

fn get_file_type(path: &Path) -> FileType {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        match ext_lower.as_str() {
            "txt" | "md" | "rs" | "js" | "ts" | "py" | "html" | "css" | "json" | "yaml" | "toml" | "xml" | "csv" => FileType::Text,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" => FileType::Image,
            "pdf" => FileType::Pdf,
            _ => FileType::Other,
        }
    } else {
        FileType::Other
    }
}

fn preview_text_file(path: &Path, max_lines: usize) -> String {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return String::from("Could not open file"),
    };

    let reader = BufReader::new(file);
    let mut content = String::new();

    for (i, line_result) in reader.lines().enumerate() {
        if i >= max_lines - 2 {  // Reserve 2 lines for "..." and filename info
            content.push_str("\n...");
            break;
        }

        if let Ok(line) = line_result {
            content.push_str(&line);
            content.push('\n');
        }
    }

    content
}

fn preview_pdf_file(path: &Path) -> String {
    use std::io::Read;
    use std::fs::File;

    // Check if it's a valid PDF by reading the header
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return String::from("Could not open PDF file"),
    };

    let mut header = [0; 5];
    if let Ok(_) = file.read_exact(&mut header) {
        let header_str = std::str::from_utf8(&header);
        if header_str != Ok("%PDF-") {
            return String::from("File is not a valid PDF");
        }
    } else {
        return String::from("Could not read PDF header");
    }

    // Try to get PDF metadata using the existing pdf_renderer module
    let metadata = match crate::pdf_renderer::get_page_count(&path.to_path_buf()) {
        Ok(page_count) => {
            format!(
                "📋 PDF Document\n\nFilename: {}\nSize: {}\nPages: {}\nModified: {}\n\n",
                path.file_name().unwrap_or_default().to_string_lossy(),
                format_size(path.metadata().map(|m| m.len()).unwrap_or(0)),
                page_count,
                crate::filesystem::format_date(path.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(std::time::SystemTime::UNIX_EPOCH))
            )
        }
        Err(_) => {
            format!(
                "📋 PDF Document\n\nFilename: {}\nSize: {}\nModified: {}\n\n",
                path.file_name().unwrap_or_default().to_string_lossy(),
                format_size(path.metadata().map(|m| m.len()).unwrap_or(0)),
                crate::filesystem::format_date(path.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(std::time::SystemTime::UNIX_EPOCH))
            )
        }
    };

    // PDF text extraction not available in TUI mode
    format!("{}PDF preview is only available in GUI mode.", metadata)
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    let mut result = String::with_capacity(max_chars + 3); // +3 for "..."
    let mut char_count = 0;

    for c in text.chars() {
        if char_count >= max_chars {
            result.push_str("...");
            break;
        }
        result.push(c);
        char_count += 1;
    }

    result
}

fn draw_pane(f: &mut Frame, app: &App, area: Rect, pane_index: usize) {
    let pane = if pane_index == 0 {
        &app.left_pane
    } else {
        &app.right_pane
    };

    let is_active = app.active_pane == pane_index;

    let block_style = if is_active {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let title = format!(" {} ", pane.current_path.display());
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(block_style);

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Calculate visible items based on scroll
    let viewport_height = inner_area.height as usize;
    let visible_items: Vec<ListItem> = pane
        .items
        .iter()
        .enumerate()
        .skip(pane.scroll_offset)
        .take(viewport_height)
        .map(|(i, item)| {
            let icon = if item.is_dir { "📁" } else { "📄" };
            let size_str = if item.is_dir {
                String::from("<DIR>")
            } else {
                format_size(item.size)
            };

            let content = format!("{} {:<40} {:>10}", icon, item.name, size_str);

            let is_selected = i == pane.selected_index;
            let is_multi_selected = pane.is_item_selected(i);

            let style = if is_selected {
                if is_active {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Black).bg(Color::Gray)
                }
            } else if is_multi_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(visible_items);
    f.render_widget(list, inner_area);
}

fn draw_mounts_popup(f: &mut Frame, _app: &App) {
    let area = f.size();
    let popup_width = area.width.saturating_sub(10).min(80);
    let popup_height = area.height.saturating_sub(6).min(30);

    let popup_area = Rect {
        x: (area.width.saturating_sub(popup_width)) / 2,
        y: (area.height.saturating_sub(popup_height)) / 2,
        width: popup_width,
        height: popup_height,
    };

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Mount Points (Press 'm' to close) ")
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let inner_area = block.inner(popup_area);
    f.render_widget(block, popup_area);

    let mounts = get_mount_points();

    let items: Vec<ListItem> = mounts
        .iter()
        .map(|mount| {
            let icon = if mount.is_removable { "🔌" } else { "💾" };
            let usage = mount.usage_percentage();
            let used = mount.total_space - mount.available_space;

            let usage_color = if usage > 90.0 {
                Color::Red
            } else if usage > 70.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            let lines = vec![
                Line::from(vec![
                    Span::raw(format!("{} ", icon)),
                    Span::styled(
                        mount.mount_point.display().to_string(),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  Device: "),
                    Span::styled(&mount.device_name, Style::default().fg(Color::White)),
                    Span::raw(format!(
                        " | Type: {} | {}",
                        mount.file_system, mount.disk_kind
                    )),
                ]),
                Line::from(vec![
                    Span::raw("  Space: "),
                    Span::styled(
                        format!("{} / {}", format_size(used), format_size(mount.total_space)),
                        Style::default().fg(Color::White),
                    ),
                    Span::raw(" | Available: "),
                    Span::styled(
                        format_size(mount.available_space),
                        Style::default().fg(Color::Green),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  Usage: "),
                    Span::styled(
                        format!("{:.1}%", usage),
                        Style::default()
                            .fg(usage_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
            ];

            ListItem::new(lines)
        })
        .collect();

    let list = List::new(items);
    f.render_widget(list, inner_area);
}
