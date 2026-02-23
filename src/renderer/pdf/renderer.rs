use crate::renderer::pdf::cache::SvgCache;
use crate::renderer::pdf::converter::PdfToSvgConverter;
use crate::renderer::pdf::loader::{PdfDocument, PdfError};

pub struct SkiaSvgRenderer {
    converter: PdfToSvgConverter,
    svg_cache: SvgCache,
}

impl SkiaSvgRenderer {
    pub fn new() -> Self {
        Self {
            converter: PdfToSvgConverter::new(),
            svg_cache: SvgCache::new(),
        }
    }

    pub fn load_pdf(&mut self, data: &[u8]) -> Result<PdfDocument, PdfError> {
        PdfDocument::load(data)
    }

    pub fn render_page(
        &mut self,
        document: &PdfDocument,
        page_index: usize,
        canvas: &skia_safe::Canvas,
        dest_rect: skia_safe::IRect,
    ) -> Result<(), PdfError> {
        let svg_string = self.converter.convert_page(document, page_index)?;

        let mut svg_dom = self.svg_cache.get_or_create(&svg_string, page_index)?;

        let rect = skia_safe::Rect::from_irect(dest_rect);

        canvas.save();
        canvas.clip_rect(rect, None, Some(false));

        // Use Dom's render method directly
        svg_dom.set_container_size((dest_rect.width() as f32, dest_rect.height() as f32));
        svg_dom.render(canvas);

        canvas.restore();

        Ok(())
    }

    #[allow(dead_code)]
    pub fn page_count(&self, document: &PdfDocument) -> usize {
        document.page_count()
    }

    #[allow(dead_code)]
    pub fn page_size(&self, document: &PdfDocument, page_index: usize) -> Option<(f32, f32)> {
        document.page_size(page_index)
    }
}

impl Default for SkiaSvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}
