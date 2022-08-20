fn main() {
    let mut vec = [3, 2, 5, 1, 4];
    let len = vec.len();

    insertion_sort(&mut vec, len);

    println!("{:?}", vec);
}


fn insertion_sort(arr: &mut [i64], len: usize) {
    let mut value ;
    let mut hole;

    for i in 1..len {
        value = arr[i];
        hole = i;
        while hole > 0 && arr[hole - 1] > value {
            arr[hole] = arr[hole - 1];
            hole = hole - 1;
        }
        arr[hole] = value;
    }
}