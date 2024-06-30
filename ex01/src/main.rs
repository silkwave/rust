fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        let mut min_index = i;
        for j in (i + 1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn main() {
    let mut arr = [64, 25, 12, 22, 11];
    println!("Unsorted array: {:?}", arr);
    selection_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
