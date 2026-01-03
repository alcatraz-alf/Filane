# Filane - Dual Pane FM

A high-performance, feature-rich dual-pane file manager written in Rust. Available in both GUI (graphical) and TUI (terminal) versions for seamless file management across Windows, Linux, and macOS.

## Overview

Filane is a modern dual-pane file manager built with Rust, featuring two efficient side-by-side panels for fast and intuitive file navigation and management.

**Available in 2 versions:**
- 🖥️ **GUI** - Graphical window application (egui) - ideal for Windows, Linux, macOS
- 📟 **TUI** - Terminal interface (ratatui) - ideal for terminal/SSH sessions

### Features

#### Core Features
- 🗂️ **Dual-Pane Interface** - Two panels for efficient file navigation
- 🏠 **Smart Start** - Automatically opens home directory (Linux/macOS) or current directory
- 📁 **Directory Navigation** - Open and explore folders
- ⌨️ **Keyboard Navigation** - Full keyboard control
- 🖱️ **Mouse Support** (GUI) - Click-based navigation
- 📋 **Copy/Cut/Paste** - Transfer files between panels
- 🗑️ **Delete Files** - Remove files or directories (with confirmation in GUI)
- 🔄 **Refresh** - Reload directory contents
- 👁️ **Hidden Files Toggle** - Show/hide hidden files (Ctrl+H)

#### Advanced Features (GUI)
- 🔤 **Column Sorting** - Click headers to sort by Name, Size, or Date (ascending/descending)
  - Visual indicator (▲▼) shows sort direction
  - Directories always listed first when sorting by Name
  - Instant sorting without reload
- 👁️ **Dual Preview Panel** - Preview files from BOTH panes side-by-side (toggle with Space)
  - **Side-by-Side:** Left preview (left pane), Right preview (right pane) - perfect for comparison
  - **Text Files:** Preview first 20 lines with syntax highlighting (txt, md, rs, toml, json, xml, html, css, js, py, c, cpp, h, sh, yaml)
  - **Images:** Thumbnail preview with auto-scaling (PNG, JPG, GIF, BMP, ICO, WEBP)
  - **PDF Files:** Preview with file info (name, size, date)
  - **File Info:** Detailed view (name, size, date, permissions) for other files
  - Auto-updates when navigating with arrow keys or clicking panes
  - Split screen design (200px height) doesn't interfere with workflow
  - Ideal for file comparison, code review, document comparison
- 📅 **Modified Date Column** - Shows file modification date (YYYY-MM-DD HH:MM)
- 🧭 **Breadcrumb Navigation** - Click path segments for quick navigation
- ⬅️➡️ **Navigation History** - Back/Forward buttons with keyboard shortcuts (Alt+Left/Right)
  - Automatically tracks navigation history (max 50 locations)
  - Visual indicators (disabled state) when history unavailable
  - Helpful tooltips explaining functions
- 🖼️ **Built-in Image Viewer** - Preview images directly in the application
  - Supports: PNG, JPG, JPEG, GIF, BMP, ICO, WEBP
  - Zoom in/out with mouse wheel or buttons (10% - 1000%)
  - Pan/drag for navigating large images
  - Reset view with one click
  - Keyboard shortcuts (ESC to close)
  - Display image size and zoom level
- 🎨 **Theme Customization** - 6 built-in themes for personalization
  - Dark (Default) - Modern dark theme
  - Light - Clean light theme
  - Dracula - Popular purple theme
  - Nord - Arctic, north-bluish theme
  - Monokai - Classic code editor theme
  - Solarized Dark - Precision colors theme
- 🔍 **Quick Filter** (Ctrl+F) - Filter files in real-time by name
- 🔎 **Advanced Search** (Ctrl+Shift+F) - Powerful search with multiple criteria
  - **File name pattern:** Wildcard search (*.rs, document*, photo*)
  - **Content search:** Search text within files
  - **File type filter:** All, Files only, or Directories only
  - **Size range:** Min/Max size in KB
  - **Date filter:** Modified within X days
  - **Options:** Case sensitive, include hidden files
  - **Results:** Clickable list to navigate to file location
  - **Recursive:** Automatically searches all subdirectories
- 📁 **New Folder** (Ctrl+N) - Create new folders with dialog
- ✏️ **Rename** (F2) - Rename files/folders with dialog
- ⚖️ **File Comparison** (Ctrl+D) - Compare 2 files side-by-side with diff viewer
  - **Visual diff:** Line-by-line comparison with color coding
  - **Statistics:** Show equal, added, removed, modified lines count
  - **Color legend:** Green (added), Red (removed), Yellow (modified), Gray (equal)
  - **Line numbers:** Display line numbers for both files
  - **Identical detection:** Instant detection for identical files
  - **Text files only:** Automatically detects and compares text files
  - **Scrollable view:** Large files with smooth scrolling
- ℹ️ **Properties** - View file details (type, size, date, permissions, path) via context menu
  - Unix/Linux: Symbolic (rwxr-xr-x) + Octal (755) + Human-readable descriptions
    - Owner: Read, Write, Execute
    - Group: Read, Execute
    - Others: Read, Execute
  - Windows: Read-only / Read-Write with descriptions
- 🔄 **Quick Transfer Buttons** - Arrow buttons in the center for easy copy/move between panes
  - ➡📋 Copy Left → Right (blue)
  - ⬅📋 Copy Right → Left (blue)
  - ➡✂ Move Left → Right (orange)
  - ⬅✂ Move Right → Left (orange)
- 🗃️ **Archive Support** - Compress and extract file archives
  - **Compress:** Create ZIP archives from files/folders (context menu → "Compress to ZIP")
  - **Extract:** Extract ZIP archives (context menu → "Extract ZIP")
  - Cross-platform support (Windows, Linux, macOS)
  - Progress feedback in status bar
- 💡 **Tooltips** - Hover over buttons to see their function
- 📋 **Smart Clipboard** - Visual indicator when items are in clipboard
- 📊 **Enhanced Status Bar** - Detailed statistics per pane
  - Item count (total, folders, files)
  - Total size of all files
  - Example: `Left: 15 items (3 folders, 12 files) • 45.2 MB`
- 🔷 **Git Integration** - Visual git status indicators for developers
  - **Status Icons:** M (Modified), A (Added), D (Deleted), ? (Untracked)
  - **Color Coding:** Yellow (modified), Green (added), Red (deleted), Purple (untracked)
  - **Branch Info:** Current branch name in status bar
  - **Ahead/Behind:** Show commits ahead/behind remote (↑2 ↓1)
  - **Change Indicator:** Diamond icon shows if repo has uncommitted changes
- 💾 **Mount Points Viewer** - View and access all mount points/storage devices
  - **Auto-detection:** Automatically detects all mount points on system
  - **Device Info:** Display device name, filesystem type, and disk type (HDD/SSD/USB)
  - **Storage Usage:** Real-time monitoring with progress bar and color coding
    - Green: < 70% (Normal)
    - Yellow: 70-90% (Warning)
    - Red: > 90% (Critical)
  - **Quick Navigation:** Click "Open" to navigate directly to mount point
  - **USB Detection:** Identify USB drives and external hard disks with icon 🔌
  - **Cross-platform:** Support Linux, macOS, and Windows
  - **Keyboard Shortcut (TUI):** Press 'm' to toggle mount points dialog
  - **Menu Access (GUI):** Commands → Mount Points
  - **Auto-detect:** Automatically detects git repository in current directory
  - **Performance:** Lightweight, doesn't slow down file browsing
- ⭐ **Bookmarks & Sidebar** - Quick access navigation with left sidebar
  - **Sidebar Panel:** Toggle with Ctrl+B, menu Files, or floating button (◀/▶)
  - **Floating Toggle Button:** Always-visible button on edge for show/hide sidebar
  - **Expand/Collapse Sections:** Each section (Quick Access, Bookmarks, Devices) can collapse with ▼/▶
  - **Left-Aligned Items:** Clean layout with items aligned left
  - **Quick Access:** Home, Documents, Downloads, Pictures, Music, Videos, Desktop, **Trash**
  - **Bookmarks:** Save favorite folders for quick access
    - Add bookmark: Bookmarks menu → Add Current Folder or click ➕
    - Remove bookmark: Click ✗ next to bookmark
    - Persistent storage: Saved in ~/.config/dual-pane-fm/bookmarks.json
  - **Devices:** Quick access to mount points and USB drives
  - **One-click Navigation:** Click item to navigate instantly
  - **Visual Organization:** Clear sections with expand/collapse controls
- 🗑️ **Trash/Recycle Bin** - Browse and manage deleted files
  - **Quick Access:** 🗑 Trash button in Quick Access section of sidebar
  - **Browse Deleted Files:** View all deleted files
  - **Restore:** Copy files from trash to another location (F5 Copy → navigate → F7 Paste)
  - **Permanent Delete:** Delete files permanently from trash (F8 or Delete)
  - **Cross-Platform:** Linux (.local/share/Trash/files or .Trash), macOS (.Trash)
  - **Easy Access:** One-click navigate to trash from sidebar
- 🎯 **Visual Feedback** - Hover effects, shadows, rounded corners, disabled states

### Installation & Running

#### Download AppImage (Linux)

**📥 [Download Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage](releases/Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage)** (8.5 MB)

For installation instructions, see [releases/README.md](releases/README.md)

---

#### Build from Source (Linux)

**Automatic Installation:**
```bash
# Clone repository
git clone https://github.com/yourusername/dual-pane-fm.git
cd dual-pane-fm

# Run installation script (builds and installs to system)
./install.sh
```

After installation:
- ✅ Binary available at `/usr/local/bin/dual-pane-fm-gui`
- ✅ Icon available in system icons
- ✅ Desktop entry in application menu
- ✅ Run from terminal: `dual-pane-fm-gui`
- ✅ Launch from application menu

**Uninstall:**
```bash
./uninstall.sh
```

---

#### Manual Build & Run

##### GUI Version (Graphical Window)

```bash
# Enter project directory
cd dual-pane-fm

# Build GUI version
cargo build --release --bin dual-pane-fm-gui

# Run GUI
cargo run --release --bin dual-pane-fm-gui

# Or directly from binary
./target/release/dual-pane-fm-gui
```

##### TUI Version (Terminal)

```bash
# Build TUI version
cargo build --release --bin dual-pane-fm-tui

# Run TUI
cargo run --release --bin dual-pane-fm-tui

# Or directly from binary
./target/release/dual-pane-fm-tui
```

### Keyboard Shortcuts

#### TUI Version (Terminal)

| Key | Function |
|-----|----------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` | Open directory |
| `Tab` | Switch between panes |
| `c` | Copy file from active pane to other pane |
| `d` / `Delete` | Delete selected file/directory |
| `r` | Refresh pane |
| `m` | Toggle mount points dialog |
| `q` | Quit |

#### GUI Version (Window) - Total Commander Style

**Function Keys (Total Commander style):**
| Key | Function |
|-----|----------|
| `F2` | Rename file/folder |
| `F3` | Open file with default app |
| `F5` | Copy file to clipboard |
| `F6` | Cut/Move file to clipboard |
| `F7` | Paste file from clipboard |
| `F8` | Delete file/directory |
| `Ctrl+N` | Create new folder |
| `Ctrl+H` | Toggle show/hide hidden files |
| `Ctrl+B` | Toggle sidebar visibility |
| `Ctrl+F` | Toggle quick filter mode |
| `Ctrl+Shift+F` | Open advanced search dialog |
| `Ctrl+D` | Compare selected files (one from each pane) |
| `Space` | Toggle quick preview panel |
| `Alt+Left` | Navigate back in history |
| `Alt+Right` | Navigate forward in history |
| `ESC` | Close dialogs / Exit filter mode |

**Navigation:**
| Key | Function |
|-----|----------|
| `↑` / `↓` | Navigate up/down |
| `Enter` | Open directory |
| `Tab` | Switch between panes |
| `Delete` | Delete file/directory (with confirmation) |

**Mouse:**
| Action | Function |
|--------|----------|
| Single Click | Select file/folder |
| Double Click | Open directory/file |
| Right Click | Context menu (Properties, Rename, Copy, Cut, Compress, Extract, Delete) |
| Click ".." | Go directly to parent directory |
| Click Column Header | Sort by Name/Size/Date (toggle asc/desc) |
| Click Breadcrumb Path | Navigate to that directory segment |
| Hover on Item | Show hover effect |
| Hover on Button | Show tooltip |

**Menu Bar:**
- Files → Refresh, Toggle Hidden Files, Toggle Sidebar, Toggle Preview Panel, Themes, Exit
- Bookmarks → Add Current Folder, [List of saved bookmarks]
- Commands → Mount Points, New Folder (Ctrl+N), Rename (F2), Open (F3), Advanced Search (Ctrl+Shift+F), Compare Files (Ctrl+D), Copy (F5), Cut (F6), Paste (F7), Delete (F8)
- Help → About, Keyboard shortcuts

**Function Button Bar (Bottom):**
- 💾 F2 Refresh | 📋 F5 Copy | ✂ F6 Cut | 📎 F7 Paste | 🗑 F8 Delete | ✗ Exit

### Project Structure

```
dual-pane-fm/
├── src/
│   ├── bin/
│   │   ├── gui.rs      # GUI entry point (egui)
│   │   └── tui.rs      # TUI entry point (ratatui)
│   ├── app.rs          # TUI application state
│   ├── app_gui.rs      # GUI application state
│   ├── bookmarks.rs    # Bookmark manager & quick access
│   ├── pane.rs         # Panel logic & navigation
│   ├── filesystem.rs   # File system operations
│   ├── ui.rs           # TUI rendering
│   └── lib.rs          # Library exports
└── Cargo.toml
```

### Dependencies

**TUI Version:**
- `ratatui` - Library for TUI (Terminal User Interface)
- `crossterm` - Cross-platform terminal manipulation library

**GUI Version:**
- `egui` - Immediate mode GUI library
- `eframe` - Framework for running egui applications
- `egui_extras` - Extra widgets for egui
- `image` - Image loading and processing
- `zip` - ZIP archive compression and extraction
- `serde` & `serde_json` - Serialization for bookmarks storage

**Shared:**
- `anyhow` - Error handling
- `dirs` - Cross-platform home directory detection
- `chrono` - Date and time handling

### Support & Donation

If you enjoy using Filane and would like to support its development, please consider making a donation. Your support helps us continue improving and adding new features!

**Donate via PayPal:** https://www.paypal.com/paypalme/AchmadFachrie

Every contribution, no matter the amount, is greatly appreciated and helps keep this project alive and growing! ☕

### License

MIT
