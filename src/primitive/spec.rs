use ::CrcSpec;
use super::{ValueType, Table, fill_table};
use std::mem::size_of;


/// An implementation of `CrcSpec` with a lookup table (for performance optimization) embedded in it.
///
/// The embedded table is of type `[T; 256]`.
pub struct CrcTable<T> {
    poly: T,
    init: T,
    refin: bool,
    refout: bool,
    xorout: T,

    table: Table<T>
}

impl<T: ValueType> CrcTable<T> {

    /// The constructor method.
    pub fn new(poly: T, init: T, refin: bool, refout: bool, xorout: T) -> CrcTable<T> {
        let mut spec = CrcTable {
            poly: poly,
            init: init,
            refin: refin,
            refout: refout,
            xorout: xorout,
            table: [T::from(0); 256]
        };
        fill_table(&mut spec.table, poly, refin);
        spec
    }

    /// Updates a CRC register with one byte of user data,
    /// taking into account this spec's `refin` value.
    pub fn update(&self, value: T, byte: u8) -> T {
        if self.refin {
            (value >> 8) ^ self.table[(value.to_u8() ^ byte) as usize]
        } else {
            (value << 8) ^ self.table[((value >> 24).to_u8() ^ byte) as usize]
        }
    }

    /// Applies the REFOUT and XOROUT stages to the supplied CRC register value,
    /// returning the resulting checksum.
    pub fn finish(&self, value: T) -> T {
        (if self.refin != self.refout { value.swap_bits() } else { value }) ^ self.xorout
    }

}

impl<T: ValueType> CrcSpec<T> for CrcTable<T> {
    fn width(&self) -> usize { size_of::<T>() * 8 }
    fn poly(&self) -> T { self.poly }
    fn init(&self) -> T { self.init }
    fn refin(&self) -> bool { self.refin }
    fn refout(&self) -> bool { self.refout }
    fn xorout(&self) -> T { self.xorout }
}

#[cfg(test)]
mod tests {
    macro_rules! common_tests_for {
        ($t:ty, $module:ident, $w:expr) => {
            mod $module {
                use super::super::CrcTable;
                use ::CrcSpec;

                #[test]
                fn width() {
                    assert_eq!(CrcTable::new(0 as $t, 0 as $t, false, false, 0 as $t).width(), $w);
                }
            }
        }
    }

    common_tests_for!(u8, test_u8, 8);
    common_tests_for!(u16, test_u16, 16);
    common_tests_for!(u32, test_u32, 32);
    common_tests_for!(u64, test_u64, 64);
}
