//! Rust in action
//! Chapter 5: Data in Depth
//! Floating-point numbers

fn main() {
    println!("Decoding");
    let result = decode_float(0, 1.325625, 132);
    println!("{result}");

    let result = isolate_sign_bit(42.42);
    println!("{result}")
}

/// float variables are stored to memory in this way
/// | Byte 0        | Byte 1          | Byte 2        | Byte 3        |
/// |---------------|-----------------|---------------|---------------|
/// | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | |
/// | 31 30 29 28 27 26 25 24 | 23 22 21 20 19 18 17 16 | 15 14 13 12 11 10 9 8 | 7 6 5 4 3 2 1 0 |
/// 31 bit - Sign bit
/// 30 - 23 bit exponent
/// 22 - 0 bit mantissa

fn decode_float(sign_bit: u8, mantissa: f32, exponent: u32) -> f32 {
    <i8 as Into<f32>>::into((-1_i8).pow(sign_bit.into()))
        * mantissa
        * 2_u8.pow(exponent - 127) as f32
}

fn isolate_sign_bit(value: f32) -> u32 {
    value.to_bits() >> 31
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decoding_float() {
        assert_eq!(42.42, decode_float(0, 1.325625, 132));
    }

    #[test]
    fn isolating_sign_bit(){
        assert_eq!(0, isolate_sign_bit(42.42));
    }
}
