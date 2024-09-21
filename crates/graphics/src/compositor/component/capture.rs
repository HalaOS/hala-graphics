use ecsrs::Id;

/// The component that support capture rendering element.
pub struct CaptureComponent {
    /// The rendering elements entity id that need be captured.
    pub next_frame: Option<Vec<Id>>,
}
