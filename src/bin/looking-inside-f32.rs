//! Rust in action
//! Chapter 5: Data in Depth
//! Floating-point numbers

fn main() {
    println!("Decoding");
    let result = decode_float(0, 1.325625, 132);
    println!("{result}");

    let result = isolate_sign_bit(42.42);
    println!("{result}");

    let result = isolate_exponent(42.42);
    println!("{result}");

    let result = isolate_mantissa(42.42);
    println!("{result}");
}

/// float variables are stored to memory in this way
///
/// | Byte 0        | Byte 1          | Byte 2        | Byte 3        |
/// |---------------|-----------------|---------------|---------------|
/// |31 30 29 28 27 26 25 24|23 22 21 20 19 18 17 16|15 14 13 12 11 10 9 8|7 6 5 4 3 2 1 0|
///
/// 31 bit - Sign bit
/// 30 - 23 bit exponent
/// 22 - 0 bit mantissa

fn decode_float(sign_bit: u8, mantissa: f32, exponent: u32) -> f32 {
    <i8 as Into<f32>>::into((-1_i8).pow(sign_bit.into()))
        * mantissa
        * 2_u8.pow(exponent - 127) as f32
}

/// The sign bit is at position 31
/// So shifting 31 positions to the right
/// will make it the most significant bit
fn isolate_sign_bit(value: f32) -> u32 {
    value.to_bits() >> 31
}

/// The exponent takes 7 bits, and it starts at
/// position 23, that means when we shift
/// that quantity to the right we are left with
/// 9 bits.
/// To take that 9 bit out, we apply a 0xFF mask
/// F = 1111
/// That will give us one byte leaving the other
/// portion out.
fn isolate_exponent(value: f32) -> i32 {
    ((value.to_bits() >> 23) & 0xff) as i32 - 127
}

/// mantissa is represented by 23 bits, which cannot
/// be isolate by just applying 0x7fffff
/// There's a most complex process
fn isolate_mantissa(value: f32) -> f32 {
    let n_bits = value.to_bits();
    let mut mantissa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }

    mantissa
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decoding_float() {
        assert_eq!(42.42, decode_float(0, 1.325625, 132));
    }

    #[test]
    fn isolating_sign_bit() {
        assert_eq!(0, isolate_sign_bit(42.42));
    }

    #[test]
    fn isolating_mantissa() {
        assert_eq!(1.325625, isolate_mantissa(42.42));
    }
}
