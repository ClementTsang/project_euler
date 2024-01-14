fn main() {
    let sum: u32 = (3..1000).filter(|v| v % 3 == 0 || v % 5 == 0).sum();
    println!("Problem 1: {sum}");
}
