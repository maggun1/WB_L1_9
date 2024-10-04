fn set_bit(value: i64, index: u32, bit: bool) -> i64 {
    if bit {
        value | (1 << index)
    } else {
        value & !(1 << index)
    }
}

fn main() {
    let value: i64 = 0b1010;

    println!("Исходное число: {} ({:b})", value, value);

    let new_value = set_bit(value, 2, true);
    println!("Число после установки 2-го бита в 1: {} ({:b})", new_value, new_value);

    let new_value = set_bit(new_value, 3, false);
    println!("Число после установки 3-го бита в 0: {} ({:b})", new_value, new_value);

}
