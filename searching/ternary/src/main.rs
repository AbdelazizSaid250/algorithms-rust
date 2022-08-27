fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let searched_num = 3;

    let found_index = ternary_search_recursion(&array, searched_num, 0, array.len() as i32 - 1);

    println!("The Element has been found at the index: {}", found_index)
}

fn ternary_search_recursion(array: &[i32], searched_num: i32, start_index: i32, end_index: i32) -> i32 {

    if start_index > end_index {
        return -1;
    }

    let first_index = start_index + (end_index - start_index) / 3;
    let second_index = end_index - (end_index - start_index) / 3;

    if searched_num == array[first_index as usize] {
        return first_index;
    } else if searched_num == array[second_index as usize] {
        return second_index;
    } else if searched_num < array[first_index as usize] {
        ternary_search_recursion(array, searched_num, start_index, first_index - 1)
    } else if searched_num < array[second_index as usize] {
        ternary_search_recursion(array, searched_num, first_index, second_index - 1)
    } else {
        ternary_search_recursion(array, searched_num, second_index, end_index)
    }
}

/*fn ternary_search_iterative(array: &[i32], searched_num: i32, mut start_index: i32, mut end_index: i32) -> i32 {
    while start_index < end_index {
        let first_index = start_index + (end_index - start_index) / 3;
        let second_index = end_index - (end_index - start_index) / 3;

        if searched_num == array[first_index as usize] {
            println!("I am here");
            return first_index;
        } else if searched_num == array[second_index as usize] {
            println!("yes");
            return second_index;
        } else if searched_num < first_index {
            println!("no");
            end_index = first_index - 1;
        } else if searched_num > second_index {
            println!("done");
            start_index = second_index;
        } else {
            println!("else");
            start_index = first_index;
            end_index = second_index - 1;
        }
    }
    -1
}*/