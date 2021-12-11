use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn from_input(s: &str) -> Point {
        let parts = s.split(",")
            .map(|s| i32::from_str(s).unwrap())
            .collect::<Vec<i32>>();

        Point {
            x: parts[0],
            y: parts[1],
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x={},y={}", self.x, self.y)
    }
}

impl Line {
    fn from_input(s: &String) -> Line {
        let points = s.split(" -> ")
            .map(|s| Point::from_input(s))
            .collect::<Vec<Point>>();

        Line {
            start: points[0],
            end: points[1],
        }
    }

    fn is_horizontal(self) -> bool {
        return self.start.x == self.end.x;
    }

    fn is_vertical(self) -> bool {
        return self.start.y == self.end.y;
    }

    fn path(self) -> Vec<Point> {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        let nx = dx.abs() as f64;
        let ny = dy.abs() as f64;

        let sign_x = match dx > 0 {
            true => { 1 }
            false => { -1 }
        };
        let sign_y = match dy > 0 {
            true => { 1 }
            false => { -1 }
        };

        let mut ix = 0.0;
        let mut iy = 0.0;

        let mut px = self.start.x;
        let mut py = self.start.y;

        let mut path = vec![self.start];
        while ix < nx || iy < ny {
            let decision = (1.0 + 2.0 * ix) * ny - (1.0 + 2.0 * iy) * nx;
            if decision == 0.0 {
                px += sign_x;
                py += sign_y;
                ix += 1.0;
                iy += 1.0;
            } else if decision < 0.0 {
                px += sign_x;
                ix += 1.0;
            } else {
                py += sign_y;
                iy += 1.0;
            }
            path.push(Point { x: px, y: py });
        }
        return path;
    }
}

pub fn task1(input: Vec<String>) {
    let mut counter = HashMap::<Point, i32>::new();

    input.iter()
        .map(|s| Line::from_input(s))
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .flat_map(|line| line.path())
        .for_each(|point| {
            *counter.entry(point).or_insert(0) += 1;
        });

    let intersects = counter.iter()
        .filter(|(_, c)| **c > 1)
        .count();

    println!("intersections {}", intersects);
}

pub fn task2(input: Vec<String>) {
    let mut counter = HashMap::<Point, i32>::new();

    input.iter()
        .map(|s| Line::from_input(s))
        .flat_map(|line| line.path())
        .for_each(|point| {
            *counter.entry(point).or_insert(0) += 1;
        });

    let intersects = counter.iter()
        .filter(|(_, c)| **c > 1)
        .count();

    println!("intersections {}", intersects);
}