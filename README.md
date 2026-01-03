# Dual-Pane File Manager

File manager dengan Rust yang memiliki fitur dual-pane (2 panel) untuk navigasi file yang lebih efisien.

**Tersedia dalam 2 versi:**
- 🖥️ **GUI** - Aplikasi window grafis (egui) - cocok untuk Windows, Linux, macOS
- 📟 **TUI** - Terminal interface (ratatui) - cocok untuk terminal/SSH

## Fitur

### Core Features
- 🗂️ **Dual-Pane Interface** - Dua panel untuk navigasi file yang efisien
- 🏠 **Smart Start** - Otomatis buka home directory (Linux/macOS) atau current directory
- 📁 **Navigasi Direktori** - Buka dan jelajahi folder
- ⌨️ **Keyboard Navigation** - Kontrol penuh dengan keyboard
- 🖱️ **Mouse Support** (GUI) - Klik untuk navigasi
- 📋 **Copy/Cut/Paste** - Salin atau pindahkan file antar panel
- 🗑️ **Delete File** - Hapus file atau direktori (dengan konfirmasi di GUI)
- 🔄 **Refresh** - Muat ulang konten direktori
- 👁️ **Hidden Files Toggle** - Show/hide hidden files (Ctrl+H)

### Advanced Features (GUI)
- 🔤 **Column Sorting** - Klik header untuk sort by Name, Size, atau Date (ascending/descending)
  - Visual indicator (▲▼) menunjukkan sort direction
  - Directories always first saat sort by Name
  - Instant sorting tanpa reload
- 👁️ **Dual Preview Panel** - Preview files dari BOTH panes side-by-side di bawah panes (toggle dengan Space)
  - **Side-by-Side:** Left preview (left pane), Right preview (right pane) - perfect untuk compare
  - **Text Files:** Preview 20 baris pertama dengan syntax highlighting (txt, md, rs, toml, json, xml, html, css, js, py, c, cpp, h, sh, yaml)
  - **Images:** Thumbnail preview dengan auto-scaling (PNG, JPG, GIF, BMP, ICO, WEBP)
  - **PDF Files:** Preview dengan file info (name, size, date) - 📋 indicator ← New!
  - **File Info:** Tampilan detail (name, size, date, permissions) untuk file lain
  - Auto-update saat navigasi dengan arrow keys atau click di salah satu pane
  - Split screen design (200px height total) tidak mengganggu workflow
  - Ideal untuk perbandingan file, code review, document comparison
- 📅 **Modified Date Column** - Tampilan tanggal modifikasi file (YYYY-MM-DD HH:MM)
- 🧭 **Breadcrumb Navigation** - Klik path segment untuk quick navigate
- ⬅️➡️ **Navigation History** - Back/Forward buttons dengan keyboard shortcuts (Alt+Left/Right)
  - Otomatis track navigation history (max 50 locations)
  - Visual indicator (disabled state) saat tidak ada history
  - Tooltips menjelaskan fungsi
- 🖼️ **Built-in Image Viewer** - Preview gambar langsung di aplikasi
  - Support: PNG, JPG, JPEG, GIF, BMP, ICO, WEBP
  - Zoom in/out dengan mouse wheel atau buttons (10% - 1000%)
  - Pan/drag untuk navigate gambar besar
  - Reset view dengan satu klik
  - Keyboard shortcuts (ESC untuk close)
  - Tampilan info ukuran gambar dan zoom level
- 🎨 **Theme Customization** - 6 built-in themes untuk personalisasi tampilan
  - Dark (Default) - Modern dark theme
  - Light - Clean light theme
  - Dracula - Popular purple theme
  - Nord - Arctic, north-bluish theme
  - Monokai - Classic code editor theme
  - Solarized Dark - Precision colors theme
- 🔍 **Quick Filter** (Ctrl+F) - Filter file secara realtime by nama
- 🔎 **Advanced Search** (Ctrl+Shift+F) - Powerful search dengan multiple criteria
  - **File name pattern:** Wildcard search (*.rs, document*, photo*)
  - **Content search:** Cari text di dalam file
  - **File type filter:** All, Files only, atau Directories only
  - **Size range:** Min/Max size dalam KB
  - **Date filter:** Modified dalam X hari terakhir
  - **Options:** Case sensitive, include hidden files
  - **Results:** Clickable list untuk navigate ke file location
  - **Recursive:** Otomatis search di semua subdirectories
- 📁 **New Folder** (Ctrl+N) - Buat folder baru dengan dialog
- ✏️ **Rename** (F2) - Rename file/folder dengan dialog
- ⚖️ **File Comparison** (Ctrl+D) - Compare 2 files side-by-side dengan diff viewer
  - **Visual diff:** Line-by-line comparison dengan color coding
  - **Statistics:** Show equal, added, removed, modified lines count
  - **Color legend:** Green (added), Red (removed), Yellow (modified), Gray (equal)
  - **Line numbers:** Display line numbers untuk both files
  - **Identical detection:** Instant detection untuk identical files
  - **Text files only:** Otomatis detect dan compare text files
  - **Scrollable view:** Large files dengan smooth scrolling
- ℹ️ **Properties** - Lihat detail file (type, size, date, permissions, path) via context menu
  - Unix/Linux: Symbolic (rwxr-xr-x) + Octal (755) + Human-readable descriptions
    - Owner: Read, Write, Execute
    - Group: Read, Execute
    - Others: Read, Execute
  - Windows: Read-only / Read-Write dengan deskripsi
- 🔄 **Quick Transfer Buttons** - Tombol panah di tengah untuk copy/move antar pane dengan mudah
  - ➡📋 Copy Left → Right (biru)
  - ⬅📋 Copy Right → Left (biru)
  - ➡✂ Move Left → Right (orange)
  - ⬅✂ Move Right → Left (orange)
- 🗃️ **Archive Support** - Compress dan extract file archive
  - **Compress:** Buat ZIP archive dari file/folder (context menu → "Compress to ZIP")
  - **Extract:** Extract ZIP archive (context menu → "Extract ZIP")
  - Cross-platform support (Windows, Linux, macOS)
  - Progress feedback di status bar
- 💡 **Tooltips** - Hover pada button untuk melihat fungsinya
- 📋 **Smart Clipboard** - Visual indicator saat ada item di clipboard
- 📊 **Enhanced Status Bar** - Detailed statistics per pane
  - Item count (total, folders, files)
  - Total size of all files
  - Example: `Left: 15 items (3 folders, 12 files) • 45.2 MB`
- 🔷 **Git Integration** - Visual git status indicators untuk developers
  - **Status Icons:** M (Modified), A (Added), D (Deleted), ? (Untracked)
  - **Color Coding:** Yellow (modified), Green (added), Red (deleted), Purple (untracked)
  - **Branch Info:** Current branch name di status bar
  - **Ahead/Behind:** Show commits ahead/behind remote (↑2 ↓1)
  - **Change Indicator:** Diamond icon shows if repo has uncommitted changes
- 💾 **Mount Points Viewer** - Lihat dan akses semua mount points/storage devices
  - **Auto-detection:** Deteksi otomatis semua mount points di sistem
  - **Device Info:** Tampilkan device name, filesystem type, dan disk type (HDD/SSD/USB)
  - **Storage Usage:** Real-time monitoring dengan progress bar dan color coding
    - Hijau: < 70% (Normal)
    - Kuning: 70-90% (Warning)
    - Merah: > 90% (Critical)
  - **Quick Navigation:** Klik "Open" untuk langsung navigate ke mount point
  - **USB Detection:** Identifikasi USB drives dan external hard disks dengan icon 🔌
  - **Cross-platform:** Support Linux, macOS, dan Windows
  - **Keyboard Shortcut (TUI):** Press 'm' untuk toggle mount points dialog
  - **Menu Access (GUI):** Commands → Mount Points
  - **Auto-detect:** Otomatis detect git repository di current directory
  - **Performance:** Lightweight, tidak slow down file browsing
- ⭐ **Bookmarks & Sidebar** - Quick access navigation dengan sidebar di kiri
   - **Sidebar Panel:** Toggle dengan Ctrl+B, menu Files, atau floating button (◀/▶)
   - **Floating Toggle Button:** Always-visible button di edge untuk show/hide sidebar
   - **Expand/Collapse Sections:** Each section (Quick Access, Bookmarks, Devices) dapat di-collapse dengan ▼/▶
   - **Left-Aligned Items:** Clean layout dengan items rata kiri
   - **Quick Access:** Home, Documents, Downloads, Pictures, Music, Videos, Desktop, **Trash** ← New!
   - **Bookmarks:** Save favorite folders untuk akses cepat
     - Add bookmark: Bookmarks menu → Add Current Folder atau klik ➕
     - Remove bookmark: Klik ✗ di samping bookmark
     - Persistent storage: Saved di ~/.config/dual-pane-fm/bookmarks.json
   - **Devices:** Quick access ke mount points dan USB drives
   - **One-click Navigation:** Klik item untuk navigate instantly
   - **Visual Organization:** Clear sections dengan expand/collapse controls
- 🗑️ **Trash/Recycle Bin** - Browse dan manage deleted files
   - **Quick Access:** 🗑 Trash button di Quick Access section sidebar
   - **Browse Deleted Files:** View all files yang sudah dihapus
   - **Restore:** Copy files dari trash ke lokasi lain (F5 Copy → navigate → F7 Paste)
   - **Permanent Delete:** Hapus files selamanya dari trash (F8 or Delete)
   - **Cross-Platform:** Linux (.local/share/Trash/files atau .Trash), macOS (.Trash)
   - **Easy Access:** One-click navigate ke trash dari sidebar
- 🎯 **Visual Feedback** - Hover effects, shadows, rounded corners, disabled states

## Instalasi & Menjalankan

### Quick Install (Linux)

**Automatic Installation:**
```bash
# Clone repository
git clone https://github.com/yourusername/dual-pane-fm.git
cd dual-pane-fm

# Run installation script (builds and installs to system)
./install.sh
```

Setelah instalasi:
- ✅ Binary tersedia di `/usr/local/bin/dual-pane-fm-gui`
- ✅ Icon tersedia di system icons
- ✅ Desktop entry di application menu
- ✅ Bisa dijalankan dari terminal: `dual-pane-fm-gui`
- ✅ Bisa diluncurkan dari application menu

**Uninstall:**
```bash
./uninstall.sh
```

---

### Manual Build & Run

#### GUI Version (Grafis Window)

```bash
# Masuk ke direktori project
cd dual-pane-fm

# Build GUI version
cargo build --release --bin dual-pane-fm-gui

# Jalankan GUI
cargo run --release --bin dual-pane-fm-gui

# Atau langsung dari binary
./target/release/dual-pane-fm-gui
```

#### TUI Version (Terminal)

```bash
# Build TUI version
cargo build --release --bin dual-pane-fm-tui

# Jalankan TUI
cargo run --release --bin dual-pane-fm-tui

# Atau langsung dari binary
./target/release/dual-pane-fm-tui
```

## Keyboard Shortcuts

### TUI Version (Terminal)

| Tombol | Fungsi |
|--------|--------|
| `↑` / `k` | Pindah ke atas |
| `↓` / `j` | Pindah ke bawah |
| `Enter` | Buka direktori |
| `Tab` | Pindah antar panel |
| `c` | Copy file dari panel aktif ke panel lain |
| `d` / `Delete` | Hapus file/direktori yang dipilih |
| `r` | Refresh panel |
| `m` | Toggle mount points dialog |
| `q` | Keluar |

### GUI Version (Window) - Total Commander Style

**Function Keys (Total Commander style):**
| Tombol | Fungsi |
|--------|--------|
| `F2` | Rename file/folder |
| `F3` | Open file dengan default app |
| `F5` | Copy file ke clipboard |
| `F6` | Cut/Move file ke clipboard |
| `F7` | Paste file dari clipboard |
| `F8` | Delete file/direktori |
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
| Tombol | Fungsi |
|--------|--------|
| `↑` / `↓` | Navigasi atas/bawah |
| `Enter` | Buka direktori |
| `Tab` | Pindah antar panel |
| `Delete` | Hapus file/direktori (dengan konfirmasi) |

**Mouse:**
| Aksi | Fungsi |
|------|--------|
| Single Click | Select file/folder |
| Double Click | Buka direktori/file |
| Right Click | Context menu (Properties, Rename, Copy, Cut, Compress, Extract, Delete) |
| Click pada ".." | Langsung naik ke parent directory |
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

## Struktur Project

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

## Dependencies

**TUI Version:**
- `ratatui` - Library untuk TUI (Terminal User Interface)
- `crossterm` - Library untuk cross-platform terminal manipulation

**GUI Version:**
- `egui` - Immediate mode GUI library
- `eframe` - Framework untuk menjalankan egui aplikasi
- `egui_extras` - Extra widgets untuk egui
- `image` - Image loading dan processing
- `zip` - ZIP archive compression dan extraction
- `serde` & `serde_json` - Serialization untuk bookmarks storage

**Shared:**
- `anyhow` - Error handling yang lebih mudah
- `dirs` - Cross-platform home directory detection
- `chrono` - Date and time handling

## Screenshot

### TUI Version (Terminal)
```
┌─────────────────────────────────────────────────────────────────────────┐
│ Dual-Pane File Manager - Tab: Switch Panes | Enter: Open | q: Quit     │
├─────────────────────────────────┬───────────────────────────────────────┤
│ /home/user/documents            │ /home/user/downloads                  │
│ 📁 ..                    <DIR>  │ 📁 ..                    <DIR>        │
│ 📁 folder1               <DIR>  │ 📄 file1.txt             1.23 KB      │
│ 📄 document.pdf          2.5 MB │ 📄 archive.zip           10.5 MB      │
└─────────────────────────────────┴───────────────────────────────────────┘
│ Selected: document.pdf | Size: 2.5 MB | Path: /home/user/documents/...  │
└─────────────────────────────────────────────────────────────────────────┘
```

### GUI Version (Total Commander Style)
```
┌─────────────────────────────────────────────────────────────────────────┐
│ Files  Commands  Help                                                   │
├───────────────────────────────────┬─────────────────────────────────────┤
│ ╔════════════════════════════════╗ │ ╔═════════════════════════════════╗ │  ← Active (blue)
│ ║ 💾 /home/user/documents     ║ │ ║ 💾 /home/user/downloads      ║ │
│ ╠────────────────────────────────╣ │ ╠─────────────────────────────────╣ │
│ ║ Name            Size       ║ │ ║ Name            Size       ║ │
│ ╠────────────────────────────────╣ │ ╠─────────────────────────────────╣ │
│ ║ 📁 ..          <DIR>      ║ │ ║ 📁 ..          <DIR>      ║ │
│ ░ 📁 folder1     <DIR>      ║ │ ║ 📄 file1.txt    1.23 KB    ║ │  ← Selected
│ ║ 📄 document.pdf 2.5 MB     ║ │ ░ 📄 archive.zip  10.5 MB    ║ │     (blue bg)
│ ░ 📄 readme.txt   5.2 KB     ║ │ ║ 📄 photo.jpg    3.1 MB     ║ │
│ ╚════════════════════════════════╝ │ ╚═════════════════════════════════╝ │
├──────────────────────────────────┴─────────────────────────────────────┤
│ [ 💾 F2 Refresh ] [ 📋 F5 Copy ] [ 🗑 F8 Delete ]       [ ✗ Exit ] │
├─────────────────────────────────────────────────────────────────────────┤
│ 📊 Left: 4 items | Right: 5 items | Selected: folder1         Ready │
└─────────────────────────────────────────────────────────────────────────┘
```

## Catatan

**TUI Version:**
- Panel aktif ditandai dengan border berwarna hijau
- File yang dipilih ditandai dengan highlight hijau
- Operasi copy hanya bekerja untuk file (bukan direktori)
- Operasi delete akan menghapus direktori secara rekursif

**GUI Version (Modern & User-Friendly):**
- ✨ **Modern UI Design:**
  - Panel aktif ditandai dengan border biru tebal
  - File yang dipilih dengan background biru untuk panel aktif, gray untuk inactive
  - Alternating row colors (zebra striping) untuk readability
  - Hover effects pada semua interactive elements
  - Tooltips pada semua buttons untuk guidance
  - Smooth rounded corners dan shadows
  
- 🎯 **Enhanced User Experience:**
  - **Quick Transfer Buttons:** Tombol panah di tengah antara pane untuk transfer file dengan mudah
    - ➡📋 Copy Left → Right (tombol biru)
    - ⬅📋 Copy Right → Left (tombol biru)
    - ➡✂ Move Left → Right (tombol orange)
    - ⬅✂ Move Right → Left (tombol orange)
    - Otomatis disabled jika tidak ada item yang dipilih
    - Tooltips menjelaskan fungsi dan requirement
  - **Smart Clipboard:** Visual indicator saat ada item di clipboard
  - **Paste Button:** Disabled state saat clipboard kosong dengan tooltip explanation
  - **Status Bar:** Real-time info tentang item count, clipboard status, dan operations
  - **Dialogs:** Modern, centered dialogs untuk New Folder, Rename, Delete, Properties, About
  - **ESC Key:** Universal close untuk semua dialogs
  
- 🖱️ **Mouse Support:**
  - Single click: Select file/folder
  - Double click: Buka direktori/file dengan aplikasi default
  - Right click: Context menu lengkap (Properties, Rename, Copy, Cut, Compress, Extract, Delete)
  - Click ".." sekali: Naik ke parent directory
  - Click column header: Sort dengan visual indicator (▲▼)
  - Click breadcrumb: Quick navigation ke parent directories
  
- 📋 **Context Menu Features:**
  - Properties: Lihat detail file (type, size, date, permissions, path)
  - Rename: Rename file/folder dengan dialog
  - Copy/Cut: Clipboard operations
  - **Compress:** Buat ZIP archive dari file/folder (cross-platform)
  - **Extract:** Extract ZIP archive (cross-platform)
  - Delete: Dengan konfirmasi dialog
  
- ⌨️ **Keyboard Shortcuts:**
  - F2: Rename | F3: Open | F5: Copy | F6: Cut | F7: Paste | F8: Delete
  - Ctrl+N: New Folder | Ctrl+H: Toggle Hidden | Ctrl+F: Filter
  - ESC: Close dialogs | Tab: Switch panes
  
- 🎨 **Visual Feedback:**
  - Color-coded file type icons (🦀 Rust, 🐍 Python, 📦 Archives, dll)
  - Disabled states untuk unavailable actions
  - Loading states dan error messages
  - Breadcrumb navigation dengan clickable segments
  
- 🌐 **Cross-platform:** Windows, Linux, macOS

## License

MIT
