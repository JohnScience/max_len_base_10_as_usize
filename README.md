# Maximum length in the numeral system base 10 as usize

This crate offers [`MaxLenBase10AsUsize`](https://docs.rs/max_len_base_10_as_usize/latest/max_len_base_10_as_usize/trait.MaxLenBase10AsUsize.html) trait that is currently implemented for all primitive integers.

According to Rust's reference, [primitive numeric integer types][primitive numeric type] in Rust are such:

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1

## Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

`usize` and `isize` are at least 16-bits wide.

> **Note**: Many pieces of Rust code may assume that pointers, `usize`, and
> `isize` are either 32-bit or 64-bit. As a consequence, 16-bit
> pointer support is limited and may require explicit care and acknowledgment
> from a library to support.

# Why this trait is needed

In order to allocate (or reserve) memory for storing stringly representations of primitive numeric types efficiently, maximum occupied capacity is needed in order to avoid reallocations and/or unnecessary copying.

# Example

You can notice that `MaxLenBase10AsUsize` is quite long to type and it contains such details as numeral system and the desired type of constant. To make it shorter and more suitable for production code (as opposed to system-level or scientific code), you are advised to rename the imported trait as `MaxLen`. For clarity, you may also choose to use [fully qualified syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name), though it is unnecessary.

```rust
  use max_len_base_10_as_usize::MaxLenBase10AsUsize as MaxLen;
  assert_eq!(u8::MAX_LEN_BASE_10_AS_USIZE, <u8 as MaxLen>::MAX_LEN_BASE_10_AS_USIZE);
  assert_eq!(u8::MAX_LEN_BASE_10_AS_USIZE, 3usize);
```

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[primitive numeric type]: https://doc.rust-lang.org/reference/types/numeric.html
