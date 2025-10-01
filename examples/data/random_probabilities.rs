fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("max of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("max of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}

/// Following function generates `f32` values in interval [0,1] from a `u8`
fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000_00000000_0000000;
    // Align n to 32 bits, then increase its value by left shift
    let large_n = (n as u32) << 15;
    // Merge the `base` with `large_n` using bitwise OR
    let f32_bits = base | large_n;
    // Interpret `f32` from bits
    let m = f32::from_bits(f32_bits);
    // Normalizing the output range
    2.0 * (m - 0.5)
}

/// Following function is slower because of division
#[allow(dead_code)]
fn mock_rand_slower(n: u8) -> f32 {
    (n as f32) / 255.0
}
