use crate::{Transform2D, Transform3D};

/// A component that contains rendering element transform2d data.
pub struct TransformComponent2D(pub Transform2D);

/// A component that contains rendering element transform3d data.
pub struct TransformComponent3D(pub Transform3D);
