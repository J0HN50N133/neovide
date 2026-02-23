use std::sync::Arc;

pub use hayro::hayro_interpret::hayro_syntax::Pdf as HayroPdf;

pub struct PdfDocument {
    inner: HayroPdf,
}

impl PdfDocument {
    pub fn load(data: &[u8]) -> Result<Self, PdfError> {
        type PdfData = Arc<dyn AsRef<[u8]> + Send + Sync>;
        let pdf_data: PdfData = Arc::new(data.to_vec());
        let inner = HayroPdf::new(pdf_data).map_err(|e| PdfError::Load(format!("{:?}", e)))?;
        Ok(Self { inner })
    }

    #[allow(dead_code)]
    pub fn page_count(&self) -> usize {
        self.inner.len()
    }

    #[allow(dead_code)]
    pub fn page_size(&self, page_index: usize) -> Option<(f32, f32)> {
        self.inner.pages().get(page_index).map(|page| {
            let (width, height) = page.render_dimensions();
            (width, height)
        })
    }

    pub fn inner(&self) -> &HayroPdf {
        &self.inner
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("Failed to load PDF: {0}")]
    Load(String),
    #[error("Page not found: {0}")]
    PageNotFound(usize),
    #[error("SVG rendering error: {0}")]
    Svg(String),
}
