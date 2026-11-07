//! Type-state resource lifecycle (requires `alloc`).
//!
//! - `TypedResource<Alive>` owns bytes and may be inspected or consumed.
//! - Consumption moves data out; the Alive handle is gone (Law: no residual karma).

#![cfg(feature = "alloc")]

use alloc::vec::Vec;
use core::marker::PhantomData;

/// Lifecycle markers.
pub mod life {
    #[derive(Debug, Default, Clone, Copy)]
    pub struct Alive;
}

use life::Alive;

/// Resource whose usability is encoded in the type parameter.
pub struct TypedResource<S> {
    id: u64,
    data: Vec<u8>,
    _life: PhantomData<S>,
}

impl TypedResource<Alive> {
    pub fn new(id: u64, data: Vec<u8>) -> Self {
        Self {
            id,
            data,
            _life: PhantomData,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Consume the resource. The Alive value is moved; no further use is possible.
    pub fn consume(self) -> (u64, Vec<u8>) {
        (self.id, self.data)
    }

    /// Map owned payload while remaining Alive.
    pub fn map_data<F>(self, f: F) -> Self
    where
        F: FnOnce(Vec<u8>) -> Vec<u8>,
    {
        Self {
            id: self.id,
            data: f(self.data),
            _life: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn consume_returns_payload() {
        let r = TypedResource::new(1, vec![9, 8, 7]);
        assert_eq!(r.id(), 1);
        assert_eq!(r.data(), &[9, 8, 7]);
        let (id, data) = r.consume();
        assert_eq!(id, 1);
        assert_eq!(data, vec![9, 8, 7]);
    }

    #[test]
    fn map_data_while_alive() {
        let r = TypedResource::new(2, vec![1, 2]).map_data(|mut d| {
            d.push(3);
            d
        });
        let (_, data) = r.consume();
        assert_eq!(data, vec![1, 2, 3]);
    }
}
