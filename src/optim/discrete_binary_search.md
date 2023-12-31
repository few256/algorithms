# 이산적 이진 탐색

이산적 이진 탐색[^discrete]은 기초적인 알고리즘 중 하나이면서도 정확하게 구현하기 어렵기로
유명한 알고리즘입니다. 여기서 이진 탐색을 정확하게 이해하고 구현하는 방법에 대해
알아봅니다.

\\(
  \newcommand{\interval}[2]{ [#1,\ #2) }
  \newcommand{\res}[3]{ #1 \\: \Big\rvert_{ \interval{#2}{#3} } }
  \newcommand{\B}[2]{ B\ \interval{#1}{#2} }
\\)

이진 탐색이 풀고자 하는 문제는 다음과 같습니다.
\\( f: \interval{\cdot}{\cdot} \subseteq \mathbb{Z} \rightarrow \\{ 0, 1 \\} \\) 이고 \\( f \\) 는 단조 증가 함수일 때
\\( f(x^\* - 1) < f(x^\*) \\) 인 \\( x^\* \\) 를 구하는 문제가 이진 탐색이 풀고자 하는
문제입니다.

함수 \\( \res{f}{i}{j} \\) 에 대해 이진 탐색을 수행하는 함수 \\( B\ \interval{i}{j} = B_f\ \interval{i}{j} \triangleq x_{ \interval{i}{j} }^{\*} \\) 를 정의합니다.
만약 \\( \res{f}{i}{j} = 1 \\) 이라면 \\( \B{i}{j} = i \\) 입니다.
만약 \\( \res{f}{i}{j} = 0 \\) 이라면 \\( \B{i}{j} = j \\) 로 정의합니다.
이렇게 정의했다면 정의역이 공집합인 경우도 \\( f(x) = 1 \\) 인 \\( x \\) 가 없는
경우로 생각할 수 있으므로 자연스럽게 \\( \B{i}{i} = i \\) 라 정의합니다.

\\( \B{i}{j} \\) 를 이렇게 정의하면 주어진 정의역에 해가 존재하지 않을 때 언제나
\\( j \\) 가 반환되게 되므로 이를 이진 탐색의 불변량으로
볼 수 있습니다. 이 불변량을 생각하는 것이 안전한 이진 탐색을 구현하는 데 있어
**매우 중요**합니다.

이 불변량을 염두하여 이진 탐색을 쉽게 구현할 수 있습니다.

\\[
  \begin{align*}
    \B{i}{j} & = \\begin{cases}
      \B{i}{k} & \text{if} \ f(k) = 1 \\\\
      \B{k + 1}{j} & \text{if} \ f(k) = 0 \\\\
    \\end{cases} \\\\
    & \quad \text{where} \quad k = \frac{i + j}{2} \in \interval{i}{j}
  \end{align*}
\\]

위 구현을 코드로 옮기면 아래와 같습니다.

```rust
fn binary_search<F>(mut i: usize, mut j: usize, mut pred: F) -> usize
where
    F: FnMut(usize) -> bool
{
    while i < j {
        let k = (i + j) >> 1;
        if pred(k) {
            j = k;
        } else {
            i = k + 1;
        }
    }
    i
}
```

만약 `(i + j) >> 1` 에서 오버플로우가 걱정된다면 오버플로우 없이 중간값을 구하는
다른 방법을 사용할 수 있습니다.

## 연습 문제
1. 만약 구간에서 중간값을 선택할 때 실수로 구간에 존재하지 않는 값을 선택한다면
   어떤 일이 발생하는가?
1. \\( \interval{i}{j} \\) 이 아닌 \\( (i,\ j] \\) 을 사용하는 이진 탐색을
   구현하라
1. \\( [i, j] \\) 를 사용하는 이진 탐색과 비교하면 어떤 점이 다른가?
1. 단조 증가 함수 \\( \varphi: \mathbb{Z} \rightarrow \mathbb{Z} \\) 에 대해서
   \\( \varphi(x) \ge k \\) 인 가장 작은 \\( x \\) 를 찾아라
1. 위 알고리즘은 캐시 라인을 효율적으로 쓰고 있는가?
1. 이산적 이진 탐색과 연속적 이진 탐색의 차이점은 무엇인가? 아이디어는
   비슷하지만 어떤 점이 다른가?
1. 위 이진 탐색 구현의 시간 복잡도는 어느 정도인가?
1. 구간에서 중간값을 선택할 때 조금 벗어나 살짝 옆에 있는 값을 선택한다고
   하자. 이것이 시간 복잡도에 어떤 영향을 주는가?
1. 살짝씩 중간에서 벗어나 선택할 때 시간 복잡도에 영향을 주지 않는 벗어난 정도가
   존재하는가?
1. 이진 탐색의 lower bound는 어느 정도인가?
1. 이진 탐색 알고리즘을 더 빠르게 동작하도록 개선할 수 있는가?
1. 위 아이디어를 그대로 적용해 연속적 이진 탐색을 구현한다면 무엇을 걱정해야
   하는가?
1. 이진 탐색의 upper bound를 유지하면서 연속적 이진 탐색 알고리즘을 더 빠르게
   동작하도록 개선하라. 즉 이진 탐색이 점진적으로 super-linear 해야 한다.

[^discrete]: 이 글에서 다루고자 하는 대상이 정수같이 이산적이기 때문에 이산적
이진 탐색이라 합니다.
