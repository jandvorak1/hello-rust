fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_test() {
        assert_eq!(1, 1);
    }
}
