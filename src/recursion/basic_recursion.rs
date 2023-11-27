/// Determine pow(base, result)
pub fn power(base: u32, exponent: u32) -> u32 {
    if exponent == 0 {
        return 1;
    }

    base * power(base, exponent - 1)
}

/// Determine pow(base, result) with tail optimization
pub fn power_tail(base: u32, exponent: u32, tail: u32) -> u32 {
    if exponent == 0 {
        return tail;
    }
    power_tail(base, exponent - 1, tail * base)
}

/// Determine factorial of `val`
pub fn factorial(val: i32) -> i32 {
    if val == 1 {
        return 1;
    }
    val * factorial(val - 1)
}

/// Determine factorial of `val` with tail optimization
pub fn factorial_tail(val: i32, tail: i32) -> i32 {
    if val == 1 {
        return tail;
    }
    factorial_tail(val - 1, tail * val)
}

/// Determine the product of all the `vec` values
pub fn product_of_array(mut vec: Vec<u32>) -> u32 {
    let val = vec.pop();

    if val == None {
        return 1;
    }

    val.unwrap() * product_of_array(vec)
}

/// Determine the product of all the `vec` values, with tail optimization
pub fn product_of_array_tail(mut vec: Vec<u32>, tail: u32) -> u32 {
    let val = vec.pop();

    if val == None {
        return tail;
    }

    product_of_array_tail(vec, tail * val.unwrap())
}

// Determine the sum of all integers from 0 to `val`
pub fn recursive_range(val: i32) -> i32 {
    if val == 1 {
        return 1;
    }
    val + recursive_range(val - 1)
}

// Determine the sum of all integers from 0 to `val`, with tail optimization
pub fn recursive_range_tail(val: i32, tail: i32) -> i32 {
    if val == 0 {
        return tail;
    }
    recursive_range_tail(val - 1, val + tail)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn power_should_work() {
        assert_eq!(power(2, 2), 4);
        assert_eq!(power(2, 4), 16);
    }

    #[test]
    fn power_tail_should_work() {
        assert_eq!(power_tail(2, 2, 1), 4);
        assert_eq!(power_tail(2, 4, 1), 16);
    }

    #[test]
    fn factorial_should_work() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(7), 5040);
    }

    #[test]
    fn factorial_tail_should_work() {
        assert_eq!(factorial_tail(1, 1), 1);
        assert_eq!(factorial_tail(2, 1), 2);
        assert_eq!(factorial_tail(4, 1), 24);
        assert_eq!(factorial_tail(7, 1), 5040);
    }

    #[test]
    fn product_of_array_should_work() {
        assert_eq!(product_of_array(vec![1, 2, 3]), 6);
        assert_eq!(product_of_array(vec![1, 2, 3, 10]), 60);
    }

    #[test]
    fn product_of_array_tail_should_work() {
        assert_eq!(product_of_array_tail(vec![1, 2, 3], 1), 6);
        assert_eq!(product_of_array_tail(vec![1, 2, 3, 10], 1), 60);
    }

    #[test]
    fn recursive_range_should_work() {
        assert_eq!(recursive_range(6), 21);
        assert_eq!(recursive_range(10), 55);
    }

    #[test]
    fn recursive_range_tail_should_work() {
        assert_eq!(recursive_range_tail(6, 0), 21);
        assert_eq!(recursive_range_tail(10, 0), 55);
    }
}
