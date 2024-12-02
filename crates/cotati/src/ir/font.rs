use super::{FrameVariable, PathEvent};

/// Reliable delivery of fonts is a requirement for SVG. Designers need to create SVG content with arbitrary
/// fonts and know that the same graphical result will appear when the content is viewed by all end users,
/// even when end users do not have the necessary fonts installed on their computers. This parallels the print
/// world, where the designer uses a given font when authoring a drawing for print, and the graphical content
/// appears exactly the same in the printed version as it appeared on the designer's authoring system.
///
/// SVG utilizes the WebFonts facility defined in CSS2 ([CSS2], section 15.1) as a key mechanism for reliable
/// delivery of font data to end users. In a common scenario, SVG authoring applications generate compressed,
/// subsetted WebFonts for all text elements used by a given SVG document fragment. Typically, the WebFonts
/// are saved in a location relative to the referencing document.
///
/// One disadvantage to the WebFont facility to date is that specifications such as CSS2 do not require support
/// of particular font formats. The result is that different implementations support different Web font formats,
/// thereby making it difficult for Web site creators to post a single Web site using WebFonts that work across
/// all user agents.
///
/// To provide a common font format for SVG that is guaranteed to be supported by all conforming SVG viewers,
/// SVG provides a facility to define fonts in SVG. This facility is called SVG fonts.
///
/// SVG fonts can improve the semantic richness of graphics that represent text. For example, many company
/// logos consist of the company name drawn artistically. In some cases, accessibility may be enhanced by expressing
/// the logo as a series of glyphs in an SVG font and then rendering the logo as a ‘text’ element which references
/// this font.9
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    /// The X-coordinate in the font coordinate system of the origin of a glyph to be used when drawing horizontally
    /// oriented text. (Note that the origin applies to all glyphs in the font.)
    ///
    /// If the attribute is not specified, the effect is as if a value of '0' were specified.
    pub hoiz_origin_x: f32,
    /// The Y-coordinate in the font coordinate system of the origin of a glyph to be used when drawing horizontally
    /// oriented text. (Note that the origin applies to all glyphs in the font.)
    ///
    /// If the attribute is not specified, the effect is as if a value of '0' were specified.
    pub hoiz_origin_y: f32,
    /// The default horizontal advance after rendering a glyph in horizontal orientation. Glyph widths are required
    /// to be non-negative, even if the glyph is typically rendered right-to-left, as in Hebrew and Arabic scripts.
    pub hoiz_adv_x: f32,

    /// The default X-coordinate in the font coordinate system of the origin of a glyph to be used when drawing
    /// vertically oriented text.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to half of the effective
    /// value of attribute ‘horiz-adv-x’.
    pub vert_origin_x: f32,
    /// The default Y-coordinate in the font coordinate system of the origin of a glyph to be used when drawing
    /// vertically oriented text.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the position specified by
    /// the font's ‘ascent’ attribute.
    pub vert_origin_y: f32,
    /// The default vertical advance after rendering a glyph in vertical orientation.
    ///
    /// If the attribute is not specified, the effect is as if a value equivalent of one em were specified
    /// (see ‘units-per-em’).
    pub vert_adv_x: f32,
}

/// See [`orientation`](Glyph::orientation) property
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GlyphOrientation {
    Horizontal,
    Vertical,
}

impl FrameVariable for GlyphOrientation {}

/// See [`arabic_form`](Glyph::arabic_form) property
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GlyphArabicForm {
    Initial,
    Medial,
    Termial,
    Isolated,
}

impl FrameVariable for GlyphArabicForm {}

/// The ‘glyph’ element defines the graphics for a given glyph. The coordinate system for the glyph is defined by the
/// various attributes in the ‘font’ element.
///
/// See [`glyph`](https://www.w3.org/TR/SVG11/fonts.html#GlyphElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Glyph {
    /// One or more Unicode characters indicating the sequence of Unicode characters which corresponds to this glyph.
    pub unicode: Option<String>,

    /// A name for the glyph. It is recommended that glyph names be unique within a font. The glyph names can be used
    /// in situations where Unicode character numbers do not provide sufficient information to access the correct glyph,
    /// such as when there are multiple glyphs per Unicode character. The glyph names can be referenced in kerning
    /// definitions.
    pub names: Vec<String>,

    /// The definition of the outline of a shape.
    pub path_data: Vec<PathEvent>,

    /// Indicates that the given glyph is only to be used for a particular inline-progression-direction
    /// (i.e., horizontal or vertical). If the attribute is not specified, then the glyph can be used in
    /// all cases (i.e., both horizontal and vertical inline-progression-direction).
    pub orientation: Option<GlyphOrientation>,

    /// For Arabic glyphs, indicates which of the four possible forms this glyph represents.
    pub arabic_form: Option<GlyphArabicForm>,

    /// The attribute value is a comma-separated list of language names as defined in BCP 47 [BCP47].
    pub lang: String,

    /// The horizontal advance after rendering the glyph in horizontal orientation. If the attribute is not specified,
    /// the effect is as if the attribute were set to the value of the font's ‘horiz-adv-x’ attribute.
    ///
    /// Glyph widths are required to be non-negative, even if the glyph is typically rendered right-to-left, as in
    /// Hebrew and Arabic scripts.
    pub hoiz_adv_x: f32,

    /// The X-coordinate in the font coordinate system of the origin of the glyph to be used when drawing vertically oriented text.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the value of the font's ‘vert-origin-x’ attribute.
    pub vert_origin_x: f32,
    /// The Y-coordinate in the font coordinate system of the origin of a glyph to be used when drawing vertically oriented text.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the value of the font's ‘vert-origin-y’ attribute.
    pub vert_origin_y: f32,
    /// The vertical advance after rendering a glyph in vertical orientation.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the value of the font's ‘vert-adv-y’ attribute.
    pub vert_adv_x: f32,
}

/// The ‘missing-glyph’ element defines the graphics to use if there is an attempt to draw a glyph from a given font and the
/// given glyph has not been defined. The attributes on the ‘missing-glyph’ element have the same meaning as the corresponding
/// attributes on the ‘glyph’ element.

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MissingGlyph {
    /// The definition of the outline of a shape.
    pub path_data: Vec<PathEvent>,

    /// The horizontal advance after rendering the glyph in horizontal orientation. If the attribute is not specified,
    /// the effect is as if the attribute were set to the value of the font's ‘horiz-adv-x’ attribute.
    ///
    /// Glyph widths are required to be non-negative, even if the glyph is typically rendered right-to-left, as in
    /// Hebrew and Arabic scripts.
    pub hoiz_adv_x: f32,

    /// The X-coordinate in the font coordinate system of the origin of the glyph to be used when drawing vertically oriented text.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the value of the font's ‘vert-origin-x’ attribute.
    pub vert_origin_x: f32,
    /// The Y-coordinate in the font coordinate system of the origin of a glyph to be used when drawing vertically oriented text.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the value of the font's ‘vert-origin-y’ attribute.
    pub vert_origin_y: f32,
    /// The vertical advance after rendering a glyph in vertical orientation.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the value of the font's ‘vert-adv-y’ attribute.
    pub vert_adv_x: f32,
}
