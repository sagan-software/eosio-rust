use crate::{NumBytes, Read, ReadError, Write, WriteError};
use core::{cmp::PartialEq, mem::size_of};

macro_rules! declare_checksum_type {
    ($ident:ident, $num_words:expr, $num_bytes:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $ident([u128; $num_words]);

        impl $ident {
            #[must_use]
            pub const fn words(&self) -> [u128; $num_words] {
                self.0
            }

            /// TODO docs.
            #[must_use]
            pub fn to_bytes(&self) -> [u8; $num_bytes] {
                let mut bytes = [0_u8; $num_bytes];

                let num_sub_words = 16;

                let last_i = self.0.len() - 1;
                for (counter, temp_word) in self.0.iter().enumerate() {
                    let mut sub_words_left = num_sub_words;
                    let mut temp_word = *temp_word;

                    if counter == last_i {
                        let padded_bytes =
                            $num_words * size_of::<u128>() - $num_bytes;
                        sub_words_left -= padded_bytes;
                        temp_word >>= 8 * padded_bytes;
                    }

                    #[allow(clippy::cast_possible_truncation)]
                    while sub_words_left > 0 {
                        let i = (sub_words_left - 1) + counter * num_sub_words;
                        let byte = bytes.get_mut(i).unwrap();
                        *byte = (temp_word & 0xFF) as u8;
                        temp_word >>= 8;
                        sub_words_left -= 1;
                    }
                }

                bytes
            }

            #[must_use]
            pub fn from_bytes(bytes: [u8; $num_bytes]) -> Self {
                const SUB_WORD_SHIFT: usize = 8 * size_of::<u8>();
                const NUM_SUB_WORDS: usize =
                    size_of::<u128>() / size_of::<u8>();

                let mut words = [0_u128; $num_words];
                let mut temp_word = 0_u128;
                let mut sub_words_left = NUM_SUB_WORDS;
                let mut num_word = 0;

                for byte in bytes.to_vec().iter() {
                    if sub_words_left > 1 {
                        temp_word |= u128::from(*byte);
                        temp_word <<= SUB_WORD_SHIFT;
                        sub_words_left -= 1;
                        continue;
                    }

                    assert!(
                        sub_words_left == 1,
                        "unexpected error in $ident::from_bytes"
                    );
                    temp_word |= u128::from(*byte);
                    sub_words_left = NUM_SUB_WORDS;
                    *words.get_mut(num_word).unwrap() = temp_word;
                    temp_word = 0;
                    num_word += 1;
                }
                if sub_words_left != NUM_SUB_WORDS {
                    if sub_words_left > 1 {
                        temp_word <<= 8 * (sub_words_left - 1);
                    }
                    *words.get_mut(num_word).unwrap() = temp_word;
                }

                Self(words)
            }
        }

        impl NumBytes for $ident {
            #[inline]
            fn num_bytes(&self) -> usize {
                $num_bytes
            }
        }

        impl Read for $ident {
            #[inline]
            fn read(bytes: &[u8], pos: &mut usize) -> Result<Self, ReadError> {
                Read::read(bytes, pos).map(Self::from_bytes)
            }
        }

        impl Write for $ident {
            #[inline]
            fn write(
                &self,
                bytes: &mut [u8],
                pos: &mut usize,
            ) -> Result<(), WriteError> {
                self.to_bytes().write(bytes, pos)
            }
        }

        impl From<[u8; $num_bytes]> for $ident {
            #[inline]
            #[must_use]
            fn from(value: [u8; $num_bytes]) -> Self {
                Self::from_bytes(value)
            }
        }

        impl From<$ident> for [u8; $num_bytes] {
            #[inline]
            #[must_use]
            fn from(value: $ident) -> Self {
                value.to_bytes()
            }
        }

        impl Default for $ident {
            fn default() -> Self {
                Self(Default::default())
            }
        }

        impl AsRef<$ident> for $ident {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl PartialEq<[u128]> for $ident {
            fn eq(&self, other: &[u128]) -> bool {
                self.0 == other
            }
        }

        impl PartialEq<$ident> for &[u128] {
            fn eq(&self, other: &$ident) -> bool {
                self == &other.0
            }
        }

        impl PartialEq<[u128; $num_words]> for $ident {
            fn eq(&self, other: &[u128; $num_words]) -> bool {
                &self.0 == other
            }
        }

        impl PartialEq<$ident> for [u128; $num_words] {
            fn eq(&self, other: &$ident) -> bool {
                self == &other.0
            }
        }

        impl PartialEq<[u8; $num_bytes]> for $ident {
            fn eq(&self, other: &[u8; $num_bytes]) -> bool {
                let other = Self::from_bytes(*other);
                self == &other
            }
        }

        impl PartialEq<$ident> for [u8; $num_bytes] {
            fn eq(&self, other: &$ident) -> bool {
                let other = other.to_bytes();
                (&self)[..] == (&other)[..]
            }
        }
    };
}

declare_checksum_type!(Checksum160, 2, 20);
declare_checksum_type!(Checksum256, 2, 32);
declare_checksum_type!(Checksum512, 4, 64);

#[cfg(test)]
mod tests {
    use super::Checksum160;

    #[test]
    fn checksum160_from_to_bytes() {
        let inputs = vec![
            [0_u8; 20],
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                19, 20,
            ],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];
        for input in inputs {
            let c160 = Checksum160::from_bytes(input);
            let output = c160.to_bytes();
            assert_eq!(input, output);
        }
    }
}
