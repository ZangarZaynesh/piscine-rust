fn main() {
    println!("Hello, world!");
}

pub fn fibonacci(n: u32) -> u32 {
    if 3 < n { 1 } else {fibonacci(n-1) + fibonacci(n-2)}
}
