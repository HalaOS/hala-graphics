use crate::ir::{self, IR};

pub trait DrawContext {
    /// push a text element.
    fn push_text(&mut self, text: ir::Text);

    /// Push a new ir code.
    fn push_ir(&mut self, ir: IR);
}

/// A vector drawing element must implement this trait.
pub trait Element<C>
where
    C: DrawContext,
{
    /// Consume self and draw element with [`DrawContext`].
    fn draw(self, ctx: &mut C);
}

/// implement [`Element`] for all `FnOnce(&mut C)`
impl<F, C> Element<C> for F
where
    for<'a> F: FnOnce(&'a mut C),
    C: DrawContext,
{
    fn draw(self, ctx: &mut C) {
        self(ctx)
    }
}
