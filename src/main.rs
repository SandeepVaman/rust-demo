use std::io::{self, Write};

fn main() {
    println!("Welcome to Rust Demo App!");
    println!("This is a simple Rust application for testing Tekton pipelines.");

    let result = calculate_fibonacci(10);
    println!("Fibonacci(10) = {}", result);

    let greeting = greet("Tekton");
    println!("{}", greeting);
}

/// Calculate the nth Fibonacci number
pub fn calculate_fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

/// Generate a greeting message
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to the Rust Demo App.", name)
}

/// Add two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Check if a number is even
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(!is_even(1));
        assert!(!is_even(-1));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(calculate_fibonacci(0), 0);
        assert_eq!(calculate_fibonacci(1), 1);
        assert_eq!(calculate_fibonacci(2), 1);
        assert_eq!(calculate_fibonacci(5), 5);
        assert_eq!(calculate_fibonacci(10), 55);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World! Welcome to the Rust Demo App.");
        assert_eq!(greet("Tekton"), "Hello, Tekton! Welcome to the Rust Demo App.");
    }
}
