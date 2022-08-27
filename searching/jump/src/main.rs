use std::cmp::min;

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let searched_num = 15;

    let found_index = jump_search(&array, searched_num, array.len() as i32);

    println!("The Element has been found at the index: {}", found_index)
}

fn jump_search(array: &[i32], searched_num: i32, len: i32) -> i32 {
    // Get the number of jumping step.
    let mut jump_step = f64::sqrt(len as f64) as i32;

    // get the current block
    let mut prev = 0;
    while array[min(jump_step, len) as usize - 1] < searched_num {
        prev = jump_step;
        jump_step += f64::sqrt(len as f64) as i32;

        if prev >= len {
            return -1;
        }
    }

    // Impl linear searching.
    while array[prev as usize] < searched_num {
        prev += 1;
        if prev == min(jump_step, len) {
            return -1;
        }
    }

    if array[prev as usize] == searched_num {
        return prev;
    }
    -1
}

