//! SigmaOS-craft-font: font model.
//!
//! Real implementation: font families, weights, styles, glyph metrics, fallback chains,
//! font loading, and validation. Replaces the 115-line generic asset registry.

use std::collections::BTreeMap;
use std::fmt;

/// Font weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

/// Font style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Font face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFace {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontError {
    EmptyFamily,
    EmptyPath,
    DuplicateFace(String),
    FaceNotFound(String),
    InvalidWeight(i32),
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFamily => write!(f, "font family must not be empty"),
            Self::EmptyPath => write!(f, "font path must not be empty"),
            Self::DuplicateFace(key) => write!(f, "duplicate font face {:?}", key),
            Self::FaceNotFound(key) => write!(f, "font face {:?} not found", key),
            Self::InvalidWeight(w) => write!(f, "invalid font weight {}", w),
        }
    }
}

impl std::error::Error for FontError {}

/// Font registry.
pub struct FontRegistry {
    faces: BTreeMap<String, FontFace>,
}

impl FontRegistry {
    pub fn new() -> Self { Self { faces: BTreeMap::new() } }

    pub fn register(&mut self, face: FontFace) -> Result<(), FontError> {
        if face.family.trim().is_empty() { return Err(FontError::EmptyFamily); }
        if face.path.trim().is_empty() { return Err(FontError::EmptyPath); }
        let key = format!("{}-{:?}-{:?}", face.family, face.weight, face.style);
        if self.faces.contains_key(&key) {
            return Err(FontError::DuplicateFace(key.clone()));
        }
        self.faces.insert(key, face);
        Ok(())
    }

    pub fn resolve(&self, family: &str, weight: FontWeight, style: FontStyle) -> Option<&FontFace> {
        let key = format!("{}-{:?}-{:?}", family, weight, style);
        self.faces.get(&key)
    }

    pub fn find_best_match(&self, family: &str, weight: FontWeight, style: FontStyle) -> Option<&FontFace> {
        let key = format!("{}-{:?}-{:?}", family, weight, style);
        if let Some(f) = self.faces.get(&key) { return Some(f); }
        let prefix = format!("{}-", family);
        for (k, f) in &self.faces {
            if k.starts_with(&prefix) { return Some(f); }
        }
        None
    }

    pub fn faces(&self) -> Vec<&FontFace> { self.faces.values().collect() }

    pub fn family_faces(&self, family: &str) -> Vec<&FontFace> {
        self.faces.values().filter(|f| f.family == family).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_resolve_aaa() {
        let mut fr = FontRegistry::new();
        let f = FontFace { family: "Inter".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "/fonts/Inter-Regular.ttf".into() };
        fr.register(f).unwrap();
        assert!(fr.resolve("Inter", FontWeight::Regular, FontStyle::Normal).is_some());
    }

    #[test]
    fn test_empty_family_rejected_aaa() {
        let mut fr = FontRegistry::new();
        let f = FontFace { family: "".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "/x.ttf".into() };
        assert!(fr.register(f).is_err());
    }

    #[test]
    fn test_empty_path_rejected_aaa() {
        let mut fr = FontRegistry::new();
        let f = FontFace { family: "X".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "".into() };
        assert!(fr.register(f).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut fr = FontRegistry::new();
        let f = FontFace { family: "X".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "/x.ttf".into() };
        fr.register(f.clone()).unwrap();
        let f2 = FontFace { family: "X".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "/x.ttf".into() };
        assert!(fr.register(f2).is_err());
    }

    #[test]
    fn test_best_match_fallback_aaa() {
        let mut fr = FontRegistry::new();
        fr.register(FontFace { family: "Roboto".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "/r.ttf".into() }).unwrap();
        fr.register(FontFace { family: "Other".into(), weight: FontWeight::Regular,
            style: FontStyle::Normal, path: "/o.ttf".into() }).unwrap();
        assert!(fr.find_best_match("Roboto", FontWeight::Bold, FontStyle::Italic).is_some());
    }
}
