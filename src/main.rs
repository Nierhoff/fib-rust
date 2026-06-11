fn fib(n: u32) -> num::BigUint {
    let mut a: num::BigUint = num::BigUint::ZERO;
    let mut b: num::BigUint = num::BigUint::from(1u32);
    let mut c: num::BigUint;
    for _ in 2..n {
        c = &a + &b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    let n = 100_000;
    let start = std::time::Instant::now();
    let result = fib(n);
    println!("Fibonacci of {} is: {}", n, result);
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
