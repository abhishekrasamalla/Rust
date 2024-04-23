fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_left = arr[len / 2 - 1];
        let mid_right = arr[len / 2];
        (mid_left as f64 + mid_right as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    let sorted_array1 = [1, 2, 3, 4, 5];
    let sorted_array2 = [1, 2, 3, 4, 5, 6];

    let median1 = find_median(&sorted_array1);
    let median2 = find_median(&sorted_array2);

    println!("Median of {:?}: {}", sorted_array1, median1);
    println!("Median of {:?}: {}", sorted_array2, median2);
}
