fn main() {
    let array = [1, 3, 4, 5, 7, 9];
    let searched_num = 9;

    let found_index =
        binary_search_iterative(&array, searched_num, 0, array.len() as i32 - 1);

    println!("The Element has been found at the index: {}", found_index)
}

fn binary_search_iterative(array: &[i32], searched_num: i32, mut start_index: i32, mut end_index: i32) -> i32 {
    while start_index <= end_index {
        let middle_index = (start_index + end_index) / 2;

        if searched_num == array[middle_index as usize] {
            return middle_index;
        } else if searched_num < array[middle_index as usize] {
            end_index = middle_index - 1;
        } else {
            start_index = middle_index + 1;
        }
    }
    -1
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
