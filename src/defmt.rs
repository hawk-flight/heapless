//! Defmt implementations for heapless types
//!

use crate::ArrayLength;
use crate::Vec;
use defmt::Formatter;

impl<T, N> defmt::Format for Vec<T, N>
where
    N: ArrayLength<T>,
    T: defmt::Format,
{
    fn format(&self, fmt: &mut Formatter) {
        fmt.fmt_slice(self)
    }
}

impl<N> defmt::Format for crate::String<N>
where
    N: ArrayLength<u8>,
    u8: defmt::Format,
{
    fn format(&self, fmt: &mut Formatter) {
        fmt.str(self.as_str());
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::{consts::*, Vec};

    #[test]
    /// Tests encoding Vec with defmt,
    /// based loosely on https://github.com/knurling-rs/defmt/blob/main/tests/encode.rs#L483
    fn test_defmt_format_vec() {
        let v: Vec<_, U8> = Vec::from_slice(b"abc").unwrap();
        let index = defmt::export::fetch_string_index();

        // borrowed from https://github.com/knurling-rs/defmt/blob/main/tests/encode.rs#L49
        let fake_interned = index.wrapping_add(1) & 0x7F;

        let timestamp = defmt::export::fetch_timestamp();
        let mut formatter = defmt::Formatter::new();
        defmt::winfo!(formatter, "{:?}", v);
        assert_eq!(
            formatter.bytes(),
            &[
                index,
                timestamp,
                v.len().try_into().unwrap(),
                fake_interned, // Faked
                // Data bytes.
                97u8,
                98u8,
                99u8,
            ]
        );
    }

    /// Tests encoding String with defmt,
    /// based loosely on https://github.com/knurling-rs/defmt/blob/main/tests/encode.rs#L483
    #[test]
    fn test_defmt_format_str() {
        let mut v: crate::String<crate::consts::U32> = crate::String::new();
        v.push_str("foo").unwrap();
        let index = defmt::export::fetch_string_index();

        let timestamp = defmt::export::fetch_timestamp();
        let mut formatter = defmt::Formatter::new();
        defmt::winfo!(formatter, "{:?}", v);
        assert_eq!(
            formatter.bytes(),
            &[
                index,
                timestamp,
                v.len().try_into().unwrap(),
                // Data bytes.
                102u8,
                111u8,
                111u8,
            ]
        );
    }
}
