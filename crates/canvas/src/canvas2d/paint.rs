#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Color {
    /// Red component of the color
    pub r: f64,
    /// Green component of the color
    pub g: f64,
    /// Blue component of the color
    pub b: f64,
    /// Alpha component of the color
    pub a: f64,
}

/// A description of the style to use when drawing on a Canvas.
///
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Paint {
    /// A blend mode to apply when a shape is drawn or a layer is composited.
    pub blend_mode: BlendMode,

    /// The color to use when stroking or filling a shape.
    pub color: Color,

    /// Controls the performance vs quality trade-off to use when sampling bitmaps
    pub filter_quality: FilterQuality,

    /// Whether the colors of the image are inverted when drawn.
    pub invert_color: bool,

    /// Whether to apply anti-aliasing to lines and images drawn on the canvas.
    pub is_anti_alias: bool,

    /// The stroke config when style is set to [`PaintingStyle.Stroke`]
    pub stroke: Option<PaintStroke>,
}

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PaintStroke {
    /// The kind of finish to place on the end of lines drawn when style is set to [`PaintingStyle.Stroke`].
    pub stroke_cap: StrokeCap,

    /// The kind of finish to place on the joins between segments.
    pub stroke_join: StrokeJoin,

    /// The limit for miters to be drawn on segments when the join is set to StrokeJoin.miter and the style is
    /// set to [`PaintingStyle::Stroke`]. If this limit is exceeded, then a [`StrokeJoin::Bevel`] join will be drawn instead.
    /// This may cause some 'popping' of the corners of a path if the angle between line segments is animated,
    /// as seen in the diagrams below.
    pub stroke_miter_limit: f32,

    /// How wide to make edges drawn when style is set to PaintingStyle.stroke.
    /// The width is given in logical pixels measured in the direction orthogonal
    /// to the direction of the path.
    pub stroke_width: f32,
}

/// Strategies for painting shapes and paths on a canvas.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PaintStyle {
    /// Apply the Paint to the inside of the shape. For example, when applied to the [`draw_circle`](super::PictureLayer::draw_circle) call,
    /// this results in a disc of the given size being painted.
    Fill,

    /// Apply the Paint to the edge of the shape. For example, when applied to the [`draw_circle`](super::PictureLayer::draw_circle) call,
    /// this results is a hoop of the given size being painted. The line drawn on the edge will be
    /// the width given by the Paint.strokeWidth property.
    Stroke,
}

impl Default for PaintStyle {
    fn default() -> Self {
        PaintStyle::Stroke
    }
}

/// Styles to use for line segment joins.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum StrokeJoin {
    /// Joins between line segments form sharp corners.
    Miter,

    /// Joins between line segments are semi-circular.
    Round,

    /// Joins between line segments connect the corners of the butt
    /// ends of the line segments to give a beveled appearance.
    Bevel,
}

impl Default for StrokeJoin {
    fn default() -> Self {
        StrokeJoin::Miter
    }
}

/// Styles to use for line endings.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum StrokeCap {
    /// Begin and end contours with a flat edge and no extension.
    Butt,

    /// Begin and end contours with a semi-circle extension.
    Round,

    /// Begin and end contours with a half square extension.
    /// This is similar to extending each contour by half the stroke width (as given by Paint.strokeWidth).
    Square,
}

impl Default for StrokeCap {
    fn default() -> Self {
        StrokeCap::Butt
    }
}

/// Quality levels for image sampling in ImageFilter and Shader objects that sample images and for Canvas operations that render images.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum FilterQuality {
    /// The fastest filtering method, albeit also the lowest quality.
    ///
    /// This value results in a "Nearest Neighbor" algorithm which just repeats or eliminates pixels as an image is scaled up or down.
    None,

    /// Better quality than none, faster than medium.
    /// This value results in a "Bilinear" algorithm which smoothly interpolates between pixels in an image.
    Low,

    /// The best all around filtering method that is only worse than high at extremely large scale factors.
    ///
    /// This value improves upon the "Bilinear" algorithm specified by low by utilizing a Mipmap that pre-computes
    /// high quality lower resolutions of the image at half (and quarter and eighth, etc.) sizes and then blends
    /// between those to prevent loss of detail at small scale sizes.
    Medium,

    /// Best possible quality when scaling up images by scale factors larger than 5-10x.
    /// When images are scaled down, this can be worse than medium for scales smaller than 0.5x, or when animating the scale factor.
    ///
    /// This option is also the slowest.
    ///
    /// This value results in a standard "Bicubic" algorithm which uses a 3rd order equation to smooth the abrupt transitions between
    /// pixels while preserving some of the sense of an edge and avoiding sharp peaks in the result.
    Hight,
}

impl Default for FilterQuality {
    fn default() -> Self {
        FilterQuality::None
    }
}

/// A blend mode to apply when a shape is drawn or a layer is composited.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum BlendMode {
    /// Drop both the source and destination images, leaving nothing.
    /// This corresponds to the "clear" Porter-Duff operator.
    Clear,
    /// Drop the destination image, only paint the source image.
    /// Conceptually, the destination is first cleared, then the source image is painted.
    /// This corresponds to the "Copy" Porter-Duff operator.
    Src,
    /// Drop the source image, only paint the destination image.
    /// Conceptually, the source image is discarded, leaving the destination untouched.
    /// This corresponds to the "Destination" Porter-Duff operator.
    Dst,
    /// Composite the source image over the destination image.
    /// This is the default value. It represents the most intuitive case,
    /// where shapes are painted on top of what is below,
    ///  with transparent areas showing the destination layer.
    ///
    /// This corresponds to the "Source over Destination" Porter-Duff operator,
    /// also known as the Painter's Algorithm.
    SrcOver,
    /// Composite the source image under the destination image.
    /// This is the opposite of srcOver.
    /// This corresponds to the "Destination over Source" Porter-Duff operator.
    DstOver,
    /// Show the source image, but only where the two images overlap.
    /// The destination image is not rendered, it is treated merely as a mask.
    /// The color channels of the destination are ignored, only the opacity has an effect.
    ///
    /// To show the destination image instead, consider [`DstIn`](BlendMode::DstIn).
    ///
    /// To reverse the semantic of the mask (only showing the source where the destination is absent,
    /// rather than where it is present), consider [`SrcOut`](BlendMode::SrcOut).
    ///
    /// This corresponds to the "Source in Destination" Porter-Duff operator.
    SrcIn,
    /// Show the destination image, but only where the two images overlap.
    /// The source image is not rendered, it is treated merely as a mask.
    /// The color channels of the source are ignored, only the opacity has an effect.
    ///
    /// To show the source image instead, consider [`SrcIn`](BlendMode::SrcIn).
    ///
    /// To reverse the semantic of the mask (only showing the source where the destination is present,
    /// rather than where it is absent), consider dstOut.
    ///
    /// This corresponds to the "Destination in Source" Porter-Duff operator.
    DstIn,
    /// Show the source image, but only where the two images do not overlap.
    /// The destination image is not rendered, it is treated merely as a mask.
    /// The color channels of the destination are ignored, only the opacity has an effect.
    ///
    /// To show the destination image instead, consider [`DstOut`](BlendMode::DstOut).
    ///
    /// To reverse the semantic of the mask (only showing the source where the destination is present,
    /// rather than where it is absent), consider [`SrcIn](BlendMode::SrcIn).
    ///
    /// This corresponds to the "Source out Destination" Porter-Duff operator.
    SrcOut,
    /// Show the destination image, but only where the two images do not overlap.
    /// The source image is not rendered, it is treated merely as a mask.
    /// The color channels of the source are ignored, only the opacity has an effect.
    ///
    /// To show the source image instead, consider [`SrcOut`](BlendMode::SrcOut).
    ///
    /// To reverse the semantic of the mask (only showing the destination where the source is present,
    /// rather than where it is absent), consider [`DstIn`](BlendMode::DstIn).
    ///
    /// This corresponds to the "Destination out Source" Porter-Duff operator.
    DstOut,

    /// Composite the source image over the destination image, but only where it overlaps the destination.
    ///
    /// This corresponds to the "Source atop Destination" Porter-Duff operator.
    ///
    /// This is essentially the [`SrcOver](BlendMode::SrcOver) operator,
    /// but with the output's opacity channel being set to that of the destination image instead of being
    /// a combination of both image's opacity channels.
    SrcATop,
    /// Composite the destination image over the source image, but only where it overlaps the source.
    ///
    /// This corresponds to the "Destination atop Source" Porter-Duff operator.
    ///
    /// This is essentially the [`DstOver](BlendMode::DstOver) operator, but with the output's opacity channel being set to that
    /// of the source image instead of being a combination of both image's opacity channels.
    ///
    /// For a variant with the source on top instead of the destination, see [`SrcATop](BlendMode::SrcATop).
    DstATop,
    /// Apply a bitwise xor operator to the source and destination images. This leaves transparency where they would overlap.
    ///
    /// This corresponds to the "Source xor Destination" Porter-Duff operator.
    Xor,
    /// Sum the components of the source and destination images.
    ///
    /// Transparency in a pixel of one of the images reduces the contribution of that image to the corresponding output pixel,
    /// as if the color of that pixel in that image was darker.
    ///
    /// This corresponds to the "Source plus Destination" Porter-Duff operator.
    Plus,

    /// Multiply the color components of the source and destination images.
    ///
    /// This can only result in the same or darker colors (multiplying by white, 1.0, results in no change;
    /// multiplying by black, 0.0, results in black).
    ///
    /// When compositing two opaque images, this has similar effect to overlapping two transparencies on a projector.
    ///
    /// For a variant that also multiplies the alpha channel, consider [`Multiply`](BlendMode::Multiply).
    Modulate,

    /// Multiply the inverse of the components of the source and destination images, and inverse the result.
    ///
    /// Inverting the components means that a fully saturated channel (opaque white) is treated as the value 0.0,
    /// and values normally treated as 0.0 (black, transparent) are treated as 1.0.
    ///
    /// This is essentially the same as modulate blend mode, but with the values of the colors inverted before
    /// the multiplication and the result being inverted back before rendering.
    ///
    /// This can only result in the same or lighter colors (multiplying by black, 1.0, results in no change;
    /// multiplying by white, 0.0, results in white). Similarly, in the alpha channel, it can only result in
    /// more opaque colors.
    ///
    /// This has similar effect to two projectors displaying their images on the same screen simultaneously.
    Screen,

    /// Multiply the components of the source and destination images after adjusting them to favor the destination.
    ///
    /// Specifically, if the destination value is smaller, this multiplies it with the source value, whereas is the
    /// source value is smaller, it multiplies the inverse of the source value with the inverse of the destination
    /// value, then inverts the result.
    ///
    /// Inverting the components means that a fully saturated channel (opaque white) is treated as the value 0.0, and
    /// values normally treated as 0.0 (black, transparent) are treated as 1.0.
    Overlay,

    /// Composite the source and destination image by choosing the lowest value from each color channel.
    ///
    /// The opacity of the output image is computed in the same way as for [`SrcOver](BlendMode::SrcOver).
    Darken,

    /// Composite the source and destination image by choosing the highest value from each color channel.
    ///
    /// The opacity of the output image is computed in the same way as for [`SrcOver](BlendMode::SrcOver).
    Lighten,

    /// Divide the destination by the inverse of the source.
    ///
    /// Inverting the components means that a fully saturated channel (opaque white) is treated as the value 0.0,
    /// and values normally treated as 0.0 (black, transparent) are treated as 1.0.
    ColorDodge,

    /// Divide the inverse of the destination by the source, and inverse the result.
    ///
    /// Inverting the components means that a fully saturated channel (opaque white) is treated as the value 0.0,
    /// and values normally treated as 0.0 (black, transparent) are treated as 1.0.
    ColorBurn,

    /// Multiply the components of the source and destination images after adjusting them to favor the source.
    ///
    /// Specifically, if the source value is smaller, this multiplies it with the destination value, whereas
    /// is the destination value is smaller, it multiplies the inverse of the destination value with the inverse
    /// of the source value, then inverts the result.
    ///
    /// Inverting the components means that a fully saturated channel (opaque white) is treated as the value 0.0,
    /// and values normally treated as 0.0 (black, transparent) are treated as 1.0.
    HardLight,

    /// Use [`ColorDodge](BlendMode::ColorDodge) for source values below 0.5 and colorBurn for source values above 0.5.
    ///
    /// This results in a similar but softer effect than [`Overlay`](BlendMode::Overlay).
    SoftLight,

    /// Subtract the smaller value from the bigger value for each channel.
    ///
    /// Compositing black has no effect; compositing white inverts the colors of the other image.
    ///
    /// The opacity of the output image is computed in the same way as for [`SrcOver`](BlendMode::SrcOver).
    ///
    /// The effect is similar to exclusion but harsher.
    Difference,

    /// Subtract double the product of the two images from the sum of the two images.
    ///
    /// Compositing black has no effect; compositing white inverts the colors of the other image.
    ///
    /// The opacity of the output image is computed in the same way as for [`SrcOver](BlendMode::SrcOver).
    ///
    /// The effect is similar to [`Difference`](BlendMode::Difference) but softer.
    Exclusion,

    /// Multiply the components of the source and destination images, including the alpha channel.
    ///
    /// This can only result in the same or darker colors (multiplying by white, 1.0, results in no change;
    /// multiplying by black, 0.0, results in black).
    ///
    /// Since the alpha channel is also multiplied, a fully-transparent pixel (opacity 0.0) in one image
    /// results in a fully transparent pixel in the output. This is similar to [`DstIn](BlendMode::DstIn), but with the colors combined.
    ///
    /// For a variant that multiplies the colors but does not multiply the alpha channel, consider [`Modulate`](BlendMode::Modulate).
    Multiply,

    /// Take the hue of the source image, and the saturation and luminosity of the destination image.
    ///
    /// The effect is to tint the destination image with the source image.
    ///
    /// The opacity of the output image is computed in the same way as for [`SrcOver`](BlendMode::SrcOver).
    /// Regions that are entirely transparent in the source image take their hue from the destination.
    Hue,

    /// Take the saturation of the source image, and the hue and luminosity of the destination image.
    ///
    /// The opacity of the output image is computed in the same way as for [`SrcOver`](BlendMode::SrcOver).
    /// Regions that are entirely transparent in the source image take their saturation from the destination.
    Saturation,

    /// Take the hue and saturation of the source image, and the luminosity of the destination image.
    ///
    /// The effect is to tint the destination image with the source image.
    ///
    /// The opacity of the output image is computed in the same way as for srcOver.
    /// Regions that are entirely transparent in the source image take their hue and
    /// saturation from the destination.
    Color,

    /// Take the luminosity of the source image, and the hue and saturation of the destination image.
    ///
    /// The opacity of the output image is computed in the same way as for srcOver.
    /// Regions that are entirely transparent in the source image take their luminosity from the destination.
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::SrcOver
    }
}
