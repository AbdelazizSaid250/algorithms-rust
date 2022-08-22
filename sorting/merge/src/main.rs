use std::fmt::Debug;

fn main() {
    let mut array = [3, 2, 5, 1, 4];

    merge_sort(&mut array);

    println!("{:?}", array);
}

// 3 2 5 1 4
// 3 2  5  1 4

fn merge_sort<T: Copy + Ord + Debug>(array: &mut [T]) {
    let len = array.len();
    let middle = len / 2;

    if len <= 1 {
        return;
    }

    merge_sort(&mut array[0..middle]);
    merge_sort(&mut array[middle..len]);

    let mut vec: Vec<T> = array.to_vec();

    merge(&array[0..middle], &array[middle..len], &mut vec);

    array.copy_from_slice(&vec);
}


fn merge<T: Copy + PartialOrd>(array1: &[T], array2: &[T], vec: &mut [T]) {
    assert_eq!(array1.len() + array2.len(), vec.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < array1.len() && j < array2.len() {
        if array1[i] < array2[j] {
            vec[k] = array1[i];
            k += 1;
            i += 1;
        } else {
            vec[k] = array2[j];
            k += 1;
            j += 1;
        }
    }
    if i < array1.len() {
        vec[k..].copy_from_slice(&array1[i..]);
    }
    if j < array2.len() {
        vec[k..].copy_from_slice(&array2[j..]);
    }
}