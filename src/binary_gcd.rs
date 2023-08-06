fn gcd(mut x: u64, mut y: u64) -> u64 {
    if x == 0 {
        return y;
    }
    if y == 0 {
        return x;
    }
    let i = x.trailing_zeros();
    let j = y.trailing_zeros();
    let shift = i.min(j);
    x >>= i;
    y >>= j;
    while y != 0 {
        y >>= y.trailing_zeros();
        if x > y {
            (x, y) = (y, x);
        }
        y -= x;
    }
    x << shift
}
