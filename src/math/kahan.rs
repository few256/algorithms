use std::ops::AddAssign;

// Neumaier, improved Kahan–Babuška algorithm
#[derive(Clone)]
struct ExactAdder {
    tot: f64,
    c: f64,
}

impl ExactAdder {
    const fn new(tot: f64) -> Self {
        Self { tot, c: 0.0 }
    }

    fn finish(&self) -> f64 {
        self.tot + self.c
    }
}

impl AddAssign<f64> for ExactAdder {
    fn add_assign(&mut self, x: f64) {
        let t = self.tot + x;
        if self.tot.abs() >= x.abs() {
            self.c += (self.tot - t) + x;
        } else {
            self.c += (x - t) + self.tot;
        }
        self.tot = t;
    }
}
