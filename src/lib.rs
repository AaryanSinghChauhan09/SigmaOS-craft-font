// SPDX-License-Identifier: MIT
//! SigmaOS-craft-font — Real OpenType Typography, Metrics, and Font Registry Core
//!
//! Principles:
//! - NO STUBS: OpenType metrics, font face registry, and glyph shaping foundations
//! - ARRANGE-ACT-ASSERT: Comprehensive unit and integration test suites

#![forbid(unsafe_code)]
#![deny(clippy::all)]

/// Font management module.
pub mod sigmafont;

/// Font weight values following CSS/OpenType specifications.
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

impl FontWeight {
    /// Return the numerical OpenType weight.
    pub const fn as_u16(&self) -> u16 {
        *self as u16
    }
}

/// Font posture/style variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

/// Fundamental font vertical metrics defined in font units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontMetrics {
    pub units_per_em: u16,
    pub ascender: i16,
    pub descender: i16,
    pub line_gap: i16,
}

impl FontMetrics {
    /// Recommended line height in font units.
    pub fn line_height(&self) -> i32 {
        (self.ascender as i32) - (self.descender as i32) + (self.line_gap as i32)
    }

    /// Ratio of ascender to total units per EM.
    pub fn cap_height_ratio(&self) -> f64 {
        if self.units_per_em == 0 {
            0.0
        } else {
            (self.ascender as f64) / (self.units_per_em as f64)
        }
    }
}

/// Glyph representation with unicode codepoint, advance width, and left-side bearing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glyph {
    pub codepoint: u32,
    pub advance_width: u16,
    pub lsb: i16,
}

impl Glyph {
    /// Return the corresponding Unicode char, if valid.
    pub fn char(&self) -> Option<char> {
        char::from_u32(self.codepoint)
    }
}

/// Typography font representation with metrics and glyph inventory.
#[derive(Debug, Clone, PartialEq)]
pub struct Font {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub metrics: FontMetrics,
    pub glyphs: Vec<Glyph>,
}

impl Font {
    /// Create a new font instance.
    pub fn new(family: &str, weight: FontWeight, style: FontStyle, metrics: FontMetrics) -> Self {
        Self {
            family: family.to_string(),
            weight,
            style,
            metrics,
            glyphs: Vec::new(),
        }
    }

    /// Add a glyph to the font.
    pub fn add_glyph(&mut self, g: Glyph) {
        self.glyphs.push(g);
    }

    /// Total number of loaded glyphs.
    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }

    /// Find a glyph by its Unicode codepoint.
    pub fn find_glyph(&self, codepoint: u32) -> Option<&Glyph> {
        self.glyphs.iter().find(|g| g.codepoint == codepoint)
    }
}

/// Individual font face descriptor for system font matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFace {
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub path: String,
}

/// Global system font face registry and matching engine.
#[derive(Debug, Clone, Default)]
pub struct FontRegistry {
    pub faces: Vec<FontFace>,
}

impl FontRegistry {
    /// Create an empty font registry.
    pub fn new() -> Self {
        Self { faces: Vec::new() }
    }

    /// Register a new font face.
    pub fn register(&mut self, face: FontFace) -> Result<(), String> {
        if face.family.is_empty() {
            return Err("Family name cannot be empty".to_string());
        }
        self.faces.push(face);
        Ok(())
    }

    /// Retrieve all font faces matching a given family.
    pub fn family_faces(&self, family: &str) -> Vec<&FontFace> {
        self.faces
            .iter()
            .filter(|f| f.family.eq_ignore_ascii_case(family))
            .collect()
    }

    /// Find the best matching font face based on family, target weight, and style.
    pub fn find_best_match(
        &self,
        family: &str,
        target_weight: FontWeight,
        target_style: FontStyle,
    ) -> Option<&FontFace> {
        let matching: Vec<&FontFace> = self.family_faces(family);
        if matching.is_empty() {
            return None;
        }

        // Try exact style match first, then closest weight
        let style_matched: Vec<&FontFace> = matching
            .iter()
            .copied()
            .filter(|f| f.style == target_style)
            .collect();

        let pool = if style_matched.is_empty() {
            matching
        } else {
            style_matched
        };

        pool.into_iter()
            .min_by_key(|f| (f.weight.as_u16() as i32 - target_weight.as_u16() as i32).abs())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_weight_value() {
        // Arrange & Act
        let w_reg = FontWeight::Regular;
        let w_bold = FontWeight::Bold;
        // Assert
        assert_eq!(w_reg.as_u16(), 400);
        assert_eq!(w_bold.as_u16(), 700);
    }

    #[test]
    fn test_line_height() {
        // Arrange: 1000 upm, ascender=800, descender=-200, line_gap=100
        let m = FontMetrics {
            units_per_em: 1000,
            ascender: 800,
            descender: -200,
            line_gap: 100,
        };
        // Act: 800 - (-200) + 100 = 1100
        let lh = m.line_height();
        // Assert
        assert_eq!(lh, 1100);
    }

    #[test]
    fn test_cap_height_ratio() {
        // Arrange
        let m = FontMetrics {
            units_per_em: 1000,
            ascender: 750,
            descender: -250,
            line_gap: 0,
        };
        // Act
        let ratio = m.cap_height_ratio();
        // Assert
        assert!((ratio - 0.75).abs() < 1e-6);
    }

    #[test]
    fn test_glyph_char() {
        // Arrange: Unicode codepoint 0x41 ('A')
        let g = Glyph {
            codepoint: 0x41,
            advance_width: 600,
            lsb: 20,
        };
        // Act & Assert
        assert_eq!(g.char(), Some('A'));
    }

    #[test]
    fn test_font_glyph_count() {
        // Arrange
        let m = FontMetrics {
            units_per_em: 1000,
            ascender: 800,
            descender: -200,
            line_gap: 0,
        };
        let mut font = Font::new("Inter", FontWeight::Regular, FontStyle::Normal, m);
        font.add_glyph(Glyph {
            codepoint: 65,
            advance_width: 500,
            lsb: 10,
        });
        // Act & Assert
        assert_eq!(font.glyph_count(), 1);
    }

    #[test]
    fn test_find_glyph_found() {
        // Arrange
        let m = FontMetrics {
            units_per_em: 1000,
            ascender: 800,
            descender: -200,
            line_gap: 0,
        };
        let mut font = Font::new("Inter", FontWeight::Regular, FontStyle::Normal, m);
        font.add_glyph(Glyph {
            codepoint: 0x20AC, // Euro symbol
            advance_width: 550,
            lsb: 15,
        });
        // Act
        let res = font.find_glyph(0x20AC);
        // Assert
        assert!(res.is_some());
        assert_eq!(res.unwrap().advance_width, 550);
    }

    #[test]
    fn test_find_glyph_not_found() {
        // Arrange
        let m = FontMetrics {
            units_per_em: 1000,
            ascender: 800,
            descender: -200,
            line_gap: 0,
        };
        let font = Font::new("Inter", FontWeight::Regular, FontStyle::Normal, m);
        // Act & Assert
        assert!(font.find_glyph(9999).is_none());
    }
}
