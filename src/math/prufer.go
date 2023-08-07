import (
	"container/heap"
)

func pruferConstruct(rng *rand.Rand, n int, biconnect func(int, int)) {
	if n >= 3 {
		v := make([]int, n-2)
		for i := range v {
			v[i] = rng.Intn(n)
		}
		pruferDecode(v, biconnect)
	} else if n >= 2 {
		biconnect(0, 1)
	}
}

func pruferDecode(v []int, biconnect func(int, int)) {
	w := make([]int, len(v)+2)
	for _, x := range v {
		w[x]++
	}
	var leafs ints
	for i := range w {
		if w[i] == 0 {
			leafs = append(leafs, i)
		}
	}
	heap.Init(&leafs)
	for _, x := range v {
		i := heap.Pop(&leafs).(int)
		w[x]--
		if w[x] == 0 {
			heap.Push(&leafs, x)
		}
		biconnect(i, x)
	}
	biconnect(leafs[0], leafs[1])
}

type ints []int

func (s ints) Len() int           { return len(s) }
func (s ints) Less(i, j int) bool { return s[i] <= s[j] }
func (s ints) Swap(i, j int)      { s[i], s[j] = s[j], s[i] }
func (s *ints) Push(x any) {
	*s = append(*s, x.(int))
}
func (s *ints) Pop() any {
	i := len(*s) - 1
	x := (*s)[i]
	*s = (*s)[:i]
	return x
}

