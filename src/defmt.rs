//! Defmt implementations for heapless types
//!

use defmt::Formatter;
use generic_array::ArrayLength;

use crate::Vec;

impl<T, N> defmt::Format for Vec<T, N>
    where N: ArrayLength<T>, T: defmt::Format {
    fn format(&self, fmt: &mut Formatter) {
        fmt.fmt_slice(self)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::{consts::*, Vec};

    #[test]
    /// Tests encoding Vec with defmt,
    /// based loosely on https://github.com/knurling-rs/defmt/blob/main/tests/encode.rs#L483
    fn test_defmt_format() {
        let v: Vec<_, U8> = Vec::from_slice(b"abc").unwrap();
        let index = defmt::export::fetch_string_index();

        // borrowed from https://github.com/knurling-rs/defmt/blob/main/tests/encode.rs#L49
        let fake_interned = index.wrapping_add(1) & 0x7F;

        let timestamp = defmt::export::fetch_timestamp();
        let mut formatter = defmt::Formatter::new();
        defmt::winfo!(formatter, "{:?}", v);
        assert_eq!(formatter.bytes(), &[
            index,
            timestamp,
            v.len().try_into().unwrap(),
            fake_interned,  // Faked
            // Data bytes.
            97u8,
            98u8,
            99u8,
        ]);
    }
}