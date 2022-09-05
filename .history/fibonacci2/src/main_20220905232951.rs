fn main() {
    println!("The element in the position {} in fibonacci series is {}",2, fibonacci(2));
    println!("The element in the position {} in fibonacci series is {}",4, fibonacci(4));
    println!("The element in the position {} in fibonacci series is {}",22, fibonacci(22));
    println!("The element in the position {} in fibonacci series is {}", 20, fibonacci(20));
}

pub fn fibonacci(n: u32) -> u32 {
    if 3 > n { 
        1 
    }
    fibonacci(n-1) + fibonacci(n-2)
}
