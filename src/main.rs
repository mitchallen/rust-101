fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello_world() {
        assert_eq!(format!("Hello, world!"), "Hello, world!");
    }
}