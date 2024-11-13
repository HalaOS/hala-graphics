use std::fmt::Display;

/// see [`https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute`]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum MeetOrSlice {
    Meet,
    Slice,
}

impl Display for MeetOrSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MeetOrSlice::Meet => write!(f, "meet"),
            MeetOrSlice::Slice => write!(f, "slice"),
        }
    }
}

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
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMinYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMidYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMaxYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMinYMid(MeetOrSlice),
    /// Force uniform scaling(the default).
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMidYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMaxYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMinYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMidYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMaxYMax(MeetOrSlice),
}

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self::xMidYMid(MeetOrSlice::Meet)
    }
}

impl Display for PreserveAspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreserveAspectRatio::xMinYMin(meet_or_slice) => write!(f, "xMinYMin {}", meet_or_slice),
            PreserveAspectRatio::xMidYMin(meet_or_slice) => write!(f, "xMidYMin {}", meet_or_slice),
            PreserveAspectRatio::xMaxYMin(meet_or_slice) => write!(f, "xMaxYMin {}", meet_or_slice),
            PreserveAspectRatio::xMinYMid(meet_or_slice) => write!(f, "xMinYMid {}", meet_or_slice),
            PreserveAspectRatio::xMidYMid(meet_or_slice) => write!(f, "xMidYMid {}", meet_or_slice),
            PreserveAspectRatio::xMaxYMid(meet_or_slice) => write!(f, "xMaxYMid {}", meet_or_slice),
            PreserveAspectRatio::xMinYMax(meet_or_slice) => write!(f, "xMinYMax {}", meet_or_slice),
            PreserveAspectRatio::xMidYMax(meet_or_slice) => write!(f, "xMidYMax {}", meet_or_slice),
            PreserveAspectRatio::xMaxYMax(meet_or_slice) => write!(f, "xMaxYMax {}", meet_or_slice),
        }
    }
}
