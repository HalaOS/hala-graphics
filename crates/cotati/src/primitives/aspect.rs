/// In some cases, typically when using the ‘viewBox’ attribute, i
/// t is desirable that the graphics stretch to fit non-uniformly
/// to take up the entire viewport. In other cases, it is desirable
/// that uniform scaling be used for the purposes of preserving
/// the aspect ratio of the graphics.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PreserveAspectRatio {
    /// Force uniform scaling
    ///
    /// Align the <min-x> of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the <min-y> of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMinYMin,
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the <min-y> of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMidYMin,
    /// Force uniform scaling.
    ///
    /// Align the <min-x>+<width> of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the <min-y> of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMaxYMin,
    /// Force uniform scaling.
    ///
    /// Align the <min-x> of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMinYMid,
    /// Force uniform scaling(the default).
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMidYMid,
    /// Force uniform scaling.
    ///
    /// Align the <min-x>+<width> of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMaxYMid,
    /// Force uniform scaling.
    ///
    /// Align the <min-x> of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the <min-y>+<height> of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMinYMax,
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the <min-y>+<height> of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMidYMax,
    /// Force uniform scaling.
    ///
    /// Align the <min-x>+<width> of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the <min-y>+<height> of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMaxYMax,
}

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self::xMidYMid
    }
}
