# 라그랑지 반전 공식

해석적 함수 \\( \phi(z) \\) 가 \\( \phi(a) = b \\) 이고 \\( \phi'(a) \ne 0 \\)
이라고 해봅시다. 그리고 \\( \psi(z) \\) 를 다음과 같이 정의합니다.

\\[ \psi(z) = \frac{z - a}{\phi(z) - a} \\]

그렇다면 해석적 함수 \\( f(z) \\) 를 다음 형태로 전개할 수 있습니다.

\\[
  f(z) = f(a) + \sum_{m = 1}^{n}{ \frac{(\phi(z) - b)^m}{m!} \left[ \frac{d^{m - 1}}{da^{m - 1}} f'(a)\psi(a)^m \right] } + R_n
\\]
여기서
\\[
  R_n = \frac{1}{2 \pi i} \int_{a}^{z}{ \int_{\gamma}{
    \left( \frac{\phi(z) - b}{\phi(t) - b} \right)^{n - 1}
    \frac{f'(t) \phi'(z)}{\phi(t) - \phi(z)} \ dt dz
  } }
\\]

이고 \\( \gamma \\) 는 \\( a \\) 와 \\( z \\) 를 포함하는 \\( t \\) 에 대한
닫힌 경로이며 \\( \zeta \\) 가 \\( \gamma \\) 내부에 있을 때 \\( \phi(t)
= \phi(\zeta) \\) 를 만족하는 근은 단순근 \\( t = \zeta \\) 말곤 없습니다.

위 식을 유도해봅시다.

\\begin{align*}
f(z) - f(a)
& = \int_a^z{f'(\zeta)d\zeta} \\\\
& = \frac{1}{2 \pi i} \int_a^z{\int_{\gamma}{ \frac{f'(t) \phi'(\zeta)}{
    \phi(t) - \phi(\zeta)}\ dt\ d\zeta }} \\\\
& = \frac{1}{2 \pi i} \int_a^z{\int_{\gamma}{
      \left[ \sum_{m = 0}^{n - 2}{\left( \frac{\phi(\zeta) - b}{\phi(t) - b}\right)^m} + \frac{(\phi(\zeta) - b)^{n-1}}{(\phi(t) - b)^{n-2}(\phi(t) - \phi(\zeta))} \right]
      \frac{f'(t) \phi'(\zeta)}{ \phi(t) - b}\ dt\ d\zeta }
    }
\\end{align*}

입니다. 그리고

\\begin{align*}
& \frac{1}{2 \pi i} \int_a^z{\int_{\gamma}{
  \left( \frac{\phi(\zeta) - b}{\phi(t) - b}\right)^m
  \frac{f'(t) \phi'(\zeta)}{ \phi(t) - b}\ dt\ d\zeta }
} \\\\
& \qquad = \frac{(\phi(z) - b)^{m + 1}}{2 \pi i (m + 1)} \int_\gamma{
\frac{f'(t)}{(\phi(t) - b)^{m + 1}}\ dt} \\\\
& \qquad = \frac{(\phi(z) - b)^{m + 1}}{(m + 1)!} \frac{d^m}{da^m} f'(a)\psi(a)^{m + 1}
\\end{align*}

이므로 이를 정리하면 라그랑지 반전 공식이 유도됩니다.
