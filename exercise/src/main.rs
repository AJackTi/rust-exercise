fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    a.sort_by(|x, y| { x.partial_cmp(y).unwrap() });

    let n_elements = a.len();
    let middle = n_elements / 2;
    let a_is_even = n_elements % 2 == 0;

    let med = if a_is_even { (a[middle] + a[middle - 1]) / 2.0 } else { a[middle] };

    Some(med)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0, 6.0]);

    dbg!(answer);
}
