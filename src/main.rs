pub mod hw5;

use hw5::{xmin, ymin, Point, closest_pair};
use rand::{Rng, thread_rng};

pub fn main() {
    let mut x_points: Vec<Point> = Vec::new();
    for _ in 0..10 {
        x_points.push(Point {
            x: thread_rng().gen_range(0..50) as f32,
            y: thread_rng().gen_range(0..50) as f32 });
    }
    let mut y_points = x_points.clone();

    println!("Sorting by x values");
    println!("Before: {:?}", x_points);
    hw5::qsort(xmin, &mut x_points);
    println!("After:  {:?}\n", x_points);

    println!("Sorting by y values");
    println!("Before: {:?}", y_points);
    hw5::qsort(ymin, &mut y_points);
    println!("After:  {:?}", y_points);

    closest_pair(x_points, y_points);
}
