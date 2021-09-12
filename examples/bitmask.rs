fn bit_twiddling(original: u8, bit: u8) {
    let mask = 1 << bit;

    println!(
        "Original: {:b}, Set: {:b}, Cleared: {:b}, Toggled: {:b}",
        original,
        original |  mask,
        original & !mask,
        original ^  mask
    );
}

fn main() {    
    bit_twiddling(8, 2);
    bit_twiddling(2, 1);

    let result = 1 << 31;
    println!("(1 << 32) => {}",result);
}