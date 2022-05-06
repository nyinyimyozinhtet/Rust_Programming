fn main() {
    let i = 6;
    match i {
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=5 => println!("3,4,5"),
        _=> println!("default")
    }
}
