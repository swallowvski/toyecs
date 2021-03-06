use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    ops,
};

use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rustc_hash::FxHashMap;

#[derive(Debug, Default)]
pub struct ResourceMap {
    cells: FxHashMap<TypeId, AtomicRefCell<AnyResource>>,
}

#[derive(Debug)]
struct AnyResource {
    any: Box<dyn Any>,
}

impl ResourceMap {
    pub fn insert<T: 'static>(&mut self, x: T) -> Option<T> {
        let new_cell = AtomicRefCell::new(AnyResource { any: Box::new(x) });
        let old_cell = self.cells.insert(TypeId::of::<T>(), new_cell)?;
        Some(Self::unwrap_res(old_cell.into_inner()))
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let old_cell = self.cells.remove(&TypeId::of::<T>())?;
        Some(Self::unwrap_res(old_cell.into_inner()))
    }

    fn unwrap_res<T: 'static>(res: AnyResource) -> T {
        let box_t = res.any.downcast::<T>().unwrap_or_else(|_| unreachable!());
        *box_t
    }

    fn contains<T: 'static>(&self) -> bool {
        self.cells.contains_key(&TypeId::of::<T>())
    }

    pub fn borrow<T: 'static>(&self) -> Option<Res<T>> {
        let cell = self.cells.get(&TypeId::of::<T>())?;
        let borrow = AtomicRef::map(cell.borrow(), |res| res.any.downcast_ref::<T>().unwrap());
        Some(Res { borrow })
    }

    pub fn borrow_mut<T: 'static>(&self) -> Option<ResMut<T>> {
        let cell = self.cells.get(&TypeId::of::<T>())?;
        let borrow = AtomicRefMut::map(cell.borrow_mut(), |res| {
            res.any
                .downcast_mut::<T>()
                .unwrap_or_else(|| unreachable!())
        });
        Some(ResMut { borrow })
    }
}

#[derive(Debug)]
pub struct Res<'r, T> {
    borrow: AtomicRef<'r, T>,
}

impl<'r, T> ops::Deref for Res<'r, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

#[derive(Debug)]
pub struct ResMut<'r, T> {
    borrow: AtomicRefMut<'r, T>,
}

impl<'r, T> ops::Deref for ResMut<'r, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.borrow.deref()
    }
}

impl<'r, T> ops::DerefMut for ResMut<'r, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow.deref_mut()
    }
}
