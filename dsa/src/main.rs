fn fact(num: i32) -> i32 {
    if num > 1 {
        return num * fact(num - 1);
    }
    return 1;
}

fn fib(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    let n1 = fib(num - 1);
    let n2 = fib(num - 2);

    return n1 + n2;
}

fn palindrom(array: &Vec<i32>, start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }

    if array[start] != array[end] {
        return false;
    }

    return palindrom(array, start + 1, end - 1);
}

// Tower of Hanoi
fn toh(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    return toh(n - 1) + 1 + toh(n - 1);
}

fn triangle(arr: &mut Vec<i8>, size: usize) {
    if size < 1 {
        return;
    }

    let mut tmp: Vec<i8> = Vec::new();

    for i in 0..size - 1 {
        let x = arr[i] + arr[i + 1];
        tmp.push(x);
    }

    triangle(&mut tmp, size - 1);

    println!("{:?}", arr);
}

fn selection_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        let mut smallest = i;
        for j in i + 1..array.len() {
            if array[j] < array[smallest] {
                smallest = j;
            }
        }
        array.swap(i, smallest);
    }
    array.to_vec();
}

fn bubble_sort(array: &mut Vec<i32>) {
    let mut sorted = false;
    for _ in 1..=array.len() - 1 {
        for j in 0..=array.len() - 2 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }

    array.to_vec();
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]); // left half
        merge_sort(&mut arr[mid..]); // right half
        merge(arr, mid);
    }

    arr.to_vec();
}

fn merge(arr: &mut [i32], mid: usize) {
    let left = arr[..mid].to_vec(); // left half to middle
    let right = arr[mid..].to_vec(); // middle to right half

    let mut l = 0;
    let mut r = 0;

    for val in arr {
        if r == right.len() || (l < left.len() && left[l] < right[r]) {
            *val = left[l];
            l += 1;
        } else {
            *val = right[r];
            r += 1;
        }
    }
}

fn main() {
    // println!("{:?}", fact(5));

    // println!("{}", fib(15));

    // println!("{:?}", palindrom(&vec![1, 2, 3, 2, 1], 0, 4));

    // println!("{}", toh(0));
    // println!("{}", toh(1));
    // println!("{}", toh(2));
    // println!("{}", toh(3));
    // println!("{}", toh(4));

    // let mut vec = vec![1, 2, 3, 4, 5];
    // triangle(&mut vec, 5);

    // let mut vec = vec![4, 3, 1, 2];
    // println!("Before sorting: {:?}", vec);
    // selection_sort(&mut vec);
    // println!("After sorting: {:?}", vec);
    // bubble_sort(&mut vec);
    // println!("After sorting: {:?}", vec);

    let mut vec = vec![4, 7, 3, 5, 1, 2];
    merge_sort(&mut vec);
    println!("{:?}", vec);
}
