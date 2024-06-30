fn insertion_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 1..len {
        let key = arr[i];
        let mut j = i as isize - 1; // `isize` 타입으로 변경하지 않음

        while j >= 0 && arr[j as usize] > key {
            arr[j as usize + 1] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key; // `j`의 타입을 usize로 변환
    }
}

fn main() {
    let mut arr = [12, 11, 13, 5, 6];
    println!("Unsorted array: {:?}", arr);
    insertion_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
