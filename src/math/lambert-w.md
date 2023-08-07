# Lambert W 함수

Lambert W 함수를 핼리의 방법으로 빠르게 계산할 수 있습니다. 아래 구현에서 서로
다른 브랜치 값을 계산하고 싶다면 적절한 초기값을 주면 됩니다.

```rust
// https://en.wikipedia.org/wiki/Lambert_W_function#Numerical_evaluation
// R. M. Corless, G. H. Gonnet, D. E. G. Hare, D. J. Jeffrey and D. E. Knuth,
// "On the Lambert W Function". https://cs.uwaterloo.ca/research/tr/1993/03/W.pdf
fn wlog(z: f64) -> f64 {
    let u = z.abs().ln();
    let mut w = u - u.abs().ln();
    for _ in 0..5 {
        let x = w.exp();
        let i = w.mul_add(x, -z);
        let j = w + 1.0;
        w -= i / (x * j - (w + 2.0) * i / 2.0 / j);
    }
    w
}
```
