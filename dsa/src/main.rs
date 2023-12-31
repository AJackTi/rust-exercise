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

fn main() {
    // println!("{:?}", fact(5));

    // println!("{}", fib(15));

    // println!("{:?}", palindrom(&vec![1, 2, 3, 2, 1], 0, 4));

    // println!("{}", toh(0));
    // println!("{}", toh(1));
    // println!("{}", toh(2));
    // println!("{}", toh(3));
    // println!("{}", toh(4));

    let mut vec = vec![1, 2, 3, 4, 5];
    triangle(&mut vec, 5);
}
