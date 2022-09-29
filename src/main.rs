fn main() {
    fibonacci(7);
}

fn fibonacci(n: u8) -> u128 {
    let agr: (u128, u128, u128) = (0, 0, 1);
    let (mut result, mut prev, mut curr) = agr;

    for i in 1..=n {
        result = prev + curr; // 1, 2
        if dbg!(i) > 1 {
            prev = curr; // 0 -> 1
            curr = result; // 1 -> 1
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_test() {
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(124), 36726740705505779255899443);
        assert_eq!(fibonacci(125), 59425114757512643212875125);
        assert_eq!(fibonacci(185), 205697230343233228174223751303346572685);
    }
}
