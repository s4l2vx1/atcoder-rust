fn nck(n: usize, k: usize) -> usize {
    if k > n { return 0; }
    if n - k < k { return nck(n, n - k); }
    (0..=n).rev().zip(1..=k).fold(1, |r, (n, d)| r * n / d)
}

fn extended_gcd<T: PrimInt>(a: T, b: T) -> (T, T) {
    if b == T::zero() {
        (T::one(), T::zero())
    } else {
        let (x, y) = extended_gcd(b, a % b);
        let q = a / b;
        (y, x - q * y)
    }
}