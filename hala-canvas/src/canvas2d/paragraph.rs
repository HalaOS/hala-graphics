/// A paragraph of text.
/// A paragraph retains the size and position of each glyph in the text and can be efficiently resized and painted.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Paragraph {
    /// The distance from the top of the paragraph to the alphabetic baseline of the first line, in logical pixels.
    pub alphabetic_baseline: f32,
    /// True if there is more vertical content, but the text was truncated, either because we reached maxLines lines
    /// of text or because the maxLines was null, ellipsis was not null, and one of the lines exceeded the width constraint.
    pub did_exceed_max_lines: bool,
    /// The amount of vertical space this paragraph occupies.
    pub height: f32,
    /// The distance from the top of the paragraph to the ideographic baseline of the first line, in logical pixels.
    pub ideo_graphic_baseline: f32,
    /// The distance from the left edge of the leftmost glyph to the right edge of the rightmost glyph in the paragraph.
    pub longest_line: f32,
    /// Returns the smallest width beyond which increasing the width never decreases the height.
    pub max_intrinsic_with: f32,
    /// The minimum width that this paragraph could be without failing to paint its contents within itself.
    pub min_intrinsic_width: f32,
    /// The amount of horizontal space this paragraph occupies.
    pub width: f32,
}
