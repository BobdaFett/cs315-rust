use crate::hw5::Point;

/// Sorts a passed array.
pub fn qsort(arr: &mut Vec<Point>, start: isize, end: isize, cmp: fn(Point, Point) -> bool) {
    if start < end {
        let p = partition(arr, start, end, cmp);
        qsort(arr, start, p - 1, cmp);
        qsort(arr, p + 1, end, cmp);
    }
}

fn partition(arr: &mut Vec<Point>, start: isize, end: isize, cmp: fn(Point, Point) -> bool) -> isize {
    let pivot = start;
    let mut left = start;
    let mut right = end;

    while left < right {
        while cmp(arr[left as usize], arr[pivot as usize]) {
            left += 1;
        }
        println!("right = {}", right);
        println!("right value = {:?}", arr[right as usize]);
        while (right >= 0) & (!cmp(arr[right as usize], arr[pivot as usize])) {
            right -= 1;
        }
        // println!("Swapping {} and {}", left, right);
        arr.swap(left as usize, right as usize);
    }

    // swap pivot and leftmost sorted value.
    arr.swap(left as usize, pivot as usize);
    left  // location where pivot now is.
}