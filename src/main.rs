pub mod hw5;

use hw5::{Point, qsort, xmin, ymin};

fn main() {
    let temp = Point::from(6.0, 7.0);
    let temp2 = Point::from(1.34, 5.6);
    
    let mut arr = vec![temp, temp2];  // starts as (6, 7), (1.34, 5.6)
    let mut arr2 = vec![temp2, temp]; // starts as (1.34, 5.6), (6, 7)
    
    qsort(xmin, &mut arr);
    qsort(ymin, &mut arr2);
    
    print_vec(&arr);
    print_vec(&arr2);
}

fn print_vec(arr: &Vec<Point>) {
    for p in arr {
        print!("({}, {}), ", p.x, p.y)
    }
    println!();
}