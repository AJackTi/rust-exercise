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

fn main() {
    println!("{:?}", fact(5));

    println!("{}", fib(15));
}
