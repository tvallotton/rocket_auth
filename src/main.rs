
use rand::random;
fn rand_string(size: usize) -> String {
    let dissallowed = ['\\', '"', '{', '}', '(',')', '`', '\''];
    (0..)
        .map(|_| random::<u8>())
        .filter(|n| 31 < *n && *n < 126)
        .map(|n| char::from(n))
        .filter(|c| !dissallowed.contains(c))
        .take(size)
        .collect()
}

fn main() {
    let x = rand_string(100000);
    println!("{:?}", x);
    println!("{}", x.len());
}
