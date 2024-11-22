use super::{Measurement, Variable, Variant};

/// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FilterUnits {
    /// see [`units`](Filter::units) and [`primitive_units`](Filter::primitive_units)
    /// for more informations.
    UserSpaceOnUse,
    /// see [`units`](Filter::units) and [`primitive_units`](Filter::primitive_units)
    /// for more informations.
    ObjectBoundingBox,
}

impl Default for FilterUnits {
    fn default() -> Self {
        Self::ObjectBoundingBox
    }
}

impl Variable for FilterUnits {}

/// This attribute takes the form x-pixels [y-pixels], and indicates the width and height
/// of the intermediate images in pixels. If not provided, then the user agent will use
/// reasonable values to produce a high-quality result on the output device.
///
/// Care should be taken when assigning a non-default value to this attribute. Too small
/// of a value may result in unwanted pixelation in the result. Too large of a value may
/// result in slow processing and large memory usage.
///
/// Negative values are an error (see Error processing). Zero values disable rendering of
/// the element which referenced the filter.
///
/// Non-integer values are truncated, i.e rounded to the closest integer value towards zero.
///
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FilterRes {
    /// `x-pixels`
    pub x: f32,
    /// optional `y-pixels`
    pub y: Option<f32>,
}

impl Variable for FilterRes {}

/// A filter effect consists of a series of graphics operations that are applied to
/// a given source graphic to produce a modified graphical result. The result of the
/// filter effect is rendered to the target device instead of the original source
/// graphic.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filter {
    /// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If filterUnits="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values
    /// in the current user coordinate system in place at the time when the ‘filter’ is
    /// referenced (i.e., the user coordinate system for the element referencing the
    /// ‘filter’ via a ‘filter’ property).
    ///
    /// If filterUnits="objectBoundingBox", then ‘x’, ‘y’, ‘width’ and ‘height’ represent
    /// fractions or percentages of the bounding box on the referencing element (see Object
    /// bounding box units).
    ///
    /// If attribute ‘filterUnits’ is not specified, then the effect is if a value of
    /// 'objectBoundingBox' were specified.
    pub units: Variant<FilterUnits>,

    /// Specifies the coordinate system for the various length values within the filter
    /// primitives and for the attributes that define the filter primitive subregion.
    ///
    /// If primitiveUnits="userSpaceOnUse", any length values within the filter definitions
    /// represent values in the current user coordinate system in place at the time when
    /// the ‘filter’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘filter’ element via a ‘filter’ property).
    ///
    /// If primitiveUnits="objectBoundingBox", then any length values within the filter
    /// definitions represent fractions or percentages of the bounding box on the referencing
    /// element (see Object bounding box units). Note that if only one number was specified in
    /// a `number-optional-number` value this number is expanded out before the ‘primitiveUnits’
    /// computation takes place.
    ///
    /// If attribute ‘primitiveUnits’ is not specified, then the effect is as if a value of
    /// userSpaceOnUse were specified.
    pub primitive_units: Variant<FilterUnits>,

    /// These attributes define a rectangular region on the canvas to which this filter applies.
    ///
    /// The amount of memory and processing time required to apply the filter are related to the
    /// size of this rectangle and the ‘filterRes’ attribute of the filter.
    ///
    /// The coordinate system for these attributes depends on the value for attribute ‘filterUnits’.
    ///
    /// Negative values for ‘width’ or ‘height’ are an error (see Error processing). Zero values
    /// disable rendering of the element which referenced the filter.
    ///
    /// The bounds of this rectangle act as a hard clipping region for each filter primitive included
    /// with a given ‘filter’ element; thus, if the effect of a given filter primitive would extend
    /// beyond the bounds of the rectangle (this sometimes happens when using a ‘feGaussianBlur’ filter
    /// primitive with a very large ‘stdDeviation’), parts of the effect will get clipped.
    ///
    /// If ‘x’ or ‘y’ is not specified, the effect is as if a value of -10% were specified.
    ///
    /// If ‘width’ or ‘height’ is not specified, the effect is as if a value of 120% were specified.
    pub x: Variant<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub y: Variant<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub width: Variant<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub height: Variant<Measurement>,

    /// See [`FilterRes`]
    pub filter_res: Variant<FilterRes>,
}
