use rust_sysid::add;
use faer::{Mat, linalg::matmul::matmul, Accum, Par};

fn main() {
    println!("Hello from rust_sysid!");
    println!("2 + 3 = {}", add(2, 3));

    let a = Mat::<f64>::from_fn(2, 3, |i, j| (i + j) as f64);
    let b = Mat::<f64>::from_fn(3, 2, |i, j| (i * j) as f64);
    let mut c = Mat::<f64>::zeros(2, 2);

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    matmul(&mut c, Accum::Replace, &a, &b, 1.0, Par::Seq);

    println!("c = {:?}", c);

}
