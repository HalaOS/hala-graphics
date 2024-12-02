use super::{Angle, Animatable, FrameVariable, Measurement};

/// See [`length_adjust`](Text::length_adjust)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextLengthAdjust {
    /// 'spacing' indicates that only the advance values are adjusted. The glyphs themselves are not stretched or compressed.
    Spacing,

    /// 'spacingAndGlyphs' indicates that the advance values are adjusted and the glyphs themselves stretched or compressed
    /// in one axis (i.e., a direction parallel to the inline-progression-direction).
    SpacingAndGlyphs,
}

impl FrameVariable for TextLengthAdjust {}

/// The ‘text’ element defines a graphics element consisting of text.
///
/// See [`text`](https://www.w3.org/TR/SVG11/text.html#TextElement)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    /// If a single `coordinate` is provided, then the value represents the new absolute X coordinate for
    /// the current text position for rendering the glyphs that correspond to the first character within
    /// this element or any of its descendants.
    ///
    /// If a comma- or space-separated list of n `coordinate`s is provided, then the values represent new
    /// absolute X coordinates for the current text position for rendering the glyphs corresponding to
    /// each of the first n characters within this element or any of its descendants.
    ///
    /// For additional processing rules, refer to the description of the ‘x’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub x: Animatable<Vec<Measurement>>,

    /// The corresponding list of absolute Y coordinates for the glyphs corresponding to the characters within this element.
    /// The processing rules for the ‘y’ attribute parallel the processing rules for the ‘x’ attribute.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub y: Animatable<Vec<Measurement>>,

    /// Shifts in the current text position along the x-axis for the characters within this element or any of its descendants.
    ///
    /// Refer to the description of the ‘dx’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental shifts along
    /// the x-axis will occur.
    pub dx: Animatable<Vec<Measurement>>,

    /// Shifts in the current text position along the y-axis for the characters within this element or any of its descendants.
    ///
    /// Refer to the description of the ‘dy’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental shifts along
    /// the y-axis will occur.
    pub dy: Animatable<Vec<Measurement>>,

    /// The supplemental rotation about the current text position that will be applied to all of the glyphs corresponding
    /// to each character within this element.
    ///
    /// Refer to the description of the ‘rotate’ attribute on the ‘tspan’ element.
    ///
    /// If the attribute is not specified on this element or any of its descendants, no supplemental rotations will occur.
    pub rotate: Animatable<Vec<Angle>>,

    /// The author's computation of the total sum of all of the advance values that correspond to character data within
    /// this element, including the advance value on the glyph (horizontal or vertical), the effect of properties ‘kerning’,
    /// ‘letter-spacing’ and ‘word-spacing’ and adjustments due to attributes ‘dx’ and ‘dy’ on ‘tspan’ elements. This value
    /// is used to calibrate the user agent's own calculations with that of the author.
    ///
    /// The purpose of this attribute is to allow the author to achieve exact alignment, in visual rendering order after any
    /// bidirectional reordering, for the first and last rendered glyphs that correspond to this element; thus, for the last
    /// rendered character (in visual rendering order after any bidirectional reordering), any supplemental inter-character
    /// spacing beyond normal glyph advances are ignored (in most cases) when the user agent determines the appropriate amount
    /// to expand/compress the text string to fit within a length of ‘textLength’.
    ///
    /// A negative value is an error (see Error processing).
    ///
    /// If the attribute is not specified, the effect is as if the author's computation exactly matched the value calculated
    /// by the user agent; thus, no advance adjustments are made.
    pub text_length: Animatable<Measurement>,

    /// Indicates the type of adjustments which the user agent shall make to make the rendered length of the text match the
    /// value specified on the ‘textLength’ attribute.
    ///
    /// The user agent is required to achieve correct start and end positions for the text strings, but the locations of
    /// intermediate glyphs are not predictable because user agents might employ advanced algorithms to stretch or compress
    /// text strings in order to balance correct start and end positioning with optimal typography.
    ///
    /// Note that, for a text string that contains n characters, the adjustments to the advance values often occur only for n−1
    /// characters (see description of attribute ‘textLength’), whereas stretching or compressing of the glyphs will be applied
    /// to all n characters.
    ///
    /// If the attribute is not specified, the effect is as a value of 'spacing' were specified.
    pub length_adjust: Animatable<TextLengthAdjust>,
}

/// Within a ‘text’ element, text and font properties and the current text position can be adjusted with absolute or
/// relative coordinate values by including a ‘tspan’ element.
///
/// See [`tspan`](https://www.w3.org/TR/SVG11/text.html#TSpanElement)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpan {
    /// If a single <coordinate> is provided, then the value represents the new absolute X coordinate for the current
    /// text position for rendering the glyphs that correspond to the first character within this element or any of its descendants.
    ///
    /// If a comma- or space-separated list of n <coordinate>s is provided, then the values represent new absolute X coordinates
    /// for the current text position for rendering the glyphs corresponding to each of the first n characters within this element
    /// or any of its descendants.
    ///
    /// If more <coordinate>s are provided than characters, then the extra <coordinate>s will have no effect on glyph positioning.
    ///
    /// If more characters exist than <coordinate>s, then for each of these extra characters: (a) if an ancestor ‘text’ or ‘tspan’
    /// element specifies an absolute X coordinate for the given character via an ‘x’ attribute, then that absolute X coordinate is
    /// used as the starting X coordinate for that character (nearest ancestor has precedence), else (b) the starting X coordinate
    /// for rendering the glyphs corresponding to the given character is the X coordinate of the resulting current text position
    /// from the most recently rendered glyph for the current ‘text’ element.
    ///
    /// If the attribute is not specified: (a) if an ancestor ‘text’ or ‘tspan’ element specifies an absolute X coordinate for a given
    /// character via an ‘x’ attribute, then that absolute X coordinate is used (nearest ancestor has precedence), else (b) the
    /// starting X coordinate for rendering the glyphs corresponding to a given character is the X coordinate of the resulting current
    /// text position from the most recently rendered glyph for the current ‘text’ element.
    pub x: Animatable<Vec<Measurement>>,

    /// The corresponding list of absolute Y coordinates for the glyphs corresponding to the characters within this element.
    /// The processing rules for the ‘y’ attribute parallel the processing rules for the ‘x’ attribute.
    pub y: Animatable<Vec<Measurement>>,

    /// If a single <length> is provided, this value represents the new relative X coordinate for the current text position for
    /// rendering the glyphs corresponding to the first character within this element or any of its descendants. The current text
    /// position is shifted along the x-axis of the current user coordinate system by <length> before the first character's glyphs
    /// are rendered.
    ///
    /// If a comma- or space-separated list of n <length>s is provided, then the values represent incremental shifts along the
    /// x-axis for the current text position before rendering the glyphs corresponding to the first n characters within this element
    /// or any of its descendants. Thus, before the glyphs are rendered corresponding to each character, the current text position
    /// resulting from drawing the glyphs for the previous character within the current ‘text’ element is shifted along the X axis
    /// of the current user coordinate system by <length>.
    ///
    /// If more <length>s are provided than characters, then any extra <length>s will have no effect on glyph positioning.
    ///
    /// If more characters exist than <length>s, then for each of these extra characters: (a) if an ancestor ‘text’ or ‘tspan’
    /// element specifies a relative X coordinate for the given character via a ‘dx’ attribute, then the current text position
    /// is shifted along the x-axis of the current user coordinate system by that amount (nearest ancestor has precedence),
    /// else (b) no extra shift along the x-axis occurs.
    ///
    /// If the attribute is not specified: (a) if an ancestor ‘text’ or ‘tspan’ element specifies a relative X coordinate for a given
    /// character via a ‘dx’ attribute, then the current text position is shifted along the x-axis of the current user coordinate
    /// system by that amount (nearest ancestor has precedence), else (b) no extra shift along the x-axis occurs.
    pub dx: Animatable<Vec<Measurement>>,

    /// The corresponding list of relative Y coordinates for the characters within the ‘tspan’ element. The processing rules for
    /// the ‘dy’ attribute parallel the processing rules for the ‘dx’ attribute.
    pub dy: Animatable<Vec<Measurement>>,

    /// The supplemental rotation about the current text position that will be applied to all of the glyphs corresponding to each
    /// character within this element.
    ///
    /// If a comma- or space-separated list of `number`s is provided, then the first `number` represents the supplemental rotation
    /// for the glyphs corresponding to the first character within this element or any of its descendants, the second `number` represents
    /// the supplemental rotation for the glyphs that correspond to the second character, and so on.
    ///
    /// If more <number>s are provided than there are characters, then the extra <number>s will be ignored.
    ///
    /// If more characters are provided than <number>s, then for each of these extra characters the rotation value specified by the last
    /// number must be used.
    ///
    /// If the attribute is not specified and if an ancestor ‘text’ or ‘tspan’ element specifies a supplemental rotation for a given character
    /// via a ‘rotate’ attribute, then the given supplemental rotation is applied to the given character (nearest ancestor has precedence).
    /// If there are more characters than <number>s specified in the ancestor's ‘rotate’ attribute, then for each of these extra characters
    /// the rotation value specified by the last number must be used.
    ///
    /// This supplemental rotation has no impact on the rules by which current text position is modified as glyphs get rendered and is
    /// supplemental to any rotation due to text on a path and to ‘glyph-orientation-horizontal’ or ‘glyph-orientation-vertical’.
    pub rotate: Animatable<Vec<Angle>>,

    /// The author's computation of the total sum of all of the advance values that correspond to character data within this element,
    /// including the advance value on the glyph (horizontal or vertical), the effect of properties ‘kerning’, ‘letter-spacing’ and
    /// ‘word-spacing’ and adjustments due to attributes ‘dx’ and ‘dy’ on this ‘tspan’ element or any descendants. This value is used
    /// to calibrate the user agent's own calculations with that of the author.
    ///
    /// The purpose of this attribute is to allow the author to achieve exact alignment, in visual rendering order after any
    /// bidirectional reordering, for the first and last rendered glyphs that correspond to this element; thus, for the last rendered
    /// character (in visual rendering order after any bidirectional reordering), any supplemental inter-character spacing beyond
    /// normal glyph advances are ignored (in most cases) when the user agent determines the appropriate amount to expand/compress
    /// the text string to fit within a length of ‘textLength’.
    ///
    /// If attribute ‘textLength’ is specified on a given element and also specified on an ancestor, the adjustments on all character
    /// data within this element are controlled by the value of ‘textLength’ on this element exclusively, with the possible side-effect
    /// that the adjustment ratio for the contents of this element might be different than the adjustment ratio used for other content
    /// that shares the same ancestor. The user agent must assume that the total advance values for the other content within that
    /// ancestor is the difference between the advance value on that ancestor and the advance value for this element.
    ///
    /// A negative value is an error (see Error processing).
    ///
    /// If the attribute is not specified anywhere within a ‘text’ element, the effect is as if the author's computation exactly matched
    /// the value calculated by the user agent; thus, no advance adjustments are made.
    pub text_length: Animatable<Measurement>,
}

/// The ‘writing-mode’ property specifies whether the initial inline-progression-direction for a ‘text’ element shall be
/// left-to-right, right-to-left, or top-to-bottom. The ‘writing-mode’ property applies only to ‘text’ elements;
/// the property is ignored for ‘tspan’, ‘tref’, ‘altGlyph’ and ‘textPath’ sub-elements. (Note that the inline-progression-direction
/// can change within a ‘text’ element due to the Unicode bidirectional algorithm and properties ‘direction’ and ‘unicode-bidi’.
/// For more on bidirectional text, see Relationship with bidirectionality.)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WritingMode {
    /// Sets the initial inline-progression-direction to left-to-right, as is common in most Latin-based documents.
    /// For most characters, the current text position is advanced from left to right after each glyph is rendered.
    /// (When the character data includes characters which are subject to the Unicode bidirectional algorithm, the text
    /// advance rules are more complex. See Relationship with bidirectionality).
    LrTb,
    /// Sets the initial inline-progression-direction to right-to-left, as is common in Arabic or Hebrew scripts.
    /// (See Relationship with bidirectionality.)
    RlTb,
    /// Sets the initial inline-progression-direction to top-to-bottom, as is common in some Asian scripts,
    /// such as Chinese and Japanese. Though hardly as frequent as horizontal, this type of vertical layout also occurs
    /// in Latin based documents, particularly in table column or row labels. In most cases, the vertical baselines
    /// running through the middle of each glyph are aligned.
    TbRl,
    /// See [`LrTb`](WritingMode::LrTb)
    Lr,
    /// See [`RlTb`](WritingMode::RlTb)
    Rl,
    /// See [`TbRl`](WritingMode::TbRl)
    Tb,
}

impl Default for WritingMode {
    fn default() -> Self {
        Self::LrTb
    }
}

/// In some cases, it is required to alter the orientation of a sequence of characters relative to the
/// inline-progression-direction. The requirement is particularly applicable to vertical layouts of East
/// Asian documents, where sometimes narrow-cell Latin text is to be displayed horizontally and other
/// times vertically.
///
/// See [`GlyphOrientation`](https://www.w3.org/TR/SVG11/text.html#GlyphOrientation)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GlyphOrientationVertical {
    Auto,
    Angle(Angle),
}

impl Default for GlyphOrientationVertical {
    fn default() -> Self {
        Self::Auto
    }
}

/// This property is applied only to text written in a horizontal ‘writing-mode’.
///
/// The glyph orientation affects the amount that the current text position advances as each glyph is rendered.
/// When the reference orientation direction is horizontal and the ‘glyph-orientation-horizontal’ results in an
/// orientation angle that is a multiple of 180 degrees, then the current text position is incremented according
/// to the horizontal metrics of the glyph. Otherwise, if the ‘glyph-orientation-horizontal’ results in an
/// orientation angle that is not a multiple of 180 degrees, then the current text position is incremented
/// according to the vertical metrics of the glyph.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlyphOrientationHorizontal(pub Angle);

/// Within text content elements, the alignment of text with regards to the ‘text-anchor’ property is determined by
/// the value of the ‘direction’ property. For example, given a ‘text’ element with a ‘text-anchor’ value of "end",
/// for a ‘direction’ value of "ltr", the text will extend to the left of the position of the ‘text’ element's ‘x’
/// attribute value, while for ‘direction’ value of "rtl", the text will extend to the right of the position of the
/// ‘text’ element's ‘x’ attribute value.
///
/// A more complete discussion of bidirectionality can be found in the Text direction section of
/// [`CSS 2`](https://www.w3.org/TR/CSS2/visuren.html#direction).
///
/// See [`direction`](https://www.w3.org/TR/SVG11/text.html#DirectionProperty)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextDirection {
    Ltr,
    Rtl,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Ltr
    }
}

/// Except for any additional information provided in this specification, the normative definition of the
/// [`unicode-bidi`] property is in CSS2
///
/// [`unicode-bidi`]: https://www.w3.org/TR/2008/REC-CSS2-20080411/visuren.html#propdef-unicode-bidi
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnicodeBidi {
    Normal,
    Embed,
    BidiOverride,
}

impl Default for UnicodeBidi {
    fn default() -> Self {
        Self::Normal
    }
}

/// The ‘text-anchor’ property is used to align (start-, middle- or end-alignment) a string of text relative to a given point.
///
/// The ‘text-anchor’ property is applied to each individual text chunk within a given ‘text’ element. Each text chunk has an
/// initial current text position, which represents the point in the user coordinate system resulting from (depending on context)
/// application of the ‘x’ and ‘y’ attributes on the ‘text’ element, any ‘x’ or ‘y’ attribute values on a ‘tspan’, ‘tref’ or
/// ‘altGlyph’ element assigned explicitly to the first rendered character in a text chunk, or determination of the initial current
/// text position for a ‘textPath’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextAnchor {
    /// The rendered characters are aligned such that the start of the resulting rendered text is at the initial current text position.
    /// For an element with a ‘direction’ property value of "ltr" (typical for most European languages), the left side of the text is
    /// rendered at the initial text position. For an element with a ‘direction’ property value of "rtl" (typical for Arabic and Hebrew),
    /// the right side of the text is rendered at the initial text position. For an element with a vertical primary text direction
    /// (often typical for Asian text), the top side of the text is rendered at the initial text position.
    Start,
    /// The rendered characters are aligned such that the geometric middle of the resulting rendered text is at the initial
    /// current text position.
    Middle,
    /// The rendered characters are aligned such that the end of the resulting rendered text is at the initial current text position.
    /// For an element with a ‘direction’ property value of "ltr" (typical for most European languages), the right side of the text is
    /// rendered at the initial text position. For an element with a ‘direction’ property value of "rtl" (typical for Arabic and Hebrew),
    /// the left side of the text is rendered at the initial text position. For an element with a vertical primary text direction (often
    /// typical for Asian text), the bottom of the text is rendered at the initial text position.
    End,
}

impl Default for TextAnchor {
    fn default() -> Self {
        Self::Start
    }
}

/// See [`baseline`](https://www.w3.org/TR/SVG11/text.html#BaselineAlignmentProperties)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DominantBaseline {
    /// If this property occurs on a ‘text’ element, then the computed value depends on the value of the ‘writing-mode’ property.
    /// If the 'writing-mode' is horizontal, then the value of the dominant-baseline component is 'alphabetic', else if the
    /// 'writing-mode' is vertical, then the value of the dominant-baseline component is 'central'.
    ///
    /// If this property occurs on a ‘tspan’, ‘tref’, ‘altGlyph’ or ‘textPath’ element, then the dominant-baseline and the
    /// baseline-table components remain the same as those of the parent text content element. If the computed ‘baseline-shift’
    /// value actually shifts the baseline, then the baseline-table font-size component is set to the value of the ‘font-size’
    /// property on the element on which the ‘dominant-baseline’ property occurs, otherwise the baseline-table font-size remains
    /// the same as that of the element. If there is no parent text content element, the scaled-baseline-table value is
    /// constructed as above for ‘text’ elements.
    Auto,
    /// The dominant-baseline and the baseline-table components are set by determining the predominant script of the character
    /// data content. The ‘writing-mode’, whether horizontal or vertical, is used to select the appropriate set of baseline-tables
    /// and the dominant baseline is used to select the baseline-table that corresponds to that baseline. The baseline-table
    /// font-size component is set to the value of the ‘font-size’ property on the element on which the ‘dominant-baseline’ property
    /// occurs.
    UseScript,
    /// The dominant-baseline, the baseline-table, and the baseline-table font-size remain the same as that of the parent text
    /// content element.
    NoChange,
    /// The dominant-baseline and the baseline-table remain the same, but the baseline-table font-size is changed to the value
    /// of the ‘font-size’ property on this element. This re-scales the baseline-table for the current ‘font-size’.
    ResetSize,
    /// The baseline-identifier for the dominant-baseline is set to be 'ideographic', the derived baseline-table is constructed
    /// using the 'ideographic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of
    /// the ‘font-size’ property on this element.
    Ideographic,
    /// The baseline-identifier for the dominant-baseline is set to be 'alphabetic', the derived baseline-table is constructed
    /// using the 'alphabetic' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of
    /// the ‘font-size’ property on this element.
    Alphabetic,
    /// The baseline-identifier for the dominant-baseline is set to be 'hanging', the derived baseline-table is constructed using
    /// the 'hanging' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of the
    /// ‘font-size’ property on this element.
    Hanging,
    /// The baseline-identifier for the dominant-baseline is set to be 'mathematical', the derived baseline-table is constructed
    /// using the 'mathematical' baseline-table in the nominal font, and the baseline-table font-size is changed to the value of
    /// the ‘font-size’ property on this element.
    Mathematical,
    /// The baseline-identifier for the dominant-baseline is set to be 'central'. The derived baseline-table is constructed from
    /// the defined baselines in a baseline-table in the nominal font. That font baseline-table is chosen using the following
    /// priority order of baseline-table names: 'ideographic', 'alphabetic', 'hanging', 'mathematical'. The baseline-table font-size
    /// is changed to the value of the ‘font-size’ property on this element.
    Central,
    /// The baseline-identifier for the dominant-baseline is set to be 'middle'. The derived baseline-table is constructed from
    /// the defined baselines in a baseline-table in the nominal font. That font baseline -table is chosen using the following
    /// priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'. The baseline-table
    /// font-size is changed to the value of the ‘font-size’ property on this element.
    Middle,
    /// The baseline-identifier for the dominant-baseline is set to be 'text-after-edge'. The derived baseline-table is constructed
    /// from the defined baselines in a baseline-table in the nominal font. The choice of which font baseline-table to use from the
    /// baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the
    /// ‘font-size’ property on this element.
    ///
    /// NOTE: using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'
    /// is probably a reasonable strategy for determining which font baseline-table to use.
    TextAfterEdge,
    /// The baseline-identifier for the dominant-baseline is set to be 'text-before-edge'. The derived baseline-table is constructed
    /// from the defined baselines in a baseline-table in the nominal font. The choice of which baseline-table to use from the
    /// baseline-tables in the nominal font is implementation defined. The baseline-table font-size is changed to the value of the
    /// ‘font-size’ property on this element.
    ///
    /// NOTE: Using the following priority order of baseline-table names: 'alphabetic', 'ideographic', 'hanging', 'mathematical'
    /// is probably a reasonable strategy for determining which font baseline-table to use.
    TextBeforeEdge,
}

impl Default for DominantBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

impl FrameVariable for DominantBaseline {}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlignmentBaseline {
    /// The value is the dominant-baseline of the script to which the character belongs - i.e.,
    /// use the dominant-baseline of the parent.
    Auto,
    /// The alignment-point of the object being aligned is aligned with the dominant-baseline of
    /// the parent text content element.
    Baseline,
    /// The alignment-point of the object being aligned is aligned with the "before-edge" baseline of
    /// the parent text content element.
    BeforeEdge,
    /// The alignment-point of the object being aligned is aligned with the "text-before-edge" baseline of
    /// the parent text content element.
    TextBeforeEdge,
    /// The alignment-point of the object being aligned is aligned with the "middle" baseline of the parent text content element.
    Middle,
    /// The alignment-point of the object being aligned is aligned with the "central" baseline of the parent text content element.
    Central,
    /// The alignment-point of the object being aligned is aligned with the "after-edge" baseline of the parent text content element.
    AfterEdge,
    /// The alignment-point of the object being aligned is aligned with the "text-after-edge" baseline of the parent text content element.
    TextAfterEdge,
    /// The alignment-point of the object being aligned is aligned with the "ideographic" baseline of the parent text content element.
    Ideographic,
    /// The alignment-point of the object being aligned is aligned with the "alphabetic" baseline of the parent text content element.
    Alphabetic,
    /// The alignment-point of the object being aligned is aligned with the "hanging" baseline of the parent text content element.
    Hanging,
    /// The alignment-point of the object being aligned is aligned with the "mathematical" baseline of the parent text content element.
    Mathematical,
}

impl Default for AlignmentBaseline {
    fn default() -> Self {
        Self::Auto
    }
}

impl FrameVariable for AlignmentBaseline {}

/// The ‘baseline-shift’ property allows repositioning of the dominant-baseline relative to the dominant-baseline of
/// the parent text content element. The shifted object might be a sub- or superscript. Within the shifted object,
/// the whole baseline-table is offset; not just a single baseline. The amount of the shift is determined from information
/// from the parent text content element, the sub- or superscript offset from the nominal font of the parent text content
/// element, percent of the "line-height" of the parent text content element or an absolute value.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BaselineShift {
    /// There is no baseline shift; the dominant-baseline remains in its original position.
    Baseline,
    /// The dominant-baseline is shifted to the default position for subscripts. The offset to this position
    /// is determined using the font data for the nominal font. Because in most fonts the subscript position
    /// is normally given relative to the "alphabetic" baseline, the user agent may compute the effective
    /// position for subscripts for superscripts when some other baseline is dominant. The suggested computation
    /// is to subtract the difference between the position of the dominant baseline and the position of the
    /// "alphabetic" baseline from the position of the subscript. The resulting offset is determined by multiplying
    /// the effective subscript position by the dominant baseline-table font-size. If there is no applicable font
    /// data the user agent may use heuristics to determine the offset.
    Sub,
    /// The dominant-baseline is shifted to the default position for superscripts. The offset to this position is
    /// determined using the font data for the nominal font. Because in most fonts the superscript position is normally
    /// given relative to the "alphabetic" baseline, the user agent may compute the effective position for superscripts
    /// when some other baseline is dominant. The suggested computation is to subtract the difference between the
    /// position of the dominant baseline and the position of the "alphabetic" baseline from the position of the
    /// superscript. The resulting offset is determined by multiplying the effective superscript position by the dominant
    /// baseline-table font-size. If there is no applicable font data the user agent may use heuristics to determine the
    /// offset.
    Super,
    /// The computed value of the property is this percentage multiplied by the computed "line-height" of the ‘text’ element.
    /// The dominant-baseline is shifted in the shift direction (positive value) or opposite to the shift direction
    /// (negative value) of the parent text content element by the computed value. A value of "0" is equivalent to "baseline".
    Value(Measurement),
}

impl Default for BaselineShift {
    fn default() -> Self {
        Self::Baseline
    }
}

impl FrameVariable for BaselineShift {}
