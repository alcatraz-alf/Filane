use std::path::PathBuf;
use pdfium_render::prelude::*;

/// Render PDF first page to DynamicImage
/// Returns error if PDFium is not available (fallback to metadata-only preview)
pub fn render_pdf_page(path: &PathBuf) -> Result<image::DynamicImage, String> {
    // Try to get system PDFium library
    let bindings = match Pdfium::bind_to_system_library() {
        Ok(b) => b,
        Err(_) => {
            // PDFium not available - return error for fallback to metadata-only preview
            return Err("PDFium library not available - using metadata-only preview".to_string());
        }
    };

    let pdfium = Pdfium::new(bindings);

    // Load PDF document
    let document = pdfium
        .load_pdf_from_file(path, None)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    // Get pages and check if has any
    let pages = document.pages();
    if pages.len() == 0 {
        return Err("PDF has no pages".to_string());
    }

    // Get first page
    let page = pages
        .get(0)
        .map_err(|e| format!("Failed to get first page: {}", e))?;

    // Render page to image using PdfRenderConfig
    let render_config = PdfRenderConfig::new()
        .set_target_width(1024)
        .set_maximum_height(1024);

    let bitmap = page
        .render_with_config(&render_config)
        .map_err(|e| format!("Failed to render page: {}", e))?;

    // Convert bitmap to DynamicImage by cloning the image reference
    Ok(bitmap.as_image().clone())
}

/// Get page count from PDF
pub fn get_page_count(path: &PathBuf) -> Result<usize, String> {
    let bindings = Pdfium::bind_to_system_library()
        .map_err(|e| format!("Failed to load PDFium: {}", e))?;

    let pdfium = Pdfium::new(bindings);

    let document = pdfium
        .load_pdf_from_file(path, None)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;

    Ok(document.pages().len() as usize)
}

