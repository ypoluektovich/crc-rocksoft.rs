use ::{CrcSpec, CrcHasher};
use super::*;
use std::borrow::Borrow;


/// An implementation of `CrcHasher` that has a way to immutably borrow
/// a reference to its `CrcSpec`. The borrowing mechanism is abstracted.
///
/// Instances can be obtained via the `From` mechanism (see below).
pub struct CrcTableHasher<T, S: Borrow<CrcTable<T>>> {
    value: T,
    spec: S
}


impl<T: ValueType, S: Borrow<CrcTable<T>>> CrcHasher<T> for CrcTableHasher<T, S> {
    fn reset(&mut self) {
        self.value = self.spec.borrow().init();
    }

    fn update(&mut self, byte: u8) {
        self.value = self.spec.borrow().update(self.value, byte);
    }

    fn finish(&self) -> T {
        self.spec.borrow().finish(self.value)
    }
}


impl<T: ValueType, S: Borrow<CrcTable<T>>> From<S> for CrcTableHasher<T, S> {
    /// Constructs and returns a hasher from anything that can provide a reference to a spec:
    /// from encapsulating a spec inside the hasher, to using a reference, a Box or any other option.
    ///
    /// ```
    /// use crc_rocksoft::*;
    /// use crc_rocksoft::primitive::*;
    ///
    /// let owned = CrcTableHasher::from(CrcTable::new(0x04C11DB7u32, 0u32, false, false, 0xFFFFFFFFu32));
    ///
    /// let spec = CrcTable::new(0x04C11DB7u32, 0u32, false, false, 0xFFFFFFFFu32);
    /// let spec_ref = &spec;
    /// let referenced = CrcTableHasher::from(spec_ref);
    ///
    /// let spec_box = Box::new(CrcTable::new(0x04C11DB7u32, 0u32, false, false, 0xFFFFFFFFu32));
    /// let boxed = CrcTableHasher::from(spec_box);
    /// ```
    fn from(spec_ref: S) -> Self {
        let mut hasher = CrcTableHasher { value: T::from(0), spec: spec_ref };
        hasher.reset();
        hasher
    }
}

#[cfg(test)]
mod tests {
    use ::CrcHasher;
    use super::super::{ValueType, CrcTable};
    use super::CrcTableHasher;

    fn feed<T: From<u8>>(hasher: &mut CrcHasher<T>) -> &mut CrcHasher<T> {
        for i in 1..10 {
            hasher.update(0x30 + i);
        }
        hasher
    }

    fn test<T: ValueType>(spec: CrcTable<T>, expected: T) {
        let mut h = CrcTableHasher::from(spec);
        assert_eq!(feed(&mut h).finish(), expected);
    }

    #[test]
    fn crc32() {
        test(CrcTable::new(0x04C11DB7u32, 0xFFFFFFFFu32, true, true, 0xFFFFFFFFu32), 0xCBF43926u32);
    }

    #[test]
    fn update_from_slice() {
        let mut bytes: Vec<u8> = Vec::new();
        for i in 1..10 {
            bytes.push(0x30 + i);
        }

        let mut h = CrcTableHasher::from(CrcTable::new(0x04C11DB7u32, 0xFFFFFFFFu32, true, true, 0xFFFFFFFFu32));
        h.update_from_slice(&bytes);
        assert_eq!(h.finish(), 0xCBF43926u32);
    }

    #[test]
    fn crc32_posix() {
        test(CrcTable::new(0x04C11DB7u32, 0u32, false, false, 0xFFFFFFFFu32), 0x765E7680u32);
    }
}
