/// Variant for render object type.
#[repr(u8)]
pub enum RenderType {
    Transform,
    Canvas,
}

/// Variant for one id's type.
#[repr(u8)]
pub enum IdType {
    Entity,
    Component,
    System,
}
