fn main() {
    let mut vec = [3, 2, 5, 1, 4];
    let len = vec.len();

    insertion_sort(&mut vec, len);

    println!("{:?}", vec);
}


fn insertion_sort(arr: &mut [i64], len: usize) {
    let mut value ;
    let mut index;

    for i in 1..len {
        value = arr[i];
        index = i;
        while index > 0 && arr[index - 1] > value {
            arr[index] = arr[index - 1];
            index = index - 1;
        }
        arr[index] = value;
    }
}