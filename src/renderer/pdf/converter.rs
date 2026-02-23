use crate::renderer::pdf::loader::{PdfDocument, PdfError};
use hayro_svg::convert;

pub struct PdfToSvgConverter;

impl PdfToSvgConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_page(
        &self,
        document: &PdfDocument,
        page_index: usize,
    ) -> Result<String, PdfError> {
        let hayro_doc = document.inner();
        let page = hayro_doc
            .pages()
            .get(page_index)
            .ok_or(PdfError::PageNotFound(page_index))?;

        let svg_string = convert(page, &Default::default());

        Ok(svg_string)
    }
}

impl Default for PdfToSvgConverter {
    fn default() -> Self {
        Self::new()
    }
}
