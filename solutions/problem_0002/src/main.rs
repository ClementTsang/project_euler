fn main() {
    let mut a = 1;
    let mut b = 2;

    let mut curr = a + b;
    let mut sum = b;
    while curr < 4_000_000 {
        if curr % 2 == 0 {
            sum += curr;
        }
        a = b;
        b = curr;

        curr = a + b;
    }

    println!("Problem 2: {sum}");
}
