//! A crate that provides a map that can be used in `no_std` environments.

#![allow(incomplete_features)]
#![feature(const_generics)]
#![warn(clippy::all)]
#![warn(missing_docs)]

/// A map that has a predetermined set of keys.
pub struct ConstMap<const N: usize, K, V>
where
    K: Eq,
{
    keys: [K; N],
    values: [V; N],
}

impl<const N: usize, K, V> ConstMap<N, K, V>
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
    pub fn get(&self, key: &K) -> Option<&V> {
        let i = self.get_index(key)?;
        Some(&self.values[i])
    }

    /// Tries to get a mutable reference to the value for a provided key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let i = self.get_index(key)?;
        Some(&mut self.values[i])
    }

    fn get_index(&self, key: &K) -> Option<usize> {
        self.keys
            .iter()
            .enumerate()
            .find(|(_, b)| K::eq(*b, key))
            .map(|(i, _)| i)
    }
}
