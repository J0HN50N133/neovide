use std::collections::HashMap;

use skia_safe::svg::Dom;

use super::loader::PdfError;

pub struct SvgCache {
    cache: HashMap<String, Dom>,
}

impl SvgCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get_or_create(&mut self, svg_string: &str, _page_index: usize) -> Result<Dom, PdfError> {
        let key = svg_string.to_string();

        // Return clone if already exists
        if let Some(svg_dom) = self.cache.get(&key) {
            return Ok(svg_dom.clone());
        }

        // Create new entry
        let data = skia_safe::Data::new_copy(svg_string.as_bytes());
        let font_mgr = skia_safe::FontMgr::default();
        let svg_dom =
            Dom::read(data.as_bytes(), font_mgr).map_err(|e| PdfError::Svg(format!("{:?}", e)))?;

        // Store and return clone
        let result = svg_dom.clone();
        self.cache.insert(key, svg_dom);

        Ok(result)
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for SvgCache {
    fn default() -> Self {
        Self::new()
    }
}
