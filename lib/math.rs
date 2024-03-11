fn nck(n: usize, k: usize) -> usize {
    if k > n { return 0; }
    if n - k < k { return nck(n, n - k); }
    (0..=n).rev().zip(1..=k).fold(1, |r, (n, d)| r * n / d)
}