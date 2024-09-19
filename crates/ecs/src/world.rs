use std::{
    any::{Any, TypeId},
    collections::HashMap,
    u64,
};

use crate::{AsComponent, ComponentType, Id, ReferenceType, Sequence};

/// A entity component container.
#[derive(Default)]
struct Entity(Vec<Id>);

impl Entity {
    fn add_component(&mut self, new_id: Id) -> bool {
        let (_, component_type, reference_type) = new_id.to_parts();

        assert_eq!(
            reference_type,
            ReferenceType::Component,
            "add component to entity with wrong reference type: {:?}",
            reference_type
        );

        if self
            .0
            .iter()
            .any(|id| id.component_type() == component_type)
        {
            return false;
        }

        self.0.push(new_id);

        return true;
    }

    fn remove_component(&mut self, removed: Id) -> bool {
        if let Some(index) =
            self.0.iter().enumerate().find_map(
                |(index, id)| {
                    if *id == removed {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
        {
            self.0.swap_remove(index);

            true
        } else {
            false
        }
    }

    fn component(&self, component_type: &ComponentType) -> Option<Id> {
        self.0
            .iter()
            .find(|id| id.component_type() == *component_type)
            .cloned()
    }
}

struct Component {
    /// How many entities reference this component.
    ref_counter: usize,
    /// value.
    value: Box<dyn Any>,
}

impl Component {
    fn new<V>(value: V, ref_counter: usize) -> Self
    where
        V: 'static,
    {
        Self {
            ref_counter,
            value: Box::new(value),
        }
    }

    /// Return true if need no one reference this component.
    fn dec_ref(&mut self, delta: usize) -> bool {
        self.ref_counter -= delta;

        self.ref_counter == 0
    }

    fn add_ref(&mut self, delta: usize) {
        self.ref_counter += delta;
    }

    fn as_ref<V>(&self) -> &V
    where
        V: 'static,
    {
        self.value.downcast_ref::<V>().expect(&format!(
            "downcast component failed: {:?} => {:?}",
            self.value.type_id(),
            TypeId::of::<V>()
        ))
    }

    fn as_mut<V>(&mut self) -> &mut V
    where
        V: 'static,
    {
        self.value
            .downcast_mut::<V>()
            .expect("Invalid component type.")
    }
}

/// A world is a collection of entities.
#[derive(Default)]
pub struct World {
    /// seqence of seqence
    idgen: u64,
    /// entities
    entities: HashMap<Id, Entity>,
    /// component_type => component_id => component.
    component_types: HashMap<ComponentType, HashMap<Id, Component>>,
}

impl World {
    fn next_sequence(&mut self) -> Sequence {
        let seq = Sequence(self.idgen);
        self.idgen = self.idgen.wrapping_add(1);
        seq
    }

    fn attach_component_to_entity(&mut self, entity_id: &Id, component_id: Id) -> bool {
        if let Some(entity) = self.entities.get_mut(entity_id) {
            return entity.add_component(component_id);
        }

        return false;
    }

    fn detach_component_from_entity(&mut self, entity_id: &Id, component_id: Id) -> bool {
        if let Some(entity) = self.entities.get_mut(entity_id) {
            entity.remove_component(component_id);
            return true;
        }

        return false;
    }
}

impl World {
    /// Create a new `World` with supports component types.
    pub fn new<I>(component_types: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<ComponentType>,
    {
        let mut this = Self::default();

        for component_type in component_types {
            this.component_types
                .insert(component_type.as_ref().clone(), Default::default());
        }

        this
    }
    /// Create a new entity in this world.
    pub fn new_entity(&mut self) -> Id {
        let id = (
            self.next_sequence(),
            // can be any value.
            ComponentType::new(0),
            ReferenceType::Entity,
        )
            .into();

        self.entities.insert(id, Default::default());

        id
    }

    /// Remove entity and all associated component.
    pub fn remove_entity(&mut self, id: &Id) {
        if let Some(Entity(removed_components)) = self.entities.remove(&id) {
            for removed_component_id in removed_components {
                let component_type = removed_component_id.component_type();

                if let Some(components) = self.component_types.get_mut(&component_type) {
                    if let Some(component) = components.get_mut(&removed_component_id) {
                        if component.dec_ref(1) {
                            components.remove(&removed_component_id);
                        }
                    }
                }
            }
        }
    }

    /// Add a new component to entities.
    ///
    /// On success, returns the new component id.
    pub fn new_component<V, I>(&mut self, value: V, entities: I) -> (Id, usize)
    where
        V: AsComponent + 'static,
        I: IntoIterator,
        I::Item: AsRef<Id>,
    {
        let component_type = V::component_type().clone();

        self.new_component_with(&component_type, value, entities)
    }

    pub fn new_component_with<V, I>(
        &mut self,
        component_type: &ComponentType,
        value: V,
        entities: I,
    ) -> (Id, usize)
    where
        V: 'static,
        I: IntoIterator,
        I::Item: AsRef<Id>,
    {
        let component_id = (
            self.next_sequence(),
            component_type.clone(),
            ReferenceType::Component,
        )
            .into();

        let mut ref_counter = 0;

        for entity_id in entities.into_iter() {
            if self.attach_component_to_entity(entity_id.as_ref(), component_id) {
                ref_counter += 1;
            }
        }

        self.component_types
            .get_mut(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .insert(component_id, Component::new(value, ref_counter));

        (component_id, ref_counter)
    }

    /// Attach component to entities by component id.
    pub fn attach_component<I>(&mut self, component_id: &Id, entities: I) -> usize
    where
        I: IntoIterator,
        I::Item: AsRef<Id>,
    {
        assert_eq!(component_id.reference_type(), ReferenceType::Component);

        let mut ref_counter = 0;
        let component_type = component_id.component_type();

        if self
            .component_types
            .get_mut(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .contains_key(component_id)
        {
            for entity_id in entities.into_iter() {
                if self.attach_component_to_entity(entity_id.as_ref(), component_id.clone()) {
                    ref_counter += 1;
                }
            }

            self.component_types
                .get_mut(&component_type)
                .expect(&format!(
                    "No system to handle component: {}",
                    component_type
                ))
                .get_mut(component_id)
                .unwrap()
                .add_ref(ref_counter);

            return ref_counter;
        }

        return 0;
    }

    /// Detach component from entities by component id.
    pub fn detach_component<I>(&mut self, component_id: &Id, entities: I)
    where
        I: IntoIterator,
        I::Item: AsRef<Id>,
    {
        assert_eq!(component_id.reference_type(), ReferenceType::Component);

        let mut ref_counter = 0;
        let component_type = component_id.component_type();

        if self
            .component_types
            .get_mut(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .contains_key(component_id)
        {
            for entity_id in entities.into_iter() {
                if self.detach_component_from_entity(entity_id.as_ref(), component_id.clone()) {
                    ref_counter += 1;
                }
            }

            self.component_types
                .get_mut(&component_type)
                .expect(&format!(
                    "No system to handle component: {}",
                    component_type
                ))
                .get_mut(component_id)
                .unwrap()
                .dec_ref(ref_counter);
        }
    }

    /// Get attached component id by component type.
    pub fn attached_component(&self, entity_id: &Id, component_type: &ComponentType) -> Option<Id> {
        assert_eq!(entity_id.reference_type(), ReferenceType::Entity);

        self.entities.get(entity_id)?.component(component_type)
    }

    pub fn component_ref<V>(&self, component_id: &Id) -> Option<&V>
    where
        V: AsComponent + 'static,
    {
        assert_eq!(*V::component_type(), component_id.component_type());
        self.component_ref_unchecked(component_id)
    }

    pub fn component_ref_unchecked<V>(&self, component_id: &Id) -> Option<&V>
    where
        V: 'static,
    {
        assert_eq!(component_id.reference_type(), ReferenceType::Component);

        let component_type = component_id.component_type();

        self.component_types
            .get(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .get(component_id)
            .map(|c| c.as_ref())
    }

    pub fn component_mut<V>(&mut self, component_id: &Id) -> Option<&mut V>
    where
        V: AsComponent + 'static,
    {
        assert_eq!(*V::component_type(), component_id.component_type());
        self.component_mut_unchecked(component_id)
    }

    pub fn component_mut_unchecked<V>(&mut self, component_id: &Id) -> Option<&mut V>
    where
        V: 'static,
    {
        assert_eq!(component_id.reference_type(), ReferenceType::Component);

        let component_type = component_id.component_type();

        self.component_types
            .get_mut(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .get_mut(component_id)
            .map(|c| c.as_mut())
    }

    pub fn component_iter<V>(&self) -> impl Iterator<Item = &V>
    where
        V: AsComponent + 'static,
    {
        self.component_iter_with(V::component_type())
    }

    /// Returns a type of component iterator.
    pub fn component_iter_with<V>(&self, component_type: &ComponentType) -> impl Iterator<Item = &V>
    where
        V: 'static,
    {
        self.component_types
            .get(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .values()
            .map(|component| component.as_ref())
    }

    /// Returns a type of component iterator.
    pub fn component_iter_mut<V>(&mut self) -> impl Iterator<Item = &mut V>
    where
        V: AsComponent + 'static,
    {
        self.component_iter_mut_with(V::component_type())
    }

    /// Returns a type of component iterator.
    pub fn component_iter_mut_with<V>(
        &mut self,
        component_type: &ComponentType,
    ) -> impl Iterator<Item = &mut V>
    where
        V: 'static,
    {
        self.component_types
            .get_mut(&component_type)
            .expect(&format!(
                "No system to handle component: {}",
                component_type
            ))
            .values_mut()
            .map(|component| component.as_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component() {
        let mut component = Component::new(1usize, 1);

        assert_eq!(*component.as_ref::<usize>(), 1);

        *component.as_mut::<usize>() = 2;

        assert_eq!(*component.as_ref::<usize>(), 2);

        component.add_ref(1);

        component.dec_ref(1);

        assert!(component.dec_ref(1));
    }
}
