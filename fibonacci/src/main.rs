fn main() {
    println!("Hello, world!");
}

pub fn fib(nbr: u32) -> u32 {
    if nbr < 2 {
        return nbr;
    }
    return fib(nbr - 1) + fib(nbr - 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_3() {
        assert_eq!(fib(3), 2);
    }
}