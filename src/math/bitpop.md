# Parallel population count

하나의 레지스터에 켜져 있는 비트가 얼마나 많은 가를 비트 수준에서 병렬적으로
계산할 수 있습니다. 이 방법을 parallel popuplation count 라고 합니다. 구현은
아래와 같습니다.

```rust
n = (n >> 1 & 0x55555555) + (n & 0x55555555);
n = (n >> 2 & 0x33333333) + (n & 0x33333333);
n = (n >> 4 & 0x0F0F0F0F) + (n & 0x0F0F0F0F);
n = (n >> 8 & 0x00FF00FF) + (n & 0x00FF00FF);
n = (n >> 16 & 0x0000FFFF) + (n & 0x0000FFFF);
```
