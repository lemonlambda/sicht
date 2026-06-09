pub mod anyvec_map;
pub mod map;

use std::any::TypeId;

pub use map::BirelationalMap;

pub trait BirelationalId<T> {
    fn get_id(&self) -> T;
}

impl BirelationalId<usize> for usize {
    fn get_id(&self) -> usize {
        *self
    }
}

macro_rules! impl_typeid {
    ($($type:ty),*) => {
        $(impl BirelationalId<TypeId> for $type {
            fn get_id(&self) -> TypeId {
                TypeId::of::<$type>()
            }
        })*
    };
}

impl_typeid! {
    isize,
    usize,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    &'_ str
}

// A <= AID => [B]
// B <= BID => [A]

// Entity <= EntityId => [Component]
// Component <= ComponentId => [Entity]
