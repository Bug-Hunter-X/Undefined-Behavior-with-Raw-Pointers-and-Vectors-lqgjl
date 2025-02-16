fn main() {
    let mut v = vec![1, 2, 3];
    let value = 10;
    v[0] = value; // Or any other safe way
    println!("v: {:?}", v);
}