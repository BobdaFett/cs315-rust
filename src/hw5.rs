/// Simply stores a coordinate pair. Could use a Pair, but this is more fun.
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn from(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

/// Utility function to compare the x values of 2 points.
/// Returns true if the first point has a smaller value.
pub fn xmin(a: Point, b: Point) -> bool {
    a.x < b.x
}

/// Utility function to compare the y values of 2 points.
/// Returns true if the first point has a smaller value.
pub fn ymin(a: Point, b: Point) -> bool {
    a.y < b.y
}

/// Utility function to find the distance between two points.
fn dist_func(a: &Point, b: &Point) -> f32 {
    f32::sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y))
}

/// Utility function that returns the minimum of two f32 values.
fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// Returns the smallest distance between a series of points.
/// Requires a list of the points sorted by their x coords, and another by their y coords.
pub fn closest_pair(sorted_x: Vec<Point>, sorted_y: Vec<Point>) -> f32 {
    let n = sorted_x.len();

    if n <= 3 {
        let mut dmin2: f32 = 0.0;
        for p1 in sorted_x.iter() {
            for p2 in sorted_x.iter() {
                let current_dist = dist_func(p1, p2);
                if (dmin2 == 0.0) | (current_dist < dmin2) {
                    dmin2 = current_dist;
                }
            }
        }
        return f32::sqrt(dmin2);
    }

    let m = sorted_x[f32::ceil(n as f32 / 2.0) as usize - 1].x;
    let m_index = f32::ceil(n as f32 / 2.0) as usize - 1;

    let mut left_x: Vec<Point> = Vec::new();
    let mut right_x: Vec<Point> = Vec::new();
    for (i, p) in sorted_x.iter().enumerate() {
        if i < m_index {
            left_x.push(*p);
        } else {
            right_x.push(*p);
        }
    }

    let mut left_y: Vec<Point> = Vec::new();
    let mut right_y: Vec<Point> = Vec::new();
    for p in sorted_y.iter() {
        if p.x <= m {
            left_y.push(*p);
        } else {
            right_y.push(*p);
        }
    }

    let dist_l = closest_pair(left_x, left_y);
    let dist_r = closest_pair(right_x, right_y);

    // Search the strip region
    let mut min_dist = min(dist_l, dist_r);

    // All points in sorted_y where abs(p.x - m) < d
    let mut strip: Vec<Point> = Vec::new();
    for p in sorted_y.iter() {
        if f32::abs(p.x - m) < min_dist {
            strip.push(*p);
        }
    }

    let strip_l = strip.len();
    println!("strip_l = {}", strip_l);

    for i in 0..(strip_l - 2) {  // usize values cannot be negative. is there another way to go about doing this?
        let mut k = i + 1;
        while (k < strip_l) & ((strip[k].y - strip[i].y) < min_dist) {
            let current_dist = dist_func(&strip[k], &strip[i]);
            if current_dist < min_dist {
                min_dist = current_dist;
            }
            k += 1;
        }
    }

    min_dist
}

/// A quick sort function that implements a function pointer for ease of access.
/// This is exclusively for use in this program. I'm not using template types for this right now.
pub fn qsort(order: fn(Point, Point) -> bool, arr: &mut Vec<Point>) {
    quick_sort(arr, 0, (arr.len() - 1) as isize, order);
}

/// Runs the full quick sort and handles partitioning.
/// Passes the function pointer into the partitioning function.
/// Private function that is only called as a result of using qsort.
fn quick_sort(arr: &mut Vec<Point>, low: isize, high: isize, order: fn(Point, Point) -> bool) {
    if low < high {
        let p = partition(arr, low, high, order);
        quick_sort(arr, low, p - 1, order);
        quick_sort(arr, p + 1, high, order);
    }
}

/// Partitions the array around a pivot value.
/// Returns the index of the pivot value.
fn partition(arr: &mut Vec<Point>, start: isize, end: isize, order: fn(Point, Point) -> bool) -> isize {
    let pivot = end as usize;
    let mut left = start - 1;
    let mut right = end;

    loop {
        left += 1;
        while order(arr[left as usize], arr[pivot]) {
            left += 1;
        }
        right -= 1;
        while right >= 0 && !order(arr[right as usize], arr[pivot]) {
            right -= 1;
        }
        if left >= right {
            break;
        } else {
            arr.swap(left as usize, right as usize);
        }
    }
    arr.swap(left as usize, pivot);
    left
}