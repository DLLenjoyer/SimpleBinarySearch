fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}
fn main() {
    let arr = vec![1, 3, 5, 7, 9, 10 , 11,];
    let target = 11;
    match binary_search(&arr, target) {
        Some(index) => println!("Element {} was found {}", target, index),
        None => println!("Element {} was not found", target),
    }
}