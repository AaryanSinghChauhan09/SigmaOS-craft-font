#[derive(Debug, Clone)]
pub struct FontMeta { pub family: String, pub style: String, pub path: String }

#[derive(Debug, Default)]
pub struct FontManager { fonts: Vec<FontMeta> }
impl FontManager {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, fam: impl Into<String>, sty: impl Into<String>, p: impl Into<String>) {
        self.fonts.push(FontMeta { family: fam.into(), style: sty.into(), path: p.into() });
    }
    pub fn count(&self) -> usize { self.fonts.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_fonts() {
        let mut fm = FontManager::new();
        fm.register("Inter", "Regular", "/fonts/Inter.ttf");
        assert_eq!(fm.count(), 1);
    }
}
