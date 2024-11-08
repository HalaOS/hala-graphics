use crate::{Drawing, Renderer};

/// Creates an entity that will not be rendered until it is referenced in the render tree.
pub fn entity<'a, R, D, E>(name: &'a str, child: D) -> impl Fn(&mut R) -> Result<(), E> + 'a
where
    R: Renderer,
    D: Drawing<R, Error = E> + 'a,
{
    move |renderer| {
        renderer.push_entity(name);

        child.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}

/// Attach a entity reference into rendering tree.
pub fn entity_ref<R, E>(name: &str) -> impl Fn(&mut R) -> Result<(), E> + '_
where
    R: Renderer,
{
    // let name = name.to_owned();

    move |renderer| {
        renderer.entity_ref(name);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{combinator::circle, Drawing, MockRenderer};

    use super::{entity, entity_ref};

    #[test]
    fn test_entity() {
        (
            entity("hello", circle((20.0, 20.0), 10.0)),
            entity_ref("hello"),
        )
            .render(&mut MockRenderer::default())
            .unwrap();
    }
}
