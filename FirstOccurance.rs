fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left <= right {
        let mid = left + (right - left) / 2;
        
        if arr[mid] == target && (mid == 0 || arr[mid - 1] < target) {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    
    None
}

fn main() {
    let sorted_array = [1, 2, 3, 3, 4, 5, 5, 6, 7, 8];
    let target_number = 5;
    
    if let Some(index) = first_occurrence_index(&sorted_array, target_number) {
        println!("The first occurrence of {} is at index {}.", target_number, index);
    } else {
        println!("{} is not found in the array.", target_number);
    }
}
