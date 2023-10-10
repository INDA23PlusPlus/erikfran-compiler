fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut n = 0;
    while (n < 10) {
    println!("{}", b);
    let mut c = (a + b);
    a = b;
    b = c;
    n = (1 + n);
}
}
