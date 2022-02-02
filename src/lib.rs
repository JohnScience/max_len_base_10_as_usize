#![no_std]

/// Trait providing the namesake associated constant
/// that stores maximum length of string representations of values
/// of the type-impementer in the numeral system base 10. That 
/// length is stored as a value of type [usize].
/// 
/// # Example
/// 
/// ```
/// use max_len_base_10_as_usize::MaxLenBase10AsUsize as MaxLen;
/// assert_eq!(u8::MAX_LEN_BASE_10_AS_USIZE, <u8 as MaxLen>::MAX_LEN_BASE_10_AS_USIZE);
/// assert_eq!(u8::MAX_LEN_BASE_10_AS_USIZE, 3usize);
///```
pub trait MaxLenBase10AsUsize {
    const MAX_LEN_BASE_10_AS_USIZE: usize;
}

impl MaxLenBase10AsUsize for u8 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 3;
}

impl MaxLenBase10AsUsize for u16 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 5;
}

impl MaxLenBase10AsUsize for u32 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 10;
}

impl MaxLenBase10AsUsize for u64 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 20;
}

impl MaxLenBase10AsUsize for u128 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 39;
}

impl MaxLenBase10AsUsize for usize {
    const MAX_LEN_BASE_10_AS_USIZE: usize = match core::mem::size_of::<usize>() {
        1 => u8::MAX_LEN_BASE_10_AS_USIZE,
        2 => u16::MAX_LEN_BASE_10_AS_USIZE,
        4 => u32::MAX_LEN_BASE_10_AS_USIZE,
        8 => u64::MAX_LEN_BASE_10_AS_USIZE,
        16 => u128::MAX_LEN_BASE_10_AS_USIZE,
        _ => unimplemented!(),
    };
}

impl MaxLenBase10AsUsize for i8 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 4;
}

impl MaxLenBase10AsUsize for i16 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 6;
}

impl MaxLenBase10AsUsize for i32 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 11;
}

impl MaxLenBase10AsUsize for i64 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 20;
}

impl MaxLenBase10AsUsize for i128 {
    const MAX_LEN_BASE_10_AS_USIZE: usize = 40;
}

impl MaxLenBase10AsUsize for isize {
    const MAX_LEN_BASE_10_AS_USIZE: usize = match core::mem::size_of::<usize>() {
        1 => i8::MAX_LEN_BASE_10_AS_USIZE,
        2 => i16::MAX_LEN_BASE_10_AS_USIZE,
        4 => i32::MAX_LEN_BASE_10_AS_USIZE,
        8 => i64::MAX_LEN_BASE_10_AS_USIZE,
        16 => i128::MAX_LEN_BASE_10_AS_USIZE,
        _ => unimplemented!(),
    };
}

// https://en.wikipedia.org/wiki/Equivalence_class#Definition_and_notation
#[cfg(test)]
trait CanonicalRepresentativeWithGreatestLenBase10 {
    const CANONICAL_REPRESENTATIVE_WITH_GREATEST_LEN_BASE_10: Self;
}

#[cfg(test)]
macro_rules! impl_canonical_representative_with_greatest_len_base_10_for_unsigned {
    ($($t:ty),+) => {
        $(
            impl CanonicalRepresentativeWithGreatestLenBase10 for $t {
                const CANONICAL_REPRESENTATIVE_WITH_GREATEST_LEN_BASE_10: Self = Self::MAX;
            }
        )+
    }
}

#[cfg(test)]
macro_rules! impl_canonical_representative_with_greatest_len_base_10_for_signed {
    ($($t:ty),+) => {
        $(
            impl CanonicalRepresentativeWithGreatestLenBase10 for $t {
                const CANONICAL_REPRESENTATIVE_WITH_GREATEST_LEN_BASE_10: Self = Self::MIN;
            }
        )+
    }
}
#[cfg(test)]
impl_canonical_representative_with_greatest_len_base_10_for_unsigned!(u8,u16,u32,u64,u128,usize);
#[cfg(test)]
impl_canonical_representative_with_greatest_len_base_10_for_signed!(i8,i16,i32,i64,i128,isize);

#[cfg(test)]
mod tests {
    use crate::MaxLenBase10AsUsize;

    #[test]
    fn non_fully_qualified_syntax_works() {
        assert_eq!(u8::MAX_LEN_BASE_10_AS_USIZE, 3);
    }

    #[test]
    fn fully_qualified_syntax_works() {
        assert_eq!(<u8 as MaxLenBase10AsUsize>::MAX_LEN_BASE_10_AS_USIZE, 3);
    }
}