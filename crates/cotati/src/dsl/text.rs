use crate::ir::{self, IR};

use super::{DrawContext, Element};

/// Create a new text element builder.
pub fn text() -> ir::Text {
    ir::Text::default()
}

impl ir::Text {
    /// use a literal string value to create a new text [`Element`]
    pub fn content<C, S>(self, value: S) -> impl Element<C>
    where
        C: DrawContext,
        S: ToOwned<Owned = String>,
    {
        move |ctx: &mut C| {
            ctx.push_text(self);
            ctx.push_ir(IR::String(value.to_owned()));
        }
    }

    /// use a animatable variable to create a new text [`Element`]
    pub fn animatable_content<C, S>(self, variable: S) -> impl Element<C>
    where
        C: DrawContext,
        S: ToOwned<Owned = String>,
    {
        move |ctx: &mut C| {
            ctx.push_text(self);
            ctx.push_ir(IR::Animatable(variable.to_owned()));
        }
    }
}
