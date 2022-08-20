// 3, 2, 5, 1, 4
// 2, 3, 5, 1, 4
// 2, 3, 5, 1, 4
// 2, 3, 1, 5, 4
// 2, 3, 1, 4, 5

//...

// 1, 2, 3, 4, 5

fn main() {
    let mut vec = [3, 2, 5, 1, 4];
    let len = vec.len();

    bubble_sort(&mut vec, len);

    println!("{:?}", vec);
}


fn bubble_sort(arr: &mut [i64], len: usize) {
    let mut flag = 0;

    for i in 0..len - 1 {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                flag = 1;
            }
        }
        if flag == 0 {
            break;
        }
    }
}