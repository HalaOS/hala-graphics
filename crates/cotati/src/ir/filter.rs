use super::{Angle, Animatable, FrameVariable, Measurement, NumberOptNumber};

/// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeUnits {
    /// see [`units`](Fe::units) and [`primitive_units`](Fe::primitive_units)
    /// for more informations.
    UserSpaceOnUse,
    /// see [`units`](Fe::units) and [`primitive_units`](Fe::primitive_units)
    /// for more informations.
    ObjectBoundingBox,
}

impl Default for FeUnits {
    fn default() -> Self {
        Self::ObjectBoundingBox
    }
}

impl FrameVariable for FeUnits {}

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
pub struct FeRes {
    /// `x-pixels`
    pub x: f32,
    /// optional `y-pixels`
    pub y: Option<f32>,
}

impl FrameVariable for FeRes {}

/// Identifies input for the given filter primitive. The value can be either one of six keywords or
/// can be a string which matches a previous ‘result’ attribute value within the same ‘filter’ element.
/// If no value is provided and this is the first filter primitive, then this filter primitive will use
/// SourceGraphic as its input. If no value is provided and this is a subsequent filter primitive, then
/// this filter primitive will use the result from the previous filter primitive as its input.
///
/// If the value for ‘result’ appears multiple times within a given ‘filter’ element, then a reference
/// to that result will use the closest preceding filter primitive with the given value for attribute
/// ‘result’. Forward references to results are an error.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeIn {
    /// This keyword represents the graphics elements that were the original input into the ‘filter’ element.
    /// For raster effects filter primitives, the graphics elements will be rasterized into an initially clear
    /// RGBA raster in image space. Pixels left untouched by the original graphic will be left clear. The image
    /// is specified to be rendered in linear RGBA pixels. The alpha channel of this image captures any
    /// anti-aliasing specified by SVG. (Since the raster is linear, the alpha channel of this image will
    /// represent the exact percent coverage of each pixel.)
    SourceGraphic,
    /// This keyword represents the graphics elements that were the original input into the ‘filter’ element.
    /// SourceAlpha has all of the same rules as SourceGraphic except that only the alpha channel is used.
    /// The input image is an RGBA image consisting of implicitly black color values for the RGB channels,
    /// but whose alpha channel is the same as SourceGraphic. If this option is used, then some implementations
    /// might need to rasterize the graphics elements in order to extract the alpha channel.
    SourceAlpha,
    /// This keyword represents an image snapshot of the canvas under the filter region at the time that the
    /// ‘filter’ element was invoked.
    BackgroundImage,
    /// Same as BackgroundImage except only the alpha channel is used. See SourceAlpha and Accessing the background image.
    BackgroundAlpha,
    /// This keyword represents the value of the ‘fill’ property on the target element for the filter effect.
    /// The FillPaint image has conceptually infinite extent. Frequently this image is opaque everywhere,
    /// but it might not be if the "paint" itself has alpha, as in the case of a gradient or pattern which
    /// itself includes transparent or semi-transparent parts.
    FillPaint,
    /// This keyword represents the value of the ‘stroke’ property on the target element for the filter effect.
    /// The StrokePaint image has conceptually infinite extent. Frequently this image is opaque everywhere,
    /// but it might not be if the "paint" itself has alpha, as in the case of a gradient or pattern which
    /// itself includes transparent or semi-transparent parts.
    StrokePaint,

    /// Reference to named register for other filter-primitive result .
    Register(String),
}

impl FrameVariable for FeIn {}

impl Default for FeIn {
    fn default() -> Self {
        Self::SourceGraphic
    }
}

/// Assign output to a named register. otherwise the filter output will only be referenced by next filter primitive.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeOut {
    Position,
    Named(String),
}

impl Default for FeOut {
    fn default() -> Self {
        Self::Position
    }
}

impl FrameVariable for FeOut {}

/// A filter effect consists of a series of graphics operations that are applied to
/// a given source graphic to produce a modified graphical result. The result of the
/// filter effect is rendered to the target device instead of the original source
/// graphic.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fe {
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
    pub units: Animatable<FeUnits>,

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
    pub primitive_units: Animatable<FeUnits>,

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
    pub x: Animatable<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub y: Animatable<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub width: Animatable<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub height: Animatable<Measurement>,

    /// See [`FeRes`]
    pub filter_res: Option<Animatable<FeRes>>,
}

impl Default for Fe {
    fn default() -> Self {
        Self {
            units: FeUnits::ObjectBoundingBox.into(),
            primitive_units: FeUnits::UserSpaceOnUse.into(),
            x: Measurement::percentage(-10.0).into(),
            y: Measurement::percentage(-10.0).into(),
            width: Measurement::percentage(120.0).into(),
            height: Measurement::percentage(120.0).into(),
            filter_res: None,
        }
    }
}

/// The common attributes are available on all filter primitive elements:
///
///
/// # Fe primitive subregion
///
/// All filter primitives have attributes ‘x’, ‘y’, ‘width’ and ‘height’ which identify a subregion
/// which restricts calculation and rendering of the given filter primitive. These attributes are
/// defined according to the same rules as other filter primitives' coordinate and length attributes
/// and thus represent values in the coordinate system established by attribute ‘primitiveUnits’ on
/// the ‘filter’ element.
///
/// ‘x’, ‘y’, ‘width’ and ‘height’ default to the union (i.e., tightest fitting bounding box) of the
/// subregions defined for all referenced nodes. If there are no referenced nodes (e.g., for ‘feImage’
/// or ‘feTurbulence’), or one or more of the referenced nodes is a standard input (one of SourceGraphic,
/// SourceAlpha, BackgroundImage, BackgroundAlpha, FillPaint or StrokePaint), or for ‘feTile’ (which is
/// special because its principal function is to replicate the referenced node in X and Y and thereby
/// produce a usually larger result), the default subregion is 0%,0%,100%,100%, where as a special-case
/// the percentages are relative to the dimensions of the filter region, thus making the the default filter
/// primitive subregion equal to the filter region.
///
/// ‘x’, ‘y’, ‘width’ and ‘height’ act as a hard clip clipping rectangle on both the filter primitive's input
/// image(s) and the filter primitive result.
///
/// All intermediate offscreens are defined to not exceed the intersection of ‘x’, ‘y’, ‘width’ and ‘height’
/// with the filter region. The filter region and any of the ‘x’, ‘y’, ‘width’ and ‘height’ subregions are to
/// be set up such that all offscreens are made big enough to accommodate any pixels which even partly
/// intersect with either the filter region or the x,y,width,height subregions.
///
/// ‘feTile’ references a previous filter primitive and then stitches the tiles together based on the ‘x’, ‘y’,
/// ‘width’ and ‘height’ values of the referenced filter primitive in order to fill its own filter primitive subregion.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FePrimitive {
    /// The minimum x coordinate for the subregion which restricts calculation and rendering of the given filter primitive.
    pub x: Animatable<Measurement>,

    /// The minimum y coordinate for the subregion which restricts calculation and rendering of the given filter primitive.
    pub y: Animatable<Measurement>,

    /// The width of the subregion which restricts calculation and rendering of the given filter primitive.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables the effect of the given filter primitive
    /// (i.e., the result is a transparent black image).
    pub width: Animatable<Measurement>,

    /// The height of the subregion which restricts calculation and rendering of the given filter primitive.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables the effect of the given
    /// filter primitive (i.e., the result is a transparent black image).
    pub height: Animatable<Measurement>,

    /// Assign the filter primitive output to `position register` or `named register`.
    pub out: Animatable<FeOut>,
}

impl Default for FePrimitive {
    fn default() -> Self {
        Self {
            x: Measurement::percentage(0.0).into(),
            y: Measurement::percentage(0.0).into(),
            width: Measurement::percentage(100.0).into(),
            height: Measurement::percentage(100.0).into(),
            out: FeOut::Position.into(),
        }
    }
}

/// Defines distant light source.
///
/// The following diagram illustrates the angles which ‘azimuth’ and ‘elevation’ represent in an XYZ coordinate system.
///
/// ![`distance light source`](https://www.w3.org/TR/SVG11/images/filters/azimuth-elevation.png)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDistantLight {
    /// Direction angle for the light source on the XY plane (clockwise), in degrees from the x axis.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub azimuth: Animatable<Angle>,

    /// Direction angle for the light source from the XY plane towards the z axis, in degrees. Note the positive Z-axis points towards the viewer of the content.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub elevation: Animatable<Angle>,
}

/// Exponent for specular term, larger is more "shiny".
///
/// Range 1.0 to 128.0.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpecularExponent(pub f32);

impl Default for FeSpecularExponent {
    fn default() -> Self {
        Self(1.0)
    }
}

impl From<f32> for FeSpecularExponent {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<FeSpecularExponent> for f32 {
    fn from(value: FeSpecularExponent) -> Self {
        value.0
    }
}

impl FrameVariable for FeSpecularExponent {}

/// Defines spot light source.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeSpotLight {
    /// X location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub x: Animatable<Measurement>,

    /// Y location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub y: Animatable<Measurement>,

    /// Z location for the light source in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element,
    /// assuming that, in the initial coordinate system, the positive Z-axis comes out towards the person viewing the content and
    /// assuming that one unit along the Z-axis equals one unit in X and Y.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub z: Animatable<Measurement>,

    /// X location in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element of the point at which
    /// the light source is pointing.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub px: Animatable<Measurement>,

    /// Y location in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element of the point at which
    /// the light source is pointing.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub py: Animatable<Measurement>,

    /// Z location in the coordinate system established by attribute ‘primitiveUnits’ on the ‘filter’ element of the point at which
    /// the light source is pointing, assuming that, in the initial coordinate system, the positive Z-axis comes out towards the
    /// person viewing the content and assuming that one unit along the Z-axis equals one unit in X and Y.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
    pub pz: Animatable<Measurement>,

    /// Exponent value controlling the focus for the light source.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub specular_exponent: Animatable<FeSpecularExponent>,

    /// A limiting cone which restricts the region where the light is projected. No light is projected outside the cone.
    /// ‘limitingConeAngle’ represents the angle in degrees between the spot light axis (i.e. the axis between the light
    /// source and the point to which it is pointing at) and the spot light cone. User agents should apply a smoothing
    /// technique such as anti-aliasing at the boundary of the cone.
    ///
    /// If no value is specified, then no limiting cone will be applied.
    pub limiting_cone_angle: Option<Animatable<Angle>>,
}

/// Image blending modes
/// For the compositing formulas below, the following definitions apply:
/// * cr = Result color (RGB) - premultiplied
/// * qa = Opacity value at a given pixel for image A
/// * qb = Opacity value at a given pixel for image B
/// * ca = Color (RGB) at a given pixel for image A - premultiplied
/// * cb = Color (RGB) at a given pixel for image B - premultiplied
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeBlendMode {
    /// cr = (1 - qa) * cb + ca
    Normal,
    /// cr = (1-qa)*cb + (1-qb)*ca + ca*cb
    Multiply,
    /// cr = cb + ca - ca * cb
    Screen,
    /// cr = Min ((1 - qa) * cb + ca, (1 - qb) * ca + cb)
    Darken,
    /// cr = Max ((1 - qa) * cb + ca, (1 - qb) * ca + cb)
    Lighten,
}

impl Default for FeBlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl FrameVariable for FeBlendMode {}

/// This filter composites two objects together using commonly used imaging software blending modes.
/// It performs a pixel-wise combination of two input images.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeBlend {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// Image blending mode
    pub mode: Animatable<FeBlendMode>,

    /// The first input image to the blending operation.
    pub a: Animatable<FeIn>,

    /// The second input image to the blending operation. This attribute can take on the same values as the ‘in’ attribute.
    pub b: Animatable<FeIn>,
}

impl Default for FeBlend {
    fn default() -> Self {
        Self {
            primitive: Default::default(),
            mode: FeBlendMode::Normal.into(),
            a: FeIn::SourceGraphic.into(),
            b: FeIn::BackgroundImage.into(),
        }
    }
}

/// Values of FeColorMatrix.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeColorMatrixValues {
    /// a list of 20 matrix values.
    Matrix([f32; 20]),

    /// `Saturate` is a single real number value (0 to 1).
    ///
    /// See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)
    Saturate(f32),

    /// `HueRotate` is a single one real number value (degrees)
    ///
    /// See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)
    HueRotate(Angle),

    /// `LuminanceToAlpha`  is not applicable.
    ///
    /// See [`feColorMatrixElement`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement)
    LuminanceToAlpha,
}

impl FrameVariable for FeColorMatrixValues {}

/// This filter applies a matrix transformation.
///
/// on the RGBA color and alpha values of every pixel on the input graphics to produce a result with a new
/// set of RGBA color and alpha values.
///
/// The calculations are performed on non-premultiplied color values. If the input graphics consists of
/// premultiplied color values, those values are automatically converted into non-premultiplied color values
/// for this operation.
///
/// These matrices often perform an identity mapping in the alpha channel. If that is the case, an implementation
/// can avoid the costly undoing and redoing of the premultiplication for all pixels with A = 1.
///
/// See [`feColorMatrix`](https://www.w3.org/TR/SVG11/filters.html#feColorMatrixElement).
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeColorMatrix {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Animatable<FeIn>,

    /// The contents of ‘values’ depends on the value of attribute ‘type’:
    pub values: Animatable<FeColorMatrixValues>,
}

impl FeColorMatrix {
    /// defaults to the identity matrix
    pub fn matrix() -> Self {
        Self {
            primitive: Default::default(),
            r#in: Default::default(),
            values: FeColorMatrixValues::Matrix([
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
            ])
            .into(),
        }
    }

    /// defaults to the value 1
    pub fn saturate() -> Self {
        Self {
            primitive: Default::default(),
            r#in: Default::default(),
            values: FeColorMatrixValues::Saturate(1.0).into(),
        }
    }

    /// defaults to the value 0
    pub fn hue_rotate() -> Self {
        Self {
            primitive: Default::default(),
            r#in: Default::default(),
            values: FeColorMatrixValues::HueRotate(Angle::deg(0.0)).into(),
        }
    }
}

/// Defines transfer function of [`feComponentTransfer`](https://www.w3.org/TR/SVG11/filters.html#feComponentTransferTypeAttribute)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeTransferFn {
    Identity,

    Table(Vec<f32>),

    Linear {
        /// the slope of the linear function.
        slope: f32,
        /// the intercept of the linear function.
        intercept: f32,
    },

    Gamma {
        /// the amplitude of the gamma function.
        /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
        amplitude: f32,
        /// the exponent of the gamma function.
        /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
        exponent: f32,
        /// the offset of the gamma function.
        /// If the attribute is not specified, then the effect is as if a value of 0 were specified.
        offset: f32,
    },
}

impl Default for FeTransferFn {
    fn default() -> Self {
        Self::Identity
    }
}
/// This filter primitive performs component-wise remapping of data as follows:
///
/// > R' = feFuncR( R )
///
/// > G' = feFuncG( G )
///
/// > B' = feFuncB( B )
///
/// > A' = feFuncA( A )
///
/// for every pixel. It allows operations like brightness adjustment, contrast adjustment, color balance or thresholding.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComponentTransfer {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Animatable<FeIn>,

    /// transfer function for the red component of the input graphic
    pub func_r: FeTransferFn,
    /// transfer function for the green component of the input graphic
    pub func_g: FeTransferFn,
    /// transfer function for the blue component of the input graphic
    pub func_b: FeTransferFn,
    /// transfer function for the alpha component of the input graphic
    pub func_a: FeTransferFn,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeCompositeArithmetic(pub f32);

impl From<f32> for FeCompositeArithmetic {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<FeCompositeArithmetic> for f32 {
    fn from(value: FeCompositeArithmetic) -> Self {
        value.0
    }
}

impl FrameVariable for FeCompositeArithmetic {}

/// The compositing operation that is to be performed. All of the ‘operator’ types except arithmetic match the
/// corresponding operation as described in [PORTERDUFF]. The arithmetic operator is described above. If attribute
/// ‘operator’ is not specified, then the effect is as if a value of over were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeCompositeOperator {
    Over,
    In,
    Out,
    Atop,
    Xor,
    Arithmetic,
}

impl Default for FeCompositeOperator {
    fn default() -> Self {
        Self::Over
    }
}

impl FrameVariable for FeCompositeOperator {}

/// This filter performs the combination of the two input images pixel-wise in image space using one of the Porter-Duff [`PORTERDUFF`]
/// compositing operations: over, in, atop, out, xor [`SVG-COMPOSITING`]. Additionally, a component-wise arithmetic operation (with
/// the result clamped between [0..1]) can be applied.
///
/// See [`feComposite`].
///
/// [`feComposite`]: https://www.w3.org/TR/SVG11/filters.html#feCompositeElement
/// [`PORTERDUFF`]: https://www.w3.org/TR/SVG11/refs.html#ref-PORTERDUFF
/// [`SVG-COMPOSITING`]: https://www.w3.org/TR/SVG11/refs.html#ref-SVG-COMPOSITING
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeComposite {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub a: Animatable<FeIn>,

    /// The second input image to the compositing operation. This attribute can take on the same values as the [`a`](Self::a) attribute.
    pub b: Animatable<FeIn>,

    /// See [`FeCompositeOperator`]
    pub operator: Animatable<FeCompositeOperator>,

    /// Only applicable if operator="arithmetic".
    /// If the attribute is not specified, the effect is as if a value of 0 were specified.
    pub k1: Animatable<FeCompositeArithmetic>,

    /// Only applicable if operator="arithmetic".
    /// If the attribute is not specified, the effect is as if a value of 0 were specified.
    pub k2: Animatable<FeCompositeArithmetic>,

    /// Only applicable if operator="arithmetic".
    /// If the attribute is not specified, the effect is as if a value of 0 were specified.
    pub k3: Animatable<FeCompositeArithmetic>,

    /// Only applicable if operator="arithmetic".
    /// If the attribute is not specified, the effect is as if a value of 0 were specified.
    pub k4: Animatable<FeCompositeArithmetic>,
}

/// Indicates the number of cells in each dimension for ‘kernelMatrix’. The values provided must be <integer>s greater than zero.
/// The first number, <orderX>, indicates the number of columns in the matrix. The second number, <orderY>, indicates the number
/// of rows in the matrix. If <orderY> is not provided, it defaults to <orderX>.
///
/// A typical value is order="3". It is recommended that only small values (e.g., 3) be used; higher values may result in very
/// high CPU overhead and usually do not produce results that justify the impact on performance.
///
/// If the attribute is not specified, the effect is as if a value of 3 were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeConvolveMatrixOrder {
    pub order_x: u32,

    pub order_y: Option<u32>,
}

impl Default for FeConvolveMatrixOrder {
    fn default() -> Self {
        Self {
            order_x: 3,
            order_y: None,
        }
    }
}

impl FrameVariable for FeConvolveMatrixOrder {}

/// Determines how to extend the input image as necessary with color values so that the matrix operations
/// can be applied when the kernel is positioned at or near the edge of the input image.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeConvolveMatrixEdgeMode {
    Duplicate,
    Wrap,
    None,
}

impl Default for FeConvolveMatrixEdgeMode {
    fn default() -> Self {
        Self::Duplicate
    }
}

/// feConvolveMatrix applies a matrix convolution filter effect. A convolution combines pixels
/// in the input image with neighboring pixels to produce a resulting image. A wide variety
/// of imaging operations can be achieved through convolutions, including blurring, edge detection,
/// sharpening, embossing and beveling.
///
/// See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feConvolveMatrixElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeConvolveMatrix {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,
    /// See [`FeIn`]
    pub r#in: Animatable<FeIn>,
    /// See [`FeConvolveMatrixOrder`]
    pub order: Animatable<FeConvolveMatrixOrder>,
    /// The list of `number`s that make up the kernel matrix for the convolution. Values are separated by space
    /// characters and/or a comma. The number of entries in the list must equal `orderX` times `orderY`.
    pub kernel: Animatable<Vec<f32>>,

    /// After applying the ‘kernelMatrix’ to the input image to yield a number, that number is divided by ‘divisor’
    /// to yield the final destination color value. A divisor that is the sum of all the matrix values tends to have
    /// an evening effect on the overall color intensity of the result. It is an error to specify a divisor of zero.
    /// The default value is the sum of all values in kernelMatrix, with the exception that if the sum is zero, then
    /// the divisor is set to 1.
    pub divisor: Option<Animatable<f32>>,

    /// After applying the ‘kernelMatrix’ to the input image to yield a number and applying the ‘divisor’, the ‘bias’
    /// attribute is added to each component. One application of ‘bias’ is when it is desirable to have .5 gray value
    /// be the zero response of the filter. The bias property shifts the range of the filter. This allows representation
    /// of values that would otherwise be clamped to 0 or 1. If ‘bias’ is not specified, then the effect is as if a
    /// value of 0 were specified.
    pub bias: Animatable<f32>,

    /// After applying the ‘kernelMatrix’ to the input image to yield a number and applying the ‘divisor’, the ‘bias’
    /// attribute is added to each component. One application of ‘bias’ is when it is desirable to have .5 gray value
    /// be the zero response of the filter. The bias property shifts the range of the filter. This allows representation
    /// of values that would otherwise be clamped to 0 or 1. If ‘bias’ is not specified, then the effect is as if a
    /// value of 0 were specified.
    pub target_x: Animatable<u32>,

    /// Determines the positioning in Y of the convolution matrix relative to a given target pixel in the input image.
    /// The topmost row of the matrix is row number zero. The value must be such that: 0 <= targetY < orderY. By default,
    /// the convolution matrix is centered in Y over each pixel of the input image (i.e., targetY = floor ( orderY / 2 )).
    pub target_y: Animatable<u32>,

    /// The first number is the <dx> value. The second number is the <dy> value. If the <dy> value is not specified,
    /// it defaults to the same value as <dx>. Indicates the intended distance in current filter units (i.e., units
    /// as determined by the value of attribute ‘primitiveUnits’) between successive columns and rows, respectively,
    /// in the ‘kernelMatrix’. By specifying value(s) for ‘kernelUnitLength’, the kernel becomes defined in a
    /// scalable, abstract coordinate system. If ‘kernelUnitLength’ is not specified, the default value is one pixel
    /// in the offscreen bitmap, which is a pixel-based coordinate system, and thus potentially not scalable. For
    /// some level of consistency across display media and user agents, it is necessary that a value be provided for
    /// at least one of ‘filterRes’ and ‘kernelUnitLength’. In some implementations, the most consistent results and
    /// the fastest performance will be achieved if the pixel grid of the temporary offscreen images aligns with the
    /// pixel grid of the kernel.
    ///
    /// A negative or zero value is an error (see Error processing).
    pub kernel_unit_len: Option<Animatable<NumberOptNumber>>,

    /// A value of false indicates that the convolution will apply to all channels, including the alpha channel.
    ///
    /// See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feConvolveMatrixElement)
    pub preserve_alpha: Animatable<bool>,
}

/// See [`feConvolveMatrix`](https://www.w3.org/TR/SVG11/filters.html#feDiffuseLightingElement)
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeDiffuseLighting {
    /// common properties.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub primitive: FePrimitive,

    /// See [`FeIn`]
    pub r#in: Animatable<FeIn>,

    /// height of surface when Ain = 1.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub surface_scale: Animatable<f32>,

    /// kd in Phong lighting model. In SVG, this can be any non-negative number.
    ///
    /// If the attribute is not specified, then the effect is as if a value of 1 were specified.
    pub diffuse_constant: Animatable<f32>,

    /// The first number is the <dx> value. The second number is the <dy> value. If the <dy> value is not specified,
    /// it defaults to the same value as <dx>. Indicates the intended distance in current filter units (i.e., units
    /// as determined by the value of attribute ‘primitiveUnits’) for dx and dy, respectively, in the surface normal
    /// calculation formulas. By specifying value(s) for ‘kernelUnitLength’, the kernel becomes defined in a scalable,
    /// abstract coordinate system. If ‘kernelUnitLength’ is not specified, the dx and dy values should represent
    /// very small deltas relative to a given (x,y) position, which might be implemented in some cases as one pixel
    /// in the intermediate image offscreen bitmap, which is a pixel-based coordinate system, and thus potentially
    /// not scalable. For some level of consistency across display media and user agents, it is necessary that a value
    /// be provided for at least one of ‘filterRes’ and ‘kernelUnitLength’. Discussion of intermediate images are in the
    /// Introduction and in the description of attribute ‘filterRes’.
    ///
    /// A negative or zero value is an error (see Error processing).
    pub kernel_unit_len: Option<Animatable<NumberOptNumber>>,
}
