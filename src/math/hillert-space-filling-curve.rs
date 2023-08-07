const fn approximate_ord(n: u32, mut x: u32, mut y: u32) -> u32 {
    let mut s = n >> 1;
    let mut d = 0;
    while s > 0 {
        let rx = if x & s > 0 { 1 } else { 0 };
        let ry = if y & s > 0 { 1 } else { 0 };
        d += s * s * (((rx << 1) + rx) ^ ry);
        (x, y) = rot(n, x, y, rx, ry);
        s >>= 1;
    }
    d
}

const fn rot(n: u32, mut x: u32, mut y: u32, rx: u32, ry: u32) -> (u32, u32) {
    if ry == 0 {
        if rx == 1 {
            x = n - 1 - x;
            y = n - 1 - y;
        }
        (y, x)
    } else {
        (x, y)
    }
}
