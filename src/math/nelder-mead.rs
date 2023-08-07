#[derive(Copy, Clone, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

// https://en.wikipedia.org/wiki/Nelder–Mead_method
fn nmm<F>(
    Triangle(mut x1, mut x2, mut x3): Triangle,
    finite: usize,
    mut evaluate: F,
) -> (Point, f64)
where
    F: FnMut(Point) -> f64,
{
    let r = 1.0; // r > 0
    let e = 2.0; // e > 1
    let c = 0.5; // 0 < c <= 0.5
    let s = 0.5; // 0 < s < 1

    let mut y1 = evaluate(x1); // the best
    let mut y2 = evaluate(x2); // the second worst
    let mut y3 = evaluate(x3); // the worst

    for _ in 0..finite {
        // 1. Order
        if y2 > y3 {
            (x2, y2, x3, y3) = (x3, y3, x2, y2);
        }
        if y1 > y2 {
            (x1, y1, x2, y2) = (x2, y2, x1, y1);
        }
        if y2 > y3 {
            (x2, y2, x3, y3) = (x3, y3, x2, y2);
        }

        // 2. Calculate the centroid of all points except x3
        let xo = x1 + (x2 - x1) / 2.0;

        // 3. Reflection
        // Compute reflected point
        let xr = xo + r * (xo - x3);
        let yr = evaluate(xr);
        if y1 <= yr && yr < y2 {
            (x3, y3) = (xr, yr);
            continue;
        }

        // 4. Expansion
        if yr < y1 {
            let xe = xo + e * (xr - xo);
            let ye = evaluate(xe);
            if ye < yr {
                (x3, y3) = (xe, ye);
            } else {
                (x3, y3) = (xr, yr);
            }
            continue;
        }

        // 5. Contraction
        if yr < y3 {
            let xc = xo + c * (xr - xo);
            let yc = evaluate(xc);
            if yc < yr {
                (x3, y3) = (xc, yc);
                continue;
            }
        } else {
            let xc = xo + c * (x3 - xo);
            let yc = evaluate(xc);
            if yc < y3 {
                (x3, y3) = (xc, yc);
                continue;
            }
        }

        // 6. Shrink
        x2 = x1 + s * (x2 - x1);
        x3 = x1 + s * (x3 - x1);
        y2 = evaluate(x2);
        y3 = evaluate(x3);
    }
    if y2 > y3 {
        (x2, y2) = (x3, y3);
    }
    if y1 > y2 {
        (x1, y1) = (x2, y2);
    }
    (x1, y1)
}

use std::ops::{Add, Div, Mul, Sub};

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;
    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
