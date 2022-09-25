fn main() {
    let base: usize = 41;

    for i in 1..base + 1 {
        if i % 2 != 0 {
            println!("{:^1$}", "3".repeat(i), base);
        }
    }

    println!("{:^1$}", "|||", base);
}
