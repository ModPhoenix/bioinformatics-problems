pub fn fib_rabbits(n: u64, k: u64) -> u64 {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib_rabbits(n - 1, k) + fib_rabbits(n - 2, k) * k,
    }
}

// pub fn fib_rabbits(month: u64, peg: u64) -> u64 {
//     (1..month)
//         .fold((0, 1), |gen, _| (gen.1, gen.0 * peg + gen.1))
//         .1
// }

#[cfg(test)]
mod tests {
    use crate::fib_rabbits;

    #[test]
    fn it_works() {
        assert_eq!(fib_rabbits(5, 3), 19);
    }
}
