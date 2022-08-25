fn main() {
    let mut array = [4, 2, 5, 1, 3];
    let len = array.len();

    quick_sort(&mut array, 0, len);

    println!("{:?}", array);
}


fn quick_sort(array: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let partition_index = partition(array, start, end);

    quick_sort(array, start, partition_index - 1);
    quick_sort(array, partition_index, end);
}

// 4, 2, 5, 1, 3
// 2, 4, 5, 1, 3
// 2, 1, 5, 4, 3
// 2, 1, 3, 4, 5
fn partition(array: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = array[end-1];
    let mut partition_index = start;

    for i in start..end {
        if array[i] <= pivot {
            array.swap(i, partition_index);
            partition_index += 1;
        }
    }

    partition_index
}