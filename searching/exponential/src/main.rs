use std::cmp::min;

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let searched_num = 9;

    let found_index = exponential_search(&array, searched_num, array.len() as i32);

    println!("The Element has been found at the index: {}", found_index)
}

fn exponential_search(array: &[i32], searched_num: i32, len: i32) -> i32 {
    if array[0] == searched_num {
        return 0;
    }

    let mut i = 1;
    while i < len && array[i as usize] <= searched_num {
        i = i * 2;
    }
    binary_search_recursion(array, searched_num, i / 2, min(i, len - 1))
}

fn binary_search_recursion(array: &[i32], searched_num: i32, start_index: i32, end_index: i32) -> i32 {
    if start_index > end_index {
        return -1;
    }

    let middle_index = (start_index + end_index) / 2;

    return if searched_num == array[middle_index as usize] {
        middle_index
    } else if searched_num < array[middle_index as usize] {
        binary_search_recursion(array, searched_num, start_index, middle_index)
    } else if searched_num > array[middle_index as usize] {
        binary_search_recursion(array, searched_num, middle_index + 1, end_index)
    } else {
        -1
    };
}