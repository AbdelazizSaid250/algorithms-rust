fn main() {
    let array = [2, 5, 3, 1, 4];
    let searched_num = 10;

    let found_index = linear_search(&array, searched_num);

    if found_index == -1 {
        eprintln!("This Element doesn't found in the array");
    } else {
        println!("The Element has been found at the index: {}", found_index)
    }

}

fn linear_search(array: &[i32], searched_num: i32) -> i32 {
    for i in 0..array.len() {
        if array[i] == searched_num {
            return i as i32;
        }
    }
    return -1;
}