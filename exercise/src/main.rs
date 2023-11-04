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

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     // let mut tmp = Vec::new();

//     // for elem in a {
//     //     if tmp.contains(&elem) {
//     //         continue;
//     //     }
//     //     tmp.push(elem);
//     // }
//     // tmp
// }

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     a.sort_unstable();
//     a.dedup();
//     a
// }

// fn unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
//     a.sort_by(|x: &T, y: &T| x.cmp(y));
//     a.dedup();
//     a
// }

fn main() {
    // let answer = median(vec![1.0, 2.0, 5.0, 6.0]);
    // dbg!(answer);

    // let answer = unique(vec![1, 1, 2, 3, 4, 4, 5]);
    // dbg!(answer);

    // let answer = unique(vec![1, 2, 3, 4, 4, 5, 5]);
    // dbg!(answer);
}
