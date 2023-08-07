fn main() {
    let pattern = b"GCAG";
    let k = preprocess(pattern);
    for i in 0..256 {
        if k[i] != !0 {
            println!("{} -> {:064b}", i as u8 as char, k[i]);
        }
    }
    shiftor(b"AB", b"ABAABAABBAB", |i| println!("OUTPUT {i}"));
    // OUTPUT 0
    // OUTPUT 3
    // OUTPUT 6
    // OUTPUT 9
}

fn preprocess(pattern: &[u8]) -> [u64; 256] {
    let mut k = [!0; 256];
    let mut i = !1;
    for &x in pattern {
        k[x as usize] &= i;
        i = i << 1 | 1;
    }
    k
}

fn shiftor<F>(pattern: &[u8], haystack: &[u8], mut pred: F)
where
    F: FnMut(usize),
{
    assert!(pattern.len() <= 64);
    let mut state = !0;
    let k = preprocess(pattern);
    let lim = !0 << (pattern.len() - 1);
    for (i, &x) in haystack.iter().enumerate() {
        state = state << 1 | k[x as usize];
        if state < lim {
            pred(i + 1 - pattern.len());
        }
    }
}
