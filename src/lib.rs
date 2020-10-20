//! A crate that provides a map that can be used in `no_std` environments.

#![feature(min_const_generics)]
#![no_std]
#![warn(clippy::all)]
#![warn(missing_docs)]

use core::borrow::Borrow;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;

/// A map that has a predetermined set of keys.
pub struct ConstMap<K, V, const N: usize>
where
    K: Eq,
{
    keys: [K; N],
    values: [V; N],
}

impl<K, V, const N: usize> ConstMap<K, V, N>
where
    K: Eq,
{
    /// Creates a new `ConstMap` with a provided set of keys and default values.
    pub fn new(keys: [K; N], value_generator: impl Fn() -> V) -> Self {
        let values: [V; N] = unsafe {
            let mut vs = core::mem::MaybeUninit::<[V; N]>::uninit();
            let ptr = vs.as_mut_ptr().cast::<V>();
            for i in 0..N {
                let el_ptr = ptr.add(i);
                el_ptr.write(value_generator());
            }

            vs.assume_init()
        };

        Self { keys, values }
    }

    /// Tries to get a references to the value for a provided key.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let i = self.get_index(key)?;
        Some(&self.values[i])
    }

    /// Tries to get a mutable reference to the value for a provided key.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let i = self.get_index(key)?;
        Some(&mut self.values[i])
    }

    fn get_index<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let keys_as_qs = self.keys.iter().map(<K as Borrow<Q>>::borrow);
        keys_as_qs
            .enumerate()
            .find(|(_, b)| Q::eq(*b.borrow(), key))
            .map(|(i, _)| i)
    }
}

impl<K, V, const N: usize> Debug for ConstMap<K, V, N>
where
    K: Eq + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut map_builder = f.debug_map();
        for i in 0..N {
            let key = &self.keys[i];
            let value = &self.values[i];
            map_builder.entry(key, value);
        }

        map_builder.finish()
    }
}
