use std::fmt::Display;

/// A seqence number part of the [`Id`] type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sequence(pub(crate) u64);

impl Sequence {
    /// Create a new [`Sequence`] from `u64` number.
    ///
    /// The input value must be smaller than or equal to `2^48`
    pub const fn new(id: u64) -> Self {
        const MAX: u64 = 0xffffffffffff;

        if id > MAX {
            panic!("ComponentType: out of range");
        }

        Self(id)
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sequence({})", self.0)
    }
}

/// The type id of one component type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentType(u16);

impl ComponentType {
    pub const fn new(id: u16) -> Self {
        if id > !(0b1 << 15) {
            panic!("ComponentType: out of range");
        }

        Self(id)
    }
}

impl AsRef<ComponentType> for ComponentType {
    fn as_ref(&self) -> &ComponentType {
        &self
    }
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ComponentType({})", self.0)
    }
}

/// The variant for two types of references.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceType {
    /// For entity reference. this is the default type.
    Entity = 0,
    /// For component instance reference.
    Component = 1,
}

impl From<u8> for ReferenceType {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value & 0b1) }
    }
}

impl Default for ReferenceType {
    fn default() -> Self {
        Self::Entity
    }
}

/// A reference id to reference one object in the ecs world.
///
/// # Memory layout
///
/// - `ReferenceType`: [0~1) bits.
/// - `ComponentType`: [1~15)bits.
/// - `Sequence`: (16~64)bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u64);

impl AsRef<Id> for Id {
    fn as_ref(&self) -> &Id {
        &self
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reference_type() {
            ReferenceType::Entity => write!(f, "entity({})", self.sequence().0),
            ReferenceType::Component => write!(
                f,
                "component({},{})",
                self.sequence().0,
                self.component_type().0
            ),
        }
    }
}

impl From<(Sequence, ComponentType, ReferenceType)> for Id {
    fn from(value: (Sequence, ComponentType, ReferenceType)) -> Self {
        let seq = value.0 .0 << 16;
        let c_type = (value.1 .0 << 2) as u64;
        let r_type = value.2 as u64;

        Self(seq + c_type + r_type)
    }
}

impl Id {
    /// Split self into parts([`Sequence`],[`ComponentType`],[`ReferenceType`]).
    pub fn to_parts(self) -> (Sequence, ComponentType, ReferenceType) {
        (
            self.sequence(),
            self.component_type(),
            self.reference_type(),
        )
    }

    /// Get id's ComponentType part.
    pub fn sequence(&self) -> Sequence {
        let seq = self.0 >> 16;

        Sequence(seq)
    }

    /// Get id's ComponentType part.
    pub fn component_type(&self) -> ComponentType {
        let c_type = (self.0 & (u16::MAX as u64)) >> 2;

        ComponentType(c_type as u16)
    }

    /// Get id's reference_type part.
    pub fn reference_type(&self) -> ReferenceType {
        (self.0 as u8).into()
    }
}

/// A component rust type must implement this trait.
pub trait AsComponent {
    /// Returns the component type.
    fn component_type() -> &'static ComponentType;
}

#[macro_export]
macro_rules! ecs_system {
    (@step $_idx:expr,) => {};

    (@step $idx:expr, $head:tt, $($tail:tt,)*) => {
        impl $crate::AsComponent for $head {
            fn component_type() -> &'static $crate::ComponentType {
                static T: $crate::ComponentType = $crate::ComponentType::new($idx as u16);

                return &T;
            }
        }

        ecs_system!(@step $idx + 1usize, $($tail,)*);
    };

    ($($n:tt),*) => {
        ecs_system!(@step 0usize, $($n,)*);
    }
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use super::{ComponentType, Sequence};

    #[test]
    fn test_sequnce_out_of_range() {
        catch_unwind(|| Sequence::new(2u64.pow(48) - 1)).expect("not out of range");
        catch_unwind(|| Sequence::new(2u64.pow(48))).expect_err("out of range");
    }

    #[test]
    fn test_component_type_out_of_range() {
        catch_unwind(|| ComponentType::new(2u16.pow(15) - 1)).expect("not out of range");
        catch_unwind(|| ComponentType::new(2u16.pow(15))).expect_err("out of range");
    }

    #[test]
    fn test_ecs_system_macro() {
        struct A;
        struct B;
        struct C;

        ecs_system!(A, B, C);

        use super::AsComponent;

        assert_eq!(*A::component_type(), ComponentType::new(0));
        assert_eq!(*B::component_type(), ComponentType::new(1));
        assert_eq!(*C::component_type(), ComponentType::new(2));
    }
}
