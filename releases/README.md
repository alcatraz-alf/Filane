# Filane - Dual Pane FM Releases

This folder contains pre-built AppImage releases of Filane for Linux.

## Latest Release

**Version:** 0.1.0  
**Date:** 2026-01-03

### Downloads

- **Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage** (8.5 MB)

## Quick Start

### Download & Run

```bash
# Download
wget https://github.com/yourusername/dual-pane-fm/raw/main/releases/Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage

# Make executable
chmod +x Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage

# Run
./Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage
```

### Install System-Wide

```bash
# Create directory
sudo mkdir -p /opt/filane

# Copy AppImage
sudo cp Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage /opt/filane/
sudo chmod +x /opt/filane/Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage

# Create symlink
sudo ln -sf /opt/filane/Filane-Dual_Pane_File_Manager-0.1.0-x86_64.AppImage /usr/local/bin/filane-gui

# Run from terminal
filane-gui
```

## System Requirements

- **OS:** Linux (x86_64)
- **Architecture:** 64-bit
- **Compatibility:** Most Linux distributions (Ubuntu, Debian, Fedora, Arch, etc.)

## Features

✨ **Dual-Pane Interface** - Two panels for efficient file navigation  
🔍 **Advanced Search** - Powerful search with multiple criteria  
🖼️ **Image Viewer** - Built-in image preview and zoom  
📄 **File Comparison** - Compare text files side-by-side  
🎨 **Theme Customization** - 6 built-in themes  
📋 **Archive Support** - Compress and extract ZIP files  
🔷 **Git Integration** - Visual git status indicators  
⭐ **Bookmarks & Sidebar** - Quick access navigation  
🗑️ **Trash Support** - Browse and manage deleted files  

For more information, visit the [main README](../README.md)

## Changelog

See [CHANGELOG.md](../CHANGELOG.md) for detailed version history.

## Support

If you find any issues or have suggestions, please open an issue on GitHub.

### Donate

Support development: https://www.paypal.com/paypalme/AchmadFachrie

---

**Last Updated:** 2026-01-03
