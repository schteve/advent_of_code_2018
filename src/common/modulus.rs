
pub fn modulo(n: i32, modulus: i32) -> i32 {
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
