#[cfg(test)]
mod tests {
    #[test]
    fn format() {
        let s = format!("Hello {0}. You are {1} years old.", "John", 39);
        assert_eq!(s, "Hello John. You are 39 years old.");
    }
}
