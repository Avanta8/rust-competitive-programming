fn group_slice<T: PartialEq>(v: &[T]) -> Vec<(usize, &T)> {
    if v.is_empty() {
        return [].into();
    }

    let mut out = vec![];

    let mut prev = &v[0];
    let mut count = 1;
    for item in v.iter().skip(1) {
        if item == prev {
            count += 1;
        } else {
            out.push((count, prev));
            prev = item;
            count = 1;
        }
    }
    out.push((count, prev));
    out
}

fn main() {
    let v = [
        1, 1, 1, 2, 2, 1, 1, 2, 1, 3, 3, 3, 3, 3, 3, 3, 2, 3, 2, 2, 2, 1, 1, 1,
    ];
    let groups = group_slice(&v);
    println!("{:?}", groups);

    let v: [i32; 0] = [];
    let groups = group_slice(&v);
    println!("{:?}", groups);

    let v = [1];
    let groups = group_slice(&v);
    println!("{:?}", groups);
}
