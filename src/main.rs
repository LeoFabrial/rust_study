fn main() {
    let base: usize = 9;

    for i in 1..base + 1 {
        if i % 2 != 0 {
            println!("{:^1$}", "3".repeat(i), base);
        }
    }
}
