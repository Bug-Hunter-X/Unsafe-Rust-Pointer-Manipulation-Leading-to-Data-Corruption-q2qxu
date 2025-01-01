fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Use safe indexing for modification
    println!("{:?}", v);
}