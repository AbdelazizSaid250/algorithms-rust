
// 3 2 5 1 4
// 1 2 5 3 4
// 1 2 3 5 4
// 1 2 3 4 5

fn main() {
    // let mut vec = vec![3, 2, 5, 1, 4];
    let mut vec = [3, 2, 5, 1, 4];
    let len = vec.len();

    selection_sort(&mut vec, len);

    println!("{:?}", vec);
}

fn selection_sort(arr: &mut [i64], len: usize) {
    let mut min;
    let mut min_index = 0;

    for i in 0..len {
        min = arr[i];
        for j in i..len {
            if arr[j] < min {
                min = arr[j];
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn selection_sort2(arr: &mut [i64], len: usize) {
    let mut min_index = 0;

    for i in 0..len {
        min_index = i;
        for j in i..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn selection_sort3(arr: &mut [i64]) {
    let len = arr.len();

    for i in 0..len {
        let mut temp = i;
        for j in (i + 1)..len {
            if arr[temp] > arr[j] {
                temp = j;
            }
        }
        arr.swap(i, temp);
    }
}
