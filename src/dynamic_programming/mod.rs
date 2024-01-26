pub fn fibonacci(x: u32) -> u32 {
    if x == 1 || x == 2 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::dynamic_programming::fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
    }
}
