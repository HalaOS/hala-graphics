use bitmask_enum::bitmask;

use super::{FrameVariable, Measurement, PathEvent, UnicodeRange};

/// Reliable delivery of fonts is a requirement for SVG. Designers need to create SVG content with arbitrary
/// fonts and know that the same graphical result will appear when the content is viewed by all end users,
/// even when end users do not have the necessary fonts installed on their computers. This parallels the print
/// world, where the designer uses a given font when authoring a drawing for print, and the graphical content
/// appears exactly the same in the printed version as it appeared on the designer's authoring system.
///
/// SVG utilizes the WebFonts facility defined in CSS2 ([`CSS2`], section 15.1) as a key mechanism for reliable
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
///
/// [`CSS2`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/
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

    /// The attribute value is a comma-separated list of language names as defined in BCP 47 [`BCP47`].
    ///
    /// [`BCP47`]: https://www.ietf.org/rfc/bcp/bcp47.txt
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

/// The ‘hkern’ element defines kerning pairs and adjustment values in the horizontal advance value when drawing
/// pairs of glyphs which the two glyphs are contiguous and are both rendered horizontally (i.e., side-by-side).
/// The spacing between characters is reduced by the kerning adjustment. (Negative kerning adjustments increase
/// the spacing between characters.)
///
/// At least one each of ‘u1’ or ‘g1’ and at least one of ‘u2’ or ‘g2’ must be provided.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VKern {
    /// A sequence (comma-separated) of Unicode characters (refer to the description of the ‘unicode’ attribute
    /// to the ‘glyph’ element for a description of how to express individual Unicode characters) and/or ranges
    /// of Unicode characters (see description of ranges of Unicode characters in CSS2; [`CSS2`, section 15.3.3)
    /// which identify a set of possible first glyphs in the kerning pair. If a given Unicode character within
    /// the set has multiple corresponding ‘glyph’ elements (i.e., there are multiple ‘glyph’ elements with the
    /// same ‘unicode’ attribute value, but different ‘glyph-name’ values), then all such glyphs are included
    /// in the set. Comma is the separator character; thus, to kern a comma, specify the comma as part of a range
    /// of Unicode characters or as a glyph name using the ‘g1’ attribute. The total set of possible first glyphs
    /// in the kerning pair is the union of glyphs specified by the ‘u1’ and ‘g1’ attributes.
    pub u1: Vec<UnicodeRange>,
    /// A sequence (comma-separated) of glyph names (i.e., values that match ‘glyph-name’ attributes on ‘glyph’
    /// elements) which identify a set of possible first glyphs in the kerning pair. All glyphs with the given
    /// glyph name are included in the set. The total set of possible first glyphs in the kerning pair is the
    /// union of glyphs specified by the ‘u1’ and ‘g1’ attributes.
    pub g1: Vec<String>,
    /// Same as the ‘u1’ attribute, except that ‘u2’ specifies possible second glyphs in the kerning pair.
    pub u2: Vec<UnicodeRange>,
    /// Same as the ‘g1’ attribute, except that ‘g2’ specifies possible second glyphs in the kerning pair.
    pub g2: Vec<String>,
    /// The amount to decrease the spacing between the two glyphs in the kerning pair. The value is in the
    /// font coordinate system. This attribute is required.
    pub k: f32,
}

/// See [`style`](FontFace::style)
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::all_flags()
    }
}

/// Same syntax and semantics as the ‘font-variant’ descriptor within an @font-face rule.
/// Indication of whether this face is the small-caps variant of a font. Takes on the same values
/// as the ‘font-variant’ property, except that a comma-separated list is permitted.
///
/// If the attribute is not specified, the effect is as if a value of 'normal' were specified.

#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontVariant {
    Normal,
    SmallCaps,
}

impl Default for FontVariant {
    fn default() -> Self {
        Self::Normal
    }
}

/// Same syntax and semantics as the ‘font-weight’ descriptor within an @font-face rule.
///
/// See [`FontFace`]
#[bitmask(u16)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontWeight {
    Normal,
    Bold,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::all_flags()
    }
}

/// Same syntax and semantics as the ‘font-stretch’ descriptor within an @font-face rule.
/// Indication of the condensed or expanded nature of the face relative to others in the same font family.
///
/// See [`FontFace`]
#[bitmask(u16)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontStretch {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Default for FontStretch {
    fn default() -> Self {
        Self::Normal
    }
}

/// The same as [`widths`](https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#descdef-widths)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlyphWidths {
    pub unicode_range: Option<UnicodeRange>,
    pub widths: Vec<f32>,
}

/// The ‘font-face’ element corresponds directly to the @font-face facility in CSS2 ([`CSS2`], section 15.3.1).
/// It can be used to describe the characteristics of any font, SVG font or otherwise.
///
/// [`CSS2`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontFace {
    /// Same syntax and semantics as the ‘font-family’ descriptor within an [`@font-face`] rule.
    ///
    /// [`@font-face`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
    pub family: String,

    /// Same syntax and semantics as the ‘font-style’ descriptor within an @font-face rule.
    /// The style of a font.
    ///
    /// If the attribute is not specified, the effect is as if a value of 'all' were specified.
    pub style: FontStyle,

    /// See [`FontVariant`]
    pub variant: FontVariant,

    /// See [`FontWeight`]
    pub weight: FontWeight,

    /// See [`FontStretch`]
    pub stretch: FontStretch,

    /// Same syntax and semantics as the ‘font-size’ descriptor within an @font-face rule.
    pub size: Vec<Measurement>,

    /// Same syntax and semantics as the ‘unicode-range’ descriptor within an @font-face rule. The range of ISO
    /// 10646 characters `UNICODE` possibly covered by the glyphs in the font. Except for any additional information
    /// provided in this specification, the normative definition of the attribute is in CSS2 (`CSS2`, section 15.3.3).
    ///
    /// If the attribute is not specified, the effect is as if a value of 'U+0-10FFFF' were specified.
    pub unicode_range: Vec<UnicodeRange>,

    /// Same syntax and semantics as the ‘units-per-em’ descriptor within an @font-face rule.
    /// The number of coordinate units on the em square, the size of the design grid on which
    /// glyphs are laid out.
    ///
    /// This value is almost always necessary as nearly every other attribute requires the definition
    /// of a design grid.
    ///
    /// If the attribute is not specified, the effect is as if a value of '1000' were specified.
    pub units_per_em: f32,

    //// Same syntax and semantics as the ‘panose-1’ descriptor within an @font-face rule.
    //// The Panose-1 number, consisting of ten decimal integers, separated by whitespace. Except for any additional
    //// information provided in this specification, the normative definition of the attribute is in CSS2
    //// (`CSS2`, section 15.3.6).
    ////
    //// If the attribute is not specified, the effect is as if a value of '[0;10]' were specified.
    pub panose_1: [u8; 10],

    /// Same syntax and semantics as the [`stemv`] descriptor within an @font-face rule.
    ///
    /// [`stemv`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#descdef-stemv
    pub stem_v: f32,

    /// Same syntax and semantics as the [`stemh`] descriptor within an @font-face rule.
    ///
    /// [`stemh`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#descdef-stemh
    pub stem_h: f32,

    /// Same syntax and semantics as the ‘slope’ descriptor within an @font-face rule.
    /// The vertical stroke angle of the font. Except for any additional information provided
    /// in this specification, the normative definition of the attribute is in CSS2 (`CSS2`,
    /// section 15.3.6).
    ///
    /// If the attribute is not specified, the effect is as if a value of '0' were specified.
    pub slope: f32,

    /// Same syntax and semantics as the ‘cap-height’ descriptor within an @font-face rule.
    /// The height of uppercase glyphs in the font within the font coordinate system.
    pub cap_height: f32,

    /// Same syntax and semantics as the ‘x-height’ descriptor within an @font-face rule.
    /// The height of lowercase glyphs in the font within the font coordinate system.
    pub x_height: f32,

    /// The distance from the origin to the top of accent characters, measured by a distance
    /// within the font coordinate system.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the
    /// value of the ‘ascent’ attribute.
    pub accent_height: Option<f32>,

    /// Same syntax and semantics as the ‘ascent’ descriptor within an @font-face rule.
    /// The maximum unaccented height of the font within the font coordinate system.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the
    /// difference between the ‘units-per-em’ value and the ‘vert-origin-y’ value for the
    /// corresponding font.
    pub ascent: Option<f32>,

    /// Same syntax and semantics as the ‘descent’ descriptor within an @font-face rule.
    /// The maximum unaccented depth of the font within the font coordinate system.
    ///
    /// If the attribute is not specified, the effect is as if the attribute were set to the
    /// ‘vert-origin-y’ value for the corresponding font.
    pub descent: Option<f32>,

    /// Same syntax and semantics as the ‘widths’ descriptor within an @font-face rule.
    pub widths: Vec<GlyphWidths>,

    /// Same syntax and semantics as the ‘bbox’ descriptor within an @font-face rule.
    pub bbox: [f32; 4],

    /// For horizontally oriented glyph layouts, indicates the alignment coordinate for
    /// glyphs to achieve ideographic baseline alignment. The value is an offset in the
    /// font coordinate system.
    pub ideographic: f32,

    /// Same syntax and semantics as the ‘baseline’ descriptor within an @font-face rule.
    /// For horizontally oriented glyph layouts, indicates the alignment coordinate for
    /// glyphs to achieve alphabetic baseline alignment. The value is an offset in the font
    /// coordinate system.
    pub alphabetic: f32,

    /// Same syntax and semantics as the ‘mathline’ descriptor within an @font-face rule.
    /// For horizontally oriented glyph layouts, indicates the alignment coordinate for glyphs
    /// to achieve mathematical baseline alignment. The value is an offset in the font coordinate
    /// system.
    pub mathematical: f32,

    /// For horizontally oriented glyph layouts, indicates the alignment coordinate for glyphs to
    /// achieve hanging baseline alignment. The value is an offset in the font coordinate system.
    pub hanging: f32,

    /// For horizontally oriented glyph layouts, indicates the alignment coordinate for
    /// glyphs to achieve ideographic baseline alignment. The value is an offset in the
    /// font coordinate system.
    pub v_ideographic: f32,

    /// For vertically oriented glyph layouts, indicates the alignment coordinate for glyphs
    /// to achieve ideographic baseline alignment. The value is an offset in the font coordinate
    /// system relative to the glyph-specific ‘vert-origin-x’ attribute.
    pub v_alphabetic: f32,

    /// For vertically oriented glyph layouts, indicates the alignment coordinate for glyphs to
    /// achieve mathematical baseline alignment. The value is an offset in the font coordinate
    /// system relative to the glyph-specific ‘vert-origin-x’ attribute.
    pub v_mathematical: f32,

    /// For vertically oriented glyph layouts, indicates the alignment coordinate for glyphs to
    /// achieve hanging baseline alignment. The value is an offset in the font coordinate system
    /// relative to the glyph-specific ‘vert-origin-x’ attribute.
    pub v_hanging: f32,

    /// The ideal position of an underline within the font coordinate system.
    pub underline_position: f32,

    /// The ideal thickness of an underline, expressed as a length within the font coordinate system.
    pub underline_thickness: f32,

    /// The ideal position of a strike-through within the font coordinate system.
    pub strikethrough_position: f32,

    /// The ideal thickness of a strike-through, expressed as a length within the font coordinate system.
    pub strikethrough_thickness: f32,

    /// The ideal position of an overline within the font coordinate system.
    pub overline_position: f32,

    /// The ideal thickness of an overline, expressed as a length within the font coordinate system.
    pub overline_thickness: f32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontFormat {
    /// TrueDoc™ Portable Font Resource
    TrueDoc,
    /// Embedded OpenType
    Eot,
    /// PostScript™ Type 1
    PostScript,
    TrueType,
    /// OpenType, including TrueType Open
    OpenType,
    TrueTypeGX,
    Speedo,
    Intellifont,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontSourceUri {
    FaceName(String),
    Url(String),
}

/// This element correspond to the ‘src’ descriptor within an @font-face rule.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontSource {
    pub uri: FontSourceUri,
    pub format: Option<Vec<FontFormat>>,
}
