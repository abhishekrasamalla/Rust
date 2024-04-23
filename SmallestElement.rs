fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort_unstable();     
    if k < sorted_arr.len() {
        Some(sorted_arr[k - 1])     } else {
        None // k is out of range
    }
}

fn main() {
    let array = [7, 10, 4, 3, 20, 15];
    let k = 3;

    if let Some(kth_smallest) = kth_smallest_element(&array, k) {
        println!("The {}th smallest element is {}.", k, kth_smallest);
    } else {
        println!("Invalid input.");
    }
}
