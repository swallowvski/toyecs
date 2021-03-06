use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    ops,
    slice::{self, SliceIndex},
};

use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;

use crate::{
    ent::Entity,
    sparse::{self, SparseIndex, SparseSet},
};

#[derive(Debug, Default)]
pub struct ComponentPoolMap {
    cells: FxHashMap<TypeId, AtomicRefCell<AnyPool>>,
}

#[derive(Debug)]
struct AnyPool {
    any: Box<dyn Any>,
}

impl ComponentPoolMap {
    pub fn is_registered<T: 'static>(&self) -> bool {
        let ty = TypeId::of::<T>();
        self.cells.contains_key(&ty)
    }

    pub fn register<T: 'static>(&mut self) -> bool {
        let ty = TypeId::of::<T>();
        if self.cells.contains_key(&ty) {
            return true;
        }

        let pool = AnyPool {
            any: Box::new(ComponentPool::<T>::default()),
        };

        self.cells.insert(ty, AtomicRefCell::new(pool));
        false
    }

    pub fn borrow<T: 'static>(&self) -> Option<Comp<T>> {
        let cell = self.cells.get(&TypeId::of::<T>())?;
        let borrow = AtomicRef::map(cell.borrow(), |pool| {
            pool.any.downcast_ref::<ComponentPool<T>>().unwrap()
        });
        Some(Comp { borrow })
    }

    pub fn borrow_mut<T: 'static>(&self) -> Option<CompMut<T>> {
        let cell = self.cells.get(&TypeId::of::<T>())?;
        let borrow = AtomicRefMut::map(cell.borrow_mut(), |pool| {
            pool.any
                .downcast_mut::<ComponentPool<T>>()
                .unwrap_or_else(|| unreachable!())
        });
        Some(CompMut { borrow })
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        let cell = self.cells.get_mut(&TypeId::of::<T>())?;
        Some(cell.get_mut().any.downcast_mut().unwrap())
    }
}

#[derive(Debug)]
pub struct ComponentPool<T> {
    set: SparseSet<T>,
}

impl<T> Default for ComponentPool<T> {
    fn default() -> Self {
        Self {
            set: Default::default(),
        }
    }
}

impl<T> ComponentPool<T> {
    pub fn contains(&self, ent: Entity) -> bool {
        self.set.contains(ent.0)
    }

    pub fn get(&self, ent: Entity) -> Option<&T> {
        self.set.get(ent.0)
    }

    pub fn get_mut(&mut self, ent: Entity) -> Option<&mut T> {
        self.set.get_mut(ent.0)
    }

    pub fn as_slice(&self) -> &[T] {
        self.set.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.set.as_mut_slice()
    }

    pub fn entity(&self) -> &[Entity] {
        Self::to_entities(self.set.indices())
    }

    pub fn as_slice_with_entities(&self) -> (&[Entity], &[T]) {
        let (sparse, comps) = self.set.as_slice_with_indices();
        (Self::to_entities(sparse), comps)
    }

    fn to_entities(sparse: &[SparseIndex]) -> &[Entity] {
        unsafe { slice::from_raw_parts(sparse as *const _ as *const _, sparse.len()) }
    }

    pub(crate) fn insert(&mut self, ent: Entity, comp: T) -> Option<T> {
        self.set.insert(ent.0, comp)
    }

    pub(crate) fn swap_remove(&mut self, ent: Entity) -> Option<T> {
        self.set.swap_remove(ent.0)
    }
}

impl<T> AsRef<[T]> for ComponentPool<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for ComponentPool<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

#[derive(Debug)]
pub struct Comp<'r, T> {
    borrow: AtomicRef<'r, ComponentPool<T>>,
}

impl<'r, T> ops::Deref for Comp<'r, T> {
    type Target = ComponentPool<T>;
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

#[derive(Debug)]
pub struct CompMut<'r, T> {
    borrow: AtomicRefMut<'r, ComponentPool<T>>,
}

impl<'r, T> ops::Deref for CompMut<'r, T> {
    type Target = ComponentPool<T>;
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'r, T> ops::DerefMut for CompMut<'r, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow.deref_mut()
    }
}
