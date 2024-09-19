mod primitives;
mod world;
pub use primitives::*;
pub use world::*;

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn test_id_cast() {
        let id: Id = (Sequence(1), ComponentType::new(1), ReferenceType::Component).into();

        assert_eq!(
            id.to_parts(),
            (Sequence(1), ComponentType::new(1), ReferenceType::Component)
        );
    }

    #[test]
    fn test_world() {
        static C1: ComponentType = ComponentType::new(1);
        static C2: ComponentType = ComponentType::new(2);
        static C3: ComponentType = ComponentType::new(3);

        let mut world = World::new(&[C1, C2, C3]);

        let entities = [
            world.new_entity(),
            world.new_entity(),
            world.new_entity(),
            world.new_entity(),
        ]
        .to_vec();

        let (component_id, adds) = world.new_component_with(&C1, 1usize, &entities[..2]);

        assert_eq!(adds, 2);

        // do nothing.
        assert_eq!(world.attach_component(&component_id, &entities[..2]), 0);

        assert_eq!(world.attached_component(&entities[2], &C1), None);

        assert_eq!(world.attach_component(&component_id, &entities[2..]), 2);

        assert_eq!(
            *world
                .component_ref_unchecked::<usize>(&component_id)
                .unwrap(),
            1
        );

        *world
            .component_mut_unchecked::<usize>(&component_id)
            .unwrap() = 4;

        assert_eq!(
            *world
                .component_ref_unchecked::<usize>(&component_id)
                .unwrap(),
            4
        );

        assert_eq!(
            world.attached_component(&entities[0], &C1),
            Some(component_id)
        );

        assert_eq!(
            world.attached_component(&entities[2], &C1),
            Some(component_id)
        );

        assert_eq!(world.attached_component(&entities[0], &C2), None);
        assert_eq!(world.attached_component(&entities[0], &C3), None);
    }
}
