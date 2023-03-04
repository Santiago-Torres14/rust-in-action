fn main() {
    println!("{}", mock_rand(2));
}

fn mock_rand(n: u8) -> f32 {
    println!("{:08b}", n);
    let base: u32 = 0b0011_1111_0000_0000_0000_0000_0000_0000;
    let large_n = (n as u32) << 15;

    println!("{:032b}", large_n);

    let f32_bits = base | large_n;

    let m = f32::from_bits(f32_bits);
    2.0 * (m - 0.5)
}

#[cfg(test)]
mod test {
    #[test]
    fn mock_rand() {}
}
