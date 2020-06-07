
pub fn modulo(n: i32, modulus: i32) -> i32 {
    assert!(modulus > 0);

    let mut m = n;

    let div = n / modulus; // Don't use the % operator because it's messy and this is almost as quick
    m -= div * modulus;

    while m < 0 {
        m += modulus;
    }
    while m >= modulus {
        m -= modulus;
    }
    m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(0, 1), 0);
        assert_eq!(modulo(5, 10), 5);
        assert_eq!(modulo(10, 5), 0);
        assert_eq!(modulo(17, 11), 6);
        assert_eq!(modulo(1_000_000_000, 29), 18);
        assert_eq!(modulo(-3, 7), 4);
        assert_eq!(modulo(-1000, 11), 1);
    }

    #[test]
    #[should_panic]
    fn test_modulo_fail_zero() {
        modulo(3, 0);
    }

    #[test]
    #[should_panic]
    fn test_modulo_fail_neg() {
        modulo(3, -1000);
    }
}
