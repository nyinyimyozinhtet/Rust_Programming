fn main() {
    let mut i = 0;
    while i < 4 {
        println!("{}", i);
        i+=1;
        if i == 3 {
            println!("exit");
            break
        }
    }
}
