//! Implementation of the CRC algorithm family.
//! Algorithms are parameterized as suggested in the 1993 paper by Ross Williams.
//!
//! In his paper, Ross names the parameterized model the "Rocksoft™ Model"
//! (for the company he was employed at), hence the name of this crate.

extern crate bit_reverse;

#[cfg(test)] #[macro_use] extern crate lazy_static;

pub mod primitive;

/// A trait that provides accessors for elements of CRC algorithm specifications.
///
/// The definitions of specification elements are taken from the
/// [1993 paper by Ross Williams](http://www.ross.net/crc/crcpaper.html),
/// with one notable exception (see [`refout()`](#tymethod.refout)).
pub trait CrcSpec<T> {
    /// This is the width of the algorithm expressed in bits.
    /// This is one less than the width of the Poly.
    fn width(&self) -> usize;

    /// This parameter is the poly. This is a binary value that
    /// should be specified as a hexadecimal number. The top bit of the
    /// poly should be omitted. For example, if the poly is 10110, you
    /// should specify 06. An important aspect of this parameter is that it
    /// represents the unreflected poly; the bottom bit of this parameter
    /// is always the LSB of the divisor during the division regardless of
    /// whether the algorithm being modelled is reflected.
    fn poly(&self) -> T;

    /// This parameter specifies the initial value of the register
    /// when the algorithm starts. Unlike `poly`, this value is not affected
    /// by `refin` and `refout`; the bits of `init` are copied to the register
    /// verbatim. This parameter should be specified as a hexadecimal number.
    fn init(&self) -> T;

    /// This is a boolean parameter. If it is FALSE, input bytes are
    /// processed with bit 7 being treated as the most significant bit
    /// and bit 0 being treated as the least significant bit. If this
    /// parameter is FALSE, each byte is reflected before being processed.
    fn refin(&self) -> bool;

    /// This is a boolean parameter. If its value is the same as `refin`,
    /// the final value in the register is fed into the XOROUT stage directly;
    /// otherwise, the final register value is reflected first.
    ///
    /// Alternatively, you can think of this value as the indicator of whether
    /// the result of the CRC computation should be reflected so that
    /// the most significant bit of the `poly` value should correspond to
    /// the result's zeroth bit (instead of the result's MSB).
    ///
    /// Note that this definition is different from the one given
    /// in the paper by Ross Williams, which reads:
    ///
    /// ```plain
    ///    REFOUT: This is a boolean parameter. If it is set to FALSE, the
    ///    final value in the register is fed into the XOROUT stage directly,
    ///    otherwise, if this parameter is TRUE, the final register value is
    ///    reflected first.
    /// ```
    ///
    /// With the current implementation (which, as far as I can determine,
    /// is otherwise correct), the original definition would require
    /// inverting `refout` values for all the documented specifications
    /// if we want the implementation to emit the same check values.
    /// Unfortunately, I couldn't find a specification that would use
    /// `refin` unequal to `refout`, so verification is hard.
    fn refout(&self) -> bool;

    /// This is an `width`-bit value that should be specified as a
    /// hexadecimal number. It is XORed to the final register value (after
    /// the REFOUT stage) before the value is returned as the official checksum.
    fn xorout(&self) -> T;
}

/// A basic trait for an object that computes a CRC hash in its own mutable internal state.
///
/// The CRC algorithm specification is implicit for instances of this trait.
/// Usually the algorithm parameters are determined on hasher creation;
/// refer to the implementors' documentation for details.
pub trait CrcHasher<T> {

    /// Reset the internal state used for CRC computation.
    /// The hasher becomes ready to accept new data as if it was newly created.
    fn reset(&mut self);

    /// Update the internal state with one byte of user data.
    fn update(&mut self, byte: u8);

    /// Compute the final stages of the CRC computation and return the final checksum
    /// *without modifying the internal state*.
    /// After an invocation of this method, the hasher is ready to accept
    /// new user data via the `update` method, as if this method wasn't invoked at all.
    fn finish(&self) -> T;


    /// Update the internal state with all the bytes in the supplied slice.
    fn update_from_slice(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.update(b);
        }
    }
}
