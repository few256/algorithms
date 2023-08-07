fn sameset(i: usize, j: usize, org: &mut [usize]) -> bool {
    discover(i, org) == discover(j, org)
}

fn discover(mut x: usize, org: &mut [usize]) -> usize {
    while x != org[x] {
        (org[x], x) = (org[org[x]], org[x]);
    }
    x
}

fn unite(mut i: usize, mut j: usize, org: &mut [usize]) -> bool {
    while org[i] != org[j] {
        if org[i] > org[j] {
            (i, j) = (j, i);
        }
        if i == org[i] {
            org[i] = org[j];
            return true;
        }
        (org[i], i) = (org[j], org[i]);
    }
    false
}
