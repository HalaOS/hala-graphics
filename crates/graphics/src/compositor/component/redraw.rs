use ecsrs::Id;

/// A component that contains redraw entity list.
pub struct RedrawComponent(pub Vec<Id>);
