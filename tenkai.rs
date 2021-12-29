#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod res {
    use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
    use rustc_hash::FxHashMap;
    use std::{
        any::{Any, TypeId},
        borrow::Borrow,
        ops,
    };
    pub struct ResourceMap {
        cells: FxHashMap<TypeId, AtomicRefCell<AnyResource>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for ResourceMap {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ResourceMap {
                    cells: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ResourceMap");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "cells",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for ResourceMap {
        #[inline]
        fn default() -> ResourceMap {
            ResourceMap {
                cells: ::core::default::Default::default(),
            }
        }
    }
    struct AnyResource {
        any: Box<dyn Any>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for AnyResource {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                AnyResource {
                    any: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "AnyResource");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "any",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
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
            let box_t = res.any.downcast::<T>().unwrap_or_else(|_| {
                ::core::panicking::panic("internal error: entered unreachable code")
            });
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
                res.any.downcast_mut::<T>().unwrap_or_else(|| {
                    ::core::panicking::panic("internal error: entered unreachable code")
                })
            });
            Some(ResMut { borrow })
        }
    }
    pub struct Res<'r, T> {
        borrow: AtomicRef<'r, T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'r, T: ::core::fmt::Debug> ::core::fmt::Debug for Res<'r, T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Res {
                    borrow: ref __self_0_0,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Res");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "borrow",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<'r, T> ops::Deref for Res<'r, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            self.borrow.deref()
        }
    }
    pub struct ResMut<'r, T> {
        borrow: AtomicRefMut<'r, T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'r, T: ::core::fmt::Debug> ::core::fmt::Debug for ResMut<'r, T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                ResMut {
                    borrow: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "ResMut");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "borrow",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
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
}
pub mod sparse {
    use std::{iter, num::NonZeroU32, slice};
    const UNIT_LEN: usize = 64;
    pub(crate) struct RawSparseIndex(pub(crate) u32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RawSparseIndex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                RawSparseIndex(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "RawSparseIndex");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for RawSparseIndex {
        #[inline]
        fn clone(&self) -> RawSparseIndex {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for RawSparseIndex {}
    impl ::core::marker::StructuralPartialEq for RawSparseIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for RawSparseIndex {
        #[inline]
        fn eq(&self, other: &RawSparseIndex) -> bool {
            match *other {
                RawSparseIndex(ref __self_1_0) => match *self {
                    RawSparseIndex(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &RawSparseIndex) -> bool {
            match *other {
                RawSparseIndex(ref __self_1_0) => match *self {
                    RawSparseIndex(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for RawSparseIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for RawSparseIndex {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for RawSparseIndex {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                RawSparseIndex(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for RawSparseIndex {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RawSparseIndex,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                RawSparseIndex(ref __self_1_0) => match *self {
                    RawSparseIndex(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for RawSparseIndex {
        #[inline]
        fn cmp(&self, other: &RawSparseIndex) -> ::core::cmp::Ordering {
            match *other {
                RawSparseIndex(ref __self_1_0) => match *self {
                    RawSparseIndex(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[allow(unused)]
    impl RawSparseIndex {
        pub const ZERO: Self = Self(0);
        pub(crate) fn from_usize(x: usize) -> Self {
            Self(x as u32)
        }
        pub fn to_usize(&self) -> usize {
            self.0 as usize
        }
    }
    pub(crate) struct RawDenseIndex(pub(crate) u32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for RawDenseIndex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                RawDenseIndex(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "RawDenseIndex");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for RawDenseIndex {
        #[inline]
        fn clone(&self) -> RawDenseIndex {
            {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for RawDenseIndex {}
    impl ::core::marker::StructuralPartialEq for RawDenseIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for RawDenseIndex {
        #[inline]
        fn eq(&self, other: &RawDenseIndex) -> bool {
            match *other {
                RawDenseIndex(ref __self_1_0) => match *self {
                    RawDenseIndex(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &RawDenseIndex) -> bool {
            match *other {
                RawDenseIndex(ref __self_1_0) => match *self {
                    RawDenseIndex(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for RawDenseIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for RawDenseIndex {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for RawDenseIndex {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                RawDenseIndex(ref __self_0_0) => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for RawDenseIndex {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RawDenseIndex,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                RawDenseIndex(ref __self_1_0) => match *self {
                    RawDenseIndex(ref __self_0_0) => {
                        match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                            }
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for RawDenseIndex {
        #[inline]
        fn cmp(&self, other: &RawDenseIndex) -> ::core::cmp::Ordering {
            match *other {
                RawDenseIndex(ref __self_1_0) => match *self {
                    RawDenseIndex(ref __self_0_0) => {
                        match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                            ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                            cmp => cmp,
                        }
                    }
                },
            }
        }
    }
    #[allow(unused)]
    impl RawDenseIndex {
        pub const ZERO: Self = Self(0);
        pub(crate) fn from_usize(x: usize) -> Self {
            Self(x as u32)
        }
        pub fn to_usize(&self) -> usize {
            self.0 as usize
        }
    }
    pub struct Generation {
        raw: NonZeroU32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Generation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Generation {
                    raw: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Generation");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "raw",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Generation {
        #[inline]
        fn clone(&self) -> Generation {
            {
                let _: ::core::clone::AssertParamIsClone<NonZeroU32>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Generation {}
    impl ::core::marker::StructuralPartialEq for Generation {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Generation {
        #[inline]
        fn eq(&self, other: &Generation) -> bool {
            match *other {
                Generation {
                    raw: ref __self_1_0,
                } => match *self {
                    Generation {
                        raw: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Generation) -> bool {
            match *other {
                Generation {
                    raw: ref __self_1_0,
                } => match *self {
                    Generation {
                        raw: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for Generation {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Generation {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<NonZeroU32>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialOrd for Generation {
        #[inline]
        fn partial_cmp(&self, other: &Generation) -> ::core::option::Option<::core::cmp::Ordering> {
            match *other {
                Generation {
                    raw: ref __self_1_0,
                } => match *self {
                    Generation {
                        raw: ref __self_0_0,
                    } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0))
                    {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        }
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Ord for Generation {
        #[inline]
        fn cmp(&self, other: &Generation) -> ::core::cmp::Ordering {
            match *other {
                Generation {
                    raw: ref __self_1_0,
                } => match *self {
                    Generation {
                        raw: ref __self_0_0,
                    } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                        ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                        cmp => cmp,
                    },
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for Generation {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                Generation {
                    raw: ref __self_0_0,
                } => ::core::hash::Hash::hash(&(*__self_0_0), state),
            }
        }
    }
    impl Generation {
        pub const INITIAL: Generation = Self {
            raw: unsafe { NonZeroU32::new_unchecked(1) },
        };
        pub(crate) fn increment(self) -> Self {
            Self {
                raw: unsafe { NonZeroU32::new_unchecked(self.raw.get() + 1) },
            }
        }
        pub fn to_usize(&self) -> usize {
            self.raw.get() as usize
        }
    }
    pub struct SparseIndex {
        raw: RawSparseIndex,
        gen: Generation,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SparseIndex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SparseIndex {
                    raw: ref __self_0_0,
                    gen: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SparseIndex");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "raw",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "gen",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SparseIndex {
        #[inline]
        fn clone(&self) -> SparseIndex {
            {
                let _: ::core::clone::AssertParamIsClone<RawSparseIndex>;
                let _: ::core::clone::AssertParamIsClone<Generation>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for SparseIndex {}
    impl ::core::marker::StructuralPartialEq for SparseIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SparseIndex {
        #[inline]
        fn eq(&self, other: &SparseIndex) -> bool {
            match *other {
                SparseIndex {
                    raw: ref __self_1_0,
                    gen: ref __self_1_1,
                } => match *self {
                    SparseIndex {
                        raw: ref __self_0_0,
                        gen: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SparseIndex) -> bool {
            match *other {
                SparseIndex {
                    raw: ref __self_1_0,
                    gen: ref __self_1_1,
                } => match *self {
                    SparseIndex {
                        raw: ref __self_0_0,
                        gen: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for SparseIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for SparseIndex {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<RawSparseIndex>;
                let _: ::core::cmp::AssertParamIsEq<Generation>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for SparseIndex {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                SparseIndex {
                    raw: ref __self_0_0,
                    gen: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    #[allow(unused)]
    impl SparseIndex {
        pub(crate) fn new(raw: RawSparseIndex, gen: Generation) -> Self {
            Self { raw, gen }
        }
        pub(crate) fn initial(raw: RawSparseIndex) -> Self {
            Self {
                raw,
                gen: Generation::INITIAL,
            }
        }
        pub(crate) fn increment_generation(self) -> Self {
            Self {
                raw: self.raw,
                gen: self.gen.increment(),
            }
        }
        pub fn generation(&self) -> Generation {
            self.gen
        }
        pub(crate) fn raw(&self) -> RawSparseIndex {
            self.raw
        }
        pub fn to_usize(&self) -> usize {
            self.raw.to_usize()
        }
    }
    pub struct DenceIndex {
        raw: RawDenseIndex,
        gen: Generation,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for DenceIndex {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                DenceIndex {
                    raw: ref __self_0_0,
                    gen: ref __self_0_1,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "DenceIndex");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "raw",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "gen",
                        &&(*__self_0_1),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for DenceIndex {
        #[inline]
        fn clone(&self) -> DenceIndex {
            {
                let _: ::core::clone::AssertParamIsClone<RawDenseIndex>;
                let _: ::core::clone::AssertParamIsClone<Generation>;
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for DenceIndex {}
    impl ::core::marker::StructuralPartialEq for DenceIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for DenceIndex {
        #[inline]
        fn eq(&self, other: &DenceIndex) -> bool {
            match *other {
                DenceIndex {
                    raw: ref __self_1_0,
                    gen: ref __self_1_1,
                } => match *self {
                    DenceIndex {
                        raw: ref __self_0_0,
                        gen: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &DenceIndex) -> bool {
            match *other {
                DenceIndex {
                    raw: ref __self_1_0,
                    gen: ref __self_1_1,
                } => match *self {
                    DenceIndex {
                        raw: ref __self_0_0,
                        gen: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for DenceIndex {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for DenceIndex {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<RawDenseIndex>;
                let _: ::core::cmp::AssertParamIsEq<Generation>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::hash::Hash for DenceIndex {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            match *self {
                DenceIndex {
                    raw: ref __self_0_0,
                    gen: ref __self_0_1,
                } => {
                    ::core::hash::Hash::hash(&(*__self_0_0), state);
                    ::core::hash::Hash::hash(&(*__self_0_1), state)
                }
            }
        }
    }
    #[allow(unused)]
    impl DenceIndex {
        pub(crate) fn new(raw: RawDenseIndex, gen: Generation) -> Self {
            Self { raw, gen }
        }
        pub(crate) fn initial(raw: RawDenseIndex) -> Self {
            Self {
                raw,
                gen: Generation::INITIAL,
            }
        }
        pub(crate) fn increment_generation(self) -> Self {
            Self {
                raw: self.raw,
                gen: self.gen.increment(),
            }
        }
        pub fn generation(&self) -> Generation {
            self.gen
        }
        pub(crate) fn raw(&self) -> RawDenseIndex {
            self.raw
        }
        pub fn to_usize(&self) -> usize {
            self.raw.to_usize()
        }
    }
    struct SparseArray {
        data: Vec<Option<DenceIndex>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SparseArray {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SparseArray {
                    data: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SparseArray");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "data",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SparseArray {
        #[inline]
        fn clone(&self) -> SparseArray {
            match *self {
                SparseArray {
                    data: ref __self_0_0,
                } => SparseArray {
                    data: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Default for SparseArray {
        fn default() -> Self {
            Self {
                data: Vec::default(),
            }
        }
    }
    impl SparseArray {
        pub fn get(&self, sparse: SparseIndex) -> Option<DenceIndex> {
            self.data.get(sparse.to_usize())?.map(|dense| {
                if true {
                    if !(sparse.gen <= dense.gen) {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["generation has to increase monotonically"],
                            &match () {
                                _args => [],
                            },
                        ))
                    };
                };
                dense.clone()
            })
        }
        pub fn set(&mut self, sparse_slot: usize, dense: DenceIndex) {
            self.data[sparse_slot] = Some(dense);
        }
        pub fn get_or_alloc_mut(&mut self, sparse: SparseIndex) -> Option<&mut DenceIndex> {
            let idx_usize = sparse.to_usize();
            self.maybe_grow(idx_usize);
            self.data.get_mut(idx_usize).unwrap().as_mut()
        }
        pub fn remove(&mut self, idx: SparseIndex) -> Option<DenceIndex> {
            self.data.get_mut(idx.to_usize())?.take()
        }
        fn maybe_grow(&mut self, target_slot: usize) -> bool {
            if self.data.len() >= target_slot + 1 {
                false
            } else {
                let n_units = (UNIT_LEN + target_slot) / UNIT_LEN;
                let new_len = n_units * UNIT_LEN;
                self.data.resize(new_len, None);
                true
            }
        }
    }
}
pub mod sys {
    use crate::{
        res::{Res, ResMut},
        World,
    };
    use std::{any, borrow::Borrow};
    pub trait BorrowWorld<'w> {
        unsafe fn borrow(w: &'w World) -> Self;
    }
    impl<'w, T: 'static> BorrowWorld<'w> for Res<'w, T> {
        unsafe fn borrow(w: &'w World) -> Self {
            w.res.borrow().unwrap_or_else(|| {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["Tried to borrow resource of type ", " for a system"],
                    &match (&any::type_name::<T>(),) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            })
        }
    }
    impl<'w, T: 'static> BorrowWorld<'w> for ResMut<'w, T> {
        unsafe fn borrow(w: &'w World) -> Self {
            w.res.borrow_mut().unwrap_or_else(|| {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["Tried to borrow resource of type ", " for a system"],
                    &match (&any::type_name::<T>(),) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            })
        }
    }
    pub unsafe trait System<'w, Param> {
        unsafe fn run(&mut self, w: &'w World);
    }
    unsafe impl<'w, P15, P14, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<
            'w,
            (
                P15,
                P14,
                P13,
                P12,
                P11,
                P10,
                P9,
                P8,
                P7,
                P6,
                P5,
                P4,
                P3,
                P2,
                P1,
                P0,
            ),
        > for F
    where
        F: FnMut(P15, P14, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P15: BorrowWorld<'w>,
        P14: BorrowWorld<'w>,
        P13: BorrowWorld<'w>,
        P12: BorrowWorld<'w>,
        P11: BorrowWorld<'w>,
        P10: BorrowWorld<'w>,
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P15 as BorrowWorld>::borrow(w),
                <P14 as BorrowWorld>::borrow(w),
                <P13 as BorrowWorld>::borrow(w),
                <P12 as BorrowWorld>::borrow(w),
                <P11 as BorrowWorld>::borrow(w),
                <P10 as BorrowWorld>::borrow(w),
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P14, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<
            'w,
            (
                P14,
                P13,
                P12,
                P11,
                P10,
                P9,
                P8,
                P7,
                P6,
                P5,
                P4,
                P3,
                P2,
                P1,
                P0,
            ),
        > for F
    where
        F: FnMut(P14, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P14: BorrowWorld<'w>,
        P13: BorrowWorld<'w>,
        P12: BorrowWorld<'w>,
        P11: BorrowWorld<'w>,
        P10: BorrowWorld<'w>,
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P14 as BorrowWorld>::borrow(w),
                <P13 as BorrowWorld>::borrow(w),
                <P12 as BorrowWorld>::borrow(w),
                <P11 as BorrowWorld>::borrow(w),
                <P10 as BorrowWorld>::borrow(w),
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<'w, (P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P13, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P13: BorrowWorld<'w>,
        P12: BorrowWorld<'w>,
        P11: BorrowWorld<'w>,
        P10: BorrowWorld<'w>,
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P13 as BorrowWorld>::borrow(w),
                <P12 as BorrowWorld>::borrow(w),
                <P11 as BorrowWorld>::borrow(w),
                <P10 as BorrowWorld>::borrow(w),
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<'w, (P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P12, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P12: BorrowWorld<'w>,
        P11: BorrowWorld<'w>,
        P10: BorrowWorld<'w>,
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P12 as BorrowWorld>::borrow(w),
                <P11 as BorrowWorld>::borrow(w),
                <P10 as BorrowWorld>::borrow(w),
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<'w, (P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P11, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P11: BorrowWorld<'w>,
        P10: BorrowWorld<'w>,
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P11 as BorrowWorld>::borrow(w),
                <P10 as BorrowWorld>::borrow(w),
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<'w, (P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P10, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P10: BorrowWorld<'w>,
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P10 as BorrowWorld>::borrow(w),
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P9, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<'w, (P9, P8, P7, P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P9, P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P9: BorrowWorld<'w>,
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P9 as BorrowWorld>::borrow(w),
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P8, P7, P6, P5, P4, P3, P2, P1, P0, F>
        System<'w, (P8, P7, P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P8, P7, P6, P5, P4, P3, P2, P1, P0),
        P8: BorrowWorld<'w>,
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P8 as BorrowWorld>::borrow(w),
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P7, P6, P5, P4, P3, P2, P1, P0, F> System<'w, (P7, P6, P5, P4, P3, P2, P1, P0)>
        for F
    where
        F: FnMut(P7, P6, P5, P4, P3, P2, P1, P0),
        P7: BorrowWorld<'w>,
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P7 as BorrowWorld>::borrow(w),
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P6, P5, P4, P3, P2, P1, P0, F> System<'w, (P6, P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P6, P5, P4, P3, P2, P1, P0),
        P6: BorrowWorld<'w>,
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P6 as BorrowWorld>::borrow(w),
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P5, P4, P3, P2, P1, P0, F> System<'w, (P5, P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P5, P4, P3, P2, P1, P0),
        P5: BorrowWorld<'w>,
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P5 as BorrowWorld>::borrow(w),
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P4, P3, P2, P1, P0, F> System<'w, (P4, P3, P2, P1, P0)> for F
    where
        F: FnMut(P4, P3, P2, P1, P0),
        P4: BorrowWorld<'w>,
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P4 as BorrowWorld>::borrow(w),
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P3, P2, P1, P0, F> System<'w, (P3, P2, P1, P0)> for F
    where
        F: FnMut(P3, P2, P1, P0),
        P3: BorrowWorld<'w>,
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P3 as BorrowWorld>::borrow(w),
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P2, P1, P0, F> System<'w, (P2, P1, P0)> for F
    where
        F: FnMut(P2, P1, P0),
        P2: BorrowWorld<'w>,
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P2 as BorrowWorld>::borrow(w),
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P1, P0, F> System<'w, (P1, P0)> for F
    where
        F: FnMut(P1, P0),
        P1: BorrowWorld<'w>,
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(
                <P1 as BorrowWorld>::borrow(w),
                <P0 as BorrowWorld>::borrow(w),
            )
        }
    }
    unsafe impl<'w, P0, F> System<'w, (P0,)> for F
    where
        F: FnMut(P0),
        P0: BorrowWorld<'w>,
    {
        unsafe fn run(&mut self, w: &'w World) {
            (self)(<P0 as BorrowWorld>::borrow(w))
        }
    }
}
use crate::res::ResourceMap;
pub struct World {
    res: ResourceMap,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for World {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            World {
                res: ref __self_0_0,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "World");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "res", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for World {
    #[inline]
    fn default() -> World {
        World {
            res: ::core::default::Default::default(),
        }
    }
}
