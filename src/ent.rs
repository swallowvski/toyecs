use std::slice;

use crate::sparse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Entity(pub(crate) SparseIndex);

impl Entity {
    fn initial(slot: RawSparseIndex) -> Self {
        Self(SparseIndex::initial(slot))
    }

    pub fn generation(&self) -> Generation {
        self.0.generation()
    }
}

#[derive(Debug, Default)]
pub struct EntityPool {
    entries: Vec<Entry>,
    data: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Entry {
    ToDense(DenceIndex),
    Empty { gen: Generation },
}

impl EntityPool {
    pub fn slice(&self) -> &[Entity] {
        &self.data
    }

    pub fn iter(&self) -> slice::Iter<Entity> {
        self.data.iter()
    }

    pub fn alloc(&mut self) -> Entity {
        if self.data.len() >= self.entries.len() {
            debug_assert_eq!(self.data.len(), self.entries.len());

            let slot = self.data.len();
            let entity = Entity::initial(RawSparseIndex::from_usize(slot));

            self.data.push(entity.clone());

            self.entries.push(Entry::ToDense(DenceIndex::initial(
                RawDenseIndex::from_usize(slot),
            )));
            entity
        } else {
            let (i_entry, gen) = self.find_empty_entry();
            let gen = gen.increment();

            let dense_slot = self.data.len();
            let entity = Entity(SparseIndex::new(
                RawSparseIndex::from_usize(dense_slot),
                gen,
            ));

            self.data.push(entity.clone());
            self.entries[i_entry] =
                Entry::ToDense(DenceIndex::new(RawDenseIndex::from_usize(dense_slot), gen));
            entity
        }
    }

    pub fn find_empty_entry(&self) -> (usize, &Generation) {
        for (i, entry) in self.entries.iter().enumerate() {
            if let Entry::Empty { gen: g } = entry {
                return (i, g);
            }
        }

        unreachable!()
    }

    pub fn dealloc(&mut self, ent: Entity) -> bool {
        let slot = ent.0.to_usize();
        if slot > self.entries.len() - 1 {
            return false;
        }

        let dense = match self.entries[slot] {
            Entry::ToDense(e) => e,
            Entry::Empty { .. } => return false,
        };

        if dense.generation() == ent.generation() {
            self.entries[slot] = Entry::Empty {
                gen: ent.generation(),
            };
            self.data.remove(dense.to_usize());
            true
        } else {
            false
        }
    }
}
