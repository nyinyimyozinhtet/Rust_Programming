fn main() {
    println!("{}", is_even(1));
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0  //return bool
}
