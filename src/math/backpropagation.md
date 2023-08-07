# 역전파 알고리즘

\\[ 
  \renewcommand{\R}{\mathbb{R}}
  \renewcommand{\d}{\mathrm{d}}
\\]

\\( \R \\) 을 매개하는 세 개의 공간이 다음과 같다고 하겠습니다.

\\[
  (x_1, \cdots, x_m) \in \R^m \overset{f}{\rightarrow}
  (y_1, \cdots, y_n) \in \R^n \rightarrow
  E \in \R
\\]

여기서 \\( \d E \\) 가 \\( y_i \\) 로 표현됨을 이미 알고 있다고
가정합니다. 이 때 우리는 \\( \d E \\) 가 \\( \d y_i \\) 를 사용해서 \\( \d x_i
\\) 로 표현되길 원합니다. 이렇게 \\( \d E \\) 가 \\( \d x_i \\) 로 표현하는
방법을 역전파라 부릅니다.

연쇄 법칙을 사용해 \\( \d E \\) 를 \\( \d x_i \\) 로 표현할 수 있습니다. 하나의
\\( x_i \\) 에 대하여

\\[
  \frac{\partial E}{\partial x_i} =
    \sum_{j}{
      \left ( \frac{\partial E}{\partial y_j} \right )
      \left ( \frac{\partial y_j}{\partial x_i} \right )
    }
\\]

이고 이 때 \\( \frac{ \partial E }{ \partial y_j } \\) 를 가정에 의해 알고
있습니다. 그리고 \\( f(\mathrm{x}) = \mathrm{y} \\) 이므로 \\( \frac{ \partial
y_j }{ \partial x_i } = \frac{ \partial f_j }{ \partial x_i }  \\)
입니다. 여기서 \\( f_j \\) 는 \\( f \\) 의 \\( j \\) 번째 함숫값입니다.

이 계산을 모든 \\( x_i \\) 에 대해 계산해줌으로써 \\( \frac{ \partial E }{
\partial \mathrm{x} } \\) 을 계산할 수 있습니다.

